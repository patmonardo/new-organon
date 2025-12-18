//! K1-Coloring Graph Coloring Algorithm
pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

pub use computation::K1ColoringComputationRuntime;
pub use spec::{K1ColoringAlgorithmSpec, K1ColoringConfig, K1ColoringResult};
pub use storage::K1ColoringStorageRuntime;
