use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GATConfig {
    pub embedding_dimension: usize,
    pub num_heads: usize,
    pub num_layers: usize,
    pub learning_rate: f64,
    pub epochs: usize,
    pub dropout: f64,
    pub alpha: f64, // LeakyReLU slope
    pub random_seed: Option<u64>,
    pub concurrency: usize,
}

impl Default for GATConfig {
    fn default() -> Self {
        Self {
            embedding_dimension: 64,
            num_heads: 8,
            num_layers: 2,
            learning_rate: 0.01,
            epochs: 100,
            dropout: 0.6,
            alpha: 0.2,
            random_seed: None,
            concurrency: 4,
        }
    }
}
