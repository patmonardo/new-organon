//! CELF Computation Runtime
//!
//! Pure computation layer for Cost-Effective Lazy Forward influence maximization.
//! Uses Independent Cascade model with Monte Carlo simulation.

use super::spec::CELFConfig;
use super::{SeedSetBuilder, SpreadPriorityQueue};
use crate::collections::{BitSet, HugeAtomicDoubleArray};
use crate::concurrency::virtual_threads::{Executor, WorkerContext};
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::paged::HugeLongArrayStack;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;

pub struct CELFComputationRuntime {
    config: CELFConfig,
    node_count: usize,
}

impl CELFComputationRuntime {
    pub fn new(config: CELFConfig, node_count: usize) -> Self {
        Self { config, node_count }
    }

    /// Main CELF computation.
    ///
    /// Structure follows Java GDS:
    /// 1) Greedy init: compute spread for all nodes and pick best.
    /// 2) Lazy forward: iteratively update marginal gains for top candidates in batches.
    pub fn compute<F>(&self, get_neighbors: F, termination: &TerminationFlag) -> HashMap<u64, f64>
    where
        F: Fn(usize) -> Vec<usize> + Send + Sync,
    {
        if self.node_count == 0 || self.config.seed_set_size == 0 {
            return HashMap::new();
        }

        let seed_set_size = self.config.seed_set_size.min(self.node_count);

        // Phase 1: Greedy initialization - find first seed node
        let (first_seed, mut spreads_queue, mut gain) =
            self.greedy_init(&get_neighbors, termination);

        let mut seed_set = SeedSetBuilder::new();
        seed_set.add_seed(first_seed, gain);

        if seed_set_size == 1 {
            return seed_set.build();
        }

        // Phase 2: Lazy forward - find remaining k-1 seeds
        self.lazy_forward(
            &get_neighbors,
            first_seed,
            &mut spreads_queue,
            &mut seed_set,
            &mut gain,
            seed_set_size,
            termination,
        );

        seed_set.build()
    }

    /// Greedy initialization: compute spread for all nodes, select best
    fn greedy_init<F>(
        &self,
        get_neighbors: &F,
        termination: &TerminationFlag,
    ) -> (usize, SpreadPriorityQueue, f64)
    where
        F: Fn(usize) -> Vec<usize> + Send + Sync,
    {
        let single_spread = HugeAtomicDoubleArray::new(self.node_count);

        let concurrency = self.config.concurrency.max(1);
        let executor = Executor::new(Concurrency::of(concurrency));

        let storage = WorkerContext::new({
            let node_count = self.node_count;
            move || ICInitStorage::new(node_count)
        });

        // Compute spread for each node independently (parallel).
        // Each node gets its own base seed so results are deterministic under parallelism.
        let base_seed = self.config.random_seed;
        let mc = self.config.monte_carlo_simulations.max(1);
        let p = self.config.propagation_probability;

        // Ignore termination errors for now (we always use running_true in this runtime).
        let _ = executor.parallel_for(0, self.node_count, termination, |node_id| {
            storage.with(|s| {
                let spread = s.compute_single_node_spread(node_id, get_neighbors, p, mc, base_seed);
                single_spread.set(node_id, spread);
            });
        });

        // Build priority queue from spreads
        let mut queue = SpreadPriorityQueue::new(self.node_count);
        for node_id in 0..self.node_count {
            queue.set(node_id, single_spread.get(node_id));
        }

        let best_node = queue.pop();
        let best_spread = single_spread.get(best_node);

        (best_node, queue, best_spread)
    }

