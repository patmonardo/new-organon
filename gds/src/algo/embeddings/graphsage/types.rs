//! Shared enums + config/parameter types for GraphSAGE.
//!
//! Java package: `org.neo4j.gds.embeddings.graphsage` (+ `...graphsage.algo` for configs).

use crate::concurrency::Concurrency;
use crate::config::base_types::{BaseConfig, Config};
use crate::config::validation::ConfigError;
use crate::core::model::ModelConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActivationFunctionType {
    Sigmoid,
    Relu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AggregatorType {
    Mean,
    Pool,
}

/// Rust counterpart to Java `LayerConfig` (value class).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerConfig {
    pub rows: usize,
    pub cols: usize,
    pub sample_size: usize,
    pub random_seed: u64,
    pub bias: Option<usize>,
    pub aggregator_type: AggregatorType,
    pub activation_function: ActivationFunctionType,
}

/// Java: `GraphSageTrainConfig` (lives in `...graphsage.algo` in Java).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSageTrainConfig {
    pub model_user: String,
    pub model_name: String,

    pub concurrency: Concurrency,
    pub batch_size: usize,
    pub max_iterations: usize,
    pub search_depth: usize,
    pub epochs: usize,
    pub learning_rate: f64,
    pub tolerance: f64,
    pub negative_sample_weight: usize,
    pub penalty_l2: f64,
    pub embedding_dimension: usize,
    pub sample_sizes: Vec<usize>,
    pub feature_properties: Vec<String>,
    pub maybe_batch_sampling_ratio: Option<f64>,
    pub random_seed: Option<u64>,
    pub aggregator: AggregatorType,
    pub activation_function: ActivationFunctionType,
    pub is_multi_label: bool,
    pub projected_feature_dimension: Option<usize>,
}

impl Config for GraphSageTrainConfig {}

impl BaseConfig for GraphSageTrainConfig {
    fn parameters(&self) -> HashMap<String, serde_json::Value> {
        // Minimal parity: capture the main knobs for inspection/logging.
        let mut m = HashMap::new();
        m.insert("batchSize".into(), self.batch_size.into());
        m.insert("maxIterations".into(), self.max_iterations.into());
        m.insert("searchDepth".into(), self.search_depth.into());
        m.insert("epochs".into(), self.epochs.into());
        m.insert("learningRate".into(), self.learning_rate.into());
        m.insert("tolerance".into(), self.tolerance.into());
        m.insert(
            "negativeSampleWeight".into(),
            (self.negative_sample_weight as u64).into(),
        );
        m.insert("penaltyL2".into(), self.penalty_l2.into());
        m.insert("embeddingDimension".into(), self.embedding_dimension.into());
        m.insert(
            "isMultiLabel".into(),
            serde_json::Value::Bool(self.is_multi_label),
        );
        if let Some(v) = self.projected_feature_dimension {
            m.insert("projectedFeatureDimension".into(), (v as u64).into());
        }
        m
    }
}

impl ModelConfig for GraphSageTrainConfig {
    fn model_name(&self) -> &str {
        &self.model_name
    }

    fn model_user(&self) -> &str {
        &self.model_user
    }
}

impl GraphSageTrainConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        crate::config::validate_non_empty_string(&self.model_user, "modelUser")?;
        crate::config::validate_non_empty_string(&self.model_name, "modelName")?;

        if self.batch_size == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "batchSize".to_string(),
                reason: "batchSize must be > 0".to_string(),
            });
        }
        if self.learning_rate <= 0.0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "learningRate".to_string(),
                reason: "learningRate must be > 0".to_string(),
            });
        }
        if self.tolerance < 0.0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "tolerance".to_string(),
                reason: "tolerance must be >= 0".to_string(),
            });
        }
        if self.embedding_dimension == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "embeddingDimension".to_string(),
                reason: "embeddingDimension must be > 0".to_string(),
            });
        }
        for &s in &self.sample_sizes {
            if s == 0 {
                return Err(ConfigError::InvalidParameter {
                    parameter: "sampleSizes".to_string(),
                    reason: "sampleSizes entries must be > 0".to_string(),
                });
            }
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for GraphSageTrainConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        GraphSageTrainConfig::validate(self)
    }
}

/// Java: `GraphSageTrainParameters` (derived from config).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSageTrainParameters {
    pub concurrency: Concurrency,
    pub batch_size: usize,
    pub max_iterations: usize,
    pub search_depth: usize,
    pub epochs: usize,
    pub learning_rate: f64,
    pub tolerance: f64,
    pub negative_sample_weight: usize,
    pub penalty_l2: f64,
    pub embedding_dimension: usize,
    pub sample_sizes: Vec<usize>,
    pub feature_properties: Vec<String>,
    pub maybe_batch_sampling_ratio: Option<f64>,
    pub random_seed: Option<u64>,
    pub aggregator: AggregatorType,
    pub activation_function: ActivationFunctionType,
    pub is_multi_label: bool,
    pub projected_feature_dimension: Option<usize>,
}

impl GraphSageTrainParameters {
    pub fn batches_per_iteration(&self, node_count: usize) -> usize {
        // Java: config.maybeBatchSamplingRatio().map(ratio -> (nodeCount/batchSize)*ratio).orElse(nodeCount/batchSize)
        let base = (node_count / self.batch_size).max(1);
        if let Some(ratio) = self.maybe_batch_sampling_ratio {
            ((base as f64) * ratio).ceil().max(1.0) as usize
        } else {
            base
        }
    }

    pub fn layer_configs(&self, feature_dimension: usize) -> Vec<LayerConfig> {
        super::graphsage_helper::layer_configs(
            feature_dimension,
            &self.sample_sizes,
            self.random_seed,
            self.aggregator,
            self.activation_function,
            self.embedding_dimension,
        )
    }
}

/// Java: `GraphSageTrainMemoryEstimateParameters`.
#[derive(Debug, Clone)]
pub struct GraphSageTrainMemoryEstimateParameters {
    pub layer_configs: Vec<LayerConfig>,
    pub is_multi_label: bool,
    pub feature_property_count: usize,
    pub estimation_feature_dimension: usize,
    pub batch_size: usize,
    pub embedding_dimension: usize,
}
