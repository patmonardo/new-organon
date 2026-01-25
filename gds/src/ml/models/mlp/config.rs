//! MLP Classifier Training Configuration
//!
//! Translated from `MLPClassifierTrainConfig.java` from Java GDS.

use crate::config::validation::ConfigError;
use crate::ml::models::base::TrainerConfigTrait;
use crate::ml::models::TrainingMethod;
use derive_builder::Builder;
use serde::Deserialize;
use serde::Serialize;

/// Configuration for MLP Classifier training
///
/// This corresponds to MLPClassifierTrainConfig in Java GDS.
/// Combines gradient descent, penalty, and class-aware configurations.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub struct MLPClassifierTrainConfig {
    // Gradient Descent Configuration
    #[builder(default = "100")]
    pub batch_size: usize,

    #[builder(default = "1")]
    pub min_epochs: usize,

    #[builder(default = "1")]
    pub patience: usize,

    #[builder(default = "100")]
    pub max_epochs: usize,

    #[builder(default = "1e-3")]
    pub tolerance: f64,

    #[builder(default = "0.001")]
    pub learning_rate: f64,

    // Penalty Configuration
    #[builder(default = "0.0")]
    pub penalty: f64,

    // Class-Aware Configuration
    #[builder(default = "0.0")]
    pub focus_weight: f64,

    #[builder(default)]
    pub class_weights: Vec<f64>,

    // MLP-Specific Configuration
    #[builder(default = "vec![100]")]
    pub hidden_layer_sizes: Vec<usize>,

    // Training method
    #[builder(default = "TrainingMethod::MLPClassification")]
    pub method: TrainingMethod,
}

impl Default for MLPClassifierTrainConfig {
    fn default() -> Self {
        Self {
            batch_size: 100,
            min_epochs: 1,
            patience: 1,
            max_epochs: 100,
            tolerance: 1e-3,
            learning_rate: 0.001,
            penalty: 0.0,
            focus_weight: 0.0,
            class_weights: Vec::new(),
            hidden_layer_sizes: vec![100],
            method: TrainingMethod::MLPClassification,
        }
    }
}

impl MLPClassifierTrainConfig {
    /// Create a new MLP classifier training configuration
    pub fn builder() -> MLPClassifierTrainConfigBuilder {
        MLPClassifierTrainConfigBuilder::default()
    }

    /// Get hidden layer sizes
    ///
    /// Java: `default List<Integer> hiddenLayerSizes() {return List.of(100);}`
    pub fn hidden_layer_sizes(&self) -> &Vec<usize> {
        &self.hidden_layer_sizes
    }

    /// Initialize class weights based on number of classes
    ///
    /// Java: `default double[] initializeClassWeights(int numberOfClasses)`
    /// Matches ClassAwareTrainerConfig.initializeClassWeights()
    pub fn initialize_class_weights(&self, number_of_classes: usize) -> Vec<f64> {
        if self.class_weights.is_empty() {
            vec![1.0; number_of_classes]
        } else {
            if self.class_weights.len() != number_of_classes {
                panic!(
                    "The classWeights list {:?} has {} entries, but it should have {} entries instead, which is the number of classes.",
                    self.class_weights,
                    self.class_weights.len(),
                    number_of_classes
                );
            }
            self.class_weights.clone()
        }
    }
}

impl MLPClassifierTrainConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.batch_size == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "batchSize".to_string(),
                reason: "batchSize must be > 0".to_string(),
            });
        }
        if self.max_epochs < self.min_epochs {
            return Err(ConfigError::InvalidParameter {
                parameter: "maxEpochs/minEpochs".to_string(),
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
        if self.hidden_layer_sizes.is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "hiddenLayerSizes".to_string(),
                reason: "hiddenLayerSizes must not be empty".to_string(),
            });
        }
        for &n in &self.hidden_layer_sizes {
            if n == 0 {
                return Err(ConfigError::InvalidParameter {
                    parameter: "hiddenLayerSizes".to_string(),
                    reason: "hiddenLayerSizes entries must be > 0".to_string(),
                });
            }
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for MLPClassifierTrainConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        MLPClassifierTrainConfig::validate(self)
    }
}

