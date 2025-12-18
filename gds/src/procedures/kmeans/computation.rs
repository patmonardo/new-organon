use super::spec::{KMeansConfig, KMeansResult, KMeansSamplerType};
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

pub struct KMeansComputationRuntime;

impl KMeansComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn compute(&mut self, points: &[Vec<f64>], config: &KMeansConfig) -> KMeansResult {
        let node_count = points.len();
        if node_count == 0 {
            return KMeansResult {
                communities: Vec::new(),
                distance_from_center: Vec::new(),
                centers: Vec::new(),
                average_distance_to_centroid: 0.0,
                silhouette: config.compute_silhouette.then_some(Vec::new()),
                average_silhouette: 0.0,
                ran_iterations: 0,
                restarts: 0,
            };
        }

        let dims = points[0].len();
        let mut best: Option<KMeansResult> = None;

        for restart in 0..config.number_of_restarts.max(1) {
            let mut rng = seeded_rng(config.random_seed, restart);

            let mut centers = if !config.seed_centroids.is_empty() {
                config.seed_centroids.clone()
            } else {
                initialize_centroids(points, config.k, dims, config.sampler_type, &mut rng)
            };

            let mut communities = vec![u64::MAX; node_count];
            let mut distance_from_center = vec![0.0; node_count];

            let swaps_bound: u64 = ((node_count as f64) * config.delta_threshold).floor() as u64;
            let mut ran_iterations = 0;

            for iter in 0..config.max_iterations.max(1) {
                ran_iterations = iter + 1;

                let mut swaps: u64 = 0;
                let mut counts = vec![0u64; config.k];
                let mut sums = vec![vec![0.0; dims]; config.k];

                for (i, point) in points.iter().enumerate() {
                    let (closest, dist) = closest_center(point, &centers);
                    if communities[i] != closest as u64 {
                        swaps += 1;
                        communities[i] = closest as u64;
                    }
                    distance_from_center[i] = dist;
                    counts[closest] += 1;
                    for d in 0..dims {
                        sums[closest][d] += point[d];
                    }
                }

                // Update centers; handle empty clusters by re-seeding to a random point.
                for c in 0..config.k {
                    if counts[c] == 0 {
                        let idx = rng.gen_range(0..node_count);
                        centers[c] = points[idx].clone();
                    } else {
                        let denom = counts[c] as f64;
                        for d in 0..dims {
                            centers[c][d] = sums[c][d] / denom;
                        }
                    }
                }

                // Stop rule (matches Java intent): after first iteration, stop when swaps <= bound.
                if iter >= 1 && swaps <= swaps_bound {
                    break;
                }
            }

            let avg_distance = if node_count == 0 {
                0.0
            } else {
                distance_from_center.iter().sum::<f64>() / (node_count as f64)
            };

            let (silhouette, avg_silhouette) = if config.compute_silhouette {
                let sil = compute_silhouette(points, &communities, config.k);
                let avg = if sil.is_empty() {
                    0.0
                } else {
                    sil.iter().sum::<f64>() / (sil.len() as f64)
                };
                (Some(sil), avg)
            } else {
                (None, 0.0)
            };

            let result = KMeansResult {
                communities,
                distance_from_center,
                centers,
                average_distance_to_centroid: avg_distance,
                silhouette,
                average_silhouette: avg_silhouette,
                ran_iterations,
                restarts: restart + 1,
            };

            best = match best {
                None => Some(result),
                Some(prev) => {
                    // Prefer higher silhouette when enabled, else lower distance.
                    let better = if config.compute_silhouette {
                        result.average_silhouette > prev.average_silhouette
                    } else {
                        result.average_distance_to_centroid < prev.average_distance_to_centroid
                    };
                    Some(if better { result } else { prev })
                }
            };
        }

        best.expect("node_count > 0 implies best is set")
    }
}

impl Default for KMeansComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}

