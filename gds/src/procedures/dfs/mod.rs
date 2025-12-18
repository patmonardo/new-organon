//! **Depth-First Search (DFS) Algorithm**
//!
//! **Translation Source**: `org.neo4j.gds.traversal.DFS`
//!
//! This module implements the Depth-First Search algorithm for graph traversal,
//! providing efficient exploration of nodes using a stack-based approach.

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

// Re-export main types
pub use computation::DfsComputationRuntime;
pub use spec::{DFSAlgorithmSpec, DfsConfig, DfsResult};
pub use storage::DfsStorageRuntime;
