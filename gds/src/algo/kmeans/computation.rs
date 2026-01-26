//! K-Means computation runtime.
//!
//! Parity goals (conceptual, not line-by-line Java translation):
//! - Supports UNIFORM and KMEANSPP initialization.
//! - Uses swaps-based iteration stopping (Java `KmeansIterationStopper`).
//! - Supports seeded centroids (fully specified).
//! - Optional per-node silhouette (centroid-based approximation).

use super::spec::{KMeansConfig, KMeansResult, KMeansSamplerType};
use crate::concurrency::{install_with_concurrency, Concurrency};
use rand::prelude::*;
use rand::seq::index::sample;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

const UNASSIGNED: u64 = u64::MAX;

#[derive(Debug, Default, Clone)]
pub struct KMeansComputationRuntime;

impl KMeansComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn compute(&mut self, points: &[Vec<f64>], config: &KMeansConfig) -> KMeansResult {
        let n = points.len();
        if n == 0 {
            return KMeansResult {
                communities: Vec::new(),
                distance_from_center: Vec::new(),
                centers: Vec::new(),
                average_distance_to_centroid: 0.0,
                silhouette: config.compute_silhouette.then_some(Vec::new()),
                average_silhouette: 0.0,
                ran_iterations: 0,
                restarts: 0,
                node_count: 0,
                execution_time: Duration::default(),
            };
        }

        let dims = points[0].len();
        let k = config.k.max(1).min(n);
        let max_iterations = config.max_iterations.max(1);
        let concurrency = Concurrency::from_usize(config.concurrency.max(1));

        // Seeded centroids behave like a single "restart".
        if !config.seed_centroids.is_empty() {
            let mut centers = normalize_seeded_centers(&config.seed_centroids, k, dims);
            let (communities, distances, ran_iterations) = kmeans_swaps_loop(
                points,
                &mut centers,
                max_iterations,
                config.delta_threshold,
                concurrency,
            );
            let avg_dist = average(&distances);
            let (silhouette, avg_sil) = if config.compute_silhouette {
                let s = silhouette_centroid(points, &centers, &communities);
                let a = average(&s);
                (Some(s), a)
            } else {
                (None, 0.0)
            };

            return KMeansResult {
                communities,
                distance_from_center: distances,
                centers,
                average_distance_to_centroid: avg_dist,
                silhouette,
                average_silhouette: avg_sil,
                ran_iterations,
                restarts: 1,
                node_count: n,
                execution_time: Duration::default(),
            };
        }

        let restarts = config.number_of_restarts.max(1);
        let mut best: Option<(Vec<u64>, Vec<f64>, Vec<Vec<f64>>, f64, u32)> = None;

        for restart in 0..restarts {
            let seed = config
                .random_seed
                .unwrap_or(0xC0FFEE)
                .wrapping_add(restart as u64);
            let mut rng = StdRng::seed_from_u64(seed);

            let mut centers = match config.sampler_type {
                KMeansSamplerType::Uniform => sample_uniform(points, k, &mut rng),
                KMeansSamplerType::KmeansPlusPlus => sample_kmeanspp(points, k, &mut rng),
            };
            if centers.is_empty() {
                continue;
            }
            ensure_center_dims(&mut centers, dims);

            let (communities, distances, ran_iterations) = kmeans_swaps_loop(
                points,
                &mut centers,
                max_iterations,
                config.delta_threshold,
                concurrency,
            );
            let avg_dist = average(&distances);

            match &best {
                Some((_bc, _bd, _bcenters, best_avg, _bi)) if avg_dist >= *best_avg => {}
                _ => {
                    best = Some((communities, distances, centers, avg_dist, ran_iterations));
                }
            }
        }

        let (communities, distances, centers, avg_dist, ran_iterations) =
            best.unwrap_or_else(|| {
                let mut centers = Vec::with_capacity(k);
                for i in 0..k {
                    centers.push(points[i].clone());
                }
                ensure_center_dims(&mut centers, dims);
                let (communities, distances, ran_iterations) = kmeans_swaps_loop(
                    points,
                    &mut centers,
                    max_iterations,
                    config.delta_threshold,
                    concurrency,
                );
                let avg_dist = average(&distances);
                (communities, distances, centers, avg_dist, ran_iterations)
            });

        let (silhouette, avg_sil) = if config.compute_silhouette {
            let s = silhouette_centroid(points, &centers, &communities);
            let a = average(&s);
            (Some(s), a)
        } else {
            (None, 0.0)
        };

        KMeansResult {
            communities,
            distance_from_center: distances,
            centers,
            average_distance_to_centroid: avg_dist,
            silhouette,
            average_silhouette: avg_sil,
            ran_iterations,
            restarts,
            node_count: n,
            execution_time: Duration::default(),
        }
    }
}

fn kmeans_swaps_loop(
    points: &[Vec<f64>],
    centers: &mut Vec<Vec<f64>>,
    max_iterations: u32,
    delta_threshold: f64,
    concurrency: Concurrency,
) -> (Vec<u64>, Vec<f64>, u32) {
    let n = points.len();
    let k = centers.len().max(1);
    let dims = centers[0].len();

    let swaps_bound: u64 = ((n as f64) * delta_threshold) as u64;

    let mut communities = vec![UNASSIGNED; n];
    let mut distances = vec![0.0f64; n];
    let mut ran_iterations = 0u32;

    for iteration in 1..=max_iterations {
        let swaps = AtomicU64::new(0);
        let mut next_communities = vec![0u64; n];
        let mut next_distances = vec![0.0f64; n];

        install_with_concurrency(concurrency, || {
            next_communities
                .par_iter_mut()
                .zip(next_distances.par_iter_mut())
                .enumerate()
                .for_each(|(i, (comm_out, dist_out))| {
                    let (best_c, best_d2) = closest_centroid(&points[i], centers);
                    let prev = communities[i];
                    if prev != best_c {
                        swaps.fetch_add(1, Ordering::Relaxed);
                    }
                    *comm_out = best_c;
                    *dist_out = best_d2.sqrt();
                });
        });

        communities = next_communities;
        distances = next_distances;

        recompute_centroids(points, &communities, centers, k, dims, concurrency);

        ran_iterations = iteration;
        let swaps = swaps.load(Ordering::Relaxed);
        if iteration == max_iterations {
            break;
        }
        if iteration > 1 && swaps <= swaps_bound {
            break;
        }
    }

    (communities, distances, ran_iterations)
}