    /// Lazy forward selection for remaining k-1 seeds
    fn lazy_forward<F>(
        &self,
        get_neighbors: &F,
        first_seed: usize,
        spreads_queue: &mut SpreadPriorityQueue,
        seed_set: &mut SeedSetBuilder,
        cumulative_gain: &mut f64,
        seed_set_size: usize,
        termination: &TerminationFlag,
    ) where
        F: Fn(usize) -> Vec<usize> + Send + Sync,
    {
        let mut seed_nodes = vec![first_seed];
        let mut last_update = vec![0; self.node_count];
        let batch_size = self.config.batch_size.min(spreads_queue.size().max(1));

        let concurrency = self.config.concurrency.max(1);
        let executor = Executor::new(Concurrency::of(concurrency));

        let lazy_storage = WorkerContext::new({
            let node_count = self.node_count;
            let max_seed_set = seed_set_size;
            let max_batch = self.config.batch_size.max(1);
            move || ICLazyForwardStorage::new(node_count, max_seed_set, max_batch)
        });

        let mc = self.config.monte_carlo_simulations.max(1);
        let p = self.config.propagation_probability;
        let base_seed = self.config.random_seed;

        for iteration in 1..seed_set_size {
            // Lazy evaluation: re-evaluate top candidates until we find one that's current
            while last_update[spreads_queue.top()] != iteration {
                // Build batch of candidates that need re-evaluation
                let mut candidates = Vec::with_capacity(batch_size);
                let check_size = (2 * batch_size).min(spreads_queue.size());

                for j in 0..check_size {
                    if candidates.len() >= batch_size {
                        break;
                    }
                    let candidate = spreads_queue.get_ith(j);
                    if last_update[candidate] != iteration {
                        candidates.push(candidate);
                    }
                }

                // Re-evaluate batch with current seed set (parallel over simulations).
                let mut candidate_spreads = vec![0.0f64; candidates.len()];
                let seeds_snapshot = seed_nodes.clone();
                let candidates_snapshot = candidates.clone();

                // Partition simulations across workers.
                let sim_count = mc;
                let chunk = (sim_count + concurrency - 1) / concurrency;
                let task_count = (sim_count + chunk - 1) / chunk;

                let partials: std::sync::Arc<Vec<std::sync::Mutex<Vec<f64>>>> = std::sync::Arc::new(
                    (0..task_count)
                        .map(|_| std::sync::Mutex::new(vec![0.0f64; candidates_snapshot.len()]))
                        .collect(),
                );

                let _ = executor.parallel_for(0, task_count, termination, |task_idx| {
                    let start = task_idx * chunk;
                    let end = (start + chunk).min(sim_count);
                    if start >= end {
                        return;
                    }

                    lazy_storage.with(|s| {
                        s.set_seed_set(&seeds_snapshot);
                        let mut local = vec![0.0f64; candidates_snapshot.len()];
                        for sim in start..end {
                            s.run_simulation_for_candidates(
                                sim as u64,
                                base_seed,
                                p,
                                get_neighbors,
                                &candidates_snapshot,
                                &mut local,
                            );
                        }
                        let mut locked = partials[task_idx].lock().unwrap();
                        for i in 0..local.len() {
                            locked[i] += local[i];
                        }
                    });
                });

                for t in 0..task_count {
                    let locked = partials[t].lock().unwrap();
                    for i in 0..candidate_spreads.len() {
                        candidate_spreads[i] += locked[i];
                    }
                }

                // Java divides by monteCarloSimulations when writing into queue.
                for i in 0..candidate_spreads.len() {
                    candidate_spreads[i] /= sim_count as f64;
                }

                // Update queue with new marginal gains.
                for (i, &candidate) in candidates.iter().enumerate() {
                    let marginal_gain = candidate_spreads[i] - *cumulative_gain;
                    spreads_queue.set(candidate, marginal_gain);
                    last_update[candidate] = iteration;
                }
            }

            // Add best seed to set
            let best_seed = spreads_queue.pop();
            let marginal_spread = spreads_queue.spread(best_seed);

            seed_nodes.push(best_seed);
            seed_set.add_seed(best_seed, marginal_spread);
            *cumulative_gain += marginal_spread;
        }
    }
}

struct ICInitStorage {
    active: BitSet,
    stack: HugeLongArrayStack,
}

impl ICInitStorage {
    fn new(node_count: usize) -> Self {
        Self {
            active: BitSet::new(node_count),
            stack: HugeLongArrayStack::new(node_count),
        }
    }

