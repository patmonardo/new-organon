use crate::projection::eval::pipeline::node_pipeline::classification::node_classification_training_pipeline::NodeClassificationTrainingPipeline;
use crate::projection::eval::pipeline::node_pipeline::node_property_training_pipeline::NodePropertyTrainingPipeline;
use crate::projection::eval::pipeline::node_pipeline::regression::node_regression_training_pipeline::NodeRegressionTrainingPipeline;
use crate::projection::eval::pipeline::{Pipeline, TrainingPipeline};
use crate::procedures::pipelines::types::{AnyMap, AnyMapList, NodePipelineInfoResult};
use serde_json::Value;

pub fn create_node_classification_info_result(
    pipeline_name: &str,
    pipeline: &NodeClassificationTrainingPipeline,
) -> NodePipelineInfoResult {
    let node_property_steps: AnyMapList = pipeline
        .node_property_steps()
        .iter()
        .map(|step| step.to_map())
        .collect();

    let split_config: AnyMap = pipeline
        .split_config()
        .to_map()
        .into_iter()
        .map(|(k, v)| (k, Value::String(v)))
        .collect();

    NodePipelineInfoResult {
        name: pipeline_name.to_string(),
        node_property_steps,
        feature_properties: pipeline.feature_properties(),
        split_config,
        auto_tuning_config: pipeline.auto_tuning_config().to_map(),
        parameter_space: pipeline.parameter_space_to_map(),
    }
}

pub fn create_node_regression_info_result(
    pipeline_name: &str,
    pipeline: &NodeRegressionTrainingPipeline,
) -> NodePipelineInfoResult {
    let node_property_steps: AnyMapList = pipeline
        .node_property_steps()
        .iter()
        .map(|step| step.to_map())
        .collect();

    let split_config: AnyMap = pipeline
        .split_config()
        .to_map()
        .into_iter()
        .map(|(k, v)| (k, Value::String(v)))
        .collect();

    NodePipelineInfoResult {
        name: pipeline_name.to_string(),
        node_property_steps,
        feature_properties: pipeline.feature_properties(),
        split_config,
        auto_tuning_config: pipeline.auto_tuning_config().to_map(),
        parameter_space: pipeline.parameter_space_to_map(),
    }
}
