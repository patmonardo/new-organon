use std::sync::Arc;

use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{Task, TaskProgressTracker, Tasks};
use crate::ml::core::subgraph::LocalIdMap;
use crate::ml::models::{ClassifierData, ClassifierFactory, FeaturesFactory};
use crate::ml::node_classification::NodeClassificationPredict;
use crate::projection::eval::pipeline::{
    node_pipeline::NodePropertyPredictPipeline, PipelineGraphFilter, PredictPipelineExecutor,
    PredictPipelineExecutorError,
};
use crate::types::graph_store::{DefaultGraphStore, GraphStore};

use super::{
    node_classification_predict_pipeline_constants::MIN_BATCH_SIZE,
    NodeClassificationPipelineResult, NodeClassificationPredictPipelineConfig,
};

pub struct NodeClassificationPredictPipelineExecutor<'a> {
    pipeline: &'a NodePropertyPredictPipeline,
    configuration: &'a dyn NodeClassificationPredictPipelineConfig,
    graph_store: Arc<DefaultGraphStore>,
    progress_tracker: TaskProgressTracker,
    classifier_data: &'a dyn ClassifierData,
    class_id_map: LocalIdMap,
    termination_flag: TerminationFlag,
}

impl<'a> NodeClassificationPredictPipelineExecutor<'a> {
    pub fn new(
        pipeline: &'a NodePropertyPredictPipeline,
        configuration: &'a dyn NodeClassificationPredictPipelineConfig,
        graph_store: Arc<DefaultGraphStore>,
        progress_tracker: TaskProgressTracker,
        classifier_data: &'a dyn ClassifierData,
        class_id_map: LocalIdMap,
        termination_flag: TerminationFlag,
    ) -> Self {
        Self {
            pipeline,
            configuration,
            graph_store,
            progress_tracker,
            classifier_data,
            class_id_map,
            termination_flag,
        }
    }

    pub fn progress_task(task_name: &str, graph_store: &DefaultGraphStore) -> Task {
        Tasks::leaf_with_volume(format!("{task_name} progress"), graph_store.node_count())
            .base()
            .clone()
    }
}

impl<'a> PredictPipelineExecutor<NodePropertyPredictPipeline, NodeClassificationPipelineResult>
    for NodeClassificationPredictPipelineExecutor<'a>
{
    fn pipeline(&self) -> &NodePropertyPredictPipeline {
        self.pipeline
    }

    fn pipeline_and_graph_store_mut(
        &mut self,
    ) -> (&NodePropertyPredictPipeline, &mut Arc<DefaultGraphStore>) {
        (self.pipeline, &mut self.graph_store)
    }

    fn graph_store_mut(&mut self) -> &mut Arc<DefaultGraphStore> {
        &mut self.graph_store
    }

    fn graph_store(&self) -> &Arc<DefaultGraphStore> {
        &self.graph_store
    }

    fn node_labels(&self) -> &[String] {
        self.configuration.node_labels()
    }

    fn relationship_types(&self) -> &[String] {
        self.configuration.relationship_types()
    }

    fn concurrency(&self) -> usize {
        self.configuration.concurrency()
    }

    fn execute(
        &mut self,
    ) -> Result<NodeClassificationPipelineResult, PredictPipelineExecutorError> {
        let graph = self.graph_store.get_graph();
        let features =
            FeaturesFactory::extract_lazy_features(graph, &self.pipeline.feature_properties());

        let classifier = ClassifierFactory::create(self.classifier_data);

        let classifier = Arc::from(classifier);

        let expected_dimension = self.classifier_data.feature_dimension();
        if features.feature_dimension() != expected_dimension {
            return Err(PredictPipelineExecutorError::ExecutionFailed(format!(
                "Model expected features {:?} to have a total dimension of `{}`, but got `{}`.",
                self.pipeline.feature_properties(),
                expected_dimension,
                features.feature_dimension()
            )));
        }

        let predict = NodeClassificationPredict::new(
            classifier,
            Arc::from(features),
            MIN_BATCH_SIZE,
            self.configuration.include_predicted_probabilities(),
            Concurrency::of(self.configuration.concurrency()),
            self.termination_flag.clone(),
            self.progress_tracker.clone(),
        );

        let result = predict.compute();
        Ok(NodeClassificationPipelineResult::of(
            &result,
            &self.class_id_map,
        ))
    }

    fn node_property_step_filter(&self) -> PipelineGraphFilter {
        PipelineGraphFilter::new(
            self.configuration.node_labels().to_vec(),
            Some(self.configuration.relationship_types().to_vec()),
        )
    }
}
