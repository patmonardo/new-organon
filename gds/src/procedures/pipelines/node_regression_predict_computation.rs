use std::sync::Arc;

use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{JobId, TaskProgressTracker};
use crate::projection::eval::pipeline::node_pipeline::NodePropertyPredictPipeline;
use crate::projection::eval::pipeline::PredictPipelineExecutor;
use crate::types::graph_store::DefaultGraphStore;

use super::{
    NodeRegressionPredictPipelineBaseConfig, NodeRegressionPredictPipelineConfig,
    NodeRegressionPredictPipelineExecutor,
};

pub struct NodeRegressionPredictComputation {
    configuration: NodeRegressionPredictPipelineBaseConfig,
    label: String,
    pipeline: NodePropertyPredictPipeline,
    regressor: Arc<dyn crate::ml::models::Regressor>,
}

impl NodeRegressionPredictComputation {
    pub fn new(
        configuration: NodeRegressionPredictPipelineBaseConfig,
        label: String,
        pipeline: NodePropertyPredictPipeline,
        regressor: Arc<dyn crate::ml::models::Regressor>,
    ) -> Self {
        Self {
            configuration,
            label,
            pipeline,
            regressor,
        }
    }

    pub fn compute(
        &self,
        graph_store: Arc<DefaultGraphStore>,
    ) -> Result<
        crate::collections::HugeDoubleArray,
        crate::projection::eval::pipeline::PredictPipelineExecutorError,
    > {
        let task =
            NodeRegressionPredictPipelineExecutor::progress_task(&self.label, graph_store.as_ref());
        let tracker = TaskProgressTracker::with_registry(
            task,
            Concurrency::of(self.configuration.concurrency()),
            JobId::new(),
            &crate::core::utils::progress::EmptyTaskRegistryFactory,
        );

        let mut executor = NodeRegressionPredictPipelineExecutor::new(
            &self.pipeline,
            &self.configuration,
            graph_store,
            tracker,
            Arc::clone(&self.regressor),
            TerminationFlag::running_true(),
        );

        executor.compute()
    }
}
