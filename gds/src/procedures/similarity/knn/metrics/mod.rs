//! KNN similarity metrics and similarity-computer implementations.
//!
//! These are clean-room implementations of standard similarity measures
//! (cosine/euclidean/pearson/jaccard/overlap) and the property-based
//! `SimilarityComputer` selection logic.

mod combined_similarity_computer;
mod cosine;
mod double_array_property_similarity_computer;
mod double_property_similarity_computer;
mod euclidean;
mod float_array_property_similarity_computer;
mod jaccard;
mod long_array_property_similarity_computer;
mod long_property_similarity_computer;
mod null_checking_node_property_values;
mod overlap;
mod pearson;
mod similarity_computer;

pub use combined_similarity_computer::CombinedSimilarityComputer;
pub use null_checking_node_property_values::NullCheckingNodePropertyValues;
pub use similarity_computer::{
    KnnNodePropertySpec, SimilarityComputer, SimilarityComputerBuildError, SimilarityMetric,
};

// Expose the concrete computers (useful for testing / diagnostics).
pub use double_array_property_similarity_computer::DoubleArrayPropertySimilarityComputer;
pub use double_property_similarity_computer::DoublePropertySimilarityComputer;
pub use float_array_property_similarity_computer::FloatArrayPropertySimilarityComputer;
pub use long_array_property_similarity_computer::LongArrayPropertySimilarityComputer;
pub use long_property_similarity_computer::LongPropertySimilarityComputer;
