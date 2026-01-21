use crate::applications::algorithms::machinery::StreamResultBuilder;
use crate::core::loading::GraphResources;
use crate::procedures::pipelines::types::NodeClassificationStreamResult;
use crate::procedures::pipelines::{
    NodeClassificationPipelineResult, NodeClassificationPredictPipelineStreamConfig,
};
use crate::types::graph_store::GraphStore;

pub struct NodeClassificationPredictPipelineStreamResultBuilder {
    _configuration: NodeClassificationPredictPipelineStreamConfig,
}

impl NodeClassificationPredictPipelineStreamResultBuilder {
    pub fn new(configuration: NodeClassificationPredictPipelineStreamConfig) -> Self {
        Self {
            _configuration: configuration,
        }
    }
}

impl StreamResultBuilder<NodeClassificationPipelineResult, NodeClassificationStreamResult>
    for NodeClassificationPredictPipelineStreamResultBuilder
{
    type Stream = std::vec::IntoIter<NodeClassificationStreamResult>;

    fn build(
        &self,
        graph_resources: &GraphResources,
        result: Option<NodeClassificationPipelineResult>,
    ) -> Self::Stream {
        let Some(result) = result else {
            return Vec::new().into_iter();
        };

        let graph = graph_resources.graph_store.get_graph();
        let predicted_classes = result.predicted_classes();
        let predicted_probabilities = result.predicted_probabilities();

        let mut rows = Vec::with_capacity(graph.node_count());
        for node_id in 0..graph.node_count() {
            let original = graph
                .to_original_node_id(node_id as i64)
                .unwrap_or(node_id as i64);
            let predicted = predicted_classes.get(node_id) as i64;
            let probs = predicted_probabilities.map(|p| p.get(node_id).clone());

            rows.push(NodeClassificationStreamResult {
                node_id: original as i64,
                predicted_class: predicted,
                predicted_probabilities: probs,
            });
        }

        rows.into_iter()
    }
}
