//! Articulation Points (Java GDS parity)
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

pub use computation::{
    ArticulationPointsComputationResult, ArticulationPointsComputationRuntime,
};
pub use spec::{ArticulationPointsConfig, ArticulationPointsResult, ARTICULATION_POINTSAlgorithmSpec};
pub use storage::ArticulationPointsStorageRuntime;

pub type ArticulationPointsAlgorithmSpec = ARTICULATION_POINTSAlgorithmSpec;
