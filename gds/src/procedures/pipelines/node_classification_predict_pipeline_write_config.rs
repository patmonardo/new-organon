use serde_json::Value;

use crate::applications::algorithms::machinery::{WriteConfigLike, WritePropertyConfigLike};
use crate::concurrency::Concurrency;
use crate::procedures::pipelines::{
    NodeClassificationPredictPipelineConfig, NodeClassificationPredictPipelineMutateOrWriteConfig,
};
use crate::procedures::pipelines::types::AnyMap;

#[derive(Debug, Clone)]
pub struct NodeClassificationPredictPipelineWriteConfig {
    base: NodeClassificationPredictPipelineMutateOrWriteConfig,
    write_property: String,
    write_concurrency: usize,
}

impl NodeClassificationPredictPipelineWriteConfig {
    pub fn new(
        base: NodeClassificationPredictPipelineMutateOrWriteConfig,
        write_property: String,
        write_concurrency: usize,
    ) -> Self {
        Self {
            base,
            write_property,
            write_concurrency,
        }
    }

    pub fn write_property(&self) -> &str {
        &self.write_property
    }

    pub fn write_concurrency(&self) -> usize {
        self.write_concurrency
    }

    pub fn predicted_probability_property(&self) -> Option<&str> {
        self.base.predicted_probability_property()
    }

    pub fn to_map(&self) -> AnyMap {
        let mut map = self.base.to_map();
        map.insert(
            "writeProperty".to_string(),
            Value::String(self.write_property.clone()),
        );
        map.insert(
            "writeConcurrency".to_string(),
            Value::Number(serde_json::Number::from(self.write_concurrency as i64)),
        );
        map
    }

    pub fn validate_write_properties_differ(&self) {
        if let Some(predicted) = self.predicted_probability_property() {
            if self.write_property == predicted {
                panic!(
                    "Configuration parameters `writeProperty` and `predictedProbabilityProperty` must be different (both were `{}`)",
                    predicted
                );
            }
        }
    }
}

impl NodeClassificationPredictPipelineConfig for NodeClassificationPredictPipelineWriteConfig {
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
        self.base.include_predicted_probabilities()
    }
}

impl WriteConfigLike for NodeClassificationPredictPipelineWriteConfig {
    fn write_concurrency(&self) -> Concurrency {
        Concurrency::of(self.write_concurrency)
    }
}

impl WritePropertyConfigLike for NodeClassificationPredictPipelineWriteConfig {
    fn write_property(&self) -> &str {
        &self.write_property
    }
}
