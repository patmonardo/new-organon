use std::sync::Arc;

use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{EmptyTaskRegistryFactory, JobId, TaskProgressTracker};
use crate::ml::link_models::LinkPredictionResult;
use crate::ml::models::Classifier;
use crate::projection::eval::pipeline::link_pipeline::LinkPredictionPredictPipeline;
use crate::projection::eval::pipeline::PredictPipelineExecutorError;
use crate::types::graph_store::DefaultGraphStore;

use super::{
    LinkPredictionPredictPipelineBaseConfig, LinkPredictionPredictPipelineConfig,
    LinkPredictionPredictPipelineExecutor,
};

pub struct LinkPredictionComputation {
    configuration: LinkPredictionPredictPipelineBaseConfig,
    label: String,
    pipeline: LinkPredictionPredictPipeline,
    classifier: Arc<dyn Classifier>,
}

impl LinkPredictionComputation {
    pub fn new(
        configuration: LinkPredictionPredictPipelineBaseConfig,
        label: String,
        pipeline: LinkPredictionPredictPipeline,
        classifier: Arc<dyn Classifier>,
    ) -> Self {
        Self {
            configuration,
            label,
            pipeline,
            classifier,
        }
    }

    pub fn compute(
        &self,
        graph_store: Arc<DefaultGraphStore>,
    ) -> Result<Box<dyn LinkPredictionResult>, PredictPipelineExecutorError> {
        let task =
            LinkPredictionPredictPipelineExecutor::progress_task(&self.label, graph_store.as_ref());
        let tracker = TaskProgressTracker::with_registry(
            task,
            Concurrency::of(self.configuration.concurrency()),
            JobId::new(),
            &EmptyTaskRegistryFactory,
        );

        let mut executor = LinkPredictionPredictPipelineExecutor::new(
            &self.pipeline,
            &self.configuration,
            graph_store,
            tracker,
            Arc::clone(&self.classifier),
            TerminationFlag::running_true(),
        );

        executor.compute()
    }
}
