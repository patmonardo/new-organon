//! Bridges (Java GDS parity)
//!
//! Standard algorithm module layout:
//! - `spec`: config + result + AlgorithmSpec integration
//! - `storage`: GraphStore-facing accessors (undirected neighbor access)
//! - `computation`: pure compute runtime (iterative DFS stack events)

pub mod computation;
pub mod spec;
pub mod storage;

#[cfg(test)]
pub mod integration_tests;

pub use computation::{Bridge, BridgesComputationResult, BridgesComputationRuntime};
pub use spec::{BRIDGESAlgorithmSpec, BridgesConfig, BridgesResult};
pub use storage::BridgesStorageRuntime;

pub type BridgesAlgorithmSpec = BRIDGESAlgorithmSpec;
