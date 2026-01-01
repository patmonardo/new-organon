//! **Breadth-First Search (BFS) Algorithm**
//!
//! **Translation Source**: `org.neo4j.gds.traversal.BFS`
//!
//! This module implements the Breadth-First Search algorithm for graph traversal,
//! providing efficient exploration of nodes level by level from a source node.

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

// Re-export main types
pub use computation::BfsComputationRuntime;
pub use spec::{BFSAlgorithmSpec, BfsConfig, BfsResult};
pub use storage::BfsStorageRuntime;
