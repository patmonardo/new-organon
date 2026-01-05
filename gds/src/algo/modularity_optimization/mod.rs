//! Modularity optimization (Louvain-style local moving).
//!
//! This module performs modularity optimization for a single graph level.
//! Louvain wraps this across multiple levels (graph aggregation / dendrogram).

pub mod computation;
pub mod spec;

#[cfg(test)]
mod integration_tests;

pub use computation::{ModularityOptimizationComputationRuntime, ModularityOptimizationInput};
pub use spec::{ModularityOptimizationConfig, ModularityOptimizationResult};
