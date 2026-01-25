use crate::config::validation::ConfigError;
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

impl GATConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.embedding_dimension == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "embeddingDimension".to_string(),
                reason: "embeddingDimension must be > 0".to_string(),
            });
        }
        if self.num_heads == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "numHeads".to_string(),
                reason: "numHeads must be > 0".to_string(),
            });
        }
        if self.num_layers == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "numLayers".to_string(),
                reason: "numLayers must be > 0".to_string(),
            });
        }
        if self.learning_rate <= 0.0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "learningRate".to_string(),
                reason: "learningRate must be > 0".to_string(),
            });
        }
        if self.epochs == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "epochs".to_string(),
                reason: "epochs must be > 0".to_string(),
            });
        }
        if !(0.0..=1.0).contains(&self.dropout) {
            return Err(ConfigError::InvalidParameter {
                parameter: "dropout".to_string(),
                reason: "dropout must be in [0.0, 1.0]".to_string(),
            });
        }
        if !(0.0..=1.0).contains(&self.alpha) {
            return Err(ConfigError::InvalidParameter {
                parameter: "alpha".to_string(),
                reason: "alpha must be in [0.0, 1.0]".to_string(),
            });
        }
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be > 0".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for GATConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        GATConfig::validate(self)
    }
}
