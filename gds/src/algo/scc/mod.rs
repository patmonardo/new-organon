//! Strongly Connected Components (SCC) Algorithm
//!
//! **Translation Source**: `org.neo4j.gds.scc.Scc`
//!
//! This module provides the Strongly Connected Components algorithm using iterative DFS
//! to avoid stack overflow on large graphs.

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

// Re-export main types
pub use computation::SccComputationRuntime;
pub use spec::{SCCAlgorithmSpec, SccConfig, SccResult};
pub use storage::SccStorageRuntime;
