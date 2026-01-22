use std::sync::Arc;

use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{EmptyTaskRegistryFactory, JobId, TaskProgressTracker};
use crate::ml::core::subgraph::LocalIdMap;
use crate::projection::eval::pipeline::node_pipeline::classification::node_classification_model_result::NodeClassificationModelResult;
use crate::projection::eval::pipeline::{PredictPipelineExecutor, PredictPipelineExecutorError};
use crate::types::graph_store::DefaultGraphStore;

use super::{
    NodeClassificationPipelineResult, NodeClassificationPredictPipelineBaseConfig,
    NodeClassificationPredictPipelineConfig, NodeClassificationPredictPipelineExecutor,
};

pub struct NodeClassificationPredictComputation {
    configuration: NodeClassificationPredictPipelineBaseConfig,
    label: String,
    model: Arc<NodeClassificationModelResult>,
}

impl NodeClassificationPredictComputation {
    pub fn new(
        configuration: NodeClassificationPredictPipelineBaseConfig,
        label: String,
        model: Arc<NodeClassificationModelResult>,
    ) -> Self {
        Self {
            configuration,
            label,
            model,
        }
    }

    pub fn compute(
        &self,
        graph_store: Arc<DefaultGraphStore>,
    ) -> Result<NodeClassificationPipelineResult, PredictPipelineExecutorError> {
        let model_info = self.model.model_info();
        let class_ids: Vec<u64> = model_info.classes().iter().map(|id| *id as u64).collect();
        let class_id_map = LocalIdMap::of(&class_ids);

        let task = NodeClassificationPredictPipelineExecutor::progress_task(
            &self.label,
            graph_store.as_ref(),
        );
        let tracker = TaskProgressTracker::with_registry(
            task,
            Concurrency::of(self.configuration.concurrency()),
            JobId::new(),
            &EmptyTaskRegistryFactory,
        );

        let classifier_data = self.model.classifier().data();

        let mut executor = NodeClassificationPredictPipelineExecutor::new(
            model_info.pipeline(),
            &self.configuration,
            graph_store,
            tracker,
            classifier_data,
            class_id_map,
            TerminationFlag::running_true(),
        );

        executor.compute()
    }
}
