//! Triangle-related utilities.

pub mod intersect;
pub mod computation;
pub mod storage;
pub mod spec;

#[cfg(test)]
mod integration_tests;

pub use computation::TriangleComputationRuntime;
pub use spec::{TriangleAlgorithmSpec, TriangleConfig, TriangleResult};
pub use storage::TriangleStorageRuntime;
