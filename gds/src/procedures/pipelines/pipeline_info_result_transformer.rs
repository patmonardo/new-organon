use crate::procedures::pipelines::types::{AnyMapList, PipelineInfoResult};
use crate::projection::eval::pipeline::link_pipeline::LinkPredictionTrainingPipeline;
use crate::projection::eval::pipeline::{FeatureStep, Pipeline, TrainingPipeline};

pub fn create_pipeline_info_result(
    pipeline_name: &str,
    pipeline: &LinkPredictionTrainingPipeline,
) -> PipelineInfoResult {
    let node_property_steps: AnyMapList = pipeline
        .node_property_steps()
        .iter()
        .map(|step| step.to_map())
        .collect();

    let feature_steps: AnyMapList = pipeline
        .feature_steps()
        .iter()
        .map(|step| step.to_map())
        .collect();

    PipelineInfoResult {
        name: pipeline_name.to_string(),
        node_property_steps,
        feature_steps,
        split_config: pipeline.split_config().to_map(),
        auto_tuning_config: pipeline.auto_tuning_config().to_map(),
        parameter_space: serde_json::json!(pipeline.parameter_space_to_map()),
    }
}
