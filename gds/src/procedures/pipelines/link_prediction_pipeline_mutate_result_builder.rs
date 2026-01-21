use crate::applications::algorithms::machinery::{AlgorithmProcessingTimings, MutateResultBuilder};
use crate::core::loading::GraphResources;
use crate::ml::link_models::LinkPredictionResult;
use crate::procedures::pipelines::types::{MutateResult, StandardMutateResult};
use crate::procedures::pipelines::{
    LinkPredictionMutateMetadata, LinkPredictionPredictPipelineMutateConfig,
};

pub struct LinkPredictionPipelineMutateResultBuilder {
    _configuration: LinkPredictionPredictPipelineMutateConfig,
}

impl LinkPredictionPipelineMutateResultBuilder {
    pub fn new(configuration: LinkPredictionPredictPipelineMutateConfig) -> Self {
        Self {
            _configuration: configuration,
        }
    }
}

impl
    MutateResultBuilder<
        LinkPredictionPredictPipelineMutateConfig,
        Box<dyn LinkPredictionResult>,
        MutateResult,
        LinkPredictionMutateMetadata,
    > for LinkPredictionPipelineMutateResultBuilder
{
    fn build(
        &self,
        _graph_resources: &GraphResources,
        configuration: &LinkPredictionPredictPipelineMutateConfig,
        result: Option<Box<dyn LinkPredictionResult>>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<LinkPredictionMutateMetadata>,
    ) -> MutateResult {
        let relationships_written = metadata
            .as_ref()
            .map(|m| m.relationships_written())
            .unwrap_or(0);
        let probability_distribution = metadata
            .map(|m| m.probability_distribution().clone())
            .unwrap_or_default();
        let sampling_stats = result
            .as_ref()
            .map(|r| r.sampling_stats())
            .unwrap_or_default();

        MutateResult {
            base: StandardMutateResult {
                pre_processing_millis: timings.pre_processing_millis,
                compute_millis: timings.compute_millis,
                mutate_millis: timings.side_effect_millis,
                configuration: configuration.to_map(),
            },
            relationships_written: if result.is_some() {
                relationships_written
            } else {
                0
            },
            probability_distribution,
            sampling_stats,
        }
    }
}
