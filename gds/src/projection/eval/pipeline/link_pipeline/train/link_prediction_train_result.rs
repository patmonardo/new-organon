// Phase 1.5: LinkPredictionTrainResult - Training output for link prediction

use crate::ml::models::Classifier;
use crate::ml::training::statistics::TrainingStatistics;

/// Result of link prediction model training.
///
/// Contains:
/// - **Classifier**: Trained binary classification model (LogisticRegression, RandomForest, etc.)
/// - **TrainingStatistics**: Training metrics, convergence history, hyperparameters
///
/// # Link Prediction Training
///
/// Link prediction trains a binary classifier to distinguish:
/// - Positive examples: Existing relationships (label = 1)
/// - Negative examples: Non-existent relationships from negative sampling (label = 0)
///
/// The classifier learns from link features (Hadamard, Cosine, L2, etc.) computed
/// on node property pairs.
///
/// # Example Flow
///
/// ```text
/// LinkPredictionTrain::compute()
///   ↓
/// 1. Extract features & labels → FeaturesAndLabels
/// 2. Train classifier → Classifier
/// 3. Track statistics → TrainingStatistics
/// 4. Return → LinkPredictionTrainResult
/// ```
#[derive(Debug)]
pub struct LinkPredictionTrainResult {
    /// Trained binary classifier
    classifier: Box<dyn Classifier>,

    /// Training metrics and convergence history
    training_statistics: TrainingStatistics,
}

impl LinkPredictionTrainResult {
    /// Creates a new LinkPredictionTrainResult.
    ///
    /// # Arguments
    ///
    /// * `classifier` - Trained binary classification model
    /// * `training_statistics` - Training metrics and history
    pub fn new(classifier: Box<dyn Classifier>, training_statistics: TrainingStatistics) -> Self {
        Self {
            classifier,
            training_statistics,
        }
    }

    /// Returns the trained classifier.
    pub fn classifier(&self) -> &dyn Classifier {
        &*self.classifier
    }

    pub fn into_classifier(self) -> Box<dyn Classifier> {
        self.classifier
    }

    /// Returns the training statistics.
    pub fn training_statistics(&self) -> &TrainingStatistics {
        &self.training_statistics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::core::tensor::Matrix;
    use crate::ml::metrics::classification::GlobalAccuracy;
    use crate::ml::metrics::Metric;
    use crate::ml::models::base::{BaseModelData, ClassifierData};
    use crate::ml::models::training_method::TrainingMethod;
    use crate::ml::models::Features;
    use std::any::Any;

    #[test]
    fn test_train_result_creation() {
        #[derive(Debug)]
        struct TestClassifierData;

        impl BaseModelData for TestClassifierData {
            fn trainer_method(&self) -> TrainingMethod {
                TrainingMethod::LogisticRegression
            }

            fn feature_dimension(&self) -> usize {
                1
            }

            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        impl ClassifierData for TestClassifierData {
            fn number_of_classes(&self) -> usize {
                2
            }
        }

        #[derive(Debug)]
        struct TestClassifier;

        impl Classifier for TestClassifier {
            fn data(&self) -> &dyn ClassifierData {
                &TestClassifierData
            }

            fn predict_probabilities(&self, _features: &[f64]) -> Vec<f64> {
                vec![0.5, 0.5]
            }

            fn predict_probabilities_batch(
                &self,
                batch: &[usize],
                _features: &dyn Features,
            ) -> Matrix {
                Matrix::new(vec![0.5; batch.len() * 2], batch.len(), 2)
            }
        }

        let classifier = Box::new(TestClassifier);
        let metrics: Vec<Box<dyn Metric>> = vec![Box::new(GlobalAccuracy::new())];
        let stats = TrainingStatistics::new(&metrics);
        let result = LinkPredictionTrainResult::new(classifier, stats);

        let _classifier = result.classifier();
        let _stats = result.training_statistics();
    }

    #[test]
    fn test_accessors() {
        #[derive(Debug)]
        struct TestClassifierData;

        impl BaseModelData for TestClassifierData {
            fn trainer_method(&self) -> TrainingMethod {
                TrainingMethod::LogisticRegression
            }

            fn feature_dimension(&self) -> usize {
                1
            }

            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        impl ClassifierData for TestClassifierData {
            fn number_of_classes(&self) -> usize {
                2
            }
        }

        #[derive(Debug)]
        struct TestClassifier;

        impl Classifier for TestClassifier {
            fn data(&self) -> &dyn ClassifierData {
                &TestClassifierData
            }

            fn predict_probabilities(&self, _features: &[f64]) -> Vec<f64> {
                vec![0.5, 0.5]
            }

            fn predict_probabilities_batch(
                &self,
                batch: &[usize],
                _features: &dyn Features,
            ) -> Matrix {
                Matrix::new(vec![0.5; batch.len() * 2], batch.len(), 2)
            }
        }

        let metrics: Vec<Box<dyn Metric>> = vec![Box::new(GlobalAccuracy::new())];
        let result = LinkPredictionTrainResult::new(
            Box::new(TestClassifier),
            TrainingStatistics::new(&metrics),
        );

        let _ = result.classifier();
        let _ = result.training_statistics();
    }

    #[test]
    fn test_multiple_results() {
        // Simulating multiple training runs (e.g., hyperparameter search)
        #[derive(Debug)]
        struct TestClassifierData;

        impl BaseModelData for TestClassifierData {
            fn trainer_method(&self) -> TrainingMethod {
                TrainingMethod::LogisticRegression
            }

            fn feature_dimension(&self) -> usize {
                1
            }

            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        impl ClassifierData for TestClassifierData {
            fn number_of_classes(&self) -> usize {
                2
            }
        }

        #[derive(Debug)]
        struct TestClassifier;

        impl Classifier for TestClassifier {
            fn data(&self) -> &dyn ClassifierData {
                &TestClassifierData
            }

            fn predict_probabilities(&self, _features: &[f64]) -> Vec<f64> {
                vec![0.5, 0.5]
            }

            fn predict_probabilities_batch(
                &self,
                batch: &[usize],
                _features: &dyn Features,
            ) -> Matrix {
                Matrix::new(vec![0.5; batch.len() * 2], batch.len(), 2)
            }
        }

        let metrics: Vec<Box<dyn Metric>> = vec![Box::new(GlobalAccuracy::new())];

        let results: Vec<LinkPredictionTrainResult> = (0..5)
            .map(|_| {
                LinkPredictionTrainResult::new(
                    Box::new(TestClassifier),
                    TrainingStatistics::new(&metrics),
                )
            })
            .collect();

        assert_eq!(results.len(), 5);
    }
}
