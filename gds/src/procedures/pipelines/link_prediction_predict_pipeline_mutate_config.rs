use serde_json::Value;

use crate::procedures::pipelines::{
    LinkPredictionPredictPipelineBaseConfig, LinkPredictionPredictPipelineConfig,
};
use crate::procedures::pipelines::types::AnyMap;

#[derive(Debug, Clone)]
pub struct LinkPredictionPredictPipelineMutateConfig {
    base: LinkPredictionPredictPipelineBaseConfig,
    mutate_relationship_type: String,
    mutate_property: String,
}

impl LinkPredictionPredictPipelineMutateConfig {
    pub fn new(
        base: LinkPredictionPredictPipelineBaseConfig,
        mutate_relationship_type: String,
        mutate_property: String,
    ) -> Self {
        Self {
            base,
            mutate_relationship_type,
            mutate_property,
        }
    }

    pub fn from_map(username: String, mut config: AnyMap) -> Self {
        let mutate_relationship_type = config
            .remove("mutateRelationshipType")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "PREDICTED".to_string());

        let mutate_property = config
            .remove("mutateProperty")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "probability".to_string());

        let base = LinkPredictionPredictPipelineBaseConfig::from_map(username, config);

        Self::new(base, mutate_relationship_type, mutate_property)
    }

    pub fn mutate_relationship_type(&self) -> &str {
        &self.mutate_relationship_type
    }

    pub fn mutate_property(&self) -> &str {
        &self.mutate_property
    }

    pub fn to_map(&self) -> AnyMap {
        let mut map = self.base.to_map();
        map.insert(
            "mutateRelationshipType".to_string(),
            Value::String(self.mutate_relationship_type.clone()),
        );
        map.insert(
            "mutateProperty".to_string(),
            Value::String(self.mutate_property.clone()),
        );
        map
    }
}

impl LinkPredictionPredictPipelineConfig for LinkPredictionPredictPipelineMutateConfig {
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

    fn sample_rate(&self) -> f64 {
        self.base.sample_rate()
    }

    fn source_node_label(&self) -> Option<&str> {
        self.base.source_node_label()
    }

    fn target_node_label(&self) -> Option<&str> {
        self.base.target_node_label()
    }

    fn relationship_types(&self) -> &[String] {
        self.base.relationship_types()
    }

    fn top_n(&self) -> Option<usize> {
        self.base.top_n()
    }

    fn threshold(&self) -> Option<f64> {
        self.base.threshold()
    }

    fn top_k(&self) -> Option<usize> {
        self.base.top_k()
    }

    fn delta_threshold(&self) -> Option<f64> {
        self.base.delta_threshold()
    }

    fn max_iterations(&self) -> Option<usize> {
        self.base.max_iterations()
    }

    fn random_joins(&self) -> Option<usize> {
        self.base.random_joins()
    }

    fn initial_sampler(&self) -> Option<&str> {
        self.base.initial_sampler()
    }
}
