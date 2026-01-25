use super::node_classification_pipeline_train_config::NodeClassificationPipelineTrainConfig;
use super::node_classification_train::NodeClassificationTrain;
use super::node_classification_train_algorithm::NodeClassificationTrainAlgorithm;
use super::node_classification_training_pipeline::NodeClassificationTrainingPipeline;
use crate::core::utils::progress::tasks::progress_tracker::NoopProgressTracker;
use crate::core::utils::progress::tasks::progress_tracker::ProgressTracker;
use crate::core::utils::progress::tasks::Task;
use crate::mem::MemoryEstimation;
use crate::mem::MemoryEstimations;
use crate::projection::eval::pipeline::node_pipeline::node_property_pipeline_base_train_config::NodePropertyPipelineBaseTrainConfig;
use crate::projection::eval::pipeline::node_pipeline::NodeFeatureProducer;
use crate::projection::eval::pipeline::pipeline_trait::Pipeline;
use crate::projection::eval::pipeline::PipelineCatalog;
use crate::projection::eval::algorithm::ExecutionContext;
use crate::types::graph_store::DefaultGraphStore;
use std::sync::Arc;

/// Factory for creating NodeClassificationTrainAlgorithm instances.
///
/// This factory handles:
/// - Retrieving pipelines from the catalog
/// - Creating NodeFeatureProducer instances
/// - Validating pipeline configuration
/// - Wiring up all dependencies for training
pub struct NodeClassificationTrainPipelineAlgorithmFactory {
    execution_context: ExecutionContext,
    gds_version: String,
    pipeline_catalog: Arc<PipelineCatalog>,
}

impl NodeClassificationTrainPipelineAlgorithmFactory {
    pub fn new(
        execution_context: ExecutionContext,
        gds_version: String,
        pipeline_catalog: Arc<PipelineCatalog>,
    ) -> Self {
        Self {
            execution_context,
            gds_version,
            pipeline_catalog,
        }
    }

    pub fn execution_context(&self) -> &ExecutionContext {
        &self.execution_context
    }

    pub fn gds_version(&self) -> &str {
        &self.gds_version
    }

    /// Build algorithm from catalog pipeline.
    ///
    /// Retrieves the pipeline from the catalog using username and pipeline name from config.
    pub fn build(
        &self,
        graph_store: Arc<DefaultGraphStore>,
        configuration: NodeClassificationPipelineTrainConfig,
        progress_tracker: Box<dyn ProgressTracker>,
    ) -> NodeClassificationTrainAlgorithm {
        let pipeline = self
            .pipeline_catalog
            .get_typed::<NodeClassificationTrainingPipeline>("", configuration.pipeline())
            .unwrap_or_else(|_| Arc::new(NodeClassificationTrainingPipeline::new()));

        self.build_with_pipeline(
            graph_store,
            configuration,
            (*pipeline).clone(),
            progress_tracker,
        )
    }

    /// Build algorithm with explicit pipeline.
    ///
    /// This is useful for testing or when the pipeline is already available.
    pub fn build_with_pipeline(
        &self,
        graph_store: Arc<DefaultGraphStore>,
        configuration: NodeClassificationPipelineTrainConfig,
        pipeline: NodeClassificationTrainingPipeline,
        progress_tracker: Box<dyn ProgressTracker>,
    ) -> NodeClassificationTrainAlgorithm {
        // Note: Pipeline/metric validation will be wired in once the metrics system is translated.
        // validate_main_metric(&pipeline, &configuration.metrics()[0].to_string());

        let node_feature_producer =
            NodeFeatureProducer::create(graph_store.clone(), configuration.clone());
        node_feature_producer
            .validate_node_property_steps_context_configs(pipeline.node_property_steps())
            .expect("Invalid node property step context configs");

        let trainer = Box::new(NodeClassificationTrain::create(
            graph_store.clone(),
            pipeline.clone(),
            configuration.clone(),
            node_feature_producer,
            progress_tracker,
        ));

        let algorithm_progress = Box::new(NoopProgressTracker);

        NodeClassificationTrainAlgorithm::new(
            trainer,
            pipeline,
            graph_store,
            configuration,
            algorithm_progress,
        )
    }

