use crate::projection::eval::pipeline::node_pipeline::{
    node_feature_step::NodeFeatureStep,
    node_property_prediction_split_config::NodePropertyPredictionSplitConfig,
    node_property_training_pipeline::NodePropertyTrainingPipeline,
};
use crate::projection::eval::pipeline::{
    AutoTuningConfig, ExecutableNodePropertyStep, Pipeline, PipelineValidationError,
    TrainingMethod, TrainingPipeline, TunableTrainerConfig,
};
use crate::types::graph_store::DefaultGraphStore;
use std::collections::HashMap;
use std::sync::Arc;

/// Node classification training pipeline.
///
/// This pipeline extends the base NodePropertyTrainingPipeline specifically for classification tasks.
/// It supports binary and multi-class classification using various ML algorithms.
pub struct NodeClassificationTrainingPipeline {
    /// Node property computation steps (graph algorithms like PageRank, etc.)
    node_property_steps: Vec<Box<dyn ExecutableNodePropertyStep>>,

    /// Feature extraction steps for node classification
    feature_steps: Vec<NodeFeatureStep>,

    /// Training parameter space: maps training methods to model candidates
    training_parameter_space: HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>>,

    /// AutoML configuration for hyperparameter tuning
    auto_tuning_config: AutoTuningConfig,

    /// Split configuration for train/test/validation sets
    split_config: NodePropertyPredictionSplitConfig,
}

impl NodeClassificationTrainingPipeline {
    pub const PIPELINE_TYPE: &'static str = "Node classification training pipeline";
    pub const MODEL_TYPE: &'static str = "NodeClassification";

    /// Create a new node classification training pipeline.
    pub fn new() -> Self {
        Self {
            node_property_steps: Vec::new(),
            feature_steps: Vec::new(),
            training_parameter_space: HashMap::new(),
            auto_tuning_config: AutoTuningConfig::default(),
            split_config: NodePropertyPredictionSplitConfig::default(),
        }
    }

    /// Add a node property step to the pipeline.
    pub fn add_node_property_step(&mut self, step: Box<dyn ExecutableNodePropertyStep>) {
        self.node_property_steps.push(step);
    }

    /// Add a feature step to the pipeline.
    pub fn add_feature_step(&mut self, step: NodeFeatureStep) {
        self.feature_steps.push(step);
    }

    /// Replace all feature steps in the pipeline.
    pub fn set_feature_steps(&mut self, steps: Vec<NodeFeatureStep>) {
        self.feature_steps = steps;
    }

    /// Get the pipeline type.
    pub fn pipeline_type(&self) -> &'static str {
        Self::PIPELINE_TYPE
    }

    /// Get the model type.
    pub fn model_type(&self) -> &'static str {
        Self::MODEL_TYPE
    }

    /// Validate pipeline-specific constraints before execution.
    ///
    /// For classification, this ensures we have at least one feature step.
    pub fn specific_validate_before_execution(
        &self,
        _graph_store: Arc<DefaultGraphStore>,
    ) -> Result<(), PipelineValidationError> {
        if self.feature_steps.is_empty() {
            return Err(PipelineValidationError::Other {
                message: "Node classification pipeline requires at least one feature step"
                    .to_string(),
            });
        }
        Ok(())
    }

    /// Check if eager feature computation is required.
    ///
    /// Returns true if RandomForestClassification is in the training parameter space,
    /// as it requires all features to be computed upfront.
    pub fn require_eager_features(&self) -> bool {
        self.training_parameter_space
            .contains_key(&TrainingMethod::RandomForestClassification)
    }
}

impl Clone for NodeClassificationTrainingPipeline {
    fn clone(&self) -> Self {
        Self {
            node_property_steps: self.node_property_steps.iter().cloned().collect(),
            feature_steps: self.feature_steps.clone(),
            training_parameter_space: self
                .training_parameter_space
                .iter()
                .map(|(method, configs)| (*method, configs.iter().cloned().collect()))
                .collect(),
            auto_tuning_config: self.auto_tuning_config.clone(),
            split_config: self.split_config.clone(),
        }
    }
}

// Pipeline trait implementation
impl Pipeline for NodeClassificationTrainingPipeline {
    type FeatureStep = NodeFeatureStep;

    fn node_property_steps(&self) -> &[Box<dyn ExecutableNodePropertyStep>] {
        &self.node_property_steps
    }

    fn feature_steps(&self) -> &[Self::FeatureStep] {
        &self.feature_steps
    }

    fn specific_validate_before_execution(
        &self,
        graph_store: &DefaultGraphStore,
    ) -> Result<(), PipelineValidationError> {
        self.specific_validate_before_execution(Arc::new(graph_store.clone()))
    }

    fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();

        // Add pipeline metadata
        map.insert(
            "pipelineType".to_string(),
            serde_json::json!(self.pipeline_type()),
        );
        map.insert(
            "modelType".to_string(),
            serde_json::json!(self.model_type()),
        );

        // Add feature pipeline description
        let feature_pipeline =
            <Self as NodePropertyTrainingPipeline>::feature_pipeline_description(self);
        map.insert(
            "featurePipeline".to_string(),
            serde_json::json!(feature_pipeline),
        );

        // Add training configuration
        map.insert(
            "trainingParameterSpace".to_string(),
            serde_json::json!(self.parameter_space_to_map()),
        );
        map.insert(
            "autoTuningConfig".to_string(),
            serde_json::json!(self.auto_tuning_config().to_map()),
        );

        // Add split configuration
        map.insert(
            "splitConfig".to_string(),
            serde_json::json!(self.split_config().to_map()),
        );

        map
    }
}

// TrainingPipeline trait implementation
impl TrainingPipeline for NodeClassificationTrainingPipeline {
    fn pipeline_type(&self) -> &str {
        Self::PIPELINE_TYPE
    }

    fn training_parameter_space(
        &self,
    ) -> &HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>> {
        &self.training_parameter_space
    }

    fn training_parameter_space_mut(
        &mut self,
    ) -> &mut HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>> {
        &mut self.training_parameter_space
    }

    fn auto_tuning_config(&self) -> &AutoTuningConfig {
        &self.auto_tuning_config
    }

    fn set_auto_tuning_config(&mut self, config: AutoTuningConfig) {
        self.auto_tuning_config = config;
    }
}

// NodePropertyTrainingPipeline trait implementation
impl NodePropertyTrainingPipeline for NodeClassificationTrainingPipeline {
    fn split_config(&self) -> &NodePropertyPredictionSplitConfig {
        &self.split_config
    }

    fn set_split_config(&mut self, split_config: NodePropertyPredictionSplitConfig) {
        self.split_config = split_config;
    }

    fn require_eager_features(&self) -> bool {
        self.require_eager_features()
    }
}

impl Default for NodeClassificationTrainingPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::NodeClassificationTrainingPipeline;

    #[test]
    fn test_new_pipeline() {
        let pipeline = NodeClassificationTrainingPipeline::new();
        assert_eq!(
            pipeline.pipeline_type(),
            NodeClassificationTrainingPipeline::PIPELINE_TYPE
        );
    }

    #[test]
    fn test_pipeline_constants() {
        assert_eq!(
            NodeClassificationTrainingPipeline::PIPELINE_TYPE,
            "Node classification training pipeline"
        );
        assert_eq!(
            NodeClassificationTrainingPipeline::MODEL_TYPE,
            "NodeClassification"
        );
    }

    #[test]
    fn test_require_eager_features() {
        let pipeline = NodeClassificationTrainingPipeline::new();
        // Should return false until training parameter space is implemented
        assert!(!pipeline.require_eager_features());
    }
}
