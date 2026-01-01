use crate::types::graph_store::DefaultGraphStore;
use std::sync::Arc;

use super::{
    NodeRegressionPipelineTrainConfig, NodeRegressionToModelConverter, NodeRegressionTrainResult,
    NodeRegressionTrainingPipeline,
};

// Placeholder types until algorithm framework is complete
pub type PipelineTrainer<T> = std::marker::PhantomData<T>;
pub type PipelineTrainAlgorithm<T, R, C, F> = std::marker::PhantomData<(T, R, C, F)>;
pub type ProgressTracker = ();

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
#[derive(Debug)]
pub struct NodeRegressionTrainAlgorithm {
    pipeline_trainer: PipelineTrainer<NodeRegressionTrainResult>,
    pipeline: NodeRegressionTrainingPipeline,
    model_converter: NodeRegressionToModelConverter,
    graph_store: Arc<DefaultGraphStore>,
    config: NodeRegressionPipelineTrainConfig,
    _progress_tracker: ProgressTracker,
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
        _pipeline_trainer: PipelineTrainer<NodeRegressionTrainResult>,
        pipeline: NodeRegressionTrainingPipeline,
        graph_store: Arc<DefaultGraphStore>,
        config: NodeRegressionPipelineTrainConfig,
        _progress_tracker: ProgressTracker,
    ) -> Self {
        let model_converter = NodeRegressionToModelConverter::new(pipeline.clone(), config.clone());

        Self {
            pipeline_trainer: std::marker::PhantomData,
            pipeline,
            model_converter,
            graph_store,
            config,
            _progress_tracker: (),
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
}

// TODO: Implement Algorithm trait when framework is available
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
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::random_graph::RandomGraphConfig;

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
            std::marker::PhantomData, // pipeline_trainer
            pipeline,
            graph_store,
            config,
            (), // progress_tracker
        );
    }

    #[test]
    fn test_algorithm_accessors() {
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::random_graph::RandomGraphConfig;

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
            std::marker::PhantomData,
            pipeline.clone(),
            graph_store.clone(),
            config,
            (),
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
