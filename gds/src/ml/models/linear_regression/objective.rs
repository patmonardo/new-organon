//! Linear Regression objective function.
//!
//! Literal translation of `LinearRegressionObjective.java` from Java GDS.

use crate::collections::HugeDoubleArray;
use crate::ml::core::functions::{
    Constant, ConstantScale, ElementSum, L2NormSquared, MeanSquareError, Weights,
};
use crate::ml::core::Batch;
use crate::ml::core::VariableRef;
use crate::ml::gradient_descent::batch_feature_matrix;
use crate::ml::gradient_descent::Objective;
use crate::ml::models::linear_regression::LinearRegressionData;
use crate::ml::models::linear_regression::LinearRegressor;
use crate::ml::models::Features;
use std::sync::Arc;

/// Objective used by gradient descent training of linear regression.
pub struct LinearRegressionObjective<'a> {
    features: &'a dyn Features,
    targets: &'a HugeDoubleArray,
    model_data: LinearRegressionData,
    penalty: f64,
}

impl<'a> LinearRegressionObjective<'a> {
    /// Create a new objective wrapping feature and target stores.
    pub fn new(features: &'a dyn Features, targets: &'a HugeDoubleArray, penalty: f64) -> Self {
        let model_data = LinearRegressionData::of(features.feature_dimension());
        Self {
            features,
            targets,
            model_data,
            penalty,
        }
    }

    fn penalty_for_batch<B: Batch>(&self, batch: &B, train_size: usize) -> VariableRef {
        let weights: VariableRef = self.model_data.weights().clone();
        let penalty_variable: VariableRef = Arc::new(L2NormSquared::new_ref(weights));
        let scale = (batch.size() as f64) * self.penalty / (train_size as f64);
        Arc::new(ConstantScale::new_ref(penalty_variable, scale))
    }

    fn batch_targets<B: Batch>(&self, batch: &B) -> VariableRef {
        let mut batched_targets = Vec::with_capacity(batch.size());
        for element_id in batch.element_ids() {
            batched_targets.push(self.targets.get(element_id as usize));
        }
        Arc::new(Constant::vector(batched_targets))
    }

    pub fn penalty(&self) -> f64 {
        self.penalty
    }
}

impl<'a> Objective for LinearRegressionObjective<'a> {
    type ModelData = LinearRegressionData;

    fn weights(&self) -> Vec<Arc<Weights>> {
        vec![
            self.model_data.weights().clone(),
            self.model_data.bias().clone(),
        ]
    }

    fn loss<B: Batch>(&self, batch: &B, train_size: usize) -> VariableRef {
        let batch_features: VariableRef = Arc::new(batch_feature_matrix(batch, self.features));
        let regressor = LinearRegressor::new(self.model_data.clone());

        // Use the same Weights instances that are returned by weights() method
        let weights_var: VariableRef = self.model_data.weights().clone();
        let bias_var: VariableRef = self.model_data.bias().clone();
        let predictions =
            regressor.predictions_variable_with_weights(batch_features, weights_var, bias_var);

        let targets = self.batch_targets(batch);

        let mse: VariableRef = Arc::new(MeanSquareError::new_ref(predictions, targets));
        let penalty = self.penalty_for_batch(batch, train_size);

        Arc::new(ElementSum::new_ref(vec![mse, penalty]))
    }

    fn model_data(&self) -> &Self::ModelData {
        &self.model_data
    }
}
