//! K-Core Decomposition
pub mod computation;
#[cfg(test)]
mod integration_tests;
pub mod spec;
pub mod storage;

pub use computation::{KCoreComputationResult, KCoreComputationRuntime};
pub use spec::{KCoreAlgorithmSpec, KCoreConfig, KCoreResult};
pub use storage::KCoreStorageRuntime;