fn seeded_rng(base_seed: Option<u64>, restart: u32) -> ChaCha8Rng {
    let seed =
        base_seed.unwrap_or(0xC0FFEE_u64) ^ ((restart as u64).wrapping_mul(0x9E3779B97F4A7C15));
    ChaCha8Rng::seed_from_u64(seed)
}

fn initialize_centroids(
    points: &[Vec<f64>],
    k: usize,
    dims: usize,
    sampler: KMeansSamplerType,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<f64>> {
    let node_count = points.len();
    if k == 0 {
        return Vec::new();
    }

    match sampler {
        KMeansSamplerType::Uniform => {
            let mut sampled = HashSet::new();
            let mut centers = Vec::with_capacity(k);
            while centers.len() < k {
                let idx = rng.gen_range(0..node_count);
                if sampled.insert(idx) {
                    let mut centroid = vec![0.0; dims];
                    centroid.clone_from_slice(&points[idx]);
                    centers.push(points[idx].clone());
                }
            }
            centers
        }
        KMeansSamplerType::KmeansPlusPlus => {
            let mut centers: Vec<Vec<f64>> = Vec::with_capacity(k);
            let first = rng.gen_range(0..node_count);
            centers.push(points[first].clone());

            while centers.len() < k {
                let mut weights: Vec<f64> = Vec::with_capacity(node_count);
                for p in points {
                    let (closest, _dist) = closest_center(p, &centers);
                    let dist2 = distance_squared(p, &centers[closest]);
                    weights.push(dist2.max(0.0));
                }

                // If all weights are zero (identical points), fall back to uniform.
                if weights.iter().all(|&w| w == 0.0) {
                    let idx = rng.gen_range(0..node_count);
                    centers.push(points[idx].clone());
                    continue;
                }

                let dist = WeightedIndex::new(&weights).expect("non-negative weights");
                let idx = dist.sample(rng);
                centers.push(points[idx].clone());
            }
            centers
        }
    }
}

fn closest_center(point: &[f64], centers: &[Vec<f64>]) -> (usize, f64) {
    let mut best_idx = 0usize;
    let mut best_dist2 = f64::INFINITY;
    for (i, c) in centers.iter().enumerate() {
        let d2 = distance_squared(point, c);
        if d2 < best_dist2 {
            best_dist2 = d2;
            best_idx = i;
        }
    }
    (best_idx, best_dist2.sqrt())
}

fn distance_squared(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| {
            let d = x - y;
            d * d
        })
        .sum()
}

fn compute_silhouette(points: &[Vec<f64>], communities: &[u64], k: usize) -> Vec<f64> {
    let n = points.len();
    if n == 0 || k == 0 {
        return Vec::new();
    }

    let mut members: Vec<Vec<usize>> = vec![Vec::new(); k];
    for (i, &c) in communities.iter().enumerate() {
        if (c as usize) < k {
            members[c as usize].push(i);
        }
    }

    let mut out = vec![0.0f64; n];
    for i in 0..n {
        let ci = communities[i] as usize;
        if ci >= k {
            continue;
        }

        // a(i): mean distance to points in same cluster
        let same = &members[ci];
        let a = if same.len() <= 1 {
            0.0
        } else {
            let mut sum = 0.0;
            for &j in same {
                if j == i {
                    continue;
                }
                sum += distance(points[i].as_slice(), points[j].as_slice());
            }
            sum / ((same.len() - 1) as f64)
        };

        // b(i): minimum mean distance to points in other clusters
        let mut b = f64::INFINITY;
        for c in 0..k {
            if c == ci || members[c].is_empty() {
                continue;
            }
            let mut sum = 0.0;
            for &j in &members[c] {
                sum += distance(points[i].as_slice(), points[j].as_slice());
            }
            let mean = sum / (members[c].len() as f64);
            if mean < b {
                b = mean;
            }
        }

        if !b.is_finite() {
            out[i] = 0.0;
            continue;
        }

        let denom = a.max(b);
        out[i] = if denom == 0.0 { 0.0 } else { (b - a) / denom };
    }

    out
}

fn distance(a: &[f64], b: &[f64]) -> f64 {
    distance_squared(a, b).sqrt()
}
