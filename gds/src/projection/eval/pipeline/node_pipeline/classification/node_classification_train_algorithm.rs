use super::node_classification_pipeline_train_config::NodeClassificationPipelineTrainConfig;
use super::node_classification_to_model_converter::NodeClassificationToModelConverter;
use super::node_classification_training_pipeline::NodeClassificationTrainingPipeline;
use crate::types::graph_store::DefaultGraphStore;
use std::sync::Arc;

// Placeholder types until pipeline infrastructure is translated
pub type PipelineTrainer = ();
pub type ProgressTracker = ();

/// Train algorithm for node classification pipelines.
///
/// This is an adapter that wires together the pipeline trainer, training pipeline,
/// model converter, graph store, and configuration.
pub struct NodeClassificationTrainAlgorithm {
    pipeline_trainer: PipelineTrainer,
    pipeline: NodeClassificationTrainingPipeline,
    converter: NodeClassificationToModelConverter,
    graph_store: Arc<DefaultGraphStore>,
    config: NodeClassificationPipelineTrainConfig,
    progress_tracker: ProgressTracker,
}

impl NodeClassificationTrainAlgorithm {
    pub fn new(
        pipeline_trainer: PipelineTrainer,
        pipeline: NodeClassificationTrainingPipeline,
        graph_store: Arc<DefaultGraphStore>,
        config: NodeClassificationPipelineTrainConfig,
        progress_tracker: ProgressTracker,
    ) -> Self {
        let converter = NodeClassificationToModelConverter::new(pipeline.clone(), config.clone());

        Self {
            pipeline_trainer,
            pipeline,
            converter,
            graph_store,
            config,
            progress_tracker,
        }
    }

    pub fn pipeline_trainer(&self) -> &PipelineTrainer {
        &self.pipeline_trainer
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

    pub fn progress_tracker(&self) -> &ProgressTracker {
        &self.progress_tracker
    }

    // Note: Add compute()/execution wiring once PipelineTrainAlgorithm is translated.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;
    use std::sync::Arc;

    #[test]
    fn test_new_train_algorithm() {
        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&config).expect("Failed to generate random graph"));
        let pipeline_trainer = ();
        let pipeline = NodeClassificationTrainingPipeline::new();
        let train_config = NodeClassificationPipelineTrainConfig::default();
        let progress_tracker = ();

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
