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

/// Node regression training pipeline.
///
/// This pipeline extends the base NodePropertyTrainingPipeline specifically for regression tasks.
/// It supports continuous value prediction using various ML algorithms.
pub struct NodeRegressionTrainingPipeline {
    /// Node property computation steps (graph algorithms like PageRank, etc.)
    node_property_steps: Vec<Box<dyn ExecutableNodePropertyStep>>,

    /// Feature extraction steps for node regression
    feature_steps: Vec<NodeFeatureStep>,

    /// Training parameter space: maps training methods to model candidates
    training_parameter_space: HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>>,

    /// AutoML configuration for hyperparameter tuning
    auto_tuning_config: AutoTuningConfig,

    /// Split configuration for train/test/validation sets
    split_config: NodePropertyPredictionSplitConfig,
}

impl NodeRegressionTrainingPipeline {
    pub const PIPELINE_TYPE: &'static str = "Node regression training pipeline";
    pub const MODEL_TYPE: &'static str = "NodeRegression";

    /// Create a new node regression training pipeline.
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
    /// For regression, this ensures we have at least one feature step.
    pub fn specific_validate_before_execution(
        &self,
        _graph_store: Arc<DefaultGraphStore>,
    ) -> Result<(), PipelineValidationError> {
        if self.feature_steps.is_empty() {
            return Err(PipelineValidationError::Other {
                message: "Node regression pipeline requires at least one feature step".to_string(),
            });
        }
        Ok(())
    }

    /// Check if eager feature computation is required.
    ///
    /// Returns true if RandomForestRegression is in the training parameter space,
    /// as it requires all features to be computed upfront.
    pub fn require_eager_features(&self) -> bool {
        self.training_parameter_space
            .contains_key(&TrainingMethod::RandomForestRegression)
    }
}

impl std::fmt::Debug for NodeRegressionTrainingPipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeRegressionTrainingPipeline")
            .field("pipeline_type", &self.pipeline_type())
            .field("model_type", &self.model_type())
            .field("node_property_steps_count", &self.node_property_steps.len())
            .field("feature_steps_count", &self.feature_steps.len())
            .field(
                "training_parameter_space_size",
                &self.training_parameter_space.len(),
            )
            .field("auto_tuning_config", &self.auto_tuning_config)
            .field("split_config", &self.split_config)
            .finish()
    }
}

impl Clone for NodeRegressionTrainingPipeline {
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

impl Default for NodeRegressionTrainingPipeline {
    fn default() -> Self {
        Self::new()
    }
}

// Pipeline trait implementation
impl Pipeline for NodeRegressionTrainingPipeline {
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
impl TrainingPipeline for NodeRegressionTrainingPipeline {
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
impl NodePropertyTrainingPipeline for NodeRegressionTrainingPipeline {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::random::RandomGraphConfig;

    #[test]
    fn test_new_pipeline() {
        let pipeline = NodeRegressionTrainingPipeline::new();
        assert_eq!(
            pipeline.pipeline_type(),
            "Node regression training pipeline"
        );
    }

    #[test]
    fn test_pipeline_constants() {
        assert_eq!(
            NodeRegressionTrainingPipeline::PIPELINE_TYPE,
            "Node regression training pipeline"
        );
        assert_eq!(NodeRegressionTrainingPipeline::MODEL_TYPE, "NodeRegression");
    }

    #[test]
    fn test_default() {
        let pipeline = NodeRegressionTrainingPipeline::default();
        assert_eq!(
            pipeline.pipeline_type(),
            "Node regression training pipeline"
        );
    }

    #[test]
    fn test_require_eager_features_default() {
        let pipeline = NodeRegressionTrainingPipeline::new();
        // Without RandomForestRegression in parameter space, should return false
        assert!(!pipeline.require_eager_features());
    }

    #[test]
    fn test_specific_validation() {
        let pipeline = NodeRegressionTrainingPipeline::new();
        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..Default::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&config).expect("Failed to generate random graph"));

        // Should fail validation when no feature steps are present
        let result = pipeline.specific_validate_before_execution(graph_store);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("requires at least one feature step"));
    }
}
