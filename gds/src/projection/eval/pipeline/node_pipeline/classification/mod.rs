//! Node classification pipeline implementation.
//!
//! This module provides the core types for node classification ML pipelines:
//! - Training pipeline configuration and execution
//! - Model training results and metadata
//! - Model catalog integration
//! - Result-to-model conversion

// Phase 2.1: Pipeline & Value Classes
pub mod node_classification_model_result;
pub mod node_classification_train_algorithm;
pub mod node_classification_train_result;
pub mod node_classification_training_pipeline;

// Phase 2.2: Configs & Converters
pub mod node_classification_pipeline_model_info;
pub mod node_classification_pipeline_train_config;
pub mod node_classification_to_model_converter;

// Phase 3.1: Utility Classes
pub mod labels_and_class_counts_extractor;

// Phase 3.2: Factory Classes
pub mod node_classification_train_pipeline_algorithm_factory;

// Phase 3.3: Training Implementation
#[path = "node_classification_train_clean.rs"]
pub mod node_classification_train;

// Re-exports
pub use labels_and_class_counts_extractor::{LabelsAndClassCounts, LabelsAndClassCountsExtractor};
pub use node_classification_model_result::NodeClassificationModelResult;
pub use node_classification_pipeline_model_info::NodeClassificationPipelineModelInfo;
pub use node_classification_pipeline_train_config::NodeClassificationPipelineTrainConfig;
pub use node_classification_to_model_converter::NodeClassificationToModelConverter;
pub use node_classification_train::NodeClassificationTrain;
pub use node_classification_train_algorithm::NodeClassificationTrainAlgorithm;
pub use node_classification_train_pipeline_algorithm_factory::NodeClassificationTrainPipelineAlgorithmFactory;
pub use node_classification_train_result::NodeClassificationTrainResult;
pub use node_classification_training_pipeline::NodeClassificationTrainingPipeline;
