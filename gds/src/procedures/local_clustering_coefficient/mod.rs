//! Local Clustering Coefficient Algorithm
//!
//! This module implements the Local Clustering Coefficient (LCC) algorithm.
//!
//! ## What is LCC?
//!
//! The local clustering coefficient measures how densely connected the neighbors of each node are.
//! It's a value between 0 and 1:
//! - 1.0: All neighbors are fully connected (form a clique)
//! - 0.0: No neighbors are connected
//!
//! ## Formula
//!
//! For a node v with degree d(v) and t(v) triangles:
//! ```
//! C(v) = 2 * t(v) / (d(v) * (d(v) - 1))
//! ```
//!
//! ## Architecture
//!
//! Following the Five-Fold Brahmachakra design:
//! - **spec.rs** - AlgorithmSpec implementation (Species)
//! - **storage.rs** - Storage Runtime (Gross pole - GraphStore access)
//! - **computation.rs** - Computation Runtime (Subtle pole - clustering scores)
//!
//! ## Dependency
//!
//! LCC depends on **Triangle Count** to enumerate triangles per node.

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

// Re-export main types
pub use computation::LocalClusteringCoefficientComputationRuntime;
pub use spec::{
    LOCAL_CLUSTERING_COEFFICIENTAlgorithmSpec, LocalClusteringCoefficientConfig,
    LocalClusteringCoefficientResult,
};
pub use storage::LocalClusteringCoefficientStorageRuntime;
