//! MLP Classifier Trainer
//!
//! Translated from `MLPClassifierTrainer.java` from Java GDS.

use crate::collections::HugeIntArray;
use crate::ml::core::batch::from_array;
use crate::ml::gradient_descent::GradientDescentConfig;
use crate::ml::gradient_descent::Training;
use crate::ml::models::Classifier;
use crate::ml::models::ClassifierTrainer;
use crate::ml::models::Features;
use parking_lot::RwLock;
use rand::rngs::StdRng;
use rand::RngCore;
use rand::SeedableRng;
use std::sync::Arc;

use super::{
    classifier::MLPClassifier, config::MLPClassifierTrainConfig, data::MLPClassifierData,
    objective::MLPClassifierObjective,
};

/// Trainer for MLP Classifier
///
/// This corresponds to MLPClassifierTrainer in Java GDS.
/// Uses the gradient descent system for training.
pub struct MLPClassifierTrainer {
    number_of_classes: usize,
    train_config: MLPClassifierTrainConfig,
    random_seed: Option<u64>,
    _termination_flag: Arc<RwLock<bool>>,
    concurrency: usize,
}

impl MLPClassifierTrainer {
    /// Create a new MLP classifier trainer
    ///
    /// Java: `public MLPClassifierTrainer(int numberOfClasses, MLPClassifierTrainConfig trainConfig, Optional<Long> randomSeed, ...)`
    pub fn new(
        number_of_classes: usize,
        train_config: MLPClassifierTrainConfig,
        random_seed: Option<u64>,
        concurrency: usize,
    ) -> Self {
        Self {
            number_of_classes,
            train_config,
            random_seed,
            _termination_flag: Arc::new(RwLock::new(false)),
            concurrency,
        }
    }
    ///
    /// Java: `public MLPClassifier train(Features features, HugeIntArray labels, ReadOnlyHugeLongArray trainSet)`
    pub fn train(
        &mut self,
        features: &dyn Features,
        labels: &HugeIntArray,
        train_set: &Arc<Vec<u64>>,
    ) -> MLPClassifier {
        let mut random = if let Some(seed) = self.random_seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::from_entropy()
        };

        // Create MLP classifier data
        let data = MLPClassifierData::create(
            self.number_of_classes,
            features.feature_dimension(),
            &self.train_config.hidden_layer_sizes,
            random.next_u64(),
        );

        let classifier = MLPClassifier::new(data);

        // Create objective function
        let objective = MLPClassifierObjective::new(
            classifier,
            features,
            labels,
            self.train_config.penalty,
            self.train_config.focus_weight,
            self.train_config
                .initialize_class_weights(self.number_of_classes),
        );

        // Create training instance
        let gradient_config = GradientDescentConfig::builder()
            .batch_size(self.train_config.batch_size)
            .min_epochs(self.train_config.min_epochs)
            .patience(self.train_config.patience)
            .max_epochs(self.train_config.max_epochs)
            .tolerance(self.train_config.tolerance)
            .learning_rate(self.train_config.learning_rate)
            .build()
            .unwrap();

        let training = Training::new(
            gradient_config,
            train_set.len(),
            Arc::clone(&self._termination_flag),
        );

        // Create batch queue supplier (matches Java's BatchQueue.fromArray(trainSet, batchSize))
        let train_ids = Arc::clone(train_set);
        let batch_size = self.train_config.batch_size;
        let queue_supplier = move || from_array(Arc::clone(&train_ids), batch_size);

        // Train the model
        training.train(&objective, queue_supplier, self.concurrency);

        // Return the trained classifier
        objective.classifier
    }

    /// Get the training configuration
    pub fn train_config(&self) -> &MLPClassifierTrainConfig {
        &self.train_config
    }

    /// Get the number of classes
    pub fn number_of_classes(&self) -> usize {
        self.number_of_classes
    }
}

impl ClassifierTrainer for MLPClassifierTrainer {
    fn train(
        &self,
        features: &dyn Features,
        labels: &HugeIntArray,
        train_set: &Arc<Vec<u64>>,
    ) -> Box<dyn Classifier> {
        let mut random = if let Some(seed) = self.random_seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::from_entropy()
        };

        // Create MLP classifier data
        let data = MLPClassifierData::create(
            self.number_of_classes,
            features.feature_dimension(),
            &self.train_config.hidden_layer_sizes,
            random.next_u64(),
        );

        let classifier = MLPClassifier::new(data);

        // Create objective function
        let objective = MLPClassifierObjective::new(
            classifier,
            features,
            labels,
            self.train_config.penalty,
            self.train_config.focus_weight,
            self.train_config
                .initialize_class_weights(self.number_of_classes),
        );

