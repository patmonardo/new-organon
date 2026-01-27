//! Linear Regressor implementation.
//!
//! Direct translation of `LinearRegressor.java` from Java GDS.

use crate::ml::core::functions::{EWiseAddMatrixScalar, MatrixMultiplyWithTransposedSecondOperand};
use crate::ml::core::VariableRef;
use crate::ml::models::Regressor;
use crate::ml::models::RegressorData;
use std::sync::Arc;

use super::data::LinearRegressionData;

/// Prediction component for linear regression.
#[derive(Debug, Clone)]
pub struct LinearRegressor {
    data: LinearRegressionData,
}

impl LinearRegressor {
    /// Create a regressor backed by the supplied model data.
    pub fn new(data: LinearRegressionData) -> Self {
        Self { data }
    }

    /// Accessor for the underlying model data.
    pub fn data(&self) -> &LinearRegressionData {
        &self.data
    }

    /// Build the computation-graph variable producing predictions for a batch of features.
    /// Mirrors `LinearRegressor.predictionsVariable(Variable<Matrix> features)`.
    /// Returns a Variable<Matrix> containing predictions for the batch.
    pub fn predictions_variable(&self, features: VariableRef) -> VariableRef {
        let weights_var: VariableRef = self.data.weights().clone();
        let weighted_features: VariableRef = Arc::new(
            MatrixMultiplyWithTransposedSecondOperand::new_ref(features, weights_var),
        );

        let bias_var: VariableRef = self.data.bias().clone();
        Arc::new(EWiseAddMatrixScalar::new_ref(weighted_features, bias_var))
    }

    /// Build predictions using provided weight variables (for gradient computation)
    pub fn predictions_variable_with_weights(
        &self,
        features: VariableRef,
        weights_var: VariableRef,
        bias_var: VariableRef,
    ) -> VariableRef {
        let weighted_features: VariableRef = Arc::new(
            MatrixMultiplyWithTransposedSecondOperand::new_ref(features, weights_var),
        );

        Arc::new(EWiseAddMatrixScalar::new_ref(weighted_features, bias_var))
    }
}

impl Regressor for LinearRegressor {
    fn data(&self) -> &dyn RegressorData {
        &self.data
    }

    fn predict(&self, features: &[f64]) -> f64 {
        let matrix_ref = self.data.weights().borrow_matrix();
        let bias_ref = self.data.bias().borrow_scalar();

        let mut prediction = 0.0;
        for (i, &feature) in features.iter().enumerate() {
            prediction += matrix_ref[(0, i)] * feature;
        }

        prediction + bias_ref.value()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
