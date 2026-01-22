use super::node_classification_model_result::NodeClassificationModelResult;
use super::node_classification_pipeline_train_config::NodeClassificationPipelineTrainConfig;
use super::node_classification_to_model_converter::NodeClassificationToModelConverter;
use super::node_classification_train_result::NodeClassificationTrainResult;
use super::node_classification_training_pipeline::NodeClassificationTrainingPipeline;
use crate::core::utils::progress::tasks::progress_tracker::ProgressTracker;
use crate::projection::eval::pipeline::node_pipeline::node_property_pipeline_base_train_config::NodePropertyPipelineBaseTrainConfig;
use crate::projection::eval::pipeline::pipeline_train_algorithm::{
    PipelineTrainAlgorithm, PipelineTrainAlgorithmError,
};
use crate::projection::eval::pipeline::PipelineTrainer;
use crate::projection::eval::pipeline::ResultToModelConverter;
use crate::types::graph_store::DefaultGraphStore;
use std::sync::Arc;

/// Train algorithm for node classification pipelines.
///
/// This is an adapter that wires together the pipeline trainer, training pipeline,
/// model converter, graph store, and configuration.
pub struct NodeClassificationTrainAlgorithm {
    pipeline_trainer: Box<dyn PipelineTrainer<Result = NodeClassificationTrainResult>>,
    pipeline: NodeClassificationTrainingPipeline,
    converter: NodeClassificationToModelConverter,
    graph_store: Arc<DefaultGraphStore>,
    config: NodeClassificationPipelineTrainConfig,
    progress_tracker: Box<dyn ProgressTracker>,
    node_labels: Vec<String>,
    relationship_types: Vec<String>,
}

impl NodeClassificationTrainAlgorithm {
    pub fn new(
        pipeline_trainer: Box<dyn PipelineTrainer<Result = NodeClassificationTrainResult>>,
        pipeline: NodeClassificationTrainingPipeline,
        graph_store: Arc<DefaultGraphStore>,
        config: NodeClassificationPipelineTrainConfig,
        progress_tracker: Box<dyn ProgressTracker>,
    ) -> Self {
        let converter = NodeClassificationToModelConverter::new(pipeline.clone(), config.clone());
        let node_labels = config.node_labels();
        let relationship_types = Vec::new();

        Self {
            pipeline_trainer,
            pipeline,
            converter,
            graph_store,
            config,
            node_labels,
            relationship_types,
            progress_tracker,
        }
    }

    pub fn pipeline_trainer(&self) -> &dyn PipelineTrainer<Result = NodeClassificationTrainResult> {
        self.pipeline_trainer.as_ref()
    }

    pub fn pipeline(&self) -> &NodeClassificationTrainingPipeline {
        &self.pipeline
    }

    pub fn converter(&self) -> &NodeClassificationToModelConverter {
        &self.converter
    }

    pub fn graph_store(&self) -> Arc<DefaultGraphStore> {
        Arc::clone(&self.graph_store)
    }

    pub fn config(&self) -> &NodeClassificationPipelineTrainConfig {
        &self.config
    }

    pub fn progress_tracker(&self) -> &dyn ProgressTracker {
        self.progress_tracker.as_ref()
    }

    pub fn compute(
        &mut self,
    ) -> Result<NodeClassificationModelResult, PipelineTrainAlgorithmError> {
        PipelineTrainAlgorithm::compute(self)
    }
}

impl
    PipelineTrainAlgorithm<
        NodeClassificationTrainResult,
        NodeClassificationModelResult,
        NodeClassificationTrainingPipeline,
    > for NodeClassificationTrainAlgorithm
{
    fn pipeline(&self) -> &NodeClassificationTrainingPipeline {
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
    ) -> &mut dyn PipelineTrainer<Result = NodeClassificationTrainResult> {
        &mut *self.pipeline_trainer
    }

    fn result_to_model_converter(
        &self,
    ) -> &dyn ResultToModelConverter<NodeClassificationModelResult, NodeClassificationTrainResult>
    {
        &self.converter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::tasks::progress_tracker::NoopProgressTracker;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;
    use std::sync::Arc;

    struct MockTrainer;

    impl PipelineTrainer for MockTrainer {
        type Result = NodeClassificationTrainResult;

        fn run(&mut self) -> Result<Self::Result, Box<dyn std::error::Error + Send + Sync>> {
            Err("not implemented".into())
        }
    }

    #[test]
    fn test_new_train_algorithm() {
        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&config).expect("Failed to generate random graph"));
        let pipeline_trainer = Box::new(MockTrainer);
        let pipeline = NodeClassificationTrainingPipeline::new();
        let train_config = NodeClassificationPipelineTrainConfig::default();
        let progress_tracker = Box::new(NoopProgressTracker);

        let algorithm = NodeClassificationTrainAlgorithm::new(
            pipeline_trainer,
            pipeline,
            graph_store.clone(),
            train_config,
            progress_tracker,
        );

        // Verify accessors work
        let _trainer = algorithm.pipeline_trainer();
        let _pipeline = algorithm.pipeline();
        let _converter = algorithm.converter();
        let _store = algorithm.graph_store();
        let _config = algorithm.config();
        let _tracker = algorithm.progress_tracker();
    }
}
