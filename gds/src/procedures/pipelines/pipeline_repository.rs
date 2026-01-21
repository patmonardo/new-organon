use std::sync::Arc;

use crate::projection::eval::pipeline::link_pipeline::LinkPredictionTrainingPipeline;
use crate::projection::eval::pipeline::node_pipeline::classification::node_classification_training_pipeline::NodeClassificationTrainingPipeline;
use crate::projection::eval::pipeline::node_pipeline::regression::node_regression_training_pipeline::NodeRegressionTrainingPipeline;
use crate::projection::eval::pipeline::{PipelineCatalog, PipelineCatalogEntry, TrainingPipeline};
use crate::types::user::User;

use super::PipelineName;

#[derive(Clone)]
pub struct PipelineRepository {
    pipeline_catalog: Arc<PipelineCatalog>,
}

impl PipelineRepository {
    pub fn new(pipeline_catalog: Arc<PipelineCatalog>) -> Self {
        Self { pipeline_catalog }
    }

    pub fn create_link_prediction_training_pipeline(
        &self,
        user: &User,
        pipeline_name: &PipelineName,
    ) -> Arc<LinkPredictionTrainingPipeline> {
        let pipeline = Arc::new(LinkPredictionTrainingPipeline::new());
        self.pipeline_catalog
            .set(
                user.username(),
                pipeline_name.as_str(),
                Arc::clone(&pipeline),
            )
            .unwrap_or_else(|e| panic!("{e}"));
        pipeline
    }

    pub fn create_node_classification_training_pipeline(
        &self,
        user: &User,
        pipeline_name: &PipelineName,
    ) -> Arc<NodeClassificationTrainingPipeline> {
        let pipeline = Arc::new(NodeClassificationTrainingPipeline::new());
        self.pipeline_catalog
            .set(
                user.username(),
                pipeline_name.as_str(),
                Arc::clone(&pipeline),
            )
            .unwrap_or_else(|e| panic!("{e}"));
        pipeline
    }

    pub fn create_node_regression_training_pipeline(
        &self,
        user: &User,
        pipeline_name: &PipelineName,
    ) -> Arc<NodeRegressionTrainingPipeline> {
        let pipeline = Arc::new(NodeRegressionTrainingPipeline::new());
        self.pipeline_catalog
            .set(
                user.username(),
                pipeline_name.as_str(),
                Arc::clone(&pipeline),
            )
            .unwrap_or_else(|e| panic!("{e}"));
        pipeline
    }

    pub fn replace<P: TrainingPipeline + Send + Sync + 'static>(
        &self,
        user: &User,
        pipeline_name: &PipelineName,
        pipeline: Arc<P>,
    ) {
        self.pipeline_catalog
            .replace(user.username(), pipeline_name.as_str(), pipeline)
            .unwrap_or_else(|e| panic!("{e}"));
    }

    pub fn drop(&self, user: &User, pipeline_name: &PipelineName) -> PipelineCatalogEntry {
        PipelineCatalog::drop(
            self.pipeline_catalog.as_ref(),
            user.username(),
            pipeline_name.as_str(),
        )
        .unwrap_or_else(|e| panic!("{e}"))
    }

    pub fn exists(&self, user: &User, pipeline_name: &PipelineName) -> bool {
        self.pipeline_catalog
            .exists(user.username(), pipeline_name.as_str())
    }

    pub fn get_all(&self, user: &User) -> Vec<PipelineCatalogEntry> {
        self.pipeline_catalog.get_all_pipelines(user.username())
    }

    pub fn get_entry(&self, user: &User, pipeline_name: &PipelineName) -> PipelineCatalogEntry {
        self.pipeline_catalog
            .get(user.username(), pipeline_name.as_str())
            .unwrap_or_else(|e| panic!("{e}"))
    }

    pub fn get_link_prediction_training_pipeline(
        &self,
        user: &User,
        pipeline_name: &PipelineName,
    ) -> Arc<LinkPredictionTrainingPipeline> {
        self.pipeline_catalog
            .get_typed::<LinkPredictionTrainingPipeline>(user.username(), pipeline_name.as_str())
            .unwrap_or_else(|e| panic!("{e}"))
    }

    pub fn get_node_classification_training_pipeline(
        &self,
        user: &User,
        pipeline_name: &PipelineName,
    ) -> Arc<NodeClassificationTrainingPipeline> {
        self.pipeline_catalog
            .get_typed::<NodeClassificationTrainingPipeline>(
                user.username(),
                pipeline_name.as_str(),
            )
            .unwrap_or_else(|e| panic!("{e}"))
    }

    pub fn get_node_regression_training_pipeline(
        &self,
        user: &User,
        pipeline_name: &PipelineName,
    ) -> Arc<NodeRegressionTrainingPipeline> {
        self.pipeline_catalog
            .get_typed::<NodeRegressionTrainingPipeline>(user.username(), pipeline_name.as_str())
            .unwrap_or_else(|e| panic!("{e}"))
    }
}
