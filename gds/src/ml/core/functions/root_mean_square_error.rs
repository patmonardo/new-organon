//! Root Mean Square Error (RMSE) loss function for ML in GDS.
//!
//! ## What is RMSE?
//! **Root Mean Square Error** is MSE with a square root - making it interpretable!
//!
//! - **Formula**: `RMSE = √((1/n) * Σ(prediction - target)²)`
//! - **Why Important**: Same units as target variable, easier to interpret than MSE
//! - **Used For**: Regression evaluation, model comparison, error analysis
//! - **Gradient**: `∇RMSE = (1/n) * (prediction - target) / RMSE` (scaled by self gradient)
//!
//! ## Mathematical Properties
//! - **Same Units as Target**: If targets are in meters, RMSE is in meters
//! - **Always Non-Negative**: RMSE ≥ 0 (square root of non-negative MSE)
//! - **Penalizes Large Errors**: Quadratic penalty like MSE
//! - **Interpretable**: "Average error magnitude" - intuitive to understand
//!
//! ## MSE vs RMSE
//! - **MSE**: Units are squared, larger values, used for optimization
//! - **RMSE**: Same units as target, smaller values, used for evaluation
//!
//! ## Usage
//! ```rust
//! let rmse = RootMeanSquareError::new(predictions, targets);
//! let loss = ctx.forward(&rmse);
//! ctx.backward(&rmse);
//! ```

use crate::ml::core::AbstractVariable;
use crate::ml::core::ComputationContext;
use crate::ml::core::dimensions;
use crate::ml::core::{Scalar, Vector, Matrix, Tensor};
use crate::ml::core::{Variable, VariableRef};
use std::fmt;

/// Root mean square error loss function.
///
/// Computes RMSE between predictions and targets.
/// Corresponds to RootMeanSquareError in Java GDS.
/// Uses composition pattern: VariableBase holds parents [predictions, targets].
pub struct RootMeanSquareError {
    base: AbstractVariable,
}

impl RootMeanSquareError {
    pub fn new(predictions: Box<dyn Variable>, targets: Box<dyn Variable>) -> Self {
        let predictions: VariableRef = predictions.into();
        let targets: VariableRef = targets.into();

        assert!(
            dimensions::is_vector(predictions.dimensions()),
            "Predictions need to be a vector"
        );
        assert!(dimensions::total_size(predictions.dimensions()) > 0);
        assert_eq!(
            dimensions::total_size(predictions.dimensions()),
            dimensions::total_size(targets.dimensions()),
            "Predictions and targets need to have the same total size"
        );

        let parents: Vec<VariableRef> = vec![predictions, targets];
        let dimensions = dimensions::scalar();
        let base = AbstractVariable::new(parents, dimensions);

        Self { base }
    }

    /// Helper to access predictions parent (index 0)
    fn predictions(&self) -> &dyn Variable {
        self.base.parents()[0].as_ref()
    }

    /// Helper to access targets parent (index 1)
    fn targets(&self) -> &dyn Variable {
        self.base.parents()[1].as_ref()
    }
}

impl Variable for RootMeanSquareError {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let predictions_data = ctx
            .data(self.predictions())
            .expect("Predictions not computed");
        let predictions = predictions_data
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Predictions must be Matrix");

        let targets_data = ctx.data(self.targets()).expect("Targets not computed");
        let targets = targets_data
            .as_any()
            .downcast_ref::<Vector>()
            .expect("Targets must be Vector");

        let mut squared_error_sum = 0.0;
        for idx in 0..targets.length() {
            // predictions is a Matrix that is vector-shaped; use flat indexing like Java.
            let error = predictions.data_at_flat(idx) - targets.data_at(idx);
            squared_error_sum += error * error;
        }

        if !squared_error_sum.is_finite() {
            return Box::new(Scalar::new(f64::MAX));
        }

        Box::new(Scalar::new(
            (squared_error_sum / targets.length() as f64).sqrt(),
        ))
    }

    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        if std::ptr::eq(parent, self.predictions()) {
            let predictions_data = ctx
                .data(self.predictions())
                .expect("Predictions not computed");
            let predictions = predictions_data
                .as_any()
                .downcast_ref::<Matrix>()
                .expect("Predictions must be Matrix");

            let targets_data = ctx.data(self.targets()).expect("Targets not computed");
            let number_of_examples = targets_data
                .as_any()
                .downcast_ref::<Vector>()
                .expect("Targets must be Vector")
                .length();

            let self_data = ctx.data(self).expect("Self data not computed");
            let root_of_sum_of_square_error_over_n = self_data
                .as_any()
                .downcast_ref::<Scalar>()
                .expect("Self data must be Scalar");

            let _parent_data = ctx.data(parent).expect("Parent data not computed");

            if root_of_sum_of_square_error_over_n.value() == 0.0 {
                return Box::new(Matrix::with_dimensions(
                    predictions.rows(),
                    predictions.cols(),
                ));
            }

            let denominator = root_of_sum_of_square_error_over_n
                .scalar_multiply(number_of_examples as f64)
                .as_any()
                .downcast_ref::<Scalar>()
                .expect("Result must be Scalar")
                .value();

            let self_gradient_data = ctx.gradient(self).expect("Self gradient not computed");
            let self_gradient = self_gradient_data
                .as_any()
                .downcast_ref::<Scalar>()
                .expect("Self gradient must be Scalar")
                .value();

            let scale = self_gradient / denominator;

            let targets = targets_data
                .as_any()
                .downcast_ref::<Vector>()
                .expect("Targets must be Vector");

            let mut parent_gradient =
                Matrix::with_dimensions(predictions.rows(), predictions.cols());
            for idx in 0..number_of_examples {
                let error = predictions.data_at_flat(idx) - targets.data_at(idx);
                parent_gradient.set_data_at_flat(idx, error * scale);
            }

            return Box::new(parent_gradient);
        }

        panic!(
            "The gradient should only be computed for the prediction parent, but got {}",
            parent
        );
    }

    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    fn parents(&self) -> &[VariableRef] {
        self.base.parents()
    }

    fn dimensions(&self) -> &[usize] {
        self.base.dimensions()
    }
}

impl fmt::Display for RootMeanSquareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RootMeanSquareError")
    }
}
