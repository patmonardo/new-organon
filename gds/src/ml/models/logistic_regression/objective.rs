use super::classifier::LogisticRegressionClassifier;
use super::data::LogisticRegressionData;
use crate::ml::core::batch::Batch;
use crate::ml::core::functions::constant::Constant;
use crate::ml::core::functions::constant_scale::ConstantScale;
use crate::ml::core::functions::element_sum::ElementSum;
use crate::ml::core::functions::l2_norm_squared::L2NormSquared;
use crate::ml::core::functions::reduced_cross_entropy_loss::ReducedCrossEntropyLoss;
use crate::ml::core::functions::reduced_focal_loss::ReducedFocalLoss;
use crate::ml::core::functions::weights::Weights;
use crate::ml::core::variable::VariableRef;
use crate::ml::gradient_descent::batch_feature_matrix;
use crate::ml::gradient_descent::Objective;
use crate::ml::models::Features;
use parking_lot::RwLock;
use std::sync::Arc;

/// Objective function for logistic regression training
pub struct LogisticRegressionObjective<'a> {
    classifier: LogisticRegressionClassifier,
    penalty: f64,
    features: &'a dyn Features,
    labels: Arc<RwLock<Vec<i32>>>,
    focus_weight: f64,
    class_weights: Vec<f64>,
}

impl<'a> LogisticRegressionObjective<'a> {
    /// Creates a new LogisticRegressionObjective
    pub fn new(
        classifier: LogisticRegressionClassifier,
        penalty: f64,
        features: &'a dyn Features,
        labels: Arc<RwLock<Vec<i32>>>,
        focus_weight: f64,
        class_weights: Vec<f64>,
    ) -> Self {
        assert!(features.size() > 0, "Features cannot be empty");

        Self {
            classifier,
            penalty,
            features,
            labels,
            focus_weight,
            class_weights,
        }
    }

    /// Computes the penalty term for the batch
    fn penalty_for_batch<B: Batch>(&self, batch: &B, train_size: usize) -> VariableRef {
        let weights: VariableRef = self.classifier.data().weights().clone();
        let penalty_variable: VariableRef = Arc::new(L2NormSquared::new_ref(weights));
        let scale = (batch.size() as f64) * self.penalty / (train_size as f64);
        Arc::new(ConstantScale::new_ref(penalty_variable, scale))
    }

    /// Computes the cross-entropy loss for the batch
    fn cross_entropy_loss<B: Batch>(&self, batch: &B) -> VariableRef {
        let batch_labels = self.batch_label_vector(batch);
        let batch_features: VariableRef = Arc::new(batch_feature_matrix(batch, self.features));
        let predictions = self.classifier.predictions_variable(batch_features.clone());

        let weights: VariableRef = self.classifier.data().weights().clone();
        let bias: VariableRef = self.classifier.data().bias().clone();

        if self.focus_weight == 0.0 {
            Arc::new(ReducedCrossEntropyLoss::new_ref(
                predictions,
                weights,
                bias,
                batch_features,
                batch_labels,
                self.class_weights.clone(),
            ))
        } else {
            Arc::new(ReducedFocalLoss::new_ref(
                predictions,
                weights,
                bias,
                batch_features,
                batch_labels,
                self.focus_weight,
                self.class_weights.clone(),
            ))
        }
    }

    /// Creates a vector of labels for the batch
    fn batch_label_vector<B: Batch>(&self, batch: &B) -> VariableRef {
        let labels = self.labels.read();
        let mut batched_targets = Vec::with_capacity(batch.size());

        for element_id in batch.element_ids() {
            batched_targets.push(labels[element_id as usize] as f64);
        }

        Arc::new(Constant::vector(batched_targets))
    }
}

impl<'a> Objective for LogisticRegressionObjective<'a> {
    type ModelData = LogisticRegressionData;

    fn weights(&self) -> Vec<Arc<Weights>> {
        vec![
            self.classifier.data().weights().clone(),
            self.classifier.data().bias().clone(),
        ]
    }

    fn loss<B: Batch>(&self, batch: &B, train_size: usize) -> VariableRef {
        let unpenalized_loss = self.cross_entropy_loss(batch);
        let penalty_variable = self.penalty_for_batch(batch, train_size);

        Arc::new(ElementSum::new_ref(vec![
            unpenalized_loss,
            penalty_variable,
        ]))
    }

    fn model_data(&self) -> &Self::ModelData {
        self.classifier.data()
    }
}
