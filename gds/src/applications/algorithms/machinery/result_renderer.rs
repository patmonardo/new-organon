//! ResultRenderer (Java parity).

use crate::core::loading::GraphResources;

use super::AlgorithmProcessingTimings;

pub trait ResultRenderer<ResultFromAlgorithm, ResultToCaller, SideEffectMetadata> {
    fn render(
        &self,
        graph_resources: &GraphResources,
        result: Option<ResultFromAlgorithm>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<SideEffectMetadata>,
    ) -> ResultToCaller;
}
