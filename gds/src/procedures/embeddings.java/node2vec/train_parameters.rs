/// Embedding initializer type.
///
/// Java: `EmbeddingInitializer` enum (not included in the snippet list, but referenced by TrainParameters/Node2VecModel).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmbeddingInitializer {
    Uniform,
    Normalized,
}

/// Training parameters for Node2Vec.
///
/// Java: `record TrainParameters(double initialLearningRate, double minLearningRate, int iterations, int windowSize, int negativeSamplingRate, int embeddingDimension, EmbeddingInitializer embeddingInitializer)`
#[derive(Debug, Clone)]
pub struct TrainParameters {
    pub initial_learning_rate: f64,
    pub min_learning_rate: f64,
    pub iterations: usize,
    pub window_size: usize,
    pub negative_sampling_rate: usize,
    pub embedding_dimension: usize,
    pub embedding_initializer: EmbeddingInitializer,
}


