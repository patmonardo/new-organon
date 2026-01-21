//! Result shapes for the Pipelines facade.
//!
//! These mirror the Java `pipelines-facade-api` result records/classes.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A loose map type for Java `Map<String, Object>`.
pub type AnyMap = HashMap<String, Value>;

/// A loose list of maps type for Java `List<Map<String, Object>>`.
pub type AnyMapList = Vec<AnyMap>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineCatalogResult {
    pub pipeline_info: AnyMap,
    pub pipeline_name: String,
    pub pipeline_type: String,
    pub creation_time: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineExistsResult {
    pub pipeline_name: String,
    pub pipeline_type: String,
    pub exists: bool,
}

impl PipelineExistsResult {
    pub fn empty(pipeline_name: impl Into<String>) -> Self {
        Self {
            pipeline_name: pipeline_name.into(),
            pipeline_type: "n/a".to_string(),
            exists: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineInfoResult {
    pub name: String,
    pub node_property_steps: AnyMapList,
    pub feature_steps: AnyMapList,
    pub split_config: AnyMap,
    pub auto_tuning_config: AnyMap,
    pub parameter_space: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePipelineInfoResult {
    pub name: String,
    pub node_property_steps: AnyMapList,
    pub feature_properties: Vec<String>,
    pub split_config: AnyMap,
    pub auto_tuning_config: AnyMap,
    pub parameter_space: HashMap<String, AnyMapList>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamResult {
    pub node1: i64,
    pub node2: i64,
    pub probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeClassificationStreamResult {
    pub node_id: i64,
    pub predicted_class: i64,
    pub predicted_probabilities: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeRegressionStreamResult {
    pub node_id: i64,
    pub predicted_value: f64,
}

/// Minimal stand-in for the Java `StandardMutateResult` supertype.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardMutateResult {
    pub pre_processing_millis: i64,
    pub compute_millis: i64,
    pub mutate_millis: i64,
    pub configuration: AnyMap,
}

/// Minimal stand-in for the Java `StandardWriteResult` supertype.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardWriteResult {
    pub pre_processing_millis: i64,
    pub compute_millis: i64,
    pub write_millis: i64,
    pub configuration: AnyMap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutateResult {
    #[serde(flatten)]
    pub base: StandardMutateResult,
    pub relationships_written: i64,
    pub probability_distribution: AnyMap,
    pub sampling_stats: AnyMap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PredictMutateResult {
    #[serde(flatten)]
    pub base: StandardMutateResult,
    pub node_properties_written: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteResult {
    #[serde(flatten)]
    pub base: StandardWriteResult,
    pub node_properties_written: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MLTrainResult {
    pub train_millis: i64,
    pub model_info: AnyMap,
    pub configuration: AnyMap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkPredictionTrainResult {
    #[serde(flatten)]
    pub base: MLTrainResult,
    pub model_selection_stats: AnyMap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeClassificationPipelineTrainResult {
    #[serde(flatten)]
    pub base: MLTrainResult,
    pub model_selection_stats: AnyMap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeRegressionPipelineTrainResult {
    #[serde(flatten)]
    pub base: MLTrainResult,
    pub model_selection_stats: AnyMap,
}
