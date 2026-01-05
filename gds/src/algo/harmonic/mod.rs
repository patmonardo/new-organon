//! Harmonic Centrality (Java GDS parity)
//!
//! Standard algorithm module layout:
//! - `spec`: config + result + AlgorithmSpec integration
//! - `storage`: GraphStore-facing accessors (oriented neighbor access)
//! - `computation`: pure compute runtime (MSBFS/ANP semantics)

pub mod computation;
pub mod spec;
pub mod storage;

#[cfg(test)]
pub mod integration_tests;

pub use computation::HarmonicComputationRuntime;
pub use spec::{HarmonicConfig, HarmonicDirection, HarmonicResult, HARMONICAlgorithmSpec};
pub use storage::HarmonicStorageRuntime;

pub type HarmonicAlgorithmSpec = HARMONICAlgorithmSpec;
