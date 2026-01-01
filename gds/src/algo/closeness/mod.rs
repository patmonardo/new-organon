//! Closeness Centrality Algorithm
//!
//! **Translation Source**: `org.neo4j.gds.closeness.ClosenessCentrality`
//!
//! Distance-based centrality measuring average distance to all other nodes.
//! Uses Multi-Source BFS for efficient computation.

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

pub use computation::ClosenessCentralityComputationRuntime;
pub use spec::{
    ClosenessCentralityAlgorithmSpec, ClosenessCentralityConfig, ClosenessCentralityResult,
};
pub use storage::ClosenessCentralityStorageRuntime;
