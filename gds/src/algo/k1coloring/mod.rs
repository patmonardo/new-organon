//! K1-Coloring (Java GDS parity)
//!
//! Standard algorithm module layout:
//! - `spec`: config + result + AlgorithmSpec integration
//! - `storage`: GraphStore-facing accessors (undirected neighbor access)
//! - `computation`: core coloring/validation loop

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

// Re-export main types
pub use computation::K1ColoringComputationRuntime;
pub use spec::{K1COLORINGAlgorithmSpec, K1ColoringConfig, K1ColoringResult};
pub use storage::K1ColoringStorageRuntime;

pub type K1ColoringAlgorithmSpec = K1COLORINGAlgorithmSpec;
