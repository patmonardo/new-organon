//! Element-wise add matrix and scalar for ML in GDS.
//!
//! Translated from Java GDS ml-core functions/EWiseAddMatrixScalar.java.
//! This is a literal 1:1 translation following repository translation policy.
//!
//! Java source:
//! ```java
//! /**
//!  * Corresponds to: result[i, j] = matrix[i, j] + scalar
//!  */
//! public class EWiseAddMatrixScalar extends AbstractVariable<Matrix> {
//!     private final Variable<Matrix> matrixVariable;
//!     private final Variable<Scalar> scalarVariable;
//!
//!     public EWiseAddMatrixScalar(Variable<Matrix> matrixVariable, Variable<Scalar> scalarVariable) {
//!         super(List.of(matrixVariable, scalarVariable), matrixVariable.dimensions());
//!         this.matrixVariable = matrixVariable;
//!         this.scalarVariable = scalarVariable;
//!     }
//!
//!     @Override
//!     public Matrix apply(ComputationContext ctx) {
//!         var matrix = ctx.data(matrixVariable);
//!         double scalarValue = ctx.data(scalarVariable).value();
//!         return matrix.map(v -> v + scalarValue);
//!     }
//!
//!     @Override
//!     public Tensor<?> gradient(Variable<?> parent, ComputationContext ctx) {
//!         Matrix selfGradient = ctx.gradient(this);
//!         if (parent == matrixVariable) {
//!             return selfGradient;
//!         } else {
//!             return new Scalar(selfGradient.aggregateSum());
//!         }
//!     }
//! }
//! ```

use crate::ml::core::functions::Weights;
use crate::ml::core::AbstractVariable;
use crate::ml::core::ComputationContext;
use crate::ml::core::{variable_id, Variable, VariableRef};
use crate::ml::core::{Matrix, Scalar, Tensor};
use std::any::Any;

/// Element-wise addition of matrix and scalar.
///
/// Computes: `result[i, j] = matrix[i, j] + scalar`
///
/// This is a two-parent variable (matrix and scalar inputs).
/// Uses composition pattern: VariableBase holds parents [matrix, scalar].
///
/// # Examples
///
/// ```rust,ignore
/// use gds::ml::core::functions::{EWiseAddMatrixScalar, Constant};
/// use gds::ml::core::{Matrix, Scalar};
///
/// let matrix = Box::new(Constant::new(Matrix::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2])));
/// let scalar = Box::new(Constant::new(Scalar::new(10.0)));
///
/// let add = EWiseAddMatrixScalar::new(matrix, scalar);
/// // Result: [[11.0, 12.0], [13.0, 14.0]]
/// ```
pub struct EWiseAddMatrixScalar {
    base: AbstractVariable, // COMPOSITION: wraps shared Variable logic (includes parents)
}

impl EWiseAddMatrixScalar {
    /// Create a new element-wise add matrix-scalar operation.
    ///
    /// # Arguments
    ///
    /// * `matrix_variable` - The matrix input
    /// * `scalar_variable` - The scalar to add to each element
    ///
    /// # Java equivalent
    ///
    /// ```java
    /// public EWiseAddMatrixScalar(Variable<Matrix> matrixVariable, Variable<Scalar> scalarVariable) {
    ///     super(List.of(matrixVariable, scalarVariable), matrixVariable.dimensions());
    ///     this.matrixVariable = matrixVariable;
    ///     this.scalarVariable = scalarVariable;
    /// }
    /// ```
    pub fn new(matrix_variable: Box<dyn Variable>, scalar_variable: Box<dyn Variable>) -> Self {
        Self::new_ref(matrix_variable.into(), scalar_variable.into())
    }

    /// Ref-based constructor for DAG-safe graph building.
    pub fn new_ref(matrix_variable: VariableRef, scalar_variable: VariableRef) -> Self {
        let dimensions = matrix_variable.dimensions().to_vec();

        // Java: super(List.of(matrixVariable, scalarVariable), matrixVariable.dimensions())
        // Store parents [matrix, scalar] in VariableBase
        let base = AbstractVariable::new(vec![matrix_variable, scalar_variable], dimensions);

        Self { base }
    }

    /// Get the matrix variable (first parent).
    fn matrix_variable(&self) -> &dyn Variable {
        self.base.parents()[0].as_ref()
    }

    /// Get the scalar variable (second parent).
    fn scalar_variable(&self) -> &dyn Variable {
        self.base.parents()[1].as_ref()
    }
}

impl Variable for EWiseAddMatrixScalar {
    /// Apply the element-wise addition.
    ///
    /// # Java equivalent
    ///
    /// ```java
    /// @Override
    /// public Matrix apply(ComputationContext ctx) {
    ///     var matrix = ctx.data(matrixVariable);
    ///     double scalarValue = ctx.data(scalarVariable).value();
    ///     return matrix.map(v -> v + scalarValue);
    /// }
    /// ```
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let matrix = ctx
            .data(self.matrix_variable())
            .expect("Matrix data not computed");

