//! Node Similarity (NodeSim) algorithms.
//!
//! This folder hosts the canonical `spec/storage/computation` split for the
//! NodeSimilarity family.

pub mod computation;
pub mod similarity_metric;
pub mod spec;
pub mod storage;
pub mod vector_computer;

#[cfg(test)]
mod tests;

pub use computation::{NodeSimilarityComputationResult, NodeSimilarityComputationRuntime};
pub use similarity_metric::{NodeSimilarityMetric, SimilarityMetric};
pub use spec::{
    NodeSimilarityAlgorithmResult, NodeSimilarityAlgorithmSpec, NodeSimilarityConfig,
    NodeSimilarityResult,
};
pub use storage::NodeSimilarityStorageRuntime;
