use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

use crate::projection::eval::pipeline::link_pipeline::LinkFeatureStepFactory;
use crate::projection::eval::pipeline::link_pipeline::LinkPredictionTrainingPipeline;
use crate::projection::eval::pipeline::node_pipeline::classification::node_classification_training_pipeline::NodeClassificationTrainingPipeline;
use crate::projection::eval::pipeline::node_pipeline::node_feature_step::NodeFeatureStep;
use crate::projection::eval::pipeline::node_pipeline::node_property_prediction_split_config::NodePropertyPredictionSplitConfig;
use crate::projection::eval::pipeline::node_pipeline::node_property_training_pipeline::NodePropertyTrainingPipeline;
use crate::projection::eval::pipeline::node_pipeline::regression::node_regression_training_pipeline::NodeRegressionTrainingPipeline;
use crate::projection::eval::pipeline::{
    AutoTuningConfig, ExecutableNodePropertyStep, Pipeline, TrainingMethod, TrainingPipeline,
};
use crate::projection::eval::pipeline::{NodePropertyStep, PipelineCatalogEntry};
use crate::types::user::User;

use super::node_pipeline_info_result_transformer::{
    create_node_classification_info_result, create_node_regression_info_result,
};
use super::pipeline_catalog_result_transformer::create_pipeline_catalog_result;
use super::pipeline_info_result_transformer::create_pipeline_info_result;
use super::{PipelineName, PipelineRepository};
use crate::procedures::pipelines::types::{
    NodePipelineInfoResult, PipelineCatalogResult, PipelineExistsResult, PipelineInfoResult,
};

pub struct PipelineApplications {
    user: User,
    pipeline_repository: PipelineRepository,
}

impl PipelineApplications {
    pub fn new(user: User, pipeline_repository: PipelineRepository) -> Self {
        Self {
            user,
            pipeline_repository,
        }
    }

    pub fn create_link_prediction_training_pipeline(
        &self,
        pipeline_name: &PipelineName,
    ) -> PipelineInfoResult {
        let pipeline = self
            .pipeline_repository
            .create_link_prediction_training_pipeline(&self.user, pipeline_name);

        create_pipeline_info_result(pipeline_name.as_str(), pipeline.as_ref())
    }

    pub fn create_node_classification_training_pipeline(
        &self,
        pipeline_name: &PipelineName,
    ) -> NodePipelineInfoResult {
        let pipeline = self
            .pipeline_repository
            .create_node_classification_training_pipeline(&self.user, pipeline_name);

        create_node_classification_info_result(pipeline_name.as_str(), pipeline.as_ref())
    }

    pub fn create_node_regression_training_pipeline(
        &self,
        pipeline_name: &PipelineName,
    ) -> NodePipelineInfoResult {
        let pipeline = self
            .pipeline_repository
            .create_node_regression_training_pipeline(&self.user, pipeline_name);

        create_node_regression_info_result(pipeline_name.as_str(), pipeline.as_ref())
    }

    pub fn add_feature_to_link_prediction_pipeline(
        &self,
        pipeline_name: &PipelineName,
        feature_type: &str,
        configuration: Value,
    ) -> PipelineInfoResult {
        let existing = self
            .pipeline_repository
            .get_link_prediction_training_pipeline(&self.user, pipeline_name);

        let mut next = (*existing).clone();
        let step = LinkFeatureStepFactory::create_from_config(feature_type, &configuration)
            .unwrap_or_else(|e| panic!("{e}"));
        next.add_feature_step(step);

        let next = Arc::new(next);
        self.pipeline_repository
            .replace(&self.user, pipeline_name, Arc::clone(&next));

        create_pipeline_info_result(pipeline_name.as_str(), next.as_ref())
    }

    pub fn add_node_property_to_link_prediction_pipeline(
        &self,
        pipeline_name: &PipelineName,
        task_name: &str,
        procedure_config: HashMap<String, Value>,
    ) -> PipelineInfoResult {
        let existing = self
            .pipeline_repository
            .get_link_prediction_training_pipeline(&self.user, pipeline_name);

        let mut next = (*existing).clone();
        let step = NodePropertyStep::new(task_name.to_string(), procedure_config);
        let step_box: Box<dyn ExecutableNodePropertyStep> = Box::new(step);

        next.validate_unique_mutate_property(step_box.as_ref())
            .unwrap_or_else(|e| panic!("{e}"));
        next.add_node_property_step(step_box);

        let next = Arc::new(next);
        self.pipeline_repository
            .replace(&self.user, pipeline_name, Arc::clone(&next));

        create_pipeline_info_result(pipeline_name.as_str(), next.as_ref())
    }

