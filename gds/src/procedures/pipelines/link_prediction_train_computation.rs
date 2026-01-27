use std::sync::Arc;

use crate::applications::algorithms::machinery::AlgorithmMachinery;
use crate::concurrency::Concurrency;
use crate::core::utils::progress::tasks::NoopProgressTracker;
use crate::projection::eval::pipeline::link_pipeline::{
    LinkPredictionTrainConfig, LinkPredictionTrainPipelineResult,
};
use crate::projection::eval::pipeline::validate_main_metric;
use crate::projection::eval::pipeline::PipelineTrainAlgorithmError;
use crate::projection::eval::pipeline::TrainingPipeline;
use crate::types::graph_store::DefaultGraphStore;
use crate::types::user::User;

use super::{PipelineName, PipelineRepository};

#[derive(Debug)]
struct LinkPredictionTrainError {
    message: String,
}

impl LinkPredictionTrainError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for LinkPredictionTrainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LinkPredictionTrainError {}

pub struct LinkPredictionTrainComputation {
    pipeline_repository: PipelineRepository,
    configuration: LinkPredictionTrainConfig,
    user: User,
}

impl LinkPredictionTrainComputation {
    pub fn new(
        pipeline_repository: PipelineRepository,
        configuration: LinkPredictionTrainConfig,
        user: User,
    ) -> Self {
        Self {
            pipeline_repository,
            configuration,
            user,
        }
    }

    pub fn compute(
        &self,
        graph_store: Arc<DefaultGraphStore>,
    ) -> Result<LinkPredictionTrainPipelineResult, PipelineTrainAlgorithmError> {
        let pipeline_name = PipelineName::parse(self.configuration.pipeline())
            .map_err(|e| PipelineTrainAlgorithmError::ConversionFailed(e.to_string()))?;

        let pipeline = self
            .pipeline_repository
            .get_link_prediction_training_pipeline(&self.user, &pipeline_name);

        self.configuration.validate().map_err(|e| {
            PipelineTrainAlgorithmError::ValidationFailed(Box::new(LinkPredictionTrainError::new(
                e,
            )))
        })?;

        let training_methods: Vec<String> = pipeline
            .training_parameter_space()
            .keys()
            .map(|method| method.to_string())
            .collect();

        if let Some(main_metric) = self.configuration.metrics().first() {
            validate_main_metric(main_metric, &training_methods)
                .map_err(|e| PipelineTrainAlgorithmError::ValidationFailed(Box::new(e)))?;
        }

        pipeline.validate_before_execution().map_err(|e| {
            PipelineTrainAlgorithmError::ValidationFailed(Box::new(LinkPredictionTrainError::new(
                e,
            )))
        })?;

        let _ = graph_store;

        let mut progress_tracker = NoopProgressTracker;
        AlgorithmMachinery::run_algorithms_and_manage_progress_tracker(
            &mut progress_tracker,
            true,
            Concurrency::available_cores(),
            |_| {
                Err(PipelineTrainAlgorithmError::TrainingFailed(Box::new(
                    LinkPredictionTrainError::new(
                        "LinkPredictionTrainPipelineExecutor is not yet implemented",
                    ),
                )))
            },
        )
    }
}
