//! Combined parameters for Node2Vec.

use super::sampling_walk_parameters::SamplingWalkParameters;
use super::train_parameters::TrainParameters;

#[derive(Debug, Clone)]
pub struct Node2VecParameters {
    pub sampling_walk_parameters: SamplingWalkParameters,
    pub train_parameters: TrainParameters,
}
