//! Neighborhood sampler for subgraphs in GDS.
//!
//! Translated from Java GDS ml-core NeighborhoodSampler.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::relationship_weights::DEFAULT_VALUE;
use crate::ml::core::samplers::{UniformSampler, WeightedUniformSampler};
use crate::types::graph::Graph;
use crate::types::properties::relationship::{
    RelationshipCursorBox, WeightedRelationshipCursorBox,
};

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

        let concurrent_graph = Graph::concurrent_copy(graph);

        // Every neighbor needs to be sampled
        if degree <= sample_size {
            return concurrent_graph
                .stream_relationships(node_id as i64, DEFAULT_VALUE)
                .map(|cursor: RelationshipCursorBox| cursor.target_id() as u64)
                .collect();
        }

        if graph.has_relationship_property() {
            let mut sampler = WeightedUniformSampler::new(self.random_seed + node_id);
            let input = concurrent_graph
                .stream_relationships_weighted(node_id as i64, DEFAULT_VALUE)
                .map(|cursor: WeightedRelationshipCursorBox| {
                    (cursor.target_id() as u64, cursor.weight())
                });
            sampler.sample(input, degree, sample_size)
        } else {
            let mut sampler = UniformSampler::new(self.random_seed + node_id);
            let input = concurrent_graph
                .stream_relationships(node_id as i64, DEFAULT_VALUE)
                .map(|cursor: RelationshipCursorBox| cursor.target_id() as u64);
            sampler.sample(input, degree as u64, sample_size)
        }
    }
}
