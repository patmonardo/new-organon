use crate::applications::algorithms::machinery::{
    GraphStoreNodePropertiesWritten, Label, WriteStep, WriteToDatabase,
};
use crate::applications::graph_store_catalog::loaders::ResultStore;
use crate::core::utils::progress::JobId;
use crate::procedures::GraphFacade;
use crate::projection::NodeLabel;
use crate::types::graph_store::{DefaultGraphStore, GraphStore};

use super::{
    predicted_probabilities::as_properties, NodeClassificationPipelineResult,
    NodeClassificationPredictPipelineConfig, NodeClassificationPredictPipelineWriteConfig,
};

pub struct NodeClassificationPredictPipelineWriteStep {
    write_to_database: WriteToDatabase,
    configuration: NodeClassificationPredictPipelineWriteConfig,
    label: Box<dyn Label + Send + Sync>,
}

impl NodeClassificationPredictPipelineWriteStep {
    pub fn new(
        write_to_database: WriteToDatabase,
        configuration: NodeClassificationPredictPipelineWriteConfig,
        label: Box<dyn Label + Send + Sync>,
    ) -> Self {
        Self {
            write_to_database,
            configuration,
            label,
        }
    }

    fn _resolve_target_labels(
        &self,
        graph_store: &DefaultGraphStore,
    ) -> std::collections::HashSet<NodeLabel> {
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

impl WriteStep<NodeClassificationPipelineResult, GraphStoreNodePropertiesWritten>
    for NodeClassificationPredictPipelineWriteStep
{
    fn execute(
        &self,
        graph: &GraphFacade,
        graph_store: &DefaultGraphStore,
        result_store: Option<&dyn ResultStore>,
        result: &NodeClassificationPipelineResult,
        job_id: &JobId,
    ) -> GraphStoreNodePropertiesWritten {
        let _ = graph;
        let _ = result_store;
        let _ = job_id;
        let _ = self.label.as_ref();
        let _ = &self.write_to_database;

        let node_properties = as_properties(
            Some(result),
            self.configuration.write_property(),
            self.configuration.predicted_probability_property(),
        );

        GraphStoreNodePropertiesWritten(node_properties.len() * graph_store.node_count())
    }
}
