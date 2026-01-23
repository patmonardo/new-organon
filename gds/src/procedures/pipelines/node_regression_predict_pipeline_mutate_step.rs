use std::collections::HashSet;
use std::sync::Arc;

use crate::applications::algorithms::machinery::{
    GraphStoreNodePropertiesWritten, GraphStoreService, MutateStep, NodeProperty,
};
use crate::collections::backends::vec::VecDouble;
use crate::collections::HugeDoubleArray;
use crate::core::loading::GraphResources;
use crate::projection::NodeLabel;
use crate::types::graph_store::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultDoubleNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;

use super::{NodeRegressionPredictPipelineConfig, NodeRegressionPredictPipelineMutateConfig};

pub struct NodeRegressionPredictPipelineMutateStep {
    graph_store_service: GraphStoreService,
    configuration: NodeRegressionPredictPipelineMutateConfig,
}

impl NodeRegressionPredictPipelineMutateStep {
    pub fn new(
        graph_store_service: GraphStoreService,
        configuration: NodeRegressionPredictPipelineMutateConfig,
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

impl MutateStep<HugeDoubleArray, GraphStoreNodePropertiesWritten>
    for NodeRegressionPredictPipelineMutateStep
{
    fn execute(
        &self,
        graph_resources: &GraphResources,
        result: &HugeDoubleArray,
    ) -> GraphStoreNodePropertiesWritten {
        let mut graph_store = Arc::clone(&graph_resources.graph_store);
        let graph_store_mut = Arc::get_mut(&mut graph_store)
            .expect("Graph store is shared; cannot mutate predicted properties");

        let labels_to_update = self.resolve_target_labels(graph_store_mut);
        let node_count = result.size();
        let mut values_vec = Vec::with_capacity(node_count);
        for idx in 0..node_count {
            values_vec.push(result.get(idx));
        }

        let values: Arc<dyn NodePropertyValues> = Arc::new(DefaultDoubleNodePropertyValues::<
            VecDouble,
        >::from_collection(
            VecDouble::from(values_vec),
            node_count,
        ));

        let node_properties = [NodeProperty::new(
            self.configuration.mutate_property(),
            values,
        )];

        self.graph_store_service
            .add_node_properties(graph_store_mut, labels_to_update, &node_properties)
            .unwrap_or_else(|e| panic!("Failed to add node properties: {e}"))
    }
}
