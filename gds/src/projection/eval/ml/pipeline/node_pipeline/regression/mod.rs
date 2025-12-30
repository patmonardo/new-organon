//! Node regression training pipeline implementation.
//!
//! This module provides the complete pipeline infrastructure for training
//! node regression models on graph data. It mirrors the Java GDS regression
//! pipeline architecture.

// Phase 4.1: Training Pipeline ✅
pub mod node_regression_training_pipeline;
pub use node_regression_training_pipeline::NodeRegressionTrainingPipeline;

// Phase 4.2: Configuration ✅
pub mod node_regression_pipeline_train_config;
pub use node_regression_pipeline_train_config::{
    NodeRegressionPipelineTrainConfig, RegressionMetrics,
};

// Phase 4.3: Training Result ✅
pub mod node_regression_train_result;
pub use node_regression_train_result::{
    NodeRegressionTrainPipelineResult, NodeRegressionTrainResult,
};

// Phase 4.4: Model Info ✅
pub mod node_regression_pipeline_model_info;
pub use node_regression_pipeline_model_info::{
    NodeRegressionPipelineModelInfo, NodeRegressionPipelineModelInfoBuilder,
};

// Phase 4.5: Model Converter ✅
pub mod node_regression_to_model_converter;
pub use node_regression_to_model_converter::{
    NodeRegressionToModelConverter, ResultToModelConverterTrait,
};

// Phase 4.7: Train Algorithm ✅
pub mod node_regression_train_algorithm;
pub use node_regression_train_algorithm::NodeRegressionTrainAlgorithm;

// Phase 4.8: Algorithm Factory ✅
pub mod node_regression_train_pipeline_algorithm_factory;
pub use node_regression_train_pipeline_algorithm_factory::NodeRegressionTrainPipelineAlgorithmFactory;

// Phase 4.6: Core Training (to be added - the big one!)
// pub mod node_regression_train;
// pub use node_regression_train::NodeRegressionTrain;

// Phase 4.4: Algorithm Wrapper & Factory (to be added)
// pub mod node_regression_train_algorithm;
// pub use node_regression_train_algorithm::NodeRegressionTrainAlgorithm;
// pub mod node_regression_train_pipeline_algorithm_factory;
// pub use node_regression_train_pipeline_algorithm_factory::NodeRegressionTrainPipelineAlgorithmFactory;

// Phase 4.5: Model Conversion & Results (to be added)
// pub mod node_regression_to_model_converter;
// pub use node_regression_to_model_converter::NodeRegressionToModelConverter;
// pub mod node_regression_pipeline_model_info;
// pub use node_regression_pipeline_model_info::NodeRegressionPipelineModelInfo;
// pub mod node_regression_train_result;
// pub use node_regression_train_result::NodeRegressionTrainResult;