        let scalar = ctx
            .data(self.scalar_variable())
            .expect("Scalar data not computed");

        // Downcast to concrete types
        let matrix = matrix
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Expected Matrix type");

        // Handle scalar - extract value from tensor data
        // Note: Scalar tensors have dimensions [1] and can be downcast to either Scalar or Matrix
        let scalar_value = if let Some(scalar_tensor) = scalar.as_any().downcast_ref::<Scalar>() {
            scalar_tensor.value()
        } else if let Some(matrix_tensor) = scalar.as_any().downcast_ref::<Matrix>() {
            // Scalar tensors are actually stored as 1x1 matrices
            if matrix_tensor.rows() == 1 && matrix_tensor.cols() == 1 {
                matrix_tensor[(0, 0)]
            } else {
                panic!(
                    "Expected 1x1 matrix for scalar, got {}x{} matrix",
                    matrix_tensor.rows(),
                    matrix_tensor.cols()
                );
            }
        } else {
            // Try to extract from Weights variable if it's a Weights wrapper
            let scalar_var = self.scalar_variable();
            if let Some(weights) = (scalar_var as &dyn Any).downcast_ref::<Weights>() {
                weights.borrow_scalar().value()
            } else {
                panic!(
                    "Expected Scalar tensor, 1x1 Matrix, or Weights variable for scalar, got: {}",
                    std::any::type_name_of_val(scalar_var)
                );
            }
        };

        // Map: add scalar to each element
        let data = matrix.data().iter().map(|&v| v + scalar_value).collect();
        let dims = matrix.dimensions();
        let result = Matrix::new(data, dims[0], dims[1]);

        Box::new(result)
    }

    /// Compute gradient with respect to parent.
    ///
    /// # Java equivalent
    ///
    /// ```java
    /// @Override
    /// public Tensor<?> gradient(Variable<?> parent, ComputationContext ctx) {
    ///     Matrix selfGradient = ctx.gradient(this);
    ///     if (parent == matrixVariable) {
    ///         return selfGradient;
    ///     } else {
    ///         return new Scalar(selfGradient.aggregateSum());
    ///     }
    /// }
    /// ```
    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let self_gradient = ctx.gradient(self).expect("Self gradient not computed");

        let self_gradient_matrix = self_gradient
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Expected Matrix gradient");

        let matrix_id = variable_id(self.matrix_variable());
        let scalar_id = variable_id(self.scalar_variable());
        let parent_id = variable_id(parent);

        if parent_id == matrix_id {
            // Gradient w.r.t. matrix: pass through
            Box::new(self_gradient_matrix.clone())
        } else if parent_id == scalar_id {
            // Gradient w.r.t. scalar: sum all gradients
            let sum = self_gradient_matrix.aggregate_sum();
            Box::new(Scalar::new(sum))
        } else {
            panic!("Gradient called with parent that is not matrix or scalar variable");
        }
    }

    fn dimensions(&self) -> &[usize] {
        self.base.dimensions()
    }

    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    fn parents(&self) -> &[VariableRef] {
        self.base.parents()
    }
}

impl std::fmt::Display for EWiseAddMatrixScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EWiseAddMatrixScalar")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::core::functions::Constant;

    #[test]
    fn test_ewise_add_matrix_scalar_creation() {
        let matrix_data = vec![1.0, 2.0, 3.0, 4.0];
        let matrix = Matrix::new(matrix_data, 2, 2);
        let scalar = Scalar::new(10.0);

        let matrix_var = Box::new(Constant::new(Box::new(matrix)));
        let scalar_var = Box::new(Constant::new(Box::new(scalar)));

        let add = EWiseAddMatrixScalar::new(matrix_var, scalar_var);

        assert_eq!(add.dimensions(), &[2, 2]);
    }

    #[test]
    fn test_ewise_add_dimensions() {
        let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3);
        let scalar = Scalar::new(5.0);

        let matrix_var = Box::new(Constant::new(Box::new(matrix)));
        let scalar_var = Box::new(Constant::new(Box::new(scalar)));

        let add = EWiseAddMatrixScalar::new(matrix_var, scalar_var);

        // Should preserve matrix dimensions
        assert_eq!(add.dimensions(), &[2, 3]);
    }

    #[test]
    fn test_parent_access() {
        let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        let scalar = Scalar::new(10.0);

        let matrix_var = Box::new(Constant::new(Box::new(matrix)));
        let scalar_var = Box::new(Constant::new(Box::new(scalar)));

        let matrix_dims = matrix_var.dimensions().to_vec();
        let scalar_dims = scalar_var.dimensions().to_vec();

        let add = EWiseAddMatrixScalar::new(matrix_var, scalar_var);

        // Parents should be accessible
        assert_eq!(add.matrix_variable().dimensions(), &matrix_dims);
        assert_eq!(add.scalar_variable().dimensions(), &scalar_dims);
    }

    // Note: Full apply() and gradient() tests require ComputationContext implementation
}
