//! Memory estimation definition for HashGNN.
//!
//! Java: `HashGNNMemoryEstimateDefinition implements MemoryEstimateDefinition`
//!
//! Rust GDS has a richer `mem` subsystem; this is currently a lightweight placeholder.

use super::hash_gnn_parameters::HashGNNParameters;
use crate::mem::Estimate;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HashGNNMemoryEstimateDefinition {
    parameters: HashGNNParameters,
}

#[allow(dead_code)]
impl HashGNNMemoryEstimateDefinition {
    pub fn new(parameters: HashGNNParameters) -> Self {
        Self { parameters }
    }

    /// Rough estimate (bytes) following the Java definition shape.
    pub fn estimate_bytes(&self, node_count: usize) -> usize {
        let embedding_density = self.parameters.embedding_density;
        let binary_dimension = self
            .parameters
            .generate_features
            .as_ref()
            .map(|c| c.dimension)
            .or_else(|| {
                self.parameters
                    .binarize_features
                    .as_ref()
                    .map(|c| c.dimension)
            })
            .unwrap_or(1024);

        let cache = node_count * Estimate::size_of_bitset(binary_dimension) * 2;
        let hashes_cache = embedding_density * Estimate::size_of_int_array(binary_dimension);

        let out_dim = self.parameters.output_dimension.unwrap_or(binary_dimension);
        let output_dense = node_count * Estimate::size_of_double_array(out_dim);

        cache + hashes_cache + output_dense
    }
}
