#[allow(clippy::module_inception)]
pub mod similarity;
pub use similarity::SimilarityBuilder;

pub mod knn;
pub use knn::KnnBuilder;

pub mod filtered_knn;
pub use filtered_knn::FilteredKnnBuilder;
