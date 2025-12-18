//! Bridges Algorithm
//!
//! **Translation Source**: `org.neo4j.gds.bridges.Bridges`
//!
//! This module finds all bridges (cut edges) in an undirected graph.

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

pub use computation::BridgesComputationRuntime;
pub use spec::{BridgesAlgorithmSpec, BridgesConfig, BridgesResult};
pub use storage::BridgesStorageRuntime;
