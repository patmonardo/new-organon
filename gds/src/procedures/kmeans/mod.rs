//! K-Means clustering (community detection on node feature vectors)
//!
//! Clusters nodes based on an array-valued node property (e.g. an embedding).
//!
//! This is based on Neo4j GDS `gds.alpha.kmeans` / `gds.kmeans` style semantics:
//! - `k` clusters
//! - initialization via Uniform sampling or KMeans++
//! - iterative Lloyd updates
//! - optional silhouette computation (naive O(n^2) in this Rust port)

pub mod spec;
pub mod computation;

#[cfg(test)]
pub mod integration_tests;

pub use spec::{KMeansConfig, KMeansResult, KMeansSamplerType};
pub use computation::KMeansComputationRuntime;