        // Create training instance
        let gradient_config = GradientDescentConfig::builder()
            .batch_size(self.train_config.batch_size)
            .min_epochs(self.train_config.min_epochs)
            .patience(self.train_config.patience)
            .max_epochs(self.train_config.max_epochs)
            .tolerance(self.train_config.tolerance)
            .learning_rate(self.train_config.learning_rate)
            .build()
            .unwrap();

        let training = Training::new(
            gradient_config,
            train_set.len(),
            Arc::clone(&self._termination_flag),
        );

        // Create batch queue supplier (matches Java's BatchQueue.fromArray(trainSet, batchSize))
        let train_ids = Arc::clone(train_set);
        let batch_size = self.train_config.batch_size;
        let queue_supplier = move || from_array(Arc::clone(&train_ids), batch_size);

        // Train the model
        training.train(&objective, queue_supplier, self.concurrency);

        // Return the trained classifier
        Box::new(objective.classifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trainer_creation() {
        use crate::ml::models::TrainingMethod;
        let config = MLPClassifierTrainConfig::default();
        let trainer = MLPClassifierTrainer::new(3, config, Some(42), 1);

        assert_eq!(trainer.number_of_classes(), 3);
        assert_eq!(
            trainer.train_config().method,
            TrainingMethod::MLPClassification
        );
    }

    #[test]
    fn test_trainer_with_custom_config() {
        let config = MLPClassifierTrainConfig::builder()
            .batch_size(50)
            .max_epochs(10)
            .learning_rate(0.01)
            .hidden_layer_sizes(vec![64, 32])
            .build()
            .unwrap();

        let trainer = MLPClassifierTrainer::new(2, config, Some(123), 2);

        assert_eq!(trainer.number_of_classes(), 2);
        assert_eq!(trainer.train_config().batch_size, 50);
        assert_eq!(trainer.train_config().max_epochs, 10);
        assert_eq!(trainer.train_config().learning_rate, 0.01);
        assert_eq!(trainer.train_config().hidden_layer_sizes(), &vec![64, 32]);
    }

    #[test]
    fn test_trainer_without_seed() {
        let config = MLPClassifierTrainConfig::default();
        let trainer = MLPClassifierTrainer::new(3, config, None, 1);

        assert_eq!(trainer.number_of_classes(), 3);
    }

    #[test]
    fn test_training_integration() {
        // Simple test features
        struct TestFeatures {
            data: Vec<Vec<f64>>,
        }
        impl TestFeatures {
            fn new() -> Self {
                Self {
                    // Need 4 feature vectors for train_set with 4 elements
                    data: vec![
                        vec![1.0, 2.0, 3.0],
                        vec![4.0, 5.0, 6.0],
                        vec![7.0, 8.0, 9.0],
                        vec![10.0, 11.0, 12.0],
                    ],
                }
            }
        }
        impl Features for TestFeatures {
            fn get(&self, node_id: usize) -> &[f64] {
                &self.data[node_id]
            }

            fn feature_dimension(&self) -> usize {
                3
            }

            fn size(&self) -> usize {
                self.data.len()
            }
        }

        let config = MLPClassifierTrainConfig::builder()
            .max_epochs(1) // Just one epoch for testing
            .batch_size(2)
            .build()
            .unwrap();

        let trainer = MLPClassifierTrainer::new(2, config, Some(456), 1);
        let features = TestFeatures::new();
        let labels = HugeIntArray::from_vec(vec![0, 1, 0, 1]);
        let train_set = Arc::new(vec![0, 1, 2, 3]);

        let classifier = {
            let trainer_mut = trainer;
            trainer_mut.train(&features, &labels, &train_set)
        };

        assert_eq!(classifier.data().number_of_classes(), 2);
        assert_eq!(classifier.data().feature_dimension(), 3);

        // Test prediction
        let test_features = vec![1.0, 2.0, 3.0];
        let probabilities = classifier.predict_probabilities(&test_features);

        assert_eq!(probabilities.len(), 2);
        let sum: f64 = probabilities.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10); // Should sum to 1.0 due to softmax
    }

    #[test]
    fn test_train_set_batch_queue_preserves_ids() {
        let train_set = Arc::new(vec![10, 20, 30, 40, 50]);
        let mut queue = from_array(Arc::clone(&train_set), 2);

        let batch1 = queue.pop().unwrap();
        let ids1: Vec<u64> = batch1.element_ids_boxed().collect();
        assert_eq!(ids1, vec![10, 20]);

        let batch2 = queue.pop().unwrap();
        let ids2: Vec<u64> = batch2.element_ids_boxed().collect();
        assert_eq!(ids2, vec![30, 40]);

        let batch3 = queue.pop().unwrap();
        let ids3: Vec<u64> = batch3.element_ids_boxed().collect();
        assert_eq!(ids3, vec![50]);

        assert!(queue.pop().is_none());
    }
}
