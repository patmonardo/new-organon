//! Memory estimation definition for Node2Vec.
//!
//! Java: `Node2VecMemoryEstimateDefinition implements MemoryEstimateDefinition`
//!
//! Rust GDS has a richer `mem` subsystem; this is currently a lightweight placeholder
//! to keep the module surface aligned.

use super::node2vec_parameters::Node2VecParameters;
use crate::mem::Estimate;

#[derive(Debug, Clone)]
pub struct Node2VecMemoryEstimateDefinition {
    parameters: Node2VecParameters,
}

impl Node2VecMemoryEstimateDefinition {
    pub fn new(parameters: Node2VecParameters) -> Self {
        Self { parameters }
    }

    /// Very rough estimate (bytes) matching the shape of the Java definition:
    /// - random walks: per-walk long array of walk_length
    /// - model: per-node float arrays for center + context embeddings
    pub fn estimate_bytes(&self, node_count: usize) -> usize {
        let walks_per_node = self.parameters.sampling_walk_parameters.walks_per_node;
        let walk_length = self.parameters.sampling_walk_parameters.walk_length;
        let embedding_dimension = self.parameters.train_parameters.embedding_dimension;

        let random_walks = node_count * walks_per_node;
        let walk_mem = Estimate::size_of_long_array(walk_length);
        let walks_mem = random_walks * walk_mem;

        let vec_mem = Estimate::size_of_float_array(embedding_dimension);
        let model_mem = 2 * node_count * vec_mem;

        walks_mem + model_mem
    }
}


