//! Float vector tensor utilities.
//!
//! Translated from Java GDS ml-core FloatVector.java.

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct FloatVector {
    data: Vec<f32>,
}

impl FloatVector {
    /// Create a new float vector with the given size (filled with zeros).
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0.0; size],
        }
    }

    /// Create a new float vector from data.
    pub fn from_vec(data: Vec<f32>) -> Self {
        Self { data }
    }

    /// Borrow the underlying data.
    pub fn data(&self) -> &[f32] {
        &self.data
    }

    /// Compute inner product with another float vector.
    pub fn inner_product(&self, other: &FloatVector) -> f32 {
        let length = self.data.len().min(other.data.len());
        let mut result = 0.0f32;
        for i in 0..length {
            result += self.data[i] * other.data[i];
        }
        result
    }
}

impl fmt::Display for FloatVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inner_product() {
        let a = FloatVector::from_vec(vec![1.0, 2.0, 3.0]);
        let b = FloatVector::from_vec(vec![4.0, 5.0, 6.0]);
        assert!((a.inner_product(&b) - 32.0).abs() < 1e-6);
    }
}
