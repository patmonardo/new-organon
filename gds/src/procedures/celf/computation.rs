//! CELF Computation Runtime
//!
//! Pure computation layer for Cost-Effective Lazy Forward influence maximization.
//! Uses Independent Cascade model with Monte Carlo simulation.

use super::spec::CELFConfig;
use super::storage::{SeedSetBuilder, SpreadPriorityQueue};
use crate::collections::HugeDoubleArray;
use crate::collections::BitSet;
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

    /// Main CELF computation
    pub fn compute<F>(&self, get_neighbors: F) -> HashMap<u64, f64>
    where
        F: Fn(usize) -> Vec<usize> + Send + Sync,
    {
        if self.node_count == 0 || self.config.seed_set_size == 0 {
            return HashMap::new();
        }

        let seed_set_size = self.config.seed_set_size.min(self.node_count);

        // Phase 1: Greedy initialization - find first seed node
        let (first_seed, mut spreads_queue, mut gain) =
            self.greedy_init(&get_neighbors);

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
        );

        seed_set.build()
    }

    /// Greedy initialization: compute spread for all nodes, select best
    fn greedy_init<F>(&self, get_neighbors: &F) -> (usize, SpreadPriorityQueue, f64)
    where
        F: Fn(usize) -> Vec<usize> + Send + Sync,
    {
        let mut single_spread = HugeDoubleArray::new(self.node_count);

        // Compute spread for each node independently
        for node_id in 0..self.node_count {
            let spread = self.compute_single_node_spread(node_id, get_neighbors);
            single_spread.set(node_id, spread);
        }

        // Build priority queue from spreads
        let mut queue = SpreadPriorityQueue::new(self.node_count);
        for node_id in 0..self.node_count {
            queue.set(node_id, single_spread.get(node_id));
        }

        let best_node = queue.pop();
        let best_spread = single_spread.get(best_node);

        (best_node, queue, best_spread)
    }

    /// Compute spread for a single seed node via Monte Carlo
    fn compute_single_node_spread<F>(&self, seed: usize, get_neighbors: &F) -> f64
    where
        F: Fn(usize) -> Vec<usize>,
    {
        let mut total_spread = 0.0;
        let mut rng = StdRng::seed_from_u64(self.config.random_seed);

        for _sim in 0..self.config.monte_carlo_simulations {
            let mut active = BitSet::new(self.node_count);
            let mut stack = HugeLongArrayStack::new(self.node_count);

            active.set(seed);
            stack.push(seed as i64);

            while !stack.is_empty() {
                let current = stack.pop() as usize;
                let neighbors = get_neighbors(current);

                for &neighbor in &neighbors {
                    if !active.get(neighbor)
                        && rng.gen::<f64>() < self.config.propagation_probability {
                            active.set(neighbor);
                            stack.push(neighbor as i64);
                        }
                }
            }

            total_spread += active.cardinality() as f64;
        }

        total_spread / self.config.monte_carlo_simulations as f64
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
    ) where
        F: Fn(usize) -> Vec<usize> + Send + Sync,
    {
        let mut seed_nodes = vec![first_seed];
        let mut last_update = vec![0; self.node_count];
        let batch_size = self.config.batch_size.min(spreads_queue.size());

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

                // Re-evaluate batch with current seed set
                let candidate_spreads = self.compute_marginal_gains(
                    &seed_nodes,
                    &candidates,
                    get_neighbors,
                );

                // Update queue with new marginal gains
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

    /// Compute marginal gains for candidates given current seed set
    fn compute_marginal_gains<F>(
        &self,
        seed_nodes: &[usize],
        candidates: &[usize],
        get_neighbors: &F,
    ) -> Vec<f64>
    where
        F: Fn(usize) -> Vec<usize>,
    {
        let mut spreads = vec![0.0; candidates.len()];
        let mut rng = StdRng::seed_from_u64(self.config.random_seed);

        for _sim in 0..self.config.monte_carlo_simulations {
            // First, propagate from seed set
            let mut seed_active = BitSet::new(self.node_count);
            let mut stack = HugeLongArrayStack::new(self.node_count);

            for &seed in seed_nodes {
                seed_active.set(seed);
                stack.push(seed as i64);
            }

            self.propagate(&mut seed_active, &mut stack, &mut rng, get_neighbors);

            // Then evaluate each candidate's marginal contribution
            for (idx, &candidate) in candidates.iter().enumerate() {
                if !seed_active.get(candidate) {
                    let mut candidate_active = BitSet::new(self.node_count);
                    let mut cand_stack = HugeLongArrayStack::new(self.node_count);

                    candidate_active.set(candidate);
                    cand_stack.push(candidate as i64);

                    // Propagate from candidate, avoiding seed_active nodes
                    while !cand_stack.is_empty() {
                        let current = cand_stack.pop() as usize;
                        let neighbors = get_neighbors(current);

                        for &neighbor in &neighbors {
                            if !seed_active.get(neighbor)
                                && !candidate_active.get(neighbor)
                                && rng.gen::<f64>() < self.config.propagation_probability {
                                    candidate_active.set(neighbor);
                                    cand_stack.push(neighbor as i64);
                                }
                        }
                    }

                    spreads[idx] += (seed_active.cardinality() + candidate_active.cardinality()) as f64;
                } else {
                    spreads[idx] += seed_active.cardinality() as f64;
                }
            }
        }

        // Average over simulations
        for spread in &mut spreads {
            *spread /= self.config.monte_carlo_simulations as f64;
        }

        spreads
    }

    /// Propagate activation through the graph
    fn propagate<F>(
        &self,
        active: &mut BitSet,
        stack: &mut HugeLongArrayStack,
        rng: &mut StdRng,
        get_neighbors: &F,
    ) where
        F: Fn(usize) -> Vec<usize>,
    {
        while !stack.is_empty() {
            let current = stack.pop() as usize;
            let neighbors = get_neighbors(current);

            for &neighbor in &neighbors {
                if !active.get(neighbor)
                    && rng.gen::<f64>() < self.config.propagation_probability {
                        active.set(neighbor);
                        stack.push(neighbor as i64);
                    }
            }
        }
    }
}
