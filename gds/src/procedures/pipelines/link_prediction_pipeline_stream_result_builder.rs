use crate::applications::algorithms::machinery::StreamResultBuilder;
use crate::core::loading::GraphResources;
use crate::ml::link_models::LinkPredictionResult;
use crate::procedures::pipelines::types::StreamResult;
use crate::types::graph_store::GraphStore;

pub struct LinkPredictionPipelineStreamResultBuilder;

impl StreamResultBuilder<Box<dyn LinkPredictionResult>, StreamResult>
    for LinkPredictionPipelineStreamResultBuilder
{
    type Stream = std::vec::IntoIter<StreamResult>;

    fn build(
        &self,
        graph_resources: &GraphResources,
        result: Option<Box<dyn LinkPredictionResult>>,
    ) -> Self::Stream {
        let Some(predictions) = result else {
            return Vec::new().into_iter();
        };

        let graph = graph_resources.graph_store.get_graph();
        let mut rows = Vec::new();

        for link in predictions.iter() {
            let source = graph
                .to_original_node_id(link.source_id())
                .unwrap_or(link.source_id());
            let target = graph
                .to_original_node_id(link.target_id())
                .unwrap_or(link.target_id());
            rows.push(StreamResult {
                node1: source as i64,
                node2: target as i64,
                probability: link.probability(),
            });
        }

        rows.into_iter()
    }
}
