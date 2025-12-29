//! Neighborhood sampler for subgraphs in GDS.
//!
//! Translated from Java GDS ml-core NeighborhoodSampler.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::samplers::UniformSampler;
use crate::types::graph::Graph;

/// Samples neighborhoods for graph neural network batch processing.
///
pub struct NeighborhoodSampler {
    random_seed: u64,
}

impl NeighborhoodSampler {
    /// Create a new neighborhood sampler with the given random seed.
    pub fn new(random_seed: u64) -> Self {
        Self { random_seed }
    }

    /// Get the random seed (for future implementation).
    #[allow(dead_code)]
    pub fn random_seed(&self) -> u64 {
        self.random_seed
    }

    /// Sample up to `sample_size` neighbors of `node_id` uniformly without replacement.
    ///
    /// Java: `NeighborhoodSampler.sample(Graph graph, long nodeId, int sampleSize)`
    pub fn sample(&self, graph: &dyn Graph, node_id: u64, sample_size: usize) -> Vec<u64> {
        let degree = graph.degree(node_id as i64);
        if degree == 0 || sample_size == 0 {
            return Vec::new();
        }

        let mut sampler = UniformSampler::new(self.random_seed ^ node_id);
        let iter =
            (0..degree).filter_map(|i| graph.nth_target(node_id as i64, i).map(|t| t as u64));
        sampler.sample(iter, degree as u64, sample_size.min(degree))
    }
}
