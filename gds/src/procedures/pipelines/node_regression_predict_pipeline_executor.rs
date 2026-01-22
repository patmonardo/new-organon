use std::sync::Arc;

use crate::collections::HugeDoubleArray;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{Task, TaskProgressTracker, Tasks};
use crate::ml::models::{FeaturesFactory, Regressor};
use crate::ml::node_prediction::regression::NodeRegressionPredict;
use crate::projection::eval::pipeline::{
    node_pipeline::NodePropertyPredictPipeline, PipelineGraphFilter, PredictPipelineExecutor,
    PredictPipelineExecutorError,
};
use crate::types::graph_store::{DefaultGraphStore, GraphStore};

use super::NodeRegressionPredictPipelineConfig;

pub struct NodeRegressionPredictPipelineExecutor<'a> {
    pipeline: &'a NodePropertyPredictPipeline,
    configuration: &'a dyn NodeRegressionPredictPipelineConfig,
    graph_store: Arc<DefaultGraphStore>,
    progress_tracker: TaskProgressTracker,
    regressor: Arc<dyn Regressor>,
    termination_flag: TerminationFlag,
}

impl<'a> NodeRegressionPredictPipelineExecutor<'a> {
    pub fn new(
        pipeline: &'a NodePropertyPredictPipeline,
        configuration: &'a dyn NodeRegressionPredictPipelineConfig,
        graph_store: Arc<DefaultGraphStore>,
        progress_tracker: TaskProgressTracker,
        regressor: Arc<dyn Regressor>,
        termination_flag: TerminationFlag,
    ) -> Self {
        Self {
            pipeline,
            configuration,
            graph_store,
            progress_tracker,
            regressor,
            termination_flag,
        }
    }

    pub fn progress_task(task_name: &str, graph_store: &DefaultGraphStore) -> Task {
        Tasks::leaf_with_volume(format!("{task_name} progress"), graph_store.node_count())
            .base()
            .clone()
    }
}

impl<'a> PredictPipelineExecutor<NodePropertyPredictPipeline, HugeDoubleArray>
    for NodeRegressionPredictPipelineExecutor<'a>
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

    fn execute(&mut self) -> Result<HugeDoubleArray, PredictPipelineExecutorError> {
        let graph = self.graph_store.get_graph();
        let features =
            FeaturesFactory::extract_lazy_features(graph, &self.pipeline.feature_properties());

        let expected_dimension = self.regressor.data().feature_dimension();
        if features.feature_dimension() != expected_dimension {
            return Err(PredictPipelineExecutorError::ExecutionFailed(format!(
                "Model expected features {:?} to have a total dimension of `{}`, but got `{}`.",
                self.pipeline.feature_properties(),
                expected_dimension,
                features.feature_dimension()
            )));
        }

        let predict = NodeRegressionPredict::new(
            Arc::clone(&self.regressor),
            Arc::from(features),
            Concurrency::of(self.configuration.concurrency()),
            self.progress_tracker.clone(),
            self.termination_flag.clone(),
        );

        Ok(predict.compute())
    }

    fn node_property_step_filter(&self) -> PipelineGraphFilter {
        PipelineGraphFilter::new(
            self.configuration.node_labels().to_vec(),
            Some(self.configuration.relationship_types().to_vec()),
        )
    }
}
