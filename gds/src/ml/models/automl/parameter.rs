//! Parameter types - re-exported from hyperparameter module for backward compatibility
//!
//! This module re-exports types from the hyperparameter module.
//! New code should use the hyperparameter module directly.

pub use super::hyperparameter::*;

// Legacy type aliases for backward compatibility
pub type DoubleRange = DoubleRangeParameter;
pub type IntegerRange = IntegerRangeParameter;
