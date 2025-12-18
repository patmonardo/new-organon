//! Bellman-Ford Algorithm Module
//!
//! **Translation Source**: `org.neo4j.gds.paths.bellmanford.BellmanFord`
//!
//! This module implements the Bellman-Ford algorithm for finding shortest paths
//! with negative cycle detection, following the same three-layer architecture
//! as other pathfinding algorithms.

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

// Re-export main types for easy access
pub use computation::BellmanFordComputationRuntime;
pub use spec::{BELLMAN_FORDAlgorithmSpec, BellmanFordConfig, BellmanFordResult};
pub use storage::BellmanFordStorageRuntime;
