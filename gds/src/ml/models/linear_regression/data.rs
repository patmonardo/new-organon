//! Linear Regression model data structure.
//!
//! 1:1 translation of `LinearRegressionData.java` from Java GDS.

use crate::ml::core::functions::Weights;
use crate::ml::core::tensor::Matrix;
use crate::ml::core::tensor::Scalar;
use crate::ml::core::tensor::Tensor;
use crate::ml::models::BaseModelData;
use crate::ml::models::ModelData;
use crate::ml::models::RegressorData;
use crate::ml::models::TrainingMethod;
use anyhow::anyhow;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;

/// Stored parameters for a trained linear regression model.
#[derive(Clone)]
pub struct LinearRegressionData {
    weights: Arc<Weights>,
    bias: Arc<Weights>,
    feature_dimension: usize,
}

impl std::fmt::Debug for LinearRegressionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinearRegressionData")
            .field("feature_dimension", &self.feature_dimension)
            .finish()
    }
}

impl LinearRegressionData {
    /// Factory method mirroring `LinearRegressionData.of(int featureDimension)` in Java.
    pub fn of(feature_dimension: usize) -> Self {
        Self {
            weights: Arc::new(Weights::of_matrix(1, feature_dimension)),
            bias: Arc::new(Weights::of_scalar(0.0)),
            feature_dimension,
        }
    }

    /// Trainable weight matrix (shape 1 × feature_dimension).
    pub fn weights(&self) -> &Arc<Weights> {
        &self.weights
    }

    /// Trainable bias scalar.
    pub fn bias(&self) -> &Arc<Weights> {
        &self.bias
    }
}

impl BaseModelData for LinearRegressionData {
    fn trainer_method(&self) -> TrainingMethod {
        TrainingMethod::LinearRegression
    }

    fn feature_dimension(&self) -> usize {
        self.feature_dimension
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl RegressorData for LinearRegressionData {}

impl ModelData for LinearRegressionData {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let weight_snapshot = self.snapshot_weights();
        let bias_snapshot = self.snapshot_bias();

        let payload = LinearRegressionDataSnapshot {
            weights: weight_snapshot.data,
            rows: weight_snapshot.rows,
            cols: weight_snapshot.cols,
            bias: bias_snapshot,
            feature_dimension: self.feature_dimension,
        };

        bincode::serialize(&payload)
            .map_err(|err| anyhow!("LinearRegressionData serialization failed: {err}"))
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let snapshot: LinearRegressionDataSnapshot = bincode::deserialize(bytes)
            .map_err(|err| anyhow!("LinearRegressionData deserialization failed: {err}"))?;

        let weights_matrix = Matrix::new(snapshot.weights, snapshot.rows, snapshot.cols);
        let bias_scalar = Scalar::new(snapshot.bias);

        Ok(Self {
            weights: Arc::new(Weights::from_tensor(Box::new(weights_matrix))),
            bias: Arc::new(Weights::from_tensor(Box::new(bias_scalar))),
            feature_dimension: snapshot.feature_dimension,
        })
    }

    fn num_features(&self) -> usize {
        self.feature_dimension
    }
}

#[derive(Serialize, Deserialize)]
struct LinearRegressionDataSnapshot {
    weights: Vec<f64>,
    rows: usize,
    cols: usize,
    bias: f64,
    feature_dimension: usize,
}

struct WeightSnapshot {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl LinearRegressionData {
    fn snapshot_weights(&self) -> WeightSnapshot {
        let matrix_ref = self.weights.borrow_matrix();
        WeightSnapshot {
            data: matrix_ref.data().to_vec(),
            rows: matrix_ref.rows(),
            cols: matrix_ref.cols(),
        }
    }

    fn snapshot_bias(&self) -> f64 {
        let scalar_ref = self.bias.borrow_scalar();
        scalar_ref.value()
    }
}
