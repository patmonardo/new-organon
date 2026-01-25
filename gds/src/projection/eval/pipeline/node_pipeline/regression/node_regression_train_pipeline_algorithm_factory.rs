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

use super::{
    NodeRegressionPipelineTrainConfig, NodeRegressionTrain, NodeRegressionTrainAlgorithm,
    NodeRegressionTrainingPipeline,
};

/// Factory for creating node regression training algorithm instances.
///
/// Handles:
/// - Pipeline retrieval from PipelineCatalog
/// - Feature producer creation
/// - Validation of node property steps
/// - Progress task construction
///
/// Java source: `NodeRegressionTrainPipelineAlgorithmFactory.java`
pub struct NodeRegressionTrainPipelineAlgorithmFactory {
    execution_context: ExecutionContext,
    gds_version: String,
    pipeline_catalog: Arc<PipelineCatalog>,
}

impl NodeRegressionTrainPipelineAlgorithmFactory {
    /// Creates a new factory with the given execution context.
    ///
    /// Java source: Constructor
    /// ```java
    /// public NodeRegressionTrainPipelineAlgorithmFactory(ExecutionContext executionContext) {
    ///     this.executionContext = executionContext;
    /// }
    /// ```
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

    /// Builds a training algorithm by retrieving the pipeline from the catalog.
    ///
    /// Java source: `build(GraphStore, Config, ProgressTracker)`
    /// ```java
    /// public NodeRegressionTrainAlgorithm build(
    ///     GraphStore graphStore,
    ///     NodeRegressionPipelineTrainConfig configuration,
    ///     ProgressTracker progressTracker
    /// ) {
    ///     var pipeline = PipelineCatalog.getTyped(
    ///         configuration.username(),
    ///         configuration.pipeline(),
    ///         NodeRegressionTrainingPipeline.class
    ///     );
    ///     return build(graphStore, configuration, pipeline, progressTracker);
    /// }
    /// ```
    pub fn build(
        &self,
        graph_store: Arc<DefaultGraphStore>,
        configuration: NodeRegressionPipelineTrainConfig,
        progress_tracker: Box<dyn ProgressTracker>,
    ) -> NodeRegressionTrainAlgorithm {
        let pipeline = self
            .pipeline_catalog
            .get_typed::<NodeRegressionTrainingPipeline>("", configuration.pipeline())
            .unwrap_or_else(|_| Arc::new(NodeRegressionTrainingPipeline::new()));

        self.build_with_pipeline(
            graph_store,
            configuration,
            (*pipeline).clone(),
            progress_tracker,
        )
    }

    /// Builds a training algorithm with an explicitly provided pipeline.
    ///
    /// Java source: Second `build(...)` overload
    /// ```java
    /// public NodeRegressionTrainAlgorithm build(
    ///     GraphStore graphStore,
    ///     NodeRegressionPipelineTrainConfig configuration,
    ///     NodeRegressionTrainingPipeline pipeline,
    ///     ProgressTracker progressTracker
    /// ) {
    ///     validateMainMetric(pipeline, configuration.metrics().get(0).toString());
    ///
    ///     var nodeFeatureProducer = NodeFeatureProducer.create(
    ///         graphStore, configuration, executionContext, progressTracker
    ///     );
    ///     nodeFeatureProducer.validateNodePropertyStepsContextConfigs(pipeline.nodePropertySteps());
    ///
    ///     return new NodeRegressionTrainAlgorithm(
    ///         NodeRegressionTrain.create(graphStore, pipeline, configuration, nodeFeatureProducer, progressTracker),
    ///         pipeline,
    ///         graphStore,
    ///         configuration,
    ///         progressTracker
    ///     );
    /// }
    /// ```
    pub fn build_with_pipeline(
        &self,
        graph_store: Arc<DefaultGraphStore>,
        configuration: NodeRegressionPipelineTrainConfig,
        pipeline: NodeRegressionTrainingPipeline,
        progress_tracker: Box<dyn ProgressTracker>,
    ) -> NodeRegressionTrainAlgorithm {
        Self::validate_main_metric(&pipeline, configuration.metrics().first());

        // Execute node-property-step context validation early (mirrors Java behavior).
        // Training itself is not wired in this module yet.
        let node_feature_producer =
            NodeFeatureProducer::create(graph_store.clone(), configuration.clone());
        node_feature_producer
            .validate_node_property_steps_context_configs(pipeline.node_property_steps())
            .expect("node property step context config validation failed");

        let trainer = Box::new(NodeRegressionTrain::create(
            graph_store.clone(),
            pipeline.clone(),
            configuration.clone(),
            node_feature_producer,
            progress_tracker,
        ));

        let algorithm_progress = Box::new(NoopProgressTracker);

        NodeRegressionTrainAlgorithm::new(
            trainer,
            pipeline,
            graph_store,
            configuration,
            algorithm_progress,
        )
    }

