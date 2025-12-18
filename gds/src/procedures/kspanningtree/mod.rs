//! K-Spanning Tree Module
//!
//! Implements k-spanning tree decomposition by computing an MST then
//! progressively cutting the k weakest edges.

pub mod computation;
pub mod spec;
pub mod storage;

#[cfg(test)]
mod integration_tests;

pub use computation::KSpanningTreeComputationRuntime;
pub use spec::{KSpanningTreeAlgorithmSpec, KSpanningTreeConfig, KSpanningTreeResult};
pub use storage::KSpanningTreeStorageRuntime;
