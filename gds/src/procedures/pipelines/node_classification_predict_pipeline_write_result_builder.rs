use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTimings, GraphStoreNodePropertiesWritten, WriteResultBuilder,
};
use crate::core::loading::GraphResources;
use crate::procedures::pipelines::types::{StandardWriteResult, WriteResult};
use crate::procedures::pipelines::{
    NodeClassificationPipelineResult, NodeClassificationPredictPipelineWriteConfig,
};

pub struct NodeClassificationPredictPipelineWriteResultBuilder {
    _configuration: NodeClassificationPredictPipelineWriteConfig,
}

impl NodeClassificationPredictPipelineWriteResultBuilder {
    pub fn new(configuration: NodeClassificationPredictPipelineWriteConfig) -> Self {
        Self {
            _configuration: configuration,
        }
    }
}

impl
    WriteResultBuilder<
        NodeClassificationPredictPipelineWriteConfig,
        NodeClassificationPipelineResult,
        WriteResult,
        GraphStoreNodePropertiesWritten,
    > for NodeClassificationPredictPipelineWriteResultBuilder
{
    fn build(
        &self,
        _graph_resources: &GraphResources,
        configuration: &NodeClassificationPredictPipelineWriteConfig,
        result: Option<NodeClassificationPipelineResult>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<GraphStoreNodePropertiesWritten>,
    ) -> WriteResult {
        let node_properties_written = metadata.map(|m| m.0 as i64).unwrap_or(0);

        WriteResult {
            base: StandardWriteResult {
                pre_processing_millis: timings.pre_processing_millis,
                compute_millis: timings.compute_millis,
                write_millis: timings.side_effect_millis,
                configuration: configuration.to_map(),
            },
            node_properties_written: if result.is_some() {
                node_properties_written
            } else {
                0
            },
        }
    }
}
