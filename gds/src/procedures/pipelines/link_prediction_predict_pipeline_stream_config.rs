use crate::procedures::pipelines::types::AnyMap;
use crate::procedures::pipelines::{
    LinkPredictionPredictPipelineBaseConfig, LinkPredictionPredictPipelineConfig,
};

#[derive(Debug, Clone)]
pub struct LinkPredictionPredictPipelineStreamConfig {
    base: LinkPredictionPredictPipelineBaseConfig,
}

impl LinkPredictionPredictPipelineStreamConfig {
    pub fn new(base: LinkPredictionPredictPipelineBaseConfig) -> Self {
        Self { base }
    }

    pub fn from_map(username: String, config: AnyMap) -> Self {
        let base = LinkPredictionPredictPipelineBaseConfig::from_map(username, config);
        Self::new(base)
    }

    pub fn to_map(&self) -> AnyMap {
        self.base.to_map()
    }
}

impl LinkPredictionPredictPipelineConfig for LinkPredictionPredictPipelineStreamConfig {
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
