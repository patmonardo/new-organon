use serde_json::Value;

use crate::procedures::pipelines::{
    NodeRegressionPredictPipelineBaseConfig, NodeRegressionPredictPipelineConfig,
};
use crate::procedures::pipelines::types::AnyMap;

#[derive(Debug, Clone)]
pub struct NodeRegressionPredictPipelineMutateConfig {
    base: NodeRegressionPredictPipelineBaseConfig,
    mutate_property: String,
    write_concurrency: usize,
}

impl NodeRegressionPredictPipelineMutateConfig {
    pub fn new(
        base: NodeRegressionPredictPipelineBaseConfig,
        mutate_property: String,
        write_concurrency: usize,
    ) -> Self {
        Self {
            base,
            mutate_property,
            write_concurrency,
        }
    }

    pub fn mutate_property(&self) -> &str {
        &self.mutate_property
    }

    pub fn write_concurrency(&self) -> usize {
        self.write_concurrency
    }

    pub fn to_map(&self) -> AnyMap {
        let mut map = self.base.to_map();
        map.insert(
            "mutateProperty".to_string(),
            Value::String(self.mutate_property.clone()),
        );
        map.insert(
            "writeConcurrency".to_string(),
            Value::Number(serde_json::Number::from(self.write_concurrency as i64)),
        );
        map
    }
}

impl NodeRegressionPredictPipelineConfig for NodeRegressionPredictPipelineMutateConfig {
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
}
