//! Tensor module for ML in GDS.
//!
//! Translated from Java GDS ml-core tensor package.

#![allow(clippy::module_inception)]

pub mod matrix;
pub mod operations;
pub mod scalar;
pub mod tensor;
pub mod vector;

pub use matrix::Matrix;
pub use scalar::Scalar;
pub use tensor::{size_in_bytes, AsAny, Tensor};
pub use vector::Vector;
