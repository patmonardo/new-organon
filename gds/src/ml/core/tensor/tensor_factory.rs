//! Tensor factory utilities.
//!
//! Translated from Java GDS ml-core TensorFactory.java.

use crate::ml::core::dimensions::{COLUMNS_INDEX, ROWS_INDEX};
use crate::ml::core::{Matrix, Scalar, Tensor, Vector};

/// Create a constant tensor filled with value `v` and the given dimensions.
///
/// Mirrors Java `TensorFactory.constant(double v, int[] dimensions)`.
pub fn constant(v: f64, dimensions: &[usize]) -> Box<dyn Tensor> {
    if dimensions.len() == 1 && dimensions[ROWS_INDEX] == 1 {
        Box::new(Scalar::new(v))
    } else if dimensions.len() == 1 && dimensions[ROWS_INDEX] > 1 {
        Box::new(Vector::create(v, dimensions[ROWS_INDEX]))
    } else if dimensions.len() == 2 && dimensions[ROWS_INDEX] > 0 && dimensions[COLUMNS_INDEX] > 0 {
        Box::new(Matrix::create(
            v,
            dimensions[ROWS_INDEX],
            dimensions[COLUMNS_INDEX],
        ))
    } else {
        panic!(
            "Tensor of dimensions greater than 2 are not supported, got {} dimensions",
            dimensions.len()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_scalar() {
        let tensor = constant(3.0, &[1]);
        let scalar = tensor
            .as_any()
            .downcast_ref::<Scalar>()
            .expect("Expected Scalar");
        assert_eq!(scalar.value(), 3.0);
    }

    #[test]
    fn test_constant_vector() {
        let tensor = constant(2.0, &[3]);
        let vector = tensor
            .as_any()
            .downcast_ref::<Vector>()
            .expect("Expected Vector");
        assert_eq!(vector.data(), &[2.0, 2.0, 2.0]);
    }

    #[test]
    fn test_constant_matrix() {
        let tensor = constant(1.5, &[2, 2]);
        let matrix = tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Expected Matrix");
        assert_eq!(matrix.data(), &[1.5, 1.5, 1.5, 1.5]);
    }
}