    pub fn add_node_property_to_node_classification_pipeline(
        &self,
        pipeline_name: &PipelineName,
        task_name: &str,
        procedure_config: HashMap<String, Value>,
    ) -> NodePipelineInfoResult {
        let existing = self
            .pipeline_repository
            .get_node_classification_training_pipeline(&self.user, pipeline_name);

        let mut next = (*existing).clone();
        let step = NodePropertyStep::new(task_name.to_string(), procedure_config);
        let step_box: Box<dyn ExecutableNodePropertyStep> = Box::new(step);

        next.validate_unique_mutate_property(step_box.as_ref())
            .unwrap_or_else(|e| panic!("{e}"));
        next.add_node_property_step(step_box);

        let next = Arc::new(next);
        self.pipeline_repository
            .replace(&self.user, pipeline_name, Arc::clone(&next));

        create_node_classification_info_result(pipeline_name.as_str(), next.as_ref())
    }

    pub fn add_node_property_to_node_regression_pipeline(
        &self,
        pipeline_name: &PipelineName,
        task_name: &str,
        procedure_config: HashMap<String, Value>,
    ) -> NodePipelineInfoResult {
        let existing = self
            .pipeline_repository
            .get_node_regression_training_pipeline(&self.user, pipeline_name);

        let mut next = (*existing).clone();
        let step = NodePropertyStep::new(task_name.to_string(), procedure_config);
        let step_box: Box<dyn ExecutableNodePropertyStep> = Box::new(step);

        next.validate_unique_mutate_property(step_box.as_ref())
            .unwrap_or_else(|e| panic!("{e}"));
        next.add_node_property_step(step_box);

        let next = Arc::new(next);
        self.pipeline_repository
            .replace(&self.user, pipeline_name, Arc::clone(&next));

        create_node_regression_info_result(pipeline_name.as_str(), next.as_ref())
    }

    pub fn select_features_for_classification(
        &self,
        pipeline_name: &PipelineName,
        feature_steps: Vec<NodeFeatureStep>,
    ) -> NodePipelineInfoResult {
        let existing = self
            .pipeline_repository
            .get_node_classification_training_pipeline(&self.user, pipeline_name);

        let next = rebuild_node_classification_with_features(existing.as_ref(), feature_steps);
        let next = Arc::new(next);
        self.pipeline_repository
            .replace(&self.user, pipeline_name, Arc::clone(&next));

        create_node_classification_info_result(pipeline_name.as_str(), next.as_ref())
    }

    pub fn select_features_for_regression(
        &self,
        pipeline_name: &PipelineName,
        feature_steps: Vec<NodeFeatureStep>,
    ) -> NodePipelineInfoResult {
        let existing = self
            .pipeline_repository
            .get_node_regression_training_pipeline(&self.user, pipeline_name);

        let next = rebuild_node_regression_with_features(existing.as_ref(), feature_steps);
        let next = Arc::new(next);
        self.pipeline_repository
            .replace(&self.user, pipeline_name, Arc::clone(&next));

        create_node_regression_info_result(pipeline_name.as_str(), next.as_ref())
    }

    pub fn configure_node_property_split(
        &self,
        pipeline_name: &PipelineName,
        split_config: NodePropertyPredictionSplitConfig,
        is_classification: bool,
    ) -> NodePipelineInfoResult {
        if is_classification {
            let existing = self
                .pipeline_repository
                .get_node_classification_training_pipeline(&self.user, pipeline_name);
            let mut next = (*existing).clone();
            next.set_split_config(split_config);
            let next = Arc::new(next);
            self.pipeline_repository
                .replace(&self.user, pipeline_name, Arc::clone(&next));
            create_node_classification_info_result(pipeline_name.as_str(), next.as_ref())
        } else {
            let existing = self
                .pipeline_repository
                .get_node_regression_training_pipeline(&self.user, pipeline_name);
            let mut next = (*existing).clone();
            next.set_split_config(split_config);
            let next = Arc::new(next);
            self.pipeline_repository
                .replace(&self.user, pipeline_name, Arc::clone(&next));
            create_node_regression_info_result(pipeline_name.as_str(), next.as_ref())
        }
    }

    pub fn configure_auto_tuning(
        &self,
        pipeline_name: &PipelineName,
        auto_tuning_config: AutoTuningConfig,
        is_classification: bool,
    ) -> NodePipelineInfoResult {
        if is_classification {
            let existing = self
                .pipeline_repository
                .get_node_classification_training_pipeline(&self.user, pipeline_name);
            let mut next = (*existing).clone();
            next.set_auto_tuning_config(auto_tuning_config);
            let next = Arc::new(next);
            self.pipeline_repository
                .replace(&self.user, pipeline_name, Arc::clone(&next));
            create_node_classification_info_result(pipeline_name.as_str(), next.as_ref())
        } else {
            let existing = self
                .pipeline_repository
                .get_node_regression_training_pipeline(&self.user, pipeline_name);
            let mut next = (*existing).clone();
            next.set_auto_tuning_config(auto_tuning_config);
            let next = Arc::new(next);
            self.pipeline_repository
                .replace(&self.user, pipeline_name, Arc::clone(&next));
            create_node_regression_info_result(pipeline_name.as_str(), next.as_ref())
        }
    }

