use crate::core::utils::progress::tasks::progress_tracker::ProgressTracker;
use crate::projection::eval::pipeline::node_pipeline::node_property_pipeline_base_train_config::NodePropertyPipelineBaseTrainConfig;
use crate::projection::eval::pipeline::pipeline_train_algorithm::{
    PipelineTrainAlgorithm, PipelineTrainAlgorithmError,
};
use crate::projection::eval::pipeline::PipelineTrainer;
use crate::projection::eval::pipeline::ResultToModelConverter;
use crate::types::graph_store::DefaultGraphStore;
use std::sync::Arc;

use super::{
    NodeRegressionPipelineTrainConfig, NodeRegressionToModelConverter,
    NodeRegressionTrainPipelineResult, NodeRegressionTrainResult, NodeRegressionTrainingPipeline,
};

/// Algorithm wrapper for node regression pipeline training.
///
/// Connects the node regression training pipeline to the GDS algorithm execution framework.
/// Delegates actual training to `NodeRegressionTrain` via `PipelineTrainer`.
///
/// Java source: `NodeRegressionTrainAlgorithm.java`
///
/// # Generic Parameters (from Java)
/// ```java
/// class NodeRegressionTrainAlgorithm extends PipelineTrainAlgorithm<
///     NodeRegressionTrainResult,           // Training result type
///     NodeRegressionTrainPipelineResult,   // Catalog model result type
///     NodeRegressionPipelineTrainConfig,   // Configuration type
///     NodeFeatureStep                      // Feature step type
/// >
/// ```
///
/// # Design Pattern
/// This is a thin wrapper that:
/// 1. Takes a `PipelineTrainer` (NodeRegressionTrain) that produces `TrainResult`
/// 2. Takes a `ModelConverter` that converts `TrainResult → CatalogModel`
/// 3. Extends `PipelineTrainAlgorithm` to integrate with Algorithm framework
///
/// The base class handles the train → convert → catalog flow.
pub struct NodeRegressionTrainAlgorithm {
    pipeline_trainer: Box<dyn PipelineTrainer<Result = NodeRegressionTrainResult>>,
    pipeline: NodeRegressionTrainingPipeline,
    model_converter: NodeRegressionToModelConverter,
    graph_store: Arc<DefaultGraphStore>,
    config: NodeRegressionPipelineTrainConfig,
    progress_tracker: Box<dyn ProgressTracker>,
    node_labels: Vec<String>,
    relationship_types: Vec<String>,
}

impl NodeRegressionTrainAlgorithm {
    /// Creates a new node regression training algorithm.
    ///
    /// Java source: Constructor
    /// ```java
    /// public NodeRegressionTrainAlgorithm(
    ///     PipelineTrainer<NodeRegressionTrainResult> pipelineTrainer,
    ///     NodeRegressionTrainingPipeline pipeline,
    ///     GraphStore graphStore,
    ///     NodeRegressionPipelineTrainConfig config,
    ///     ProgressTracker progressTracker
    /// )
    /// ```
    pub fn new(
        pipeline_trainer: Box<dyn PipelineTrainer<Result = NodeRegressionTrainResult>>,
        pipeline: NodeRegressionTrainingPipeline,
        graph_store: Arc<DefaultGraphStore>,
        config: NodeRegressionPipelineTrainConfig,
        progress_tracker: Box<dyn ProgressTracker>,
    ) -> Self {
        let model_converter = NodeRegressionToModelConverter::new(pipeline.clone(), config.clone());
        let node_labels = config.node_labels();
        let relationship_types = Vec::new();

        Self {
            pipeline_trainer,
            pipeline,
            model_converter,
            graph_store,
            config,
            progress_tracker,
            node_labels,
            relationship_types,
        }
    }

    /// Returns the pipeline being trained.
    pub fn pipeline(&self) -> &NodeRegressionTrainingPipeline {
        &self.pipeline
    }

    /// Returns the training configuration.
    pub fn config(&self) -> &NodeRegressionPipelineTrainConfig {
        &self.config
    }

