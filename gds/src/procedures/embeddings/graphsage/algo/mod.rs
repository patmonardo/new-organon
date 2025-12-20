//! GraphSAGE `algo/` package (Java parity layer).
//!
//! Java package: `org.neo4j.gds.embeddings.graphsage.algo.*`
//!
//! This layer is the orchestration layer around the already-translated core
//! GraphSAGE components (`GraphSageEmbeddingsGenerator`, `GraphSageModelTrainer`, etc.).

pub mod graph_sage;
pub mod graph_sage_algorithm_factory;
pub mod graph_sage_memory_estimate_definition;
pub mod graph_sage_model;
pub mod graph_sage_model_data;
pub mod graph_sage_model_resolver;
pub mod graph_sage_result;
pub mod graph_sage_train;
pub mod graph_sage_train_algorithm_factory;
pub mod graph_sage_train_estimate_definition;
pub mod multi_label_feature_extractors;
pub mod multi_label_graph_sage_train;
pub mod single_label_graph_sage_train;

pub use graph_sage::GraphSage;
pub use graph_sage_algorithm_factory::GraphSageAlgorithmFactory;
pub use graph_sage_memory_estimate_definition::GraphSageMemoryEstimateDefinition;
pub use graph_sage_model::GraphSageModel;
pub use graph_sage_model_data::GraphSageModelData;
pub use graph_sage_model_resolver::GraphSageModelResolver;
pub use graph_sage_result::GraphSageResult;
pub use graph_sage_train::GraphSageTrain;
pub use graph_sage_train_algorithm_factory::GraphSageTrainAlgorithmFactory;
pub use graph_sage_train_estimate_definition::GraphSageTrainEstimateDefinition;
pub use multi_label_feature_extractors::MultiLabelFeatureExtractors;
pub use multi_label_graph_sage_train::MultiLabelGraphSageTrain;
pub use single_label_graph_sage_train::SingleLabelGraphSageTrain;

#[cfg(test)]
mod tests;


