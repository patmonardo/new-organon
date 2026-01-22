// Phase 4.1: LinkPredictionTrainingPipeline - Training pipeline for link prediction

use super::{LinkFeatureStep, LinkPredictionSplitConfig};
use crate::projection::eval::pipeline::PipelineValidationError;
use crate::projection::eval::pipeline::{
    AutoTuningConfig, ExecutableNodePropertyStep, FeatureStep, Pipeline, TrainingMethod,
    TrainingPipeline, TunableTrainerConfig,
};
use crate::types::graph_store::DefaultGraphStore;
use std::collections::HashMap;
use std::marker::PhantomData;

/// Training pipeline for link prediction.
///
/// Pipeline structure:
/// - Node property steps compute properties used by features.
/// - Link feature steps derive pairwise features from node properties.
/// - Split configuration controls train/test/validation and negative sampling.
#[derive(Clone)]
pub struct LinkFeatureStepWrapper {
    step: Box<dyn LinkFeatureStep>,
    name: String,
    input_node_properties: Vec<String>,
    configuration: HashMap<String, serde_json::Value>,
}

impl LinkFeatureStepWrapper {
    pub fn new(step: Box<dyn LinkFeatureStep>) -> Self {
        let name = step.name().to_string();
        let input_node_properties = step.input_node_properties();
        let configuration = step.configuration();
        Self {
            step,
            name,
            input_node_properties,
            configuration,
        }
    }

    pub fn as_link_step(&self) -> &dyn LinkFeatureStep {
        self.step.as_ref()
    }

    pub fn to_link_step_box(&self) -> Box<dyn LinkFeatureStep> {
        self.step.clone_box()
    }
}

impl FeatureStep for LinkFeatureStepWrapper {
    fn input_node_properties(&self) -> &[String] {
        &self.input_node_properties
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn configuration(&self) -> &HashMap<String, serde_json::Value> {
        &self.configuration
    }

    fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert("name".to_string(), serde_json::json!(self.name()));
        map.insert(
            "config".to_string(),
            serde_json::json!(self.configuration().clone()),
        );
        map
    }
}

#[derive(Clone)]
pub struct LinkPredictionTrainingPipeline {
    /// Pipeline type identifier
    pub pipeline_type: &'static str,

    /// Model type identifier
    pub model_type: &'static str,

    /// Node property steps (preprocessing)
    node_property_steps: Vec<Box<dyn ExecutableNodePropertyStep>>,

    /// Link feature steps (feature extraction)
    feature_steps: Vec<LinkFeatureStepWrapper>,

    /// Split configuration (train/test/validation + negative sampling)
    split_config: LinkPredictionSplitConfig,

    /// Training parameter space (model candidates)
    training_parameter_space: HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>>,

    /// AutoML / tuning configuration
    auto_tuning_config: AutoTuningConfig,
}

impl LinkPredictionTrainingPipeline {
    /// Pipeline type constant
    pub const PIPELINE_TYPE: &'static str = "Link prediction training pipeline";

    /// Model type constant
    pub const MODEL_TYPE: &'static str = "LinkPrediction";

    /// Creates a new LinkPredictionTrainingPipeline with default settings.
    pub fn new() -> Self {
        Self {
            pipeline_type: Self::PIPELINE_TYPE,
            model_type: Self::MODEL_TYPE,
            node_property_steps: Vec::new(),
            feature_steps: Vec::new(),
            split_config: LinkPredictionSplitConfig::default(),
            training_parameter_space: HashMap::new(),
            auto_tuning_config: AutoTuningConfig::default(),
        }
    }

    /// Returns the pipeline type.
    pub fn pipeline_type(&self) -> &str {
        self.pipeline_type
    }

    /// Returns the model type.
    pub fn model_type(&self) -> &str {
        self.model_type
    }

    /// Returns the link feature steps as boxed trait objects.
    ///
    /// This is a convenience API for link-specific runtime code.
    pub fn link_feature_steps(&self) -> Vec<Box<dyn LinkFeatureStep>> {
        self.feature_steps
            .iter()
            .map(|s| s.to_link_step_box())
            .collect()
    }

    /// Adds a feature step to the pipeline.
    pub fn add_feature_step(&mut self, step: Box<dyn LinkFeatureStep>) {
        self.feature_steps.push(LinkFeatureStepWrapper::new(step));
    }

    /// Adds a node property step to the pipeline.
    pub fn add_node_property_step(&mut self, step: Box<dyn ExecutableNodePropertyStep>) {
        self.node_property_steps.push(step);
    }

    /// Returns the split configuration.
    pub fn split_config(&self) -> &LinkPredictionSplitConfig {
        &self.split_config
    }

