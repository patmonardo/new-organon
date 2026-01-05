//! Betweenness Centrality (Brandes)
//!
//! Translation target: Neo4j GDS `org.neo4j.gds.betweenness.*`.
//!
//! Module layout follows the standard "algo" pattern used across this crate:
//! - `spec`: config + result + AlgorithmSpec integration
//! - `storage`: GraphStore-facing view + neighbor/weight access
//! - `computation`: pure Brandes runtime (parallel over sources)

pub mod computation;
pub mod spec;
pub mod storage;

pub use computation::BetweennessCentralityComputationRuntime;
pub use spec::{BetweennessCentralityConfig, BetweennessCentralityResult, BETWEENNESSAlgorithmSpec};
pub use storage::BetweennessCentralityStorageRuntime;

pub type BetweennessCentralityAlgorithmSpec = BETWEENNESSAlgorithmSpec;

#[cfg(test)]
mod integration_tests;
