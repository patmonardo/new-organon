use crate::applications::algorithms::machinery::{AlgorithmProcessingTimings, ResultRenderer};
use crate::core::loading::GraphResources;
use crate::procedures::pipelines::types::{MLTrainResult, NodeRegressionPipelineTrainResult};
use crate::projection::eval::pipeline::node_pipeline::regression::node_regression_train_result::NodeRegressionTrainPipelineResult;
use crate::projection::eval::pipeline::node_pipeline::node_property_pipeline_base_train_config::NodePropertyPipelineBaseTrainConfig;
use serde_json::Value;
use std::collections::HashMap;

pub struct NodeRegressionTrainResultRenderer;

impl ResultRenderer<NodeRegressionTrainPipelineResult, Vec<NodeRegressionPipelineTrainResult>, ()>
    for NodeRegressionTrainResultRenderer
{
    fn render(
        &self,
        _graph_resources: &GraphResources,
        result: Option<NodeRegressionTrainPipelineResult>,
        timings: AlgorithmProcessingTimings,
        _metadata: Option<()>,
    ) -> Vec<NodeRegressionPipelineTrainResult> {
        match result {
            None => vec![],
            Some(train_result) => vec![NodeRegressionPipelineTrainResult {
                base: MLTrainResult {
                    train_millis: timings.compute_millis,
                    model_info: render_model_info(&train_result),
                    configuration: render_train_config(&train_result),
                },
                model_selection_stats: train_result.training_statistics().to_map(),
            }],
        }
    }
}

fn render_model_info(result: &NodeRegressionTrainPipelineResult) -> HashMap<String, Value> {
    let mut info = HashMap::new();
    info.insert(
        "modelType".to_string(),
        Value::String("NodeRegression".to_string()),
    );
    info.insert(
        "modelInfo".to_string(),
        Value::Object(result.model_info().to_map().into_iter().collect()),
    );
    info
}

fn render_train_config(result: &NodeRegressionTrainPipelineResult) -> HashMap<String, Value> {
    let config = result.train_config();
    let mut map = HashMap::new();
    map.insert(
        "pipeline".to_string(),
        Value::String(config.pipeline().to_string()),
    );
    map.insert(
        "targetNodeLabels".to_string(),
        Value::Array(
            config
                .target_node_labels()
                .into_iter()
                .map(Value::String)
                .collect(),
        ),
    );
    map.insert(
        "targetProperty".to_string(),
        Value::String(config.target_property().to_string()),
    );
    map.insert(
        "randomSeed".to_string(),
        config
            .random_seed()
            .map(|seed| Value::Number(serde_json::Number::from(seed)))
            .unwrap_or(Value::Null),
    );
    map.insert(
        "metrics".to_string(),
        Value::Array(
            config
                .metrics()
                .iter()
                .map(|metric| Value::String(metric.to_string()))
                .collect(),
        ),
    );
    map
}
