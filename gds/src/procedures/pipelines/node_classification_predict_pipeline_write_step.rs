use crate::applications::algorithms::machinery::{Label, WriteStep, WriteToDatabase};
use crate::core::utils::progress::JobId;
use crate::procedures::GraphFacade;
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
    ) -> std::collections::HashSet<crate::projection::NodeLabel> {
        let labels = self.configuration.target_node_labels();
        if labels.is_empty() || (labels.len() == 1 && labels[0] == "*") {
            graph_store.node_labels()
        } else {
            labels
                .iter()
                .map(|label| crate::projection::NodeLabel::of(label.clone()))
                .collect()
        }
    }
}

impl
    WriteStep<
        NodeClassificationPipelineResult,
        crate::applications::algorithms::machinery::GraphStoreNodePropertiesWritten,
    > for NodeClassificationPredictPipelineWriteStep
{
    fn execute(
        &self,
        graph: &GraphFacade,
        graph_store: &DefaultGraphStore,
        result_store: Option<&dyn crate::applications::graph_store_catalog::loaders::ResultStore>,
        result: &NodeClassificationPipelineResult,
        job_id: &JobId,
    ) -> crate::applications::algorithms::machinery::GraphStoreNodePropertiesWritten {
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

        crate::applications::algorithms::machinery::GraphStoreNodePropertiesWritten(
            node_properties.len() * graph_store.node_count(),
        )
    }
}
