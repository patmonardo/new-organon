use std::collections::HashSet;
use std::sync::Arc;

use crate::applications::algorithms::machinery::{
    GraphStoreNodePropertiesWritten, GraphStoreService, MutateStep,
};
use crate::core::loading::GraphResources;
use crate::projection::NodeLabel;
use crate::types::graph_store::{DefaultGraphStore, GraphStore};

use super::{
    predicted_probabilities::as_properties, NodeClassificationPipelineResult,
    NodeClassificationPredictPipelineConfig, NodeClassificationPredictPipelineMutateConfig,
};

pub struct NodeClassificationPredictPipelineMutateStep {
    graph_store_service: GraphStoreService,
    configuration: NodeClassificationPredictPipelineMutateConfig,
}

impl NodeClassificationPredictPipelineMutateStep {
    pub fn new(
        graph_store_service: GraphStoreService,
        configuration: NodeClassificationPredictPipelineMutateConfig,
    ) -> Self {
        Self {
            graph_store_service,
            configuration,
        }
    }

    fn resolve_target_labels(&self, graph_store: &DefaultGraphStore) -> HashSet<NodeLabel> {
        let labels = self.configuration.target_node_labels();
        if labels.is_empty() || (labels.len() == 1 && labels[0] == "*") {
            graph_store.node_labels()
        } else {
            labels
                .iter()
                .map(|label| NodeLabel::of(label.clone()))
                .collect()
        }
    }
}

impl MutateStep<NodeClassificationPipelineResult, GraphStoreNodePropertiesWritten>
    for NodeClassificationPredictPipelineMutateStep
{
    fn execute(
        &self,
        graph_resources: &GraphResources,
        result: &NodeClassificationPipelineResult,
    ) -> GraphStoreNodePropertiesWritten {
        let mut graph_store = Arc::clone(&graph_resources.graph_store);
        let graph_store_mut = Arc::get_mut(&mut graph_store)
            .expect("Graph store is shared; cannot mutate predicted properties");

        let labels_to_update = self.resolve_target_labels(graph_store_mut);
        let node_properties = as_properties(
            Some(result),
            self.configuration.mutate_property(),
            self.configuration.predicted_probability_property(),
        );

        self.graph_store_service
            .add_node_properties(graph_store_mut, labels_to_update, &node_properties)
            .unwrap_or_else(|e| panic!("Failed to add node properties: {e}"))
    }
}