    /// Returns the graph store.
    pub fn graph_store(&self) -> &Arc<DefaultGraphStore> {
        &self.graph_store
    }

    /// Returns the model converter.
    pub fn model_converter(&self) -> &NodeRegressionToModelConverter {
        &self.model_converter
    }

    pub fn progress_tracker(&self) -> &dyn ProgressTracker {
        self.progress_tracker.as_ref()
    }

    pub fn compute(
        &mut self,
    ) -> Result<NodeRegressionTrainPipelineResult, PipelineTrainAlgorithmError> {
        PipelineTrainAlgorithm::compute(self)
    }
}

impl
    PipelineTrainAlgorithm<
        NodeRegressionTrainResult,
        NodeRegressionTrainPipelineResult,
        NodeRegressionTrainingPipeline,
    > for NodeRegressionTrainAlgorithm
{
    fn pipeline(&self) -> &NodeRegressionTrainingPipeline {
        &self.pipeline
    }

    fn graph_store(&self) -> &Arc<DefaultGraphStore> {
        &self.graph_store
    }

    fn node_labels(&self) -> &[String] {
        &self.node_labels
    }

    fn relationship_types(&self) -> &[String] {
        &self.relationship_types
    }

    fn pipeline_trainer_mut(
        &mut self,
    ) -> &mut dyn PipelineTrainer<Result = NodeRegressionTrainResult> {
        &mut *self.pipeline_trainer
    }

    fn result_to_model_converter(
        &self,
    ) -> &dyn ResultToModelConverter<NodeRegressionTrainPipelineResult, NodeRegressionTrainResult>
    {
        &self.model_converter
    }
}

// Note: implementing the Algorithm trait is deferred until the broader
// algorithm/task framework is wired into this crate.
// impl Algorithm for NodeRegressionTrainAlgorithm {
//     type Result = NodeRegressionTrainPipelineResult;
//
//     fn run(&mut self) -> Self::Result {
//         let train_result = self.pipeline_trainer.run();
//         self.model_converter.to_model(train_result, self.graph_store.schema())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_new() {
        use crate::core::utils::progress::tasks::progress_tracker::NoopProgressTracker;
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::random_graph::RandomGraphConfig;

        struct MockTrainer;

        impl PipelineTrainer for MockTrainer {
            type Result = NodeRegressionTrainResult;

            fn run(&mut self) -> Result<Self::Result, Box<dyn std::error::Error + Send + Sync>> {
                Err("not implemented".into())
            }
        }

        let pipeline = NodeRegressionTrainingPipeline::new();
        let config = NodeRegressionPipelineTrainConfig::default();
        let random_config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&random_config).expect("random graph"));

        let _algorithm = NodeRegressionTrainAlgorithm::new(
            Box::new(MockTrainer),
            pipeline,
            graph_store,
            config,
            Box::new(NoopProgressTracker),
        );
    }

    #[test]
    fn test_algorithm_accessors() {
        use crate::core::utils::progress::tasks::progress_tracker::NoopProgressTracker;
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::random_graph::RandomGraphConfig;

        struct MockTrainer;

        impl PipelineTrainer for MockTrainer {
            type Result = NodeRegressionTrainResult;

            fn run(&mut self) -> Result<Self::Result, Box<dyn std::error::Error + Send + Sync>> {
                Err("not implemented".into())
            }
        }

        let pipeline = NodeRegressionTrainingPipeline::new();
        let config = NodeRegressionPipelineTrainConfig::default();
        let random_config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&random_config).expect("random graph"));

        let algorithm = NodeRegressionTrainAlgorithm::new(
            Box::new(MockTrainer),
            pipeline.clone(),
            graph_store.clone(),
            config,
            Box::new(NoopProgressTracker),
        );

        assert_eq!(
            algorithm.pipeline().pipeline_type(),
            pipeline.pipeline_type()
        );
        // Config and graph_store accessors work
        let _ = algorithm.config();
        let _ = algorithm.graph_store();
        let _ = algorithm.model_converter();
    }
}