    /// Estimate memory requirements for training.
    pub fn memory_estimation(
        &self,
        _configuration: &NodeClassificationPipelineTrainConfig,
    ) -> Box<dyn MemoryEstimation> {
        // Note: Implement once MemoryEstimations and NodeClassificationTrain are translated.
        // let pipeline = PipelineCatalog::get_typed::<NodeClassificationTrainingPipeline>(
        //     configuration.username(),
        //     configuration.pipeline(),
        // );
        //
        // MemoryEstimations::builder("NodeClassificationTrain")
        //     .add(NodeClassificationTrain::estimate(
        //         &pipeline,
        //         configuration,
        //         &self.execution_context.model_catalog(),
        //         &self.execution_context.algorithms_procedure_facade(),
        //     ))
        //     .build()

        // Placeholder
        MemoryEstimations::empty()
    }

    /// Get task name for progress tracking.
    pub fn task_name(&self) -> &'static str {
        "Node Classification Train Pipeline"
    }

    /// Create progress task for training.
    pub fn progress_task(
        &self,
        _graph_store: &DefaultGraphStore,
        _config: &NodeClassificationPipelineTrainConfig,
    ) -> Task {
        // Note: Implement once Task and NodeClassificationTrain are translated.
        // let pipeline = PipelineCatalog::get_typed::<NodeClassificationTrainingPipeline>(
        //     config.username(),
        //     config.pipeline(),
        // );
        // Self::progress_task_with_pipeline(graph_store, &pipeline)

        // Placeholder
        Task::new("Node Classification Train Pipeline".to_string(), vec![])
    }

    /// Create progress task with explicit pipeline.
    pub fn progress_task_with_pipeline(
        _graph_store: &DefaultGraphStore,
        _pipeline: &NodeClassificationTrainingPipeline,
    ) -> Task {
        // Note: Implement once Task and NodeClassificationTrain are translated.
        // NodeClassificationTrain::progress_task(pipeline, graph_store.node_count())

        // Placeholder
        Task::new("Node Classification Train Pipeline".to_string(), vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;

    #[test]
    fn test_new_factory() {
        let execution_context = ExecutionContext::empty();
        let pipeline_catalog = Arc::new(PipelineCatalog::new());
        let gds_version = "2.5.0".to_string();

        let factory = NodeClassificationTrainPipelineAlgorithmFactory::new(
            execution_context,
            gds_version.clone(),
            pipeline_catalog,
        );

        assert_eq!(factory.gds_version(), "2.5.0");
    }

    #[test]
    fn test_task_name() {
        let factory = NodeClassificationTrainPipelineAlgorithmFactory::new(
            ExecutionContext::empty(),
            "2.5.0".to_string(),
            Arc::new(PipelineCatalog::new()),
        );
        assert_eq!(factory.task_name(), "Node Classification Train Pipeline");
    }

    #[test]
    fn test_build_factory() {
        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let _graph_store =
            DefaultGraphStore::random(&config).expect("Failed to generate random graph");
        let _factory = NodeClassificationTrainPipelineAlgorithmFactory::new(
            ExecutionContext::empty(),
            "2.5.0".to_string(),
            Arc::new(PipelineCatalog::new()),
        );
        let _train_config = NodeClassificationPipelineTrainConfig::default();

        // Placeholder: build method not yet fully implemented
        // let _algorithm = factory.build(&graph_store, &train_config);
    }

    #[test]
    fn test_progress_task() {
        let factory = NodeClassificationTrainPipelineAlgorithmFactory::new(
            ExecutionContext::empty(),
            "2.5.0".to_string(),
            Arc::new(PipelineCatalog::new()),
        );
        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            DefaultGraphStore::random(&config).expect("Failed to generate random graph");
        let train_config = NodeClassificationPipelineTrainConfig::default();

        // Should return placeholder for now
        factory.progress_task(&graph_store, &train_config);
    }

    #[test]
    fn test_progress_task_with_pipeline() {
        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            DefaultGraphStore::random(&config).expect("Failed to generate random graph");
        let pipeline = NodeClassificationTrainingPipeline::new();

        // Should return placeholder for now
        NodeClassificationTrainPipelineAlgorithmFactory::progress_task_with_pipeline(
            &graph_store,
            &pipeline,
        );
    }

    #[test]
    fn test_memory_estimation() {
        let factory = NodeClassificationTrainPipelineAlgorithmFactory::new(
            ExecutionContext::empty(),
            "2.5.0".to_string(),
            Arc::new(PipelineCatalog::new()),
        );
        let config = NodeClassificationPipelineTrainConfig::default();

        // Should return placeholder for now
        let _ = factory.memory_estimation(&config);
    }

    // Note: duplicated progress-task tests removed.
}
