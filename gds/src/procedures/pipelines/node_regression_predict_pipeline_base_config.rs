use serde_json::Value;

use crate::procedures::pipelines::types::AnyMap;

pub trait NodeRegressionPredictPipelineConfig {
    fn graph_name(&self) -> &str;
    fn concurrency(&self) -> usize;
    fn model_name(&self) -> &str;
    fn model_user(&self) -> &str;
    fn username(&self) -> String;
    fn target_node_labels(&self) -> &[String];
    fn relationship_types(&self) -> &[String];

    fn node_labels(&self) -> &[String] {
        self.target_node_labels()
    }
}

#[derive(Debug, Clone)]
pub struct NodeRegressionPredictPipelineBaseConfig {
    graph_name: String,
    concurrency: usize,
    model_name: String,
    model_user: String,
    username_override: Option<String>,
    target_node_labels: Vec<String>,
    relationship_types: Vec<String>,
}

impl NodeRegressionPredictPipelineBaseConfig {
    pub fn new(
        graph_name: String,
        concurrency: usize,
        model_name: String,
        model_user: String,
        username_override: Option<String>,
        target_node_labels: Vec<String>,
        relationship_types: Vec<String>,
    ) -> Self {
        Self {
            graph_name,
            concurrency,
            model_name,
            model_user,
            username_override,
            target_node_labels,
            relationship_types,
        }
    }

    pub fn from_map(username: String, mut config: AnyMap) -> Self {
        let graph_name = config
            .remove("graphName")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "graph".to_string());

        let concurrency = config
            .remove("concurrency")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or_else(num_cpus::get);

        let model_name = config
            .remove("modelName")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "default_model".to_string());

        let model_user = config
            .remove("modelUser")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| username.clone());

        let target_node_labels = config
            .remove("targetNodeLabels")
            .and_then(|v| v.as_array().cloned())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_else(|| vec!["*".to_string()]);

        let relationship_types = config
            .remove("relationshipTypes")
            .and_then(|v| v.as_array().cloned())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        Self::new(
            graph_name,
            concurrency,
            model_name,
            model_user,
            None,
            target_node_labels,
            relationship_types,
        )
    }

    pub fn to_map(&self) -> AnyMap {
        let mut map = AnyMap::new();
        map.insert("graphName".to_string(), Value::String(self.graph_name.clone()));
        map.insert(
            "concurrency".to_string(),
            Value::Number(serde_json::Number::from(self.concurrency as i64)),
        );
        map.insert("modelName".to_string(), Value::String(self.model_name.clone()));
        map.insert("modelUser".to_string(), Value::String(self.model_user.clone()));
        map.insert(
            "targetNodeLabels".to_string(),
            Value::Array(
                self.target_node_labels
                    .iter()
                    .map(|v| Value::String(v.clone()))
                    .collect(),
            ),
        );
        map.insert(
            "relationshipTypes".to_string(),
            Value::Array(
                self.relationship_types
                    .iter()
                    .map(|v| Value::String(v.clone()))
                    .collect(),
            ),
        );
        map
    }
}

impl NodeRegressionPredictPipelineConfig for NodeRegressionPredictPipelineBaseConfig {
    fn graph_name(&self) -> &str {
        &self.graph_name
    }

    fn concurrency(&self) -> usize {
        self.concurrency
    }

    fn model_name(&self) -> &str {
        &self.model_name
    }

    fn model_user(&self) -> &str {
        &self.model_user
    }

    fn username(&self) -> String {
        self.username_override
            .clone()
            .unwrap_or_else(|| self.model_user.clone())
    }

    fn target_node_labels(&self) -> &[String] {
        &self.target_node_labels
    }

    fn relationship_types(&self) -> &[String] {
        &self.relationship_types
    }
}