// Remove trait implementations - these are structs, not traits

impl TrainerConfigTrait for MLPClassifierTrainConfig {
    fn method(&self) -> TrainingMethod {
        self.method
    }

    fn to_map(&self) -> std::collections::HashMap<String, serde_json::Value> {
        let mut map = std::collections::HashMap::new();
        map.insert(
            "method".to_string(),
            serde_json::Value::String("MLPClassification".to_string()),
        );
        map.insert(
            "methodName".to_string(),
            serde_json::Value::String("MLPClassification".to_string()),
        );
        map.insert(
            "batch_size".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.batch_size)),
        );
        map.insert(
            "min_epochs".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.min_epochs)),
        );
        map.insert(
            "patience".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.patience)),
        );
        map.insert(
            "max_epochs".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.max_epochs)),
        );
        map.insert(
            "tolerance".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(self.tolerance).unwrap()),
        );
        map.insert(
            "learning_rate".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(self.learning_rate).unwrap()),
        );
        map.insert(
            "penalty".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(self.penalty).unwrap()),
        );
        map.insert(
            "focus_weight".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(self.focus_weight).unwrap()),
        );
        if !self.class_weights.is_empty() {
            let weights_array: Vec<serde_json::Value> = self
                .class_weights
                .iter()
                .map(|&w| serde_json::Value::Number(serde_json::Number::from_f64(w).unwrap()))
                .collect();
            map.insert(
                "class_weights".to_string(),
                serde_json::Value::Array(weights_array),
            );
        }
        let hidden_layers: Vec<serde_json::Value> = self
            .hidden_layer_sizes
            .iter()
            .map(|&v| serde_json::Value::Number(serde_json::Number::from(v)))
            .collect();
        map.insert(
            "hidden_layer_sizes".to_string(),
            serde_json::Value::Array(hidden_layers),
        );
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = MLPClassifierTrainConfig::default();

        assert_eq!(config.batch_size, 100);
        assert_eq!(config.min_epochs, 1);
        assert_eq!(config.patience, 1);
        assert_eq!(config.max_epochs, 100);
        assert_eq!(config.tolerance, 1e-3);
        assert_eq!(config.learning_rate, 0.001);
        assert_eq!(config.penalty, 0.0);
        assert_eq!(config.focus_weight, 0.0);
        assert_eq!(config.hidden_layer_sizes, vec![100]);
        assert_eq!(config.method, TrainingMethod::MLPClassification);
    }

    #[test]
    fn test_builder_config() {
        let config = MLPClassifierTrainConfig::builder()
            .batch_size(50)
            .max_epochs(200)
            .learning_rate(0.01)
            .penalty(0.1)
            .focus_weight(2.0)
            .hidden_layer_sizes(vec![64, 32])
            .build()
            .unwrap();

        assert_eq!(config.batch_size, 50);
        assert_eq!(config.max_epochs, 200);
        assert_eq!(config.learning_rate, 0.01);
        assert_eq!(config.penalty, 0.1);
        assert_eq!(config.focus_weight, 2.0);
        assert_eq!(config.hidden_layer_sizes, vec![64, 32]);
    }

    #[test]
    fn test_class_weights_initialization() {
        let config = MLPClassifierTrainConfig::default();
        let class_weights = config.initialize_class_weights(3);
        assert_eq!(class_weights, vec![1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_class_weights_custom() {
        let config = MLPClassifierTrainConfig::builder()
            .class_weights(vec![2.0, 3.0, 4.0])
            .build()
            .unwrap();
        let class_weights = config.initialize_class_weights(3);
        assert_eq!(class_weights, vec![2.0, 3.0, 4.0]);
    }

    #[test]
    #[should_panic(expected = "classWeights")]
    fn test_class_weights_mismatch() {
        let config = MLPClassifierTrainConfig::builder()
            .class_weights(vec![1.0, 2.0])
            .build()
            .unwrap();
        config.initialize_class_weights(3); // Should panic
    }
}
