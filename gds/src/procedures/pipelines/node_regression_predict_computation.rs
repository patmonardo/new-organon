use std::sync::Arc;

use crate::collections::HugeDoubleArray;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{EmptyTaskRegistryFactory, JobId, TaskProgressTracker};
use crate::ml::models::Regressor;
use crate::projection::eval::pipeline::node_pipeline::NodePropertyPredictPipeline;
use crate::projection::eval::pipeline::{PredictPipelineExecutor, PredictPipelineExecutorError};
use crate::types::graph_store::DefaultGraphStore;

use super::{
    NodeRegressionPredictPipelineBaseConfig, NodeRegressionPredictPipelineConfig,
    NodeRegressionPredictPipelineExecutor,
};

pub struct NodeRegressionPredictComputation {
    configuration: NodeRegressionPredictPipelineBaseConfig,
    label: String,
    pipeline: NodePropertyPredictPipeline,
    regressor: Arc<dyn Regressor>,
}

impl NodeRegressionPredictComputation {
    pub fn new(
        configuration: NodeRegressionPredictPipelineBaseConfig,
        label: String,
        pipeline: NodePropertyPredictPipeline,
        regressor: Arc<dyn Regressor>,
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
    ) -> Result<HugeDoubleArray, PredictPipelineExecutorError> {
        let task =
            NodeRegressionPredictPipelineExecutor::progress_task(&self.label, graph_store.as_ref());
        let tracker = TaskProgressTracker::with_registry(
            task,
            Concurrency::of(self.configuration.concurrency()),
            JobId::new(),
            &EmptyTaskRegistryFactory,
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
