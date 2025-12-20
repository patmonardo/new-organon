//! GraphSAGE embeddings (Java: `org.neo4j.gds.embeddings.graphsage`).
//!
//! This module is a close translation of the Java Graph Data Science implementation.
//! The design follows the same "support layer" + "algorithm layer" split:
//! - Sampling (`BatchSampler`)
//! - Feature initialization (`GraphSageHelper`)
//! - Embeddings generation (`GraphSageEmbeddingsGenerator`)
//! - Training (`GraphSageModelTrainer`, `GraphSageLoss`)

pub mod types;

pub mod activation_function;
pub mod activation_function_factory;
pub mod activation_function_wrapper;
pub mod sigmoid_wrapper;
pub mod relu_wrapper;

pub mod aggregator;
pub mod aggregator_memory_estimator;
pub mod batch_sampler;
pub mod feature_function;
pub mod graphsage_embeddings_generator;
pub mod graphsage_helper;
pub mod graphsage_loss;
pub mod graphsage_model_trainer;

pub mod layer;
pub mod layer_config;
pub mod layer_factory;

pub mod mean_aggregator;
pub mod mean_aggregating_layer;
pub mod mean_aggregator_memory_estimator;

pub mod max_pooling_aggregator;
pub mod max_pool_aggregating_layer;
pub mod pool_aggregator_memory_estimator;

pub mod algo;

pub mod model_data;
pub mod multi_label_feature_function;
pub mod single_label_feature_function;
pub mod train_config_transformer;

// Re-export the main surface
pub use graphsage_embeddings_generator::GraphSageEmbeddingsGenerator;
pub use graphsage_model_trainer::GraphSageModelTrainer;
pub use types::*;


