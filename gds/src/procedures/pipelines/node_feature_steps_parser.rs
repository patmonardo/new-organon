use crate::projection::eval::pipeline::node_pipeline::node_feature_step::NodeFeatureStep;
use serde_json::Value;

pub struct NodeFeatureStepsParser;

impl NodeFeatureStepsParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, node_feature_steps: Value, label: &str) -> Vec<NodeFeatureStep> {
        match node_feature_steps {
            Value::String(value) => vec![NodeFeatureStep::of(&value)],
            Value::Array(values) => values
                .into_iter()
                .map(|value| match value {
                    Value::String(value) => NodeFeatureStep::of(&value),
                    _ => panic!("The list `{}` is required to contain only strings.", label),
                })
                .collect(),
            _ => panic!(
                "The value of `{}` is required to be a list of strings.",
                label
            ),
        }
    }
}