    pub fn add_training_method(
        &self,
        pipeline_name: &PipelineName,
        method: TrainingMethod,
        is_classification: bool,
    ) -> NodePipelineInfoResult {
        if is_classification {
            let existing = self
                .pipeline_repository
                .get_node_classification_training_pipeline(&self.user, pipeline_name);
            let mut next = (*existing).clone();
            next.training_parameter_space_mut()
                .entry(method)
                .or_default();
            let next = Arc::new(next);
            self.pipeline_repository
                .replace(&self.user, pipeline_name, Arc::clone(&next));
            create_node_classification_info_result(pipeline_name.as_str(), next.as_ref())
        } else {
            let existing = self
                .pipeline_repository
                .get_node_regression_training_pipeline(&self.user, pipeline_name);
            let mut next = (*existing).clone();
            next.training_parameter_space_mut()
                .entry(method)
                .or_default();
            let next = Arc::new(next);
            self.pipeline_repository
                .replace(&self.user, pipeline_name, Arc::clone(&next));
            create_node_regression_info_result(pipeline_name.as_str(), next.as_ref())
        }
    }

    pub fn list(&self) -> Vec<PipelineCatalogResult> {
        self.pipeline_repository
            .get_all(&self.user)
            .into_iter()
            .map(|entry| pipeline_catalog_entry_to_result(entry))
            .collect()
    }

    pub fn exists(&self, pipeline_name: &PipelineName) -> PipelineExistsResult {
        if !self.pipeline_repository.exists(&self.user, pipeline_name) {
            return PipelineExistsResult::empty(pipeline_name.as_str());
        }

        let entry = self
            .pipeline_repository
            .get_entry(&self.user, pipeline_name);

        PipelineExistsResult {
            pipeline_name: pipeline_name.as_str().to_string(),
            pipeline_type: entry.pipeline_type().to_string(),
            exists: true,
        }
    }

    pub fn drop(&self, pipeline_name: &PipelineName) -> PipelineCatalogResult {
        let entry = self.pipeline_repository.drop(&self.user, pipeline_name);
        pipeline_catalog_entry_to_result(entry)
    }
}

fn pipeline_catalog_entry_to_result(entry: PipelineCatalogEntry) -> PipelineCatalogResult {
    match entry
        .pipeline_as::<LinkPredictionTrainingPipeline>()
    {
        Some(pipeline) => {
            return create_pipeline_catalog_result(pipeline.as_ref(), entry.pipeline_name());
        }
        None => {}
    }

    if let Some(pipeline) = entry.pipeline_as::<NodeClassificationTrainingPipeline>() {
        return create_pipeline_catalog_result(pipeline.as_ref(), entry.pipeline_name());
    }

    if let Some(pipeline) = entry.pipeline_as::<NodeRegressionTrainingPipeline>() {
        return create_pipeline_catalog_result(pipeline.as_ref(), entry.pipeline_name());
    }

    PipelineCatalogResult {
        pipeline_info: HashMap::new(),
        pipeline_name: entry.pipeline_name().to_string(),
        pipeline_type: entry.pipeline_type().to_string(),
        creation_time: chrono::Utc::now()
            .with_timezone(&chrono::FixedOffset::east_opt(0).expect("UTC offset")),
    }
}

fn rebuild_node_classification_with_features(
    pipeline: &NodeClassificationTrainingPipeline,
    feature_steps: Vec<NodeFeatureStep>,
) -> NodeClassificationTrainingPipeline {
    let mut next = NodeClassificationTrainingPipeline::new();

    for step in pipeline.node_property_steps() {
        next.add_node_property_step(step.clone());
    }

    for step in feature_steps {
        next.add_feature_step(step);
    }

    for (method, configs) in pipeline.training_parameter_space() {
        next.training_parameter_space_mut()
            .insert(*method, configs.iter().cloned().collect());
    }

    next.set_auto_tuning_config(pipeline.auto_tuning_config().clone());
    next.set_split_config(pipeline.split_config().clone());

    next
}

fn rebuild_node_regression_with_features(
    pipeline: &NodeRegressionTrainingPipeline,
    feature_steps: Vec<NodeFeatureStep>,
) -> NodeRegressionTrainingPipeline {
    let mut next = NodeRegressionTrainingPipeline::new();

    for step in pipeline.node_property_steps() {
        next.add_node_property_step(step.clone());
    }

    for step in feature_steps {
        next.add_feature_step(step);
    }

    for (method, configs) in pipeline.training_parameter_space() {
        next.training_parameter_space_mut()
            .insert(*method, configs.iter().cloned().collect());
    }

    next.set_auto_tuning_config(pipeline.auto_tuning_config().clone());
    next.set_split_config(pipeline.split_config().clone());

    next
}
