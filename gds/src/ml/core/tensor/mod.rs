//! Tensor module for ML in GDS.
//!
//! Translated from Java GDS ml-core tensor package.

#![allow(clippy::module_inception)]

pub mod float_vector;
pub mod matrix;
pub mod operations;
pub mod scalar;
pub mod tensor;
pub mod tensor_factory;
pub mod tensor_functions;
pub mod vector;

pub use float_vector::FloatVector;
pub use matrix::Matrix;
pub use scalar::Scalar;
pub use tensor::{size_in_bytes, Tensor};
pub use tensor_factory::*;
pub use tensor_functions::*;
pub use vector::Vector;