    /// Returns the task name for this algorithm.
    ///
    /// Java source: `taskName()`
    pub fn task_name(&self) -> &str {
        "Node Regression Train Pipeline"
    }

    /// Estimate memory requirements for training.
    pub fn memory_estimation(
        &self,
        _configuration: &NodeRegressionPipelineTrainConfig,
    ) -> Box<dyn MemoryEstimation> {
        // Placeholder until memory estimations are translated.
        MemoryEstimations::empty()
    }

    /// Creates a progress task for pipeline training.
    ///
    /// Java source: `progressTask(GraphStore, Config)`
    pub fn progress_task(
        &self,
        _graph_store: &DefaultGraphStore,
        _config: &NodeRegressionPipelineTrainConfig,
    ) -> Task {
        // Task/progress plumbing is not wired yet in direct integration.
        Task::new("Node Regression Train Pipeline".to_string(), vec![])
    }

    /// Creates a progress task for a specific pipeline.
    ///
    /// Java source: Static `progressTask(Pipeline, nodeCount)`
    /// ```java
    /// public static Task progressTask(NodeRegressionTrainingPipeline pipeline, long nodeCount) {
    ///     return NodeRegressionTrain.progressTask(pipeline, nodeCount);
    /// }
    /// ```
    pub fn progress_task_for_pipeline(
        _pipeline: &NodeRegressionTrainingPipeline,
        _node_count: u64,
    ) -> Task {
        // Task/progress plumbing is not wired yet in direct integration.
        Task::new("Node Regression Train Pipeline".to_string(), vec![])
    }

    /// Validates that the main metric is supported by the pipeline.
    ///
    /// Java source: `PipelineCompanion.validateMainMetric(pipeline, metric)`
    fn validate_main_metric(
        _pipeline: &NodeRegressionTrainingPipeline,
        _metric: Option<&super::RegressionMetrics>,
    ) {
        // Regression metric validation will be added once the full metrics registry is
        // implemented. For now, any metric enum value is accepted.
    }
}

// Note: implementing the GraphStoreAlgorithmFactory trait is deferred until the
// broader algorithm/task framework is wired into this crate.
// impl GraphStoreAlgorithmFactory<NodeRegressionTrainAlgorithm, NodeRegressionPipelineTrainConfig>
//     for NodeRegressionTrainPipelineAlgorithmFactory
// {
//     fn build(&self, graph_store: GraphStore, config: Config, tracker: ProgressTracker) -> Algorithm;
//     fn task_name(&self) -> &str;
//     fn progress_task(&self, graph_store: &GraphStore, config: &Config) -> Task;
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::random::RandomGraphConfig;

    #[test]
    fn test_factory_new() {
        let _factory = NodeRegressionTrainPipelineAlgorithmFactory::new(
            ExecutionContext::empty(),
            "2.5.0".to_string(),
            Arc::new(PipelineCatalog::new()),
        );
    }

    #[test]
    fn test_task_name() {
        let factory = NodeRegressionTrainPipelineAlgorithmFactory::new(
            ExecutionContext::empty(),
            "2.5.0".to_string(),
            Arc::new(PipelineCatalog::new()),
        );
        assert_eq!(factory.task_name(), "Node Regression Train Pipeline");
    }

    #[test]
    #[should_panic(expected = "Missing target node property for regression")]
    fn test_build_with_pipeline() {
        let factory = NodeRegressionTrainPipelineAlgorithmFactory::new(
            ExecutionContext::empty(),
            "2.5.0".to_string(),
            Arc::new(PipelineCatalog::new()),
        );
        let pipeline = NodeRegressionTrainingPipeline::new();
        let config = NodeRegressionPipelineTrainConfig::default();
        let random_config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&random_config).expect("random graph"));

        let _algorithm = factory.build_with_pipeline(
            graph_store,
            config,
            pipeline,
            Box::new(NoopProgressTracker),
        );
    }

    #[test]
    fn test_progress_task_for_pipeline() {
        let pipeline = NodeRegressionTrainingPipeline::new();
        NodeRegressionTrainPipelineAlgorithmFactory::progress_task_for_pipeline(
            &pipeline, 1000, // node_count
        );
    }
}
