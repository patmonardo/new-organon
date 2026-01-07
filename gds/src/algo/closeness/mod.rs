//! Closeness Centrality (Java GDS parity)
//!
//! Standard algorithm module layout:
//! - `spec`: config + result + AlgorithmSpec integration
//! - `storage`: GraphStore-facing controller (projection + orchestration)
//! - `computation`: pure compute kernels (MSBFS farness + score computation)

pub mod computation;
pub mod spec;
pub mod storage;

#[cfg(test)]
pub mod integration_tests;

pub use computation::ClosenessCentralityComputationRuntime;
pub use spec::{CLOSENESSAlgorithmSpec, ClosenessCentralityConfig, ClosenessCentralityResult};
pub use storage::ClosenessCentralityStorageRuntime;

pub type ClosenessCentralityAlgorithmSpec = CLOSENESSAlgorithmSpec;
