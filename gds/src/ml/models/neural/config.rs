//! MLP Classifier Training Configuration
//!
//! Translated from `MLPClassifierTrainConfig.java` from Java GDS.

use crate::ml::models::TrainingMethod;
use crate::ml::models::base::TrainerConfigTrait;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

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

// Remove trait implementations - these are structs, not traits

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

impl TrainerConfigTrait for MLPClassifierTrainConfig {
    fn method(&self) -> TrainingMethod {
        self.method
    }

    fn to_map(&self) -> std::collections::HashMap<String, serde_json::Value> {
        // TODO: Implement serialization
        std::collections::HashMap::new()
    }
}
