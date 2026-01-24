//! MLP Classifier Objective
//!
//! Translated from `MLPClassifierObjective.java` from Java GDS.

use super::classifier::MLPClassifier;
use super::data::MLPClassifierData;
use crate::collections::HugeIntArray;
use crate::ml::core::batch::Batch;
use crate::ml::core::functions::Constant;
use crate::ml::core::functions::ConstantScale;
use crate::ml::core::functions::CrossEntropyLoss;
use crate::ml::core::functions::ElementSum;
use crate::ml::core::functions::FocalLoss;
use crate::ml::core::functions::L2NormSquared;
use crate::ml::core::functions::Weights;
use crate::ml::core::tensor::Vector;
use crate::ml::core::variable::VariableRef;
use crate::ml::gradient_descent::batch_feature_matrix;
use crate::ml::gradient_descent::Objective;
use crate::ml::models::Features;
use std::sync::Arc;

/// Objective function for MLP classifier training
///
/// This corresponds to MLPClassifierObjective in Java GDS.
/// Implements the Objective trait for gradient descent training.
pub struct MLPClassifierObjective<'a> {
    pub classifier: MLPClassifier,
    features: &'a dyn Features,
    labels: &'a HugeIntArray,
    penalty: f64,
    focus_weight: f64,
    class_weights: Vec<f64>,
}

impl<'a> MLPClassifierObjective<'a> {
    /// Create a new MLP classifier objective
    ///
    /// Java: `public MLPClassifierObjective(MLPClassifier classifier, Features features, HugeIntArray labels, double penalty, double focusWeight, double[] classWeights)`
    pub fn new(
        classifier: MLPClassifier,
        features: &'a dyn Features,
        labels: &'a HugeIntArray,
        penalty: f64,
        focus_weight: f64,
        class_weights: Vec<f64>,
    ) -> Self {
        Self {
            classifier,
            features,
            labels,
            penalty,
            focus_weight,
            class_weights,
        }
    }

    /// Compute cross-entropy loss for a batch
    ///
    /// Java: `CrossEntropyLoss crossEntropyLoss(Batch batch)`
    fn cross_entropy_loss<B: Batch>(&self, batch: &B) -> VariableRef {
        let batch_labels = self.batch_label_vector(batch);
        let batch_features: VariableRef = Arc::new(batch_feature_matrix(batch, self.features));
        let predictions = self.classifier.predictions_variable(batch_features);

        if self.focus_weight == 0.0 {
            Arc::new(CrossEntropyLoss::new_ref(
                predictions,
                batch_labels,
                self.class_weights.clone(),
            ))
        } else {
            Arc::new(FocalLoss::new_ref(
                predictions,
                batch_labels,
                self.focus_weight,
                self.class_weights.clone(),
            ))
        }
    }

    /// Compute L2 penalty for a batch
    ///
    /// Java: `ConstantScale<Scalar> penaltyForBatch(Batch batch, long trainSize)`
    fn penalty_for_batch<B: Batch>(&self, batch: &B, train_size: usize) -> VariableRef {
        let l2_norms: Vec<VariableRef> = self
            .classifier
            .data()
            .weights()
            .iter()
            .map(|weights| {
                let w: VariableRef = weights.clone();
                Arc::new(L2NormSquared::new_ref(w)) as VariableRef
            })
            .collect();

        let penalty_sum: VariableRef = Arc::new(ElementSum::new_ref(l2_norms));
        let scale = (batch.size() as f64) * self.penalty / (train_size as f64);

        Arc::new(ConstantScale::new_ref(penalty_sum, scale))
    }

    /// Create batch label vector
    ///
    /// Java: `Constant<Vector> batchLabelVector(Batch batch)`
    fn batch_label_vector<B: Batch>(&self, batch: &B) -> VariableRef {
        let mut batched_labels = Vec::with_capacity(batch.size());

        for element_id in batch.element_ids() {
            batched_labels.push(self.labels.get(element_id as usize) as f64);
        }

        let vector = Vector::new(batched_labels);
        Arc::new(Constant::new(Box::new(vector)))
    }
}

impl<'a> Objective for MLPClassifierObjective<'a> {
    type ModelData = MLPClassifierData;

    /// Get all weights used in the computation graph
    ///
    /// Java: `public List<Weights<? extends Tensor<?>>> weights()`
    fn weights(&self) -> Vec<Arc<Weights>> {
        let mut combined_weights = Vec::new();

        // Add all weight matrices
        for weight in self.classifier.data().weights() {
            combined_weights.push(weight.clone());
        }

        // Add all bias vectors
        for bias in self.classifier.data().biases() {
            combined_weights.push(bias.clone());
        }

        combined_weights
    }

