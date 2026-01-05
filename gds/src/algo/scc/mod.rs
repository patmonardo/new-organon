//! Strongly Connected Components (SCC) Algorithm
//!
//! **Translation Source**: `org.neo4j.gds.scc.Scc`
//!
//! Finds SCCs in a directed graph.

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

pub use computation::SccComputationRuntime;
pub use spec::{SCCAlgorithmSpec, SccConfig, SccResult};
pub use storage::SccStorageRuntime;
