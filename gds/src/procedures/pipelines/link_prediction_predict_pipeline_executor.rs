use std::sync::Arc;

use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{Task, TaskProgressTracker, Tasks};
use crate::ml::link_models::LinkPredictionResult;
use crate::ml::models::Classifier;
use crate::projection::eval::pipeline::link_pipeline::LinkPredictionPredictPipeline;
use crate::projection::eval::pipeline::PredictPipelineExecutorError;
use crate::types::graph_store::{DefaultGraphStore, GraphStore};

use super::LinkPredictionPredictPipelineConfig;

pub struct LinkPredictionPredictPipelineExecutor<'a> {
    #[allow(dead_code)]
    pipeline: &'a LinkPredictionPredictPipeline,
    #[allow(dead_code)]
    configuration: &'a dyn LinkPredictionPredictPipelineConfig,
    #[allow(dead_code)]
    graph_store: Arc<DefaultGraphStore>,
    #[allow(dead_code)]
    progress_tracker: TaskProgressTracker,
    #[allow(dead_code)]
    classifier: Arc<dyn Classifier>,
    #[allow(dead_code)]
    termination_flag: TerminationFlag,
}

impl<'a> LinkPredictionPredictPipelineExecutor<'a> {
    pub fn new(
        pipeline: &'a LinkPredictionPredictPipeline,
        configuration: &'a dyn LinkPredictionPredictPipelineConfig,
        graph_store: Arc<DefaultGraphStore>,
        progress_tracker: TaskProgressTracker,
        classifier: Arc<dyn Classifier>,
        termination_flag: TerminationFlag,
    ) -> Self {
        Self {
            pipeline,
            configuration,
            graph_store,
            progress_tracker,
            classifier,
            termination_flag,
        }
    }

    pub fn progress_task(task_name: &str, graph_store: &DefaultGraphStore) -> Task {
        let volume = graph_store.node_count().saturating_mul(10);
        Tasks::leaf_with_volume(format!("{task_name} progress"), volume)
            .base()
            .clone()
    }

    pub fn compute(
        &mut self,
    ) -> Result<Box<dyn LinkPredictionResult>, PredictPipelineExecutorError> {
        let _ = Concurrency::available_cores();
        Err(PredictPipelineExecutorError::ExecutionFailed(
            "LinkPredictionPredictPipelineExecutor not yet implemented".to_string(),
        ))
    }
}
