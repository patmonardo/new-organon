//! Training parameters for Node2Vec.

#[derive(Debug, Clone)]
pub enum EmbeddingInitializer {
    Uniform,
    Normalized,
}

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
