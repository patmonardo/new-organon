use crate::config::validation::ConfigError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub struct GradientDescentConfig {
    #[builder(default = "100")]
    batch_size: usize,

    #[builder(default = "1")]
    min_epochs: usize,

    #[builder(default = "1")]
    patience: usize,

    #[builder(default = "100")]
    max_epochs: usize,

    #[builder(default = "1e-3")]
    tolerance: f64,

    #[builder(default = "0.001")]
    learning_rate: f64,
}

impl GradientDescentConfig {
    pub fn builder() -> GradientDescentConfigBuilder {
        GradientDescentConfigBuilder::default()
    }

    pub fn batch_size(&self) -> usize {
        self.batch_size
    }

    pub fn min_epochs(&self) -> usize {
        self.min_epochs
    }

    pub fn patience(&self) -> usize {
        self.patience
    }

    pub fn max_epochs(&self) -> usize {
        self.max_epochs
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn learning_rate(&self) -> f64 {
        self.learning_rate
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.batch_size == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "batchSize".to_string(),
                reason: "batchSize must be > 0".to_string(),
            });
        }
        if self.min_epochs == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "minEpochs".to_string(),
                reason: "minEpochs must be >= 1".to_string(),
            });
        }
        if self.max_epochs < self.min_epochs {
            return Err(ConfigError::InvalidParameter {
                parameter: "maxEpochs".to_string(),
                reason: "maxEpochs must be >= minEpochs".to_string(),
            });
        }
        if self.learning_rate <= 0.0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "learningRate".to_string(),
                reason: "learningRate must be > 0".to_string(),
            });
        }
        if self.tolerance < 0.0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "tolerance".to_string(),
                reason: "tolerance must be >= 0".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for GradientDescentConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        GradientDescentConfig::validate(self)
    }
}

impl Default for GradientDescentConfig {
    fn default() -> Self {
        Self {
            batch_size: 100,
            min_epochs: 1,
            patience: 1,
            max_epochs: 100,
            tolerance: 1e-3,
            learning_rate: 0.001,
        }
    }
}
