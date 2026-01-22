use crate::applications::algorithms::machinery::{AlgorithmProcessingTimings, ResultRenderer};
use crate::core::loading::GraphResources;
use crate::procedures::pipelines::types::{LinkPredictionTrainResult, MLTrainResult};
use crate::projection::eval::pipeline::link_pipeline::train::LinkPredictionTrainPipelineResult;
use serde_json::{Map, Value};
use std::collections::HashMap;

pub struct LinkPredictionTrainResultRenderer;

impl ResultRenderer<LinkPredictionTrainPipelineResult, Vec<LinkPredictionTrainResult>, ()>
    for LinkPredictionTrainResultRenderer
{
    fn render(
        &self,
        _graph_resources: &GraphResources,
        result: Option<LinkPredictionTrainPipelineResult>,
        timings: AlgorithmProcessingTimings,
        _metadata: Option<()>,
    ) -> Vec<LinkPredictionTrainResult> {
        match result {
            None => vec![],
            Some(_train_result) => vec![LinkPredictionTrainResult {
                base: MLTrainResult {
                    train_millis: timings.compute_millis,
                    model_info: render_model_info(),
                    configuration: HashMap::new(),
                },
                model_selection_stats: HashMap::new(),
            }],
        }
    }
}

fn render_model_info() -> HashMap<String, Value> {
    let mut info = HashMap::new();
    info.insert(
        "modelType".to_string(),
        Value::String("LinkPrediction".to_string()),
    );
    info.insert("modelInfo".to_string(), Value::Object(Map::new()));
    info
}
