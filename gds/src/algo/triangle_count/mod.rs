//! Triangle Count Algorithm
//!
//! Counts triangles in the graph using efficient edge-based enumeration.

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

pub use computation::TriangleCountComputationRuntime;
pub use spec::{TriangleCountAlgorithmSpec, TriangleCountConfig, TriangleCountResult};
pub use storage::TriangleCountStorageRuntime;
