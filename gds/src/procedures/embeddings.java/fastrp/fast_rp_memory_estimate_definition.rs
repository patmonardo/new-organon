//! Memory estimation definition for FastRP.
//!
//! Java: `FastRPMemoryEstimateDefinition implements MemoryEstimateDefinition`
//!
//! Rust memory usage differs (Vec overhead, allocation strategy, etc.). This is a
//! deliberately rough estimate aligned to the Java definition's *shape*.

use super::fast_rp_parameters::FastRPParameters;
use crate::mem::Estimate;

#[derive(Debug, Clone)]
pub struct FastRPMemoryEstimateDefinition {
    parameters: FastRPParameters,
}

impl FastRPMemoryEstimateDefinition {
    pub fn new(parameters: FastRPParameters) -> Self {
        Self { parameters }
    }

    /// Very rough estimate (bytes):
    /// - property vectors: feature_count * property_dimension floats
    /// - three embedding buffers: per-node float array of embedding_dimension
    pub fn estimate_bytes(&self, node_count: usize, feature_count: usize) -> usize {
        let embedding_dim = self.parameters.embedding_dimension;
        let property_dim = self.parameters.property_dimension;

        let property_vectors = Estimate::size_of_float_array(feature_count.saturating_mul(property_dim));
        let embedding_vec = Estimate::size_of_float_array(embedding_dim);
        let buffers = 3 * node_count * embedding_vec;

        property_vectors + buffers
    }
}