    /// Compute loss for a batch
    ///
    /// Java: `public Variable<Scalar> loss(Batch batch, long trainSize)`
    fn loss<B: Batch>(&self, batch: &B, train_size: usize) -> VariableRef {
        let cross_entropy_loss = self.cross_entropy_loss(batch);
        let penalty = self.penalty_for_batch(batch, train_size);

        Arc::new(ElementSum::new_ref(vec![cross_entropy_loss, penalty]))
    }

    /// Get the model data
    ///
    /// Java: `public MLPClassifierData modelData() {return classifier.data();}`
    fn model_data(&self) -> &Self::ModelData {
        self.classifier.data()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::core::batch::ListBatch;
    use crate::ml::core::batch::RangeBatch;
    use crate::ml::core::ComputationContext;

    #[test]
    fn test_mlp_objective_creation() {
        let data = MLPClassifierData::create(3, 5, &[10], 42);
        let classifier = MLPClassifier::new(data);

        struct TestFeatures {
            data: Vec<Vec<f64>>,
        }
        impl TestFeatures {
            fn new() -> Self {
                Self {
                    data: vec![vec![1.0, 2.0, 3.0, 4.0, 5.0]],
                }
            }
        }
        impl Features for TestFeatures {
            fn get(&self, node_id: usize) -> &[f64] {
                &self.data[node_id]
            }

            fn feature_dimension(&self) -> usize {
                5
            }

            fn size(&self) -> usize {
                self.data.len()
            }
        }

        let features = TestFeatures::new();
        let labels = HugeIntArray::from_vec(vec![0, 1, 2]);

        let objective = MLPClassifierObjective::new(
            classifier,
            &features,
            &labels,
            0.01,
            0.0,
            vec![1.0, 1.0, 1.0],
        );

        assert_eq!(objective.weights().len(), 4); // 2 weights + 2 biases
        assert_eq!(objective.model_data().number_of_classes(), 3);
    }

    #[test]
    fn test_loss_computation() {
        let data = MLPClassifierData::create(2, 3, &[4], 123);
        let classifier = MLPClassifier::new(data);

        struct TestFeatures {
            data: Vec<Vec<f64>>,
        }
        impl TestFeatures {
            fn new() -> Self {
                Self {
                    // Need 2 feature vectors for batch size 2
                    data: vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
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

        let features = TestFeatures::new();
        let labels = HugeIntArray::from_vec(vec![0, 1]);

        let objective =
            MLPClassifierObjective::new(classifier, &features, &labels, 0.01, 0.0, vec![1.0, 1.0]);

        let batch = RangeBatch::new(0, 2, 2);
        let loss_variable = objective.loss(&batch, 2);

        // Should be able to forward pass
        let ctx = ComputationContext::new();
        let loss_value = ctx.forward(loss_variable.as_ref());

        assert!(loss_value.aggregate_sum() > 0.0);
    }

    #[test]
    fn test_batch_label_vector() {
        let data = MLPClassifierData::create(2, 3, &[4], 456);
        let classifier = MLPClassifier::new(data);

        struct TestFeatures {
            data: Vec<Vec<f64>>,
        }
        impl TestFeatures {
            fn new() -> Self {
                Self {
                    data: vec![vec![1.0, 2.0, 3.0]],
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

        let features = TestFeatures::new();
        let labels = HugeIntArray::from_vec(vec![0, 1, 2, 0]);

        let objective =
            MLPClassifierObjective::new(classifier, &features, &labels, 0.01, 0.0, vec![1.0, 1.0]);

        let batch = RangeBatch::new(0, 2, 2);
        let label_vector = objective.batch_label_vector(&batch);

        // Should be able to forward pass
        let ctx = ComputationContext::new();
        let result = ctx.forward(label_vector.as_ref());

        assert_eq!(result.data().len(), 2);
    }

    #[test]
    fn test_batch_label_vector_respects_ids() {
        let data = MLPClassifierData::create(2, 3, &[4], 777);
        let classifier = MLPClassifier::new(data);

        struct TestFeatures {
            data: Vec<Vec<f64>>,
        }
        impl TestFeatures {
            fn new() -> Self {
                Self {
                    data: vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
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

        let features = TestFeatures::new();
        let labels = HugeIntArray::from_vec(vec![10, 11, 12, 13]);

        let objective =
            MLPClassifierObjective::new(classifier, &features, &labels, 0.01, 0.0, vec![1.0, 1.0]);

        let batch = ListBatch::new(vec![3, 1]);
        let label_vector = objective.batch_label_vector(&batch);

        let ctx = ComputationContext::new();
        let result = ctx.forward(label_vector.as_ref());

        assert_eq!(result.data(), &[13.0, 11.0]);
    }
}
