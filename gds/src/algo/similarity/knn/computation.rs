use super::metrics::SimilarityComputer;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct KnnComputationResult {
    pub source: u64,
    pub target: u64,
    pub similarity: f64,
}

#[derive(Debug, Clone, Default)]
pub struct KnnNnDescentStats {
    pub ran_iterations: usize,
    pub did_converge: bool,
    pub node_pairs_considered: u64,
    pub update_count: u64,
}

#[derive(Debug, Clone)]
pub struct KnnNnDescentConfig {
    pub k: usize,
    pub sampled_k: usize,
    pub max_iterations: usize,
    pub similarity_cutoff: f64,
    pub perturbation_rate: f64,
    pub random_joins: usize,
    pub update_threshold: u64,
    pub random_seed: u64,
}

#[derive(Debug, Clone)]
struct Entry {
    target: u64,
    similarity: f64,
    checked: bool,
}

#[derive(Debug)]
struct NeighborList {
    capacity: usize,
    entries: Vec<Entry>,
}

impl NeighborList {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: Vec::new(),
        }
    }

    fn contains(&self, target: u64) -> bool {
        self.entries.iter().any(|e| e.target == target)
    }

    fn prune_below_cutoff(&mut self, cutoff: f64) {
        if cutoff <= 0.0 {
            return;
        }
        self.entries.retain(|e| e.similarity >= cutoff);
    }

    fn sort_and_truncate(&mut self) {
        self.entries.sort_by(|a, b| {
            b.similarity
                .partial_cmp(&a.similarity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        if self.entries.len() > self.capacity {
            self.entries.truncate(self.capacity);
        }
    }

    fn try_add(
        &mut self,
        target: u64,
        similarity: f64,
        checked: bool,
        perturbation_rate: f64,
        random_draw: f64,
    ) -> bool {
        if self.contains(target) {
            return false;
        }
        if self.entries.len() < self.capacity {
            self.entries.push(Entry {
                target,
                similarity,
                checked,
            });
            return true;
        }

        let mut worst_i = 0;
        let mut worst_sim = self.entries[0].similarity;
        for (i, e) in self.entries.iter().enumerate().skip(1) {
            if e.similarity < worst_sim {
                worst_sim = e.similarity;
                worst_i = i;
            }
        }

        let should_replace = similarity > worst_sim || random_draw < perturbation_rate;
        if !should_replace {
            return false;
        }

        self.entries[worst_i] = Entry {
            target,
            similarity,
            checked,
        };
        true
    }

    fn split_old_new(&mut self, sampled_k: usize, rng: &mut ChaCha8Rng) -> (Vec<u64>, Vec<u64>) {
        let mut old = Vec::new();
        let mut new_candidates: Vec<usize> = Vec::new();

        for (idx, e) in self.entries.iter().enumerate() {
            if e.checked {
                new_candidates.push(idx);
            } else {
                old.push(e.target);
            }
        }

        let take = sampled_k.min(new_candidates.len());
        let mut sampled = Vec::with_capacity(take);
        for _ in 0..take {
            let pick_i = rng.gen_range(0..new_candidates.len());
            sampled.push(new_candidates.swap_remove(pick_i));
        }

        let mut new = Vec::with_capacity(sampled.len());
        for idx in sampled {
            if let Some(e) = self.entries.get_mut(idx) {
                e.checked = false;
                new.push(e.target);
            }
        }

        (old, new)
    }
}

#[derive(Default)]
pub struct KnnComputationRuntime;

impl KnnComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    /// NN-Descent style kNN approximation (Java GDS parity direction).
    ///
    /// This is intentionally graph-free: `initial_neighbors` must already be sampled by the
    /// storage/controller layer.
    pub fn compute_nn_descent(
        &self,
        node_count: usize,
        initial_neighbors: Vec<Vec<u64>>,
        cfg: KnnNnDescentConfig,
        similarity: Arc<dyn SimilarityComputer>,
        source_allowed: Option<Arc<Vec<bool>>>,
        target_allowed: Option<Arc<Vec<bool>>>,
    ) -> (Vec<KnnComputationResult>, KnnNnDescentStats) {
        if node_count == 0 || cfg.k == 0 {
            return (Vec::new(), KnnNnDescentStats::default());
        }

        let cutoff = cfg.similarity_cutoff.max(0.0);
        let k = cfg.k;
        let sampled_k = cfg.sampled_k.max(1).min(k);
        let max_iterations = cfg.max_iterations.max(1);
        let perturbation_rate = cfg.perturbation_rate.clamp(0.0, 1.0);
        let random_joins = cfg.random_joins;
        let update_threshold = cfg.update_threshold;
        let is_symmetric = similarity.is_symmetric();

        let lists: Vec<Mutex<NeighborList>> = (0..node_count)
            .map(|_| Mutex::new(NeighborList::new(k)))
            .collect();

        // Seed neighbor lists from controller-provided candidates.
        for (source, candidates) in initial_neighbors.into_iter().enumerate().take(node_count) {
            if let Some(allowed) = source_allowed.as_ref() {
                if !allowed[source] {
                    continue;
                }
            }
            let source_u = source as u64;
            let mut seen: HashSet<u64> = HashSet::new();
            let mut guard = lists[source].lock().expect("neighbor list lock");
            for target in candidates {
                if target == source_u {
                    continue;
                }
                if !seen.insert(target) {
                    continue;
                }
                if let Some(allowed) = target_allowed.as_ref() {
                    let t = target as usize;
                    if t < allowed.len() && !allowed[t] {
                        continue;
                    }
                }

                let s = similarity.safe_similarity(source_u, target);
                if s < cutoff {
                    continue;
                }
                // Mark as checked/new for the first split.
                let _ = guard.try_add(target, s, true, 0.0, 1.0);
            }
            guard.sort_and_truncate();
        }

        let mut stats = KnnNnDescentStats::default();

        for iter in 0..max_iterations {
            // Split old/new for each node and build reverse lists.
            let old_new: Vec<(Vec<u64>, Vec<u64>)> = (0..node_count)
                .into_par_iter()
                .map(|node| {
                    let mut rng = ChaCha8Rng::seed_from_u64(
                        cfg.random_seed
                            .wrapping_add(node as u64)
                            .wrapping_add((iter as u64) << 32),
                    );
                    let mut guard = lists[node].lock().expect("neighbor list lock");
                    guard.split_old_new(sampled_k, &mut rng)
                })
                .collect();

            let mut reverse_old: Vec<Vec<u64>> = vec![Vec::new(); node_count];
            let mut reverse_new: Vec<Vec<u64>> = vec![Vec::new(); node_count];

            for (v, (old, new)) in old_new.iter().enumerate() {
                for &u in old {
                    let ui = u as usize;
                    if ui < node_count {
                        reverse_old[ui].push(v as u64);
                    }
                }
                for &u in new {
                    let ui = u as usize;
                    if ui < node_count {
                        reverse_new[ui].push(v as u64);
                    }
                }
            }

            let (iteration_updates, iteration_considered): (u64, u64) = (0..node_count)
                .into_par_iter()
                .map(|v| {
                    if let Some(allowed) = source_allowed.as_ref() {
                        if !allowed[v] {
                            return (0u64, 0u64);
                        }
                    }

                    let (ref old, ref new) = old_new[v];
                    let mut candidates_new: Vec<u64> =
                        Vec::with_capacity(new.len() + reverse_new[v].len());
                    candidates_new.extend_from_slice(new);
                    candidates_new.extend_from_slice(&reverse_new[v]);

                    let mut candidates_old: Vec<u64> =
                        Vec::with_capacity(old.len() + reverse_old[v].len());
                    candidates_old.extend_from_slice(old);
                    candidates_old.extend_from_slice(&reverse_old[v]);

                    // Local RNG for perturbation + random joins.
                    let mut rng = ChaCha8Rng::seed_from_u64(
                        cfg.random_seed
                            .wrapping_add(v as u64)
                            .wrapping_add(0x9E37_79B9_7F4A_7C15)
                            .wrapping_add(iter as u64),
                    );

                    let mut updates = 0u64;
                    let mut considered = 0u64;

                    // Join between new-new and new-old.
                    for &a in &candidates_new {
                        for &b in &candidates_new {
                            if a == b {
                                continue;
                            }
                            considered += 1;
                            updates += join_pair(
                                node_count,
                                &lists,
                                &*similarity,
                                cutoff,
                                perturbation_rate,
                                &mut rng,
                                is_symmetric,
                                a,
                                b,
                                target_allowed.as_deref(),
                            );
                        }

                        for &b in &candidates_old {
                            if a == b {
                                continue;
                            }
                            considered += 1;
                            updates += join_pair(
                                node_count,
                                &lists,
                                &*similarity,
                                cutoff,
                                perturbation_rate,
                                &mut rng,
                                is_symmetric,
                                a,
                                b,
                                target_allowed.as_deref(),
                            );
                        }
                    }

                    // Random joins for node v itself (cheap exploration).
                    let v_u = v as u64;
                    for _ in 0..random_joins {
                        if node_count <= 1 {
                            break;
                        }
                        let mut r = rng.gen_range(0..node_count) as u64;
                        if r == v_u {
                            r = (r + 1) % (node_count as u64);
                        }
                        considered += 1;
                        updates += join_pair(
                            node_count,
                            &lists,
                            &*similarity,
                            cutoff,
                            perturbation_rate,
                            &mut rng,
                            is_symmetric,
                            v_u,
                            r,
                            target_allowed.as_deref(),
                        );
                    }

                    (updates, considered)
                })
                .reduce(
                    || (0u64, 0u64),
                    |a, b| (a.0.saturating_add(b.0), a.1.saturating_add(b.1)),
                );

            // Prune below cutoff and keep top-k per node.
            (0..node_count).into_par_iter().for_each(|node| {
                let mut guard = lists[node].lock().expect("neighbor list lock");
                guard.prune_below_cutoff(cutoff);
                guard.sort_and_truncate();
            });

            stats.ran_iterations = iter + 1;
            stats.node_pairs_considered = stats
                .node_pairs_considered
                .saturating_add(iteration_considered);
            stats.update_count = stats.update_count.saturating_add(iteration_updates);

            if iteration_updates <= update_threshold {
                stats.did_converge = true;
                break;
            }
        }

        // Collect results (top-k per source).
        let rows: Vec<KnnComputationResult> = (0..node_count)
            .into_par_iter()
            .flat_map_iter(|source| {
                if let Some(allowed) = source_allowed.as_ref() {
                    if !allowed[source] {
                        return Vec::new();
                    }
                }
                let guard = lists[source].lock().expect("neighbor list lock");
                guard
                    .entries
                    .iter()
                    .filter(|e| {
                        if e.target == source as u64 {
                            return false;
                        }
                        if let Some(allowed) = target_allowed.as_ref() {
                            let t = e.target as usize;
                            if t < allowed.len() {
                                return allowed[t];
                            }
                        }
                        true
                    })
                    .map(|e| KnnComputationResult {
                        source: source as u64,
                        target: e.target,
                        similarity: e.similarity,
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        (rows, stats)
    }
}

fn join_pair(
    node_count: usize,
    lists: &[Mutex<NeighborList>],
    similarity: &dyn SimilarityComputer,
    cutoff: f64,
    perturbation_rate: f64,
    rng: &mut ChaCha8Rng,
    is_symmetric: bool,
    a: u64,
    b: u64,
    target_allowed: Option<&Vec<bool>>,
) -> u64 {
    if a as usize >= node_count || b as usize >= node_count {
        return 0;
    }
    if a == b {
        return 0;
    }
    if let Some(allowed) = target_allowed {
        if !allowed.get(b as usize).copied().unwrap_or(true) {
            return 0;
        }
        if !allowed.get(a as usize).copied().unwrap_or(true) {
            // If `a` is not a valid target, we still allow inserting `b` into `a`'s list
            // (since `a` is the source). But we must avoid inserting `a` into `b`'s list.
        }
    }

    let s_ab = similarity.safe_similarity(a, b);
    if s_ab < cutoff {
        return 0;
    }

    let draw_ab: f64 = rng.gen();

    let draw_ba: f64 = rng.gen();

    // Acquire locks in node-id order to avoid deadlock.
    let (lo, hi) = if a < b { (a, b) } else { (b, a) };
    let mut lo_guard = match lists[lo as usize].lock() {
        Ok(g) => g,
        Err(_) => return 0,
    };
    let mut hi_guard = match lists[hi as usize].lock() {
        Ok(g) => g,
        Err(_) => return 0,
    };

    let (a_guard, b_guard) = if a == lo {
        (&mut *lo_guard, &mut *hi_guard)
    } else {
        (&mut *hi_guard, &mut *lo_guard)
    };

    let mut updates = 0u64;

    // Insert b into a's list.
    let updated_a = a_guard.try_add(b, s_ab, true, perturbation_rate, draw_ab);
    if updated_a {
        updates += 1;
    }

    // Insert a into b's list.
    if is_symmetric {
        if target_allowed
            .and_then(|allowed| allowed.get(a as usize).copied())
            .unwrap_or(true)
        {
            let updated_b = b_guard.try_add(a, s_ab, true, perturbation_rate, draw_ba);
            if updated_b {
                updates += 1;
            }
        }
    } else {
        let s_ba = similarity.safe_similarity(b, a);
        if s_ba >= cutoff
            && target_allowed
                .and_then(|allowed| allowed.get(a as usize).copied())
                .unwrap_or(true)
        {
            let updated_b = b_guard.try_add(a, s_ba, true, perturbation_rate, draw_ba);
            if updated_b {
                updates += 1;
            }
        }
    }

    updates
}
