use super::{sampling_walk_parameters::SamplingWalkParameters, train_parameters::TrainParameters};

/// Node2Vec parameters.
///
/// Java: `record Node2VecParameters(SamplingWalkParameters samplingWalkParameters, TrainParameters trainParameters)`
#[derive(Debug, Clone)]
pub struct Node2VecParameters {
    pub sampling_walk_parameters: SamplingWalkParameters,
    pub train_parameters: TrainParameters,
}


