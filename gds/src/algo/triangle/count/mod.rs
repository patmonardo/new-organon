//! Triangle count.
//!
//! Counts triangles in an undirected graph.
//!
//! Java parity note:
//! Neo4j GDS implements an intersect-based approach with an optional `maxDegree` filter.
//! This Rust module uses the same fundamental idea: neighbor-set intersections over
//! sorted adjacency lists.

pub mod computation;
pub mod spec;
pub mod storage;

#[cfg(test)]
mod integration_tests;

pub use computation::TriangleCountComputationRuntime;
pub use spec::{TriangleCountAlgorithmSpec, TriangleCountConfig, TriangleCountResult};
pub use storage::TriangleCountStorageRuntime;
