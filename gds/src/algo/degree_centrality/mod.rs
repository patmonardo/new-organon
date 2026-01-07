//! Degree Centrality (Java GDS parity)
//!
//! Standard algorithm module layout:
//! - `spec`: config + result + AlgorithmSpec integration
//! - `storage`: GraphStore-facing orchestration (projection + concurrency)
//! - `computation`: pure kernel helpers (per-range compute + normalization)

pub mod computation;
pub mod spec;
pub mod storage;

#[cfg(test)]
pub mod integration_tests;

pub use computation::DegreeCentralityComputationRuntime;
pub use spec::{DEGREE_CENTRALITYAlgorithmSpec, DegreeCentralityConfig, DegreeCentralityResult};
pub use storage::{DegreeCentralityStorageRuntime, Orientation};

pub type DegreeCentralityAlgorithmSpec = DEGREE_CENTRALITYAlgorithmSpec;
