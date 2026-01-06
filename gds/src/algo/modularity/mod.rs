//! Modularity quality metric.
//!
//! This module computes modularity scores for an existing community assignment.
//! It does **not** find communities; see Louvain/Leiden for modularity optimization.

pub mod computation;
pub mod storage;
pub mod spec;

#[cfg(test)]
mod integration_tests;

pub use computation::ModularityComputationRuntime;
pub use storage::ModularityStorageRuntime;
pub use spec::{CommunityModularity, ModularityResult};
