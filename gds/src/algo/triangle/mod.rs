//! Triangle-related utilities.

pub mod computation;
pub mod intersect;
pub mod spec;
pub mod storage;

#[cfg(test)]
mod integration_tests;

pub use computation::TriangleComputationRuntime;
pub use spec::{TriangleAlgorithmSpec, TriangleConfig, TriangleResult};
pub use storage::TriangleStorageRuntime;
