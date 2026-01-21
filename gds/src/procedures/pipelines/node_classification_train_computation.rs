use std::sync::Arc;

use crate::applications::algorithms::machinery::AlgorithmMachinery;
use crate::concurrency::Concurrency;
use crate::core::utils::progress::tasks::progress_tracker::NoopProgressTracker;
use crate::projection::eval::pipeline::pipeline_companion::validate_main_metric;
use crate::projection::eval::pipeline::node_pipeline::NodeFeatureProducer;
use crate::projection::eval::pipeline::node_pipeline::node_property_pipeline_base_train_config::NodePropertyPipelineBaseTrainConfig;
use crate::projection::eval::pipeline::node_pipeline::classification::node_classification_pipeline_train_config::NodeClassificationPipelineTrainConfig;
use crate::projection::eval::pipeline::node_pipeline::classification::node_classification_train::NodeClassificationTrain;
use crate::projection::eval::pipeline::node_pipeline::classification::node_classification_train_algorithm::NodeClassificationTrainAlgorithm;
use crate::projection::eval::pipeline::node_pipeline::classification::node_classification_model_result::NodeClassificationModelResult;
use crate::projection::eval::pipeline::pipeline_train_algorithm::PipelineTrainAlgorithmError;
use crate::projection::eval::pipeline::{Pipeline, TrainingPipeline};
use crate::types::graph_store::DefaultGraphStore;
use crate::types::user::User;

use super::{PipelineName, PipelineRepository};

pub struct NodeClassificationTrainComputation {
    pipeline_repository: PipelineRepository,
    configuration: NodeClassificationPipelineTrainConfig,
    user: User,
}

impl NodeClassificationTrainComputation {
    pub fn new(
        pipeline_repository: PipelineRepository,
        configuration: NodeClassificationPipelineTrainConfig,
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
    ) -> Result<NodeClassificationModelResult, PipelineTrainAlgorithmError> {
        let pipeline_name = PipelineName::parse(self.configuration.pipeline())
            .map_err(|e| PipelineTrainAlgorithmError::ConversionFailed(e.to_string()))?;

        let pipeline = self
            .pipeline_repository
            .get_node_classification_training_pipeline(&self.user, &pipeline_name);

        let training_methods: Vec<String> = pipeline
            .training_parameter_space()
            .keys()
            .map(|method| method.to_string())
            .collect();

        if let Some(main_metric) = self
            .configuration
            .metrics_specs()
            .first()
            .map(|spec| spec.to_string())
        {
            validate_main_metric(&main_metric, &training_methods)
                .map_err(|e| PipelineTrainAlgorithmError::ConversionFailed(e.to_string()))?;
        }

        let node_feature_producer =
            NodeFeatureProducer::create(Arc::clone(&graph_store), self.configuration.clone());

        node_feature_producer
            .validate_node_property_steps_context_configs(pipeline.node_property_steps())
            .map_err(|e| PipelineTrainAlgorithmError::ValidationFailed(Box::new(e)))?;

        let pipeline_trainer = NodeClassificationTrain::create(
            Arc::clone(&graph_store),
            (*pipeline).clone(),
            self.configuration.clone(),
            node_feature_producer,
            Box::new(NoopProgressTracker),
        );

        let mut algorithm = NodeClassificationTrainAlgorithm::new(
            Box::new(pipeline_trainer),
            (*pipeline).clone(),
            Arc::clone(&graph_store),
            self.configuration.clone(),
            Box::new(NoopProgressTracker),
        );

        let mut progress_tracker = NoopProgressTracker;
        AlgorithmMachinery::run_algorithms_and_manage_progress_tracker(
            &mut progress_tracker,
            true,
            Concurrency::available_cores(),
            |_| algorithm.compute(),
        )
    }
}
