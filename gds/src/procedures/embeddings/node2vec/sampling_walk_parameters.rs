//! Sampling walk parameters for Node2Vec.

#[derive(Debug, Clone)]
pub struct SamplingWalkParameters {
    pub walks_per_node: usize,
    pub walk_length: usize,
    pub return_factor: f64,
    pub in_out_factor: f64,
    pub positive_sampling_factor: f64,
    pub negative_sampling_exponent: f64,
}
