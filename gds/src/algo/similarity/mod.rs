pub mod filtered_knn;
pub mod filtered_node_similarity;
pub mod knn;
pub mod node_similarity;

// Preserve the existing public surface by re-exporting NodeSim types at the
// similarity root.
pub use node_similarity::{
    NodeSimilarityAlgorithmSpec, NodeSimilarityComputationRuntime, NodeSimilarityConfig,
    NodeSimilarityMetric, NodeSimilarityResult, NodeSimilarityStorageRuntime,
};

pub use filtered_knn::{FilteredKnnAlgorithmSpec, FilteredKnnConfig};
