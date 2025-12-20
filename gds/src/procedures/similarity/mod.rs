pub mod knn;
pub mod nodesim;
pub mod filteredknn;

// Preserve the existing public surface by re-exporting NodeSim types at the
// similarity root.
pub use nodesim::{
	NodeSimilarityAlgorithmSpec,
	NodeSimilarityConfig,
	NodeSimilarityComputationRuntime,
	NodeSimilarityMetric,
	NodeSimilarityResult,
	NodeSimilarityStorageRuntime,
};

pub use filteredknn::{
	FilteredKnnAlgorithmSpec,
	FilteredKnnConfig,
};

