use super::{
    node2vec_parameters::Node2VecParameters, sampling_walk_parameters::SamplingWalkParameters,
    train_parameters::TrainParameters,
};

/// Mirrors Java's `Node2VecConfigTransformer`.
///
/// In Rust GDS we usually construct `Node2VecParameters` directly, but keeping this
/// helper preserves the Java surface area for translation/compatibility.
pub struct Node2VecConfigTransformer;

impl Node2VecConfigTransformer {
    pub fn node2vec_parameters(
        walks_per_node: usize,
        walk_length: usize,
        return_factor: f64,
        in_out_factor: f64,
        positive_sampling_factor: f64,
        negative_sampling_exponent: f64,
        train_parameters: TrainParameters,
    ) -> Node2VecParameters {
        let sampling = SamplingWalkParameters {
            walks_per_node,
            walk_length,
            return_factor,
            in_out_factor,
            positive_sampling_factor,
            negative_sampling_exponent,
        };

        Node2VecParameters {
            sampling_walk_parameters: sampling,
            train_parameters,
        }
    }
}


