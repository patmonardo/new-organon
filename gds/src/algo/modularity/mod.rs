//! Modularity quality metric.
//!
//! This module computes modularity scores for an existing community assignment.
//! It does **not** find communities; see Louvain/Leiden for modularity optimization.

pub mod computation;
pub mod spec;
pub mod storage;

#[cfg(test)]
mod integration_tests;

pub use computation::ModularityComputationRuntime;
pub use spec::{CommunityModularity, ModularityResult};
pub use storage::ModularityStorageRuntime;
