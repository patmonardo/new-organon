//! Node property prediction pipeline infrastructure.
//!
//! This module contains the node-specific pipeline types for:
//! - Node classification (predicting categorical node properties)
//! - Node regression (predicting continuous node properties)

// Phase 1.1: Simple Types
pub mod node_feature_step;
pub mod node_property_pipeline_base_train_config;

// Phase 1.2: Pipeline Types
pub mod node_property_predict_pipeline;
pub mod node_property_prediction_split_config;
pub mod node_property_training_pipeline;

// Phase 1.3: Feature Producer
pub mod node_feature_producer;

// Phase 2: Node Classification
pub mod classification;

// Phase 4: Node Regression
pub mod regression;

// Re-exports for convenience
pub use node_feature_producer::{NodeFeatureProducer, NodeFeatureProducerError};
pub use node_feature_step::NodeFeatureStep;
pub use node_property_pipeline_base_train_config::NodePropertyPipelineBaseTrainConfig;
pub use node_property_predict_pipeline::NodePropertyPredictPipeline;
pub use node_property_prediction_split_config::NodePropertyPredictionSplitConfig;
pub use node_property_training_pipeline::NodePropertyTrainingPipeline;
