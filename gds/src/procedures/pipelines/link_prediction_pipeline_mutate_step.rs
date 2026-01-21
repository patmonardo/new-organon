use crate::applications::algorithms::machinery::MutateStep;
use crate::core::loading::GraphResources;
use crate::ml::link_models::LinkPredictionResult;
use crate::procedures::pipelines::{
    LinkPredictionMutateMetadata, LinkPredictionPredictPipelineMutateConfig,
};
use crate::procedures::pipelines::types::AnyMap;

pub struct LinkPredictionPipelineMutateStep {
    #[allow(dead_code)]
    configuration: LinkPredictionPredictPipelineMutateConfig,
}

impl LinkPredictionPipelineMutateStep {
    pub fn new(configuration: LinkPredictionPredictPipelineMutateConfig) -> Self {
        Self { configuration }
    }
}

impl MutateStep<Box<dyn LinkPredictionResult>, LinkPredictionMutateMetadata>
    for LinkPredictionPipelineMutateStep
{
    fn execute(
        &self,
        _graph_resources: &GraphResources,
        _result: &Box<dyn LinkPredictionResult>,
    ) -> LinkPredictionMutateMetadata {
        // Placeholder: relationship writes require graph-store mutation plumbing.
        LinkPredictionMutateMetadata::new(0, AnyMap::new())
    }
}
