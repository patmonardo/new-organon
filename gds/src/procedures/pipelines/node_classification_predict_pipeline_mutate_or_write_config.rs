use serde_json::Value;

use crate::procedures::pipelines::types::AnyMap;
use crate::procedures::pipelines::{
    NodeClassificationPredictPipelineBaseConfig, NodeClassificationPredictPipelineConfig,
};

#[derive(Debug, Clone)]
pub struct NodeClassificationPredictPipelineMutateOrWriteConfig {
    base: NodeClassificationPredictPipelineBaseConfig,
    predicted_probability_property: Option<String>,
}

impl NodeClassificationPredictPipelineMutateOrWriteConfig {
    pub fn new(
        base: NodeClassificationPredictPipelineBaseConfig,
        predicted_probability_property: Option<String>,
    ) -> Self {
        Self {
            base,
            predicted_probability_property,
        }
    }

    pub fn predicted_probability_property(&self) -> Option<&str> {
        self.predicted_probability_property.as_deref()
    }

    pub fn base(&self) -> &NodeClassificationPredictPipelineBaseConfig {
        &self.base
    }

    pub fn to_map(&self) -> AnyMap {
        let mut map = self.base.to_map();
        if let Some(prop) = &self.predicted_probability_property {
            map.insert(
                "predictedProbabilityProperty".to_string(),
                Value::String(prop.clone()),
            );
        }
        map
    }
}

impl NodeClassificationPredictPipelineConfig
    for NodeClassificationPredictPipelineMutateOrWriteConfig
{
    fn graph_name(&self) -> &str {
        self.base.graph_name()
    }

    fn concurrency(&self) -> usize {
        self.base.concurrency()
    }

    fn model_name(&self) -> &str {
        self.base.model_name()
    }

    fn model_user(&self) -> &str {
        self.base.model_user()
    }

    fn username(&self) -> String {
        self.base.username()
    }

    fn target_node_labels(&self) -> &[String] {
        self.base.target_node_labels()
    }

    fn relationship_types(&self) -> &[String] {
        self.base.relationship_types()
    }

    fn include_predicted_probabilities(&self) -> bool {
        self.predicted_probability_property.is_some()
    }
}
