//! K-Core Decomposition (Java GDS parity)
//!
//! Standard algorithm module layout:
//! - `spec`: config + result + executor AlgorithmSpec integration
//! - `storage`: GraphStore-facing accessors (undirected neighbor access)
//! - `computation`: core k-core decomposition runtime

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

pub use computation::{KCoreComputationResult, KCoreComputationRuntime};
pub use spec::{KCOREAlgorithmSpec, KCoreConfig, KCoreResult};
pub use storage::KCoreStorageRuntime;

pub type KCoreAlgorithmSpec = KCOREAlgorithmSpec;
