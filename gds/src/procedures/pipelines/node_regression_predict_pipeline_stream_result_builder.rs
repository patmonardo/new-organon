use crate::applications::algorithms::machinery::StreamResultBuilder;
use crate::collections::HugeDoubleArray;
use crate::core::loading::GraphResources;
use crate::procedures::pipelines::types::NodeRegressionStreamResult;
use crate::types::graph_store::GraphStore;

pub struct NodeRegressionPredictPipelineStreamResultBuilder;

impl StreamResultBuilder<HugeDoubleArray, NodeRegressionStreamResult>
    for NodeRegressionPredictPipelineStreamResultBuilder
{
    type Stream = std::vec::IntoIter<NodeRegressionStreamResult>;

    fn build(
        &self,
        graph_resources: &GraphResources,
        result: Option<HugeDoubleArray>,
    ) -> Self::Stream {
        let Some(predicted) = result else {
            return Vec::new().into_iter();
        };

        let graph = graph_resources.graph_store.get_graph();
        let mut rows = Vec::with_capacity(graph.node_count());
        for node_id in 0..graph.node_count() {
            let original = graph
                .to_original_node_id(node_id as i64)
                .unwrap_or(node_id as i64);
            rows.push(NodeRegressionStreamResult {
                node_id: original as i64,
                predicted_value: predicted.get(node_id),
            });
        }

        rows.into_iter()
    }
}
