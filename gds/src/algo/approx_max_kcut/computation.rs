//! ApproxMaxKCut computation runtime.

use super::spec::{ApproxMaxKCutConfig, ApproxMaxKCutResult};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ApproxMaxKCutComputationRuntime {
    config: ApproxMaxKCutConfig,
}

impl ApproxMaxKCutComputationRuntime {
    pub fn new(config: ApproxMaxKCutConfig) -> Self {
        Self { config }
    }

    pub fn compute<F>(&self, node_count: usize, get_neighbors: F) -> ApproxMaxKCutResult
    where
        F: Fn(usize) -> Vec<(usize, f64)>,
    {
        if node_count == 0 {
            return ApproxMaxKCutResult {
                communities: Vec::new(),
                cut_cost: 0.0,
                k: self.config.k,
                node_count,
                execution_time: Duration::default(),
            };
        }

        let k = self.config.k.max(2) as usize;
        let iterations = self.config.iterations.max(1);

        let mut adjacency: Vec<Vec<(usize, f64)>> = Vec::with_capacity(node_count);
        for node in 0..node_count {
            let mut neigh = get_neighbors(node);
            neigh.retain(|(t, _w)| *t < node_count);
            adjacency.push(neigh);
        }

        let mut reverse: Vec<Vec<(usize, f64)>> = vec![Vec::new(); node_count];
        for (src, edges) in adjacency.iter().enumerate() {
            for &(dst, w) in edges {
                reverse[dst].push((src, w));
            }
        }

        let mut min_sizes = self.config.min_community_sizes.clone();
        min_sizes.resize(k, 0);
        let total_min: usize = min_sizes.iter().sum();
        if total_min > node_count {
            min_sizes.fill(0);
        }

        let mut best_communities: Vec<u8> = vec![0; node_count];
        let mut best_cost: f64 = if self.config.minimize {
            f64::INFINITY
        } else {
            f64::NEG_INFINITY
        };

        for iter in 0..iterations {
            let mut rng = StdRng::seed_from_u64(self.config.random_seed.wrapping_add(iter as u64));
            let mut communities = random_assignment(node_count, k, &mut rng);
            let mut sizes = compute_sizes(k, &communities);
            enforce_min_sizes(&mut communities, &mut sizes, &min_sizes, &mut rng);

            local_search(
                &adjacency,
                &reverse,
                &min_sizes,
                self.config.minimize,
                &mut communities,
                &mut sizes,
            );

            let cost = cut_cost(&adjacency, &communities);
            let is_better = if self.config.minimize {
                cost < best_cost
            } else {
                cost > best_cost
            };

            if is_better {
                best_cost = cost;
                best_communities.clone_from(&communities);
            }
        }

        ApproxMaxKCutResult {
            communities: best_communities,
            cut_cost: best_cost.max(0.0),
            k: self.config.k,
            node_count,
            execution_time: Duration::default(),
        }
    }
}

fn random_assignment(node_count: usize, k: usize, rng: &mut StdRng) -> Vec<u8> {
    (0..node_count).map(|_| rng.gen_range(0..k) as u8).collect()
}

fn compute_sizes(k: usize, communities: &[u8]) -> Vec<usize> {
    let mut sizes = vec![0; k];
    for &c in communities {
        let idx = c as usize;
        if idx < k {
            sizes[idx] += 1;
        }
    }
    sizes
}

fn enforce_min_sizes(
    communities: &mut [u8],
    sizes: &mut [usize],
    min_sizes: &[usize],
    rng: &mut StdRng,
) {
    let k = sizes.len();

    for target in 0..k {
        while sizes[target] < min_sizes[target] {
            let mut source = None;
            let mut attempts = 0;
            while source.is_none() && attempts < k {
                let candidate = rng.gen_range(0..k);
                if candidate != target && sizes[candidate] > min_sizes[candidate] {
                    source = Some(candidate);
                }
                attempts += 1;
            }

            let Some(source) = source else {
                break;
            };

            let mut moved = false;
            for idx in 0..communities.len() {
                if communities[idx] as usize == source {
                    communities[idx] = target as u8;
                    sizes[source] -= 1;
                    sizes[target] += 1;
                    moved = true;
                    break;
                }
            }

            if !moved {
                break;
            }
        }
    }
}

fn local_search(
    adjacency: &[Vec<(usize, f64)>],
    reverse: &[Vec<(usize, f64)>],
    min_sizes: &[usize],
    minimize: bool,
    communities: &mut [u8],
    sizes: &mut [usize],
) {
    let node_count = communities.len();
    let k = sizes.len();
    let max_passes = (node_count.max(1)).min(10_000);

    for _pass in 0..max_passes {
        let mut did_change = false;

        for node in 0..node_count {
            let from = communities[node] as usize;
            if from >= k {
                continue;
            }

            if sizes[from] <= min_sizes[from] {
                continue;
            }

            let mut best_to = from;
            let mut best_delta = 0.0;

            for to in 0..k {
                if to == from {
                    continue;
                }
                let delta = move_delta(adjacency, reverse, communities, node, from, to);
                let improves = if minimize {
                    delta < best_delta
                } else {
                    delta > best_delta
                };
                if improves {
                    best_delta = delta;
                    best_to = to;
                }
            }

            let should_move = if minimize {
                best_delta < -1e-12
            } else {
                best_delta > 1e-12
            };
            if best_to != from && should_move {
                communities[node] = best_to as u8;
                sizes[from] -= 1;
                sizes[best_to] += 1;
                did_change = true;
            }
        }

        if !did_change {
            break;
        }
    }
}

fn cut_cost(adjacency: &[Vec<(usize, f64)>], communities: &[u8]) -> f64 {
    let mut cost = 0.0;
    for (src, edges) in adjacency.iter().enumerate() {
        let c_src = communities[src];
        for &(dst, w) in edges {
            if dst < communities.len() && c_src != communities[dst] {
                cost += w;
            }
        }
    }
    cost
}

fn move_delta(
    adjacency: &[Vec<(usize, f64)>],
    reverse: &[Vec<(usize, f64)>],
    communities: &[u8],
    node: usize,
    from: usize,
    to: usize,
) -> f64 {
    let mut delta = 0.0;
    let from_u8 = from as u8;
    let to_u8 = to as u8;

    for &(dst, w) in &adjacency[node] {
        let c_dst = communities[dst];
        let before = from_u8 != c_dst;
        let after = to_u8 != c_dst;
        if before && !after {
            delta -= w;
        } else if !before && after {
            delta += w;
        }
    }

    for &(src, w) in &reverse[node] {
        let c_src = communities[src];
        let before = c_src != from_u8;
        let after = c_src != to_u8;
        if before && !after {
            delta -= w;
        } else if !before && after {
            delta += w;
        }
    }

    delta
}