    fn compute_single_node_spread<F>(
        &mut self,
        seed_node: usize,
        get_neighbors: &F,
        propagation_probability: f64,
        monte_carlo_simulations: usize,
        base_seed: u64,
    ) -> f64
    where
        F: Fn(usize) -> Vec<usize>,
    {
        let mut total = 0.0;
        for sim in 0..monte_carlo_simulations {
            self.active.clear_all();
            self.stack.clear();

            self.active.set(seed_node);
            self.stack.push(seed_node as i64);

            let mut rng = StdRng::seed_from_u64(base_seed.wrapping_add(sim as u64));
            while !self.stack.is_empty() {
                let current = self.stack.pop() as usize;
                for neighbor in (get_neighbors)(current) {
                    if !self.active.get(neighbor) && rng.gen::<f64>() < propagation_probability {
                        self.active.set(neighbor);
                        self.stack.push(neighbor as i64);
                    }
                }
            }

            total += self.active.cardinality() as f64;
        }

        total / monte_carlo_simulations as f64
    }
}

struct ICLazyForwardStorage {
    seed_set_nodes: Vec<usize>,
    seed_active: BitSet,
    candidate_active: BitSet,
    stack: HugeLongArrayStack,
}

impl ICLazyForwardStorage {
    fn new(node_count: usize, max_seed_set: usize, _max_batch: usize) -> Self {
        Self {
            seed_set_nodes: Vec::with_capacity(max_seed_set.max(1)),
            seed_active: BitSet::new(node_count),
            candidate_active: BitSet::new(node_count),
            stack: HugeLongArrayStack::new(node_count),
        }
    }

    fn set_seed_set(&mut self, seeds: &[usize]) {
        self.seed_set_nodes.clear();
        self.seed_set_nodes.extend_from_slice(seeds);
    }

    fn run_simulation_for_candidates<F>(
        &mut self,
        sim: u64,
        base_seed: u64,
        propagation_probability: f64,
        get_neighbors: &F,
        candidates: &[usize],
        out: &mut [f64],
    ) where
        F: Fn(usize) -> Vec<usize>,
    {
        debug_assert!(out.len() == candidates.len());

        // 1) Seed traverse.
        self.seed_active.clear_all();
        self.stack.clear();

        for &seed in &self.seed_set_nodes {
            self.seed_active.set(seed);
            self.stack.push(seed as i64);
        }

        let mut rng = StdRng::seed_from_u64(base_seed.wrapping_add(sim));
        while !self.stack.is_empty() {
            let node = self.stack.pop() as usize;
            for neighbor in (get_neighbors)(node) {
                if !self.seed_active.get(neighbor) && rng.gen::<f64>() < propagation_probability {
                    self.seed_active.set(neighbor);
                    self.stack.push(neighbor as i64);
                }
            }
        }

        let seed_card = self.seed_active.cardinality() as f64;

        // 2) Candidate traverse (per candidate).
        for (j, &candidate) in candidates.iter().enumerate() {
            if self.seed_active.get(candidate) {
                out[j] += seed_card;
                continue;
            }

            self.candidate_active.clear_all();
            self.stack.clear();
            self.candidate_active.set(candidate);
            self.stack.push(candidate as i64);

            // Use a deterministic but different stream for candidate traversal.
            let mut cand_rng =
                StdRng::seed_from_u64(base_seed.wrapping_add(sim).wrapping_add(0x9E3779B97F4A7C15));
            while !self.stack.is_empty() {
                let node = self.stack.pop() as usize;
                for neighbor in (get_neighbors)(node) {
                    if !self.seed_active.get(neighbor)
                        && !self.candidate_active.get(neighbor)
                        && cand_rng.gen::<f64>() < propagation_probability
                    {
                        self.candidate_active.set(neighbor);
                        self.stack.push(neighbor as i64);
                    }
                }
            }

            out[j] += seed_card + (self.candidate_active.cardinality() as f64);
        }
    }
}