    /// Sets the split configuration.
    pub fn set_split_config(&mut self, config: LinkPredictionSplitConfig) {
        self.split_config = config;
    }

    /// Returns the feature pipeline description.
    ///
    /// Returns map with:
    /// - "nodePropertySteps": preprocessing steps
    /// - "featureSteps": feature extraction steps
    pub fn feature_pipeline_description(
        &self,
    ) -> HashMap<String, Vec<HashMap<String, serde_json::Value>>> {
        let mut description = HashMap::new();

        // Node property steps
        let node_steps: Vec<HashMap<String, serde_json::Value>> = self
            .node_property_steps
            .iter()
            .map(|step| step.to_map())
            .collect();
        description.insert("nodePropertySteps".to_string(), node_steps);

        // Feature steps
        let feature_steps_maps: Vec<HashMap<String, serde_json::Value>> = self
            .feature_steps
            .iter()
            .map(|step| step.configuration().clone())
            .collect();
        description.insert("featureSteps".to_string(), feature_steps_maps);

        description
    }

    /// Returns additional pipeline entries.
    ///
    /// Link prediction pipelines include split config (not in node pipelines).
    pub fn additional_entries(&self) -> HashMap<String, serde_json::Value> {
        let mut entries = HashMap::new();
        entries.insert(
            "splitConfig".to_string(),
            serde_json::json!(self.split_config.to_map()),
        );
        entries
    }

    /// Validates the pipeline before execution.
    pub fn validate_before_execution(&self) -> Result<(), String> {
        if self.feature_steps.is_empty() {
            return Err(
                "Training a Link prediction pipeline requires at least one feature. \
                 You can add features with the procedure `gds.beta.pipeline.linkPrediction.addFeature`."
                    .to_string(),
            );
        }
        Ok(())
    }

    pub fn split_config_mut(&mut self) -> &mut LinkPredictionSplitConfig {
        &mut self.split_config
    }

    /// Returns tasks grouped by relationship property.
    ///
    /// Some node property steps (like embeddings) may use relationship weights.
    /// This method groups tasks by the relationship property they use.
    ///
    /// # Arguments
    ///
    /// * `model_catalog` - Model catalog for looking up model configs
    /// * `username` - Username for model lookup
    ///
    /// # Returns
    ///
    /// Map of relationship property name → list of task names using that property.
    pub fn tasks_by_relationship_property(
        &self,
        _model_catalog: PhantomData<()>, // Note: replace with ModelCatalog
        _username: &str,
    ) -> HashMap<String, Vec<String>> {
        // Note: Implement when ModelCatalog is available.
        // For each node property step:
        //   - Check if config has RELATIONSHIP_WEIGHT_PROPERTY key
        //   - Or check if config has MODEL_NAME_KEY and look up model
        //   - Group tasks by the relationship property they use
        HashMap::new()
    }

    /// Returns the relationship weight property used by the pipeline (if any).
    ///
    /// Some algorithms (FastRP, Node2Vec) can use relationship weights.
    /// This method extracts the weight property from node property steps.
    pub fn relationship_weight_property(
        &self,
        _model_catalog: PhantomData<()>, // Note: replace with ModelCatalog
        _username: &str,
    ) -> Option<String> {
        // Note: Implement when ModelCatalog is available.
        // Call tasks_by_relationship_property()
        // Return first property if any
        None
    }
}

impl Pipeline for LinkPredictionTrainingPipeline {
    type FeatureStep = LinkFeatureStepWrapper;

    fn node_property_steps(&self) -> &[Box<dyn ExecutableNodePropertyStep>] {
        &self.node_property_steps
    }

    fn feature_steps(&self) -> &[Self::FeatureStep] {
        &self.feature_steps
    }

    fn specific_validate_before_execution(
        &self,
        _graph_store: &DefaultGraphStore,
    ) -> Result<(), PipelineValidationError> {
        // Reuse the link-specific check for "must have at least one feature".
        self.validate_before_execution()
            .map_err(|e| PipelineValidationError::Other { message: e })
    }

    fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();

        // Basic identity
        map.insert("type".to_string(), serde_json::json!(self.pipeline_type()));
        map.insert(
            "modelType".to_string(),
            serde_json::json!(self.model_type()),
        );

        // Steps
        map.insert(
            "nodePropertySteps".to_string(),
            serde_json::json!(self
                .node_property_steps()
                .iter()
                .map(|s| s.to_map())
                .collect::<Vec<_>>()),
        );
        map.insert(
            "featureSteps".to_string(),
            serde_json::json!(self
                .feature_steps()
                .iter()
                .map(|s| s.to_map())
                .collect::<Vec<_>>()),
        );

