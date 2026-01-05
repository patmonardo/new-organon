//! Closeness Centrality Algorithm (parallel, Java GDS parity direction)
//!
//! Translation target: Neo4j GDS `org.neo4j.gds.closeness.*`.
//!
//! Implementation notes:
//! - Phase 1: compute per-node farness and reachable-source counts using ANP MSBFS batching.
//! - Phase 2: compute closeness scores from (component, farness) in parallel.

pub mod computation;
pub mod spec;
pub mod storage;

pub use computation::{ClosenessCentralityComputationResult, ClosenessCentralityComputationRuntime};
pub use spec::{
	ClosenessCentralityAlgorithmSpec, ClosenessCentralityConfig, ClosenessCentralityResult,
};
pub use storage::ClosenessCentralityStorageRuntime;
