//! All Shortest Paths Algorithm
//!
//! This module implements the All Shortest Paths algorithm for computing shortest paths
//! between all pairs of nodes in a graph.
//!
//! **Translation Source**: `org.neo4j.gds.allshortestpaths.*`
//! **Algorithm Types**:
//! - Unweighted: Multi-Source BFS (MSBFS)
//! - Weighted: Multi-Source Dijkstra with Priority Queue
//!
//! **Key Features**:
//! - Multi-source parallelization
//! - Streaming results to avoid O(V²) memory usage
//! - Weighted/unweighted graph support
//! - Termination support

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

// Re-exports for public API
pub use computation::AllShortestPathsComputationRuntime;
pub use spec::{ALL_SHORTEST_PATHSAlgorithmSpec, AllShortestPathsConfig, AllShortestPathsResult};
pub use storage::ShortestPathResult;
pub use storage::{AlgorithmType, AllShortestPathsStorageRuntime};