fn recompute_centroids(
    points: &[Vec<f64>],
    communities: &[u64],
    centers: &mut [Vec<f64>],
    k: usize,
    dims: usize,
    concurrency: Concurrency,
) {
    let (sums, counts) = install_with_concurrency(concurrency, || {
        points
            .par_iter()
            .zip(communities.par_iter())
            .fold(
                || (vec![0.0f64; k * dims], vec![0usize; k]),
                |(mut sums, mut counts), (point, &cid)| {
                    let c = (cid as usize).min(k - 1);
                    counts[c] += 1;
                    let base = c * dims;
                    for d in 0..dims {
                        sums[base + d] += point[d];
                    }
                    (sums, counts)
                },
            )
            .reduce(
                || (vec![0.0f64; k * dims], vec![0usize; k]),
                |(mut a_sums, mut a_counts), (b_sums, b_counts)| {
                    for i in 0..a_sums.len() {
                        a_sums[i] += b_sums[i];
                    }
                    for i in 0..k {
                        a_counts[i] += b_counts[i];
                    }
                    (a_sums, a_counts)
                },
            )
    });

    // Java parity: clusters with 0 assigned nodes keep their previous centroid.
    for c in 0..k {
        let count = counts[c];
        if count == 0 {
            continue;
        }
        let base = c * dims;
        for d in 0..dims {
            centers[c][d] = sums[base + d] / (count as f64);
        }
    }
}

fn closest_centroid(point: &[f64], centers: &[Vec<f64>]) -> (u64, f64) {
    let mut best_c = 0usize;
    let mut best_d2 = f64::INFINITY;
    for (ci, c) in centers.iter().enumerate() {
        let d2 = squared_euclidean(point, c);
        if d2 < best_d2 || (d2 == best_d2 && ci < best_c) {
            best_d2 = d2;
            best_c = ci;
        }
    }
    (best_c as u64, best_d2)
}

fn sample_uniform(points: &[Vec<f64>], k: usize, rng: &mut impl Rng) -> Vec<Vec<f64>> {
    let n = points.len();
    if k >= n {
        return points.to_vec();
    }

    let indices = sample(rng, n, k).into_vec();
    indices.into_iter().map(|i| points[i].clone()).collect()
}

fn sample_kmeanspp(points: &[Vec<f64>], k: usize, rng: &mut impl Rng) -> Vec<Vec<f64>> {
    let n = points.len();
    if k >= n {
        return points.to_vec();
    }

    let mut centers: Vec<Vec<f64>> = Vec::with_capacity(k);
    centers.push(points[rng.gen_range(0..n)].clone());

    let mut min_d2: Vec<f64> = vec![0.0; n];

    while centers.len() < k {
        for (i, p) in points.iter().enumerate() {
            min_d2[i] = centers
                .iter()
                .map(|c| squared_euclidean(p, c))
                .fold(f64::INFINITY, f64::min);
        }

        let total: f64 = min_d2.iter().sum();
        if !total.is_finite() || total <= 0.0 {
            let remaining = k - centers.len();
            centers.extend(sample_uniform(points, remaining, rng));
            break;
        }

        let mut threshold = rng.gen::<f64>() * total;
        let mut chosen = 0usize;
        for (i, w) in min_d2.iter().enumerate() {
            threshold -= *w;
            if threshold <= 0.0 {
                chosen = i;
                break;
            }
        }

        centers.push(points[chosen].clone());
    }

    centers
}

fn normalize_seeded_centers(seeded: &[Vec<f64>], k: usize, dims: usize) -> Vec<Vec<f64>> {
    let mut centers = Vec::with_capacity(k);
    for i in 0..k {
        if let Some(c) = seeded.get(i) {
            centers.push(c.clone());
        } else {
            centers.push(vec![0.0; dims]);
        }
    }
    ensure_center_dims(&mut centers, dims);
    centers
}

fn ensure_center_dims(centers: &mut [Vec<f64>], dims: usize) {
    for c in centers.iter_mut() {
        if c.len() < dims {
            c.resize(dims, 0.0);
        } else if c.len() > dims {
            c.truncate(dims);
        }
    }
}

fn squared_euclidean(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| {
            let d = x - y;
            d * d
        })
        .sum()
}

fn average(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

fn silhouette_centroid(points: &[Vec<f64>], centers: &[Vec<f64>], communities: &[u64]) -> Vec<f64> {
    let n = points.len();
    if n == 0 {
        return Vec::new();
    }
    if centers.len() <= 1 {
        return vec![0.0; n];
    }

    let mut out = vec![0.0f64; n];
    for i in 0..n {
        let ci = communities[i] as usize;
        let a = squared_euclidean(&points[i], &centers[ci]).sqrt();

        let mut b = f64::INFINITY;
        for (cj, c) in centers.iter().enumerate() {
            if cj == ci {
                continue;
            }
            let d = squared_euclidean(&points[i], c).sqrt();
            if d < b {
                b = d;
            }
        }

        let denom = a.max(b);
        out[i] = if denom > 0.0 { (b - a) / denom } else { 0.0 };
    }

    out
}
