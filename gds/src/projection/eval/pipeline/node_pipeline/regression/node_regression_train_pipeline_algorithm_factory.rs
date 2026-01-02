use crate::types::graph_store::DefaultGraphStore;
use crate::projection::eval::pipeline::Pipeline;
use std::sync::Arc;

use super::{
    NodeRegressionPipelineTrainConfig, NodeRegressionTrainAlgorithm, NodeRegressionTrainingPipeline,
};

use crate::projection::eval::pipeline::node_pipeline::NodeFeatureProducer;

// Placeholder types until full framework is available
pub type ExecutionContext = ();
pub type ProgressTracker = ();
pub type Task = ();

/// Factory for creating node regression training algorithm instances.
///
/// Handles:
/// - Pipeline retrieval from PipelineCatalog
/// - Feature producer creation
/// - Validation of node property steps
/// - Progress task construction
///
/// Java source: `NodeRegressionTrainPipelineAlgorithmFactory.java`
#[derive(Debug, Clone)]
pub struct NodeRegressionTrainPipelineAlgorithmFactory {
    _execution_context: ExecutionContext,
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
    pub fn new(execution_context: ExecutionContext) -> Self {
        Self {
            _execution_context: execution_context,
        }
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
        progress_tracker: ProgressTracker,
    ) -> NodeRegressionTrainAlgorithm {
        // Direct integration: catalog lookup isn't wired yet, so this overload uses a
        // default in-memory pipeline instance.
        let pipeline = NodeRegressionTrainingPipeline::new();
        self.build_with_pipeline(graph_store, configuration, pipeline, progress_tracker)
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
        progress_tracker: ProgressTracker,
    ) -> NodeRegressionTrainAlgorithm {
        Self::validate_main_metric(&pipeline, configuration.metrics().first());

        // Execute node-property-step context validation early (mirrors Java behavior).
        // Training itself is not wired in this module yet.
        let node_feature_producer = NodeFeatureProducer::create(graph_store.clone(), configuration.clone());
        node_feature_producer
            .validate_node_property_steps_context_configs(pipeline.node_property_steps())
            .expect("node property step context config validation failed");

        NodeRegressionTrainAlgorithm::new(
            std::marker::PhantomData, // pipeline_trainer (placeholder)
            pipeline,
            graph_store,
            configuration,
            progress_tracker,
        )
    }

    /// Returns the task name for this algorithm.
    ///
    /// Java source: `taskName()`
    pub fn task_name(&self) -> &str {
        "Node Regression Train Pipeline"
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

    #[test]
    fn test_factory_new() {
        let _factory = NodeRegressionTrainPipelineAlgorithmFactory::new(());
    }

    #[test]
    fn test_task_name() {
        let factory = NodeRegressionTrainPipelineAlgorithmFactory::new(());
        assert_eq!(factory.task_name(), "Node Regression Train Pipeline");
    }

    #[test]
    fn test_build_with_pipeline() {
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::random_graph::RandomGraphConfig;

        let factory = NodeRegressionTrainPipelineAlgorithmFactory::new(());
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
            (), // progress_tracker
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
