//! HITS (Hyperlink-Induced Topic Search)
//!
//! This module follows the repo’s standard algo layout:
//! - `spec`: config + result + AlgorithmSpec integration
//! - `storage`: GraphStore-facing orchestration (projection + pregel run)
//! - `computation`: pure Pregel kernels (init/compute/master)

pub mod computation;
pub mod spec;
pub mod storage;

#[cfg(test)]
pub mod integration_tests;

pub use computation::run_hits;
pub use spec::{HITSAlgorithmSpec, HitsConfig, HitsResult};
pub use storage::{HitsRunResult, HitsStorageRuntime};