        // Training config
        let parameter_space: HashMap<String, Vec<HashMap<String, serde_json::Value>>> = self
            .training_parameter_space()
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    v.iter().map(|cfg| cfg.to_map()).collect::<Vec<_>>(),
                )
            })
            .collect();

        map.insert(
            "parameterSpace".to_string(),
            serde_json::json!(parameter_space),
        );
        map.insert(
            "autoTuningConfig".to_string(),
            serde_json::json!(self.auto_tuning_config().to_map()),
        );
        map.insert(
            "splitConfig".to_string(),
            serde_json::json!(self.split_config().to_map()),
        );

        map
    }
}

impl TrainingPipeline for LinkPredictionTrainingPipeline {
    fn pipeline_type(&self) -> &str {
        self.pipeline_type
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

impl Default for LinkPredictionTrainingPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::pipeline::link_pipeline::link_functions::{
        CosineFeatureStep, HadamardFeatureStep,
    };

    #[test]
    fn test_pipeline_creation() {
        let pipeline = LinkPredictionTrainingPipeline::new();
        assert_eq!(
            pipeline.pipeline_type(),
            "Link prediction training pipeline"
        );
        assert_eq!(pipeline.model_type(), "LinkPrediction");
    }

    #[test]
    fn test_pipeline_constants() {
        assert_eq!(
            LinkPredictionTrainingPipeline::PIPELINE_TYPE,
            "Link prediction training pipeline"
        );
        assert_eq!(LinkPredictionTrainingPipeline::MODEL_TYPE, "LinkPrediction");
    }

    #[test]
    fn test_add_feature_step() {
        let mut pipeline = LinkPredictionTrainingPipeline::new();
        pipeline.add_feature_step(Box::new(HadamardFeatureStep::new(vec!["prop".to_string()])));

        assert_eq!(pipeline.feature_steps().len(), 1);
        assert_eq!(pipeline.feature_steps()[0].name(), "HADAMARD");
    }

    #[test]
    fn test_multiple_feature_steps() {
        let mut pipeline = LinkPredictionTrainingPipeline::new();
        pipeline.add_feature_step(Box::new(HadamardFeatureStep::new(vec![
            "embedding".to_string()
        ])));
        pipeline.add_feature_step(Box::new(CosineFeatureStep::new(vec![
            "features".to_string()
        ])));

        assert_eq!(pipeline.feature_steps().len(), 2);
    }

    #[test]
    fn test_split_config() {
        let pipeline = LinkPredictionTrainingPipeline::new();
        let _config = pipeline.split_config();
        // Default split config should exist
    }

    #[test]
    fn test_set_split_config() {
        let mut pipeline = LinkPredictionTrainingPipeline::new();
        let custom_config = LinkPredictionSplitConfig::default();
        pipeline.set_split_config(custom_config);
        // Config should be set
    }

    #[test]
    fn test_validation_empty_features() {
        let pipeline = LinkPredictionTrainingPipeline::new();
        let result = pipeline.validate_before_execution();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("at least one feature"));
    }

    #[test]
    fn test_validation_with_features() {
        let mut pipeline = LinkPredictionTrainingPipeline::new();
        pipeline.add_feature_step(Box::new(HadamardFeatureStep::new(vec!["prop".to_string()])));

        let result = pipeline.validate_before_execution();
        assert!(result.is_ok());
    }

    #[test]
    fn test_feature_pipeline_description() {
        let mut pipeline = LinkPredictionTrainingPipeline::new();
        pipeline.add_feature_step(Box::new(HadamardFeatureStep::new(vec![
            "embedding".to_string()
        ])));

        let description = pipeline.feature_pipeline_description();
        assert!(description.contains_key("nodePropertySteps"));
        assert!(description.contains_key("featureSteps"));
        assert_eq!(description.get("featureSteps").unwrap().len(), 1);
    }

    #[test]
    fn test_additional_entries() {
        let pipeline = LinkPredictionTrainingPipeline::new();
        let entries = pipeline.additional_entries();

        assert!(entries.contains_key("splitConfig"));
    }

    #[test]
    fn test_feature_steps_preserve_order() {
        let mut pipeline = LinkPredictionTrainingPipeline::new();

        pipeline.add_feature_step(Box::new(HadamardFeatureStep::new(vec![
            "embedding".to_string()
        ])));
        pipeline.add_feature_step(Box::new(CosineFeatureStep::new(vec![
            "features".to_string()
        ])));

        assert_eq!(pipeline.feature_steps().len(), 2);
        assert_eq!(pipeline.feature_steps()[0].name(), "HADAMARD");
        assert_eq!(pipeline.feature_steps()[1].name(), "COSINE");
    }
}
