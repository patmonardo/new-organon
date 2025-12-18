//! Weakly Connected Components (WCC) Algorithm
//!
//! **Translation Source**: `org.neo4j.gds.wcc.Wcc`
//!
//! Finds all weakly connected components in an undirected graph.

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

pub use computation::WccComputationRuntime;
pub use spec::{WccAlgorithmSpec, WccConfig, WccResult};
pub use storage::WccStorageRuntime;
