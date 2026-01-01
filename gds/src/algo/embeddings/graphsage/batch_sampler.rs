//! Java: `BatchSampler`.

use crate::concurrency::TerminationFlag;
use crate::core::utils::partition::{Partition, PartitionUtils};
use crate::ml::core::samplers::WeightedUniformSampler;
use crate::types::graph::Graph;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;
use std::sync::Arc;

pub struct BatchSampler {
    graph: Arc<dyn Graph>,
    termination_flag: TerminationFlag,
}

impl BatchSampler {
    pub const DEGREE_SMOOTHING_FACTOR: f64 = 0.75;

    pub fn new(graph: Arc<dyn Graph>, termination_flag: TerminationFlag) -> Self {
        Self {
            graph,
            termination_flag,
        }
    }

    pub fn extended_batches(
        &self,
        batch_size: usize,
        search_depth: usize,
        random_seed: u64,
    ) -> Vec<Vec<u64>> {
        PartitionUtils::range_partition_with_batch_size(
            self.graph.node_count(),
            batch_size,
            |batch| {
                self.termination_flag.assert_running();
                let local_seed = (batch.start_node() as u64
                    / self.graph.node_count().max(1) as u64)
                    + random_seed;
                self.sample_neighbor_and_negative_node_per_batch_node(
                    batch,
                    search_depth,
                    local_seed,
                )
            },
        )
    }

    /// For each node in the batch we sample one neighbor node and one negative node from the graph.
    pub fn sample_neighbor_and_negative_node_per_batch_node(
        &self,
        batch: Partition,
        search_depth: usize,
        random_seed: u64,
    ) -> Vec<u64> {
        let neighbours = self.neighbor_batch(batch, random_seed, search_depth);
        let negatives = self.negative_batch(batch.node_count(), &neighbours, random_seed);

        let mut out = Vec::with_capacity(batch.node_count() * 3);
        out.extend(batch.iter().map(|n| n as u64));
        out.extend(neighbours);
        out.extend(negatives);
        out
    }

    fn neighbor_batch(
        &self,
        batch: Partition,
        batch_local_seed: u64,
        search_depth: usize,
    ) -> Vec<u64> {
        let mut rng = ChaCha8Rng::seed_from_u64(batch_local_seed);
        let mut neighbors = Vec::with_capacity(batch.node_count());

        for node_id in batch.iter() {
            // random walk with at most searchDepth steps, save last node
            let mut current = node_id as u64;
            let mut depth = rng.gen_range(1..=search_depth.max(1));

            while depth > 0 {
                let degree = self.graph.degree(current as i64);
                if degree == 0 {
                    break;
                }
                let idx = rng.gen_range(0..degree);
                let next = self
                    .graph
                    .nth_target(current as i64, idx)
                    .expect("nth_target within degree");
                current = next as u64;
                depth -= 1;
            }

            neighbors.push(current);
        }

        neighbors
    }

    fn negative_batch(
        &self,
        batch_size: usize,
        batch_neighbors: &[u64],
        batch_local_seed: u64,
    ) -> Vec<u64> {
        let node_count = self.graph.node_count();
        let mut sampler = WeightedUniformSampler::new(batch_local_seed);

        let neighbor_set: HashSet<u64> = batch_neighbors.iter().copied().collect();

        let input = (0..node_count as u64).map(|node_id| {
            let degree = self.graph.degree(node_id as i64) as f64;
            (node_id, degree.powf(Self::DEGREE_SMOOTHING_FACTOR))
        });

        sampler.sample_filtered(input, node_count, batch_size, |sample| {
            !neighbor_set.contains(&sample)
        })
    }
}
