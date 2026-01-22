use crate::applications::algorithms::machinery::{AlgorithmProcessingTimings, ResultRenderer};
use crate::core::loading::GraphResources;
use crate::procedures::pipelines::types::{MLTrainResult, NodeClassificationPipelineTrainResult};
use crate::projection::eval::pipeline::node_pipeline::classification::node_classification_model_result::NodeClassificationModelResult;
use crate::projection::eval::pipeline::node_pipeline::node_property_pipeline_base_train_config::NodePropertyPipelineBaseTrainConfig;
use serde_json::Value;
use std::collections::HashMap;

pub struct NodeClassificationTrainResultRenderer;

impl ResultRenderer<NodeClassificationModelResult, Vec<NodeClassificationPipelineTrainResult>, ()>
    for NodeClassificationTrainResultRenderer
{
    fn render(
        &self,
        _graph_resources: &GraphResources,
        result: Option<NodeClassificationModelResult>,
        timings: AlgorithmProcessingTimings,
        _metadata: Option<()>,
    ) -> Vec<NodeClassificationPipelineTrainResult> {
        match result {
            None => vec![],
            Some(model_result) => vec![NodeClassificationPipelineTrainResult {
                base: MLTrainResult {
                    train_millis: timings.compute_millis,
                    model_info: render_model_info(&model_result),
                    configuration: render_train_config(&model_result),
                },
                model_selection_stats: model_result.training_statistics().to_map(),
            }],
        }
    }
}

fn render_model_info(model_result: &NodeClassificationModelResult) -> HashMap<String, Value> {
    let mut info = HashMap::new();
    info.insert(
        "modelType".to_string(),
        Value::String("NodeClassification".to_string()),
    );
    info.insert(
        "modelInfo".to_string(),
        Value::Object(model_result.model_info().to_map().into_iter().collect()),
    );
    info
}

fn render_train_config(model_result: &NodeClassificationModelResult) -> HashMap<String, Value> {
    let config = model_result.train_config();
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
                .metrics_specs()
                .iter()
                .map(|spec| Value::String(spec.to_string()))
                .collect(),
        ),
    );
    map
}
