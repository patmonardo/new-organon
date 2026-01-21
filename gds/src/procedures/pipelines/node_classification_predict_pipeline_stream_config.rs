use crate::procedures::pipelines::{
    NodeClassificationPredictPipelineBaseConfig, NodeClassificationPredictPipelineConfig,
};

#[derive(Debug, Clone)]
pub struct NodeClassificationPredictPipelineStreamConfig {
    base: NodeClassificationPredictPipelineBaseConfig,
}

impl NodeClassificationPredictPipelineStreamConfig {
    pub fn new(base: NodeClassificationPredictPipelineBaseConfig) -> Self {
        Self { base }
    }
}

impl NodeClassificationPredictPipelineConfig for NodeClassificationPredictPipelineStreamConfig {
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
        false
    }
}
