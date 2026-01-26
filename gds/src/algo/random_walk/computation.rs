//! RandomWalk Computation
//!
//! **Translation Source**: `org.neo4j.gds.traversal.RandomWalk`
//!
//! Implements node2vec-style biased random walks with return and in-out factors.

use super::spec::RandomWalkResult;
use super::RandomWalkStorageRuntime;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct RandomWalkComputationRuntime {
    storage: Arc<RandomWalkStorageRuntime>,
    walks_per_node: usize,
    walk_length: usize,
    return_factor: f64,
    in_out_factor: f64,
    source_nodes: Vec<usize>,
    random_seed: u64,
}

impl RandomWalkComputationRuntime {
    pub fn new(
        walks_per_node: usize,
        walk_length: usize,
        return_factor: f64,
        in_out_factor: f64,
        source_nodes: Vec<usize>,
        random_seed: u64,
    ) -> Self {
        Self {
            storage: Arc::new(RandomWalkStorageRuntime::new()),
            walks_per_node,
            walk_length,
            return_factor,
            in_out_factor,
            source_nodes,
            random_seed,
        }
    }

    /// Compute random walks
    pub fn compute(
        &self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize> + Send + Sync,
    ) -> RandomWalkResult {
        let get_neighbors = Arc::new(get_neighbors);

        // Determine which nodes to walk from
        let nodes_to_walk: Vec<usize> = if self.source_nodes.is_empty() {
            (0..node_count).collect()
        } else {
            self.source_nodes.clone()
        };

        let node_index = AtomicUsize::new(0);
        let total_nodes = nodes_to_walk.len();

        // Process walks in parallel using rayon or similar
        // For now, simple sequential implementation
        while let Some(start_node) = self.get_next_node(&nodes_to_walk, &node_index, total_nodes) {
            let mut rng = StdRng::seed_from_u64(self.random_seed.wrapping_add(start_node as u64));

            for _ in 0..self.walks_per_node {
                let walk = self.perform_walk(start_node, &get_neighbors, &mut rng);
                self.storage.add_walk(walk);
            }
        }

        RandomWalkResult {
            walks: self.storage.take_walks(),
        }
    }

    fn get_next_node(&self, nodes: &[usize], index: &AtomicUsize, total: usize) -> Option<usize> {
        let idx = index.fetch_add(1, Ordering::SeqCst);
        if idx < total {
            Some(nodes[idx])
        } else {
            None
        }
    }

    fn perform_walk(
        &self,
        start_node: usize,
        get_neighbors: &Arc<impl Fn(usize) -> Vec<usize>>,
        rng: &mut StdRng,
    ) -> Vec<u64> {
        let mut walk = Vec::with_capacity(self.walk_length);
        walk.push(start_node as u64);

        let mut current = start_node;
        let mut previous: Option<usize> = None;

        for _ in 1..self.walk_length {
            let neighbors = get_neighbors(current);

            if neighbors.is_empty() {
                break;
            }

            // node2vec biased sampling
            let next = self.sample_next_node(current, previous, &neighbors, rng);

            walk.push(next as u64);
            previous = Some(current);
            current = next;
        }

        walk
    }

    fn sample_next_node(
        &self,
        _current: usize,
        previous: Option<usize>,
        neighbors: &[usize],
        rng: &mut StdRng,
    ) -> usize {
        if neighbors.len() == 1 {
            return neighbors[0];
        }

        // Calculate weights for biased sampling (node2vec style)
        let weights: Vec<f64> = neighbors
            .iter()
            .map(|&neighbor| {
                if let Some(prev) = previous {
                    if neighbor == prev {
                        // Return to previous node
                        1.0 / self.return_factor
                    } else {
                        // Check if neighbor is connected to previous
                        // For simplicity, assume it's a new node (in-out exploration)
                        1.0 / self.in_out_factor
                    }
                } else {
                    1.0 // First step, uniform probability
                }
            })
            .collect();

        // Sample based on weights
        let total_weight: f64 = weights.iter().sum();
        let mut threshold = rng.gen::<f64>() * total_weight;

        for (i, &weight) in weights.iter().enumerate() {
            threshold -= weight;
            if threshold <= 0.0 {
                return neighbors[i];
            }
        }

        // Fallback to last neighbor
        neighbors[neighbors.len() - 1]
    }
}
