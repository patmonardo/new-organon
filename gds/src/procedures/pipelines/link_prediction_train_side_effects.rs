use crate::applications::algorithms::machinery::SideEffect;
use crate::core::loading::GraphResources;
use crate::projection::eval::pipeline::link_pipeline::train::{
    LinkPredictionTrainConfig, LinkPredictionTrainPipelineResult,
};

pub struct LinkPredictionTrainSideEffects {
    #[allow(dead_code)]
    configuration: LinkPredictionTrainConfig,
}

impl LinkPredictionTrainSideEffects {
    pub fn new(configuration: LinkPredictionTrainConfig) -> Self {
        Self { configuration }
    }
}

impl SideEffect<LinkPredictionTrainPipelineResult, ()> for LinkPredictionTrainSideEffects {
    fn process(
        &self,
        _graph_resources: &GraphResources,
        _result: Option<&LinkPredictionTrainPipelineResult>,
    ) -> Option<()> {
        None
    }
}
