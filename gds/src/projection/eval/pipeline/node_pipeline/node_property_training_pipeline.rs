use super::node_property_prediction_split_config::NodePropertyPredictionSplitConfig;
use crate::projection::eval::pipeline::training_pipeline::TrainingPipeline;
use std::collections::HashMap;

/// Abstract base for node property training pipelines.
///
/// This trait extends TrainingPipeline with node-specific configuration
/// for splitting datasets and controlling feature computation.
pub trait NodePropertyTrainingPipeline: TrainingPipeline {
    /// Returns the split configuration for train/test/validation sets.
    fn split_config(&self) -> &NodePropertyPredictionSplitConfig;

    /// Sets the split configuration.
    fn set_split_config(&mut self, split_config: NodePropertyPredictionSplitConfig);

    /// Returns whether eager feature computation is required.
    ///
    /// When true, all features are computed upfront before training.
    /// When false, features can be computed lazily during training.
    fn require_eager_features(&self) -> bool;

    /// Returns the feature pipeline description for serialization.
    fn feature_pipeline_description(
        &self,
    ) -> HashMap<String, Vec<HashMap<String, serde_json::Value>>> {
        use crate::projection::eval::pipeline::FeatureStep;
        let mut desc = HashMap::new();

        // Node property steps
        let node_property_steps: Vec<HashMap<String, serde_json::Value>> = self
            .node_property_steps()
            .iter()
            .map(|step| step.to_map())
            .collect();
        desc.insert("nodePropertySteps".to_string(), node_property_steps);

        // Feature properties
        let feature_steps: Vec<HashMap<String, serde_json::Value>> = self
            .feature_steps()
            .iter()
            .map(|step| step.to_map())
            .collect();
        desc.insert("featureProperties".to_string(), feature_steps);

        desc
    }

    /// Returns additional entries for pipeline serialization.
    fn additional_entries(&self) -> HashMap<String, HashMap<String, String>> {
        let mut entries = HashMap::new();
        entries.insert("splitConfig".to_string(), self.split_config().to_map());
        entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::projection::eval::pipeline::PipelineValidationError;
    use crate::projection::eval::pipeline::auto_tuning_config::AutoTuningConfig;
    use crate::projection::eval::pipeline::training_pipeline::TunableTrainerConfig;
    use crate::projection::eval::pipeline::{
        ExecutableNodePropertyStep, FeatureStep, Pipeline, TrainingMethod,
    };
    use serde_json::Value;

    // Mock feature step for testing
    #[derive(Clone, Debug)]
    struct MockFeatureStep {
        input_properties: Vec<String>,
        configuration: HashMap<String, Value>,
    }

    impl FeatureStep for MockFeatureStep {
        fn input_node_properties(&self) -> &[String] {
            &self.input_properties
        }
        fn name(&self) -> &str {
            "mock"
        }
        fn configuration(&self) -> &HashMap<String, Value> {
            &self.configuration
        }
        fn to_map(&self) -> HashMap<String, Value> {
            let mut map = HashMap::new();
            map.insert("type".to_string(), Value::String("mock".to_string()));
            map
        }
    }

    // Mock training pipeline for testing
    struct MockNodeTrainingPipeline {
        split_config: NodePropertyPredictionSplitConfig,
        node_property_steps: Vec<Box<dyn ExecutableNodePropertyStep>>,
        feature_steps: Vec<MockFeatureStep>,
        training_parameter_space: HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>>,
        auto_tuning_config: AutoTuningConfig,
    }

    impl Pipeline for MockNodeTrainingPipeline {
        type FeatureStep = MockFeatureStep;

        fn to_map(&self) -> HashMap<String, Value> {
            HashMap::new()
        }

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
            Ok(())
        }
    }

    impl TrainingPipeline for MockNodeTrainingPipeline {
        fn pipeline_type(&self) -> &str {
            "mock-node-training-pipeline"
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

    impl NodePropertyTrainingPipeline for MockNodeTrainingPipeline {
        fn split_config(&self) -> &NodePropertyPredictionSplitConfig {
            &self.split_config
        }

        fn set_split_config(&mut self, split_config: NodePropertyPredictionSplitConfig) {
            self.split_config = split_config;
        }

        fn require_eager_features(&self) -> bool {
            false
        }
    }

    #[test]
    fn test_split_config_access() {
        let mut pipeline = MockNodeTrainingPipeline {
            split_config: NodePropertyPredictionSplitConfig::default(),
            node_property_steps: vec![],
            feature_steps: vec![],
            training_parameter_space: HashMap::new(),
            auto_tuning_config: AutoTuningConfig::default(),
        };

        assert_eq!(pipeline.split_config().test_fraction(), 0.3);
        assert_eq!(pipeline.split_config().validation_folds(), 3);

        let new_config = NodePropertyPredictionSplitConfig::new(0.2, 5).unwrap();
        pipeline.set_split_config(new_config);

        assert_eq!(pipeline.split_config().test_fraction(), 0.2);
        assert_eq!(pipeline.split_config().validation_folds(), 5);
    }

    #[test]
    fn test_require_eager_features() {
        let pipeline = MockNodeTrainingPipeline {
            split_config: NodePropertyPredictionSplitConfig::default(),
            node_property_steps: vec![],
            feature_steps: vec![],
            training_parameter_space: HashMap::new(),
            auto_tuning_config: AutoTuningConfig::default(),
        };

        assert!(!pipeline.require_eager_features());
    }

    #[test]
    fn test_feature_pipeline_description() {
        let pipeline = MockNodeTrainingPipeline {
            split_config: NodePropertyPredictionSplitConfig::default(),
            node_property_steps: vec![],
            feature_steps: vec![MockFeatureStep {
                input_properties: vec!["age".to_string()],
                configuration: HashMap::new(),
            }],
            training_parameter_space: HashMap::new(),
            auto_tuning_config: AutoTuningConfig::default(),
        };

        let desc = pipeline.feature_pipeline_description();
        assert!(desc.contains_key("nodePropertySteps"));
        assert!(desc.contains_key("featureProperties"));
        assert_eq!(desc.get("featureProperties").unwrap().len(), 1);
    }

    #[test]
    fn test_additional_entries() {
        let pipeline = MockNodeTrainingPipeline {
            split_config: NodePropertyPredictionSplitConfig::new(0.25, 4).unwrap(),
            node_property_steps: vec![],
            feature_steps: vec![],
            training_parameter_space: HashMap::new(),
            auto_tuning_config: AutoTuningConfig::default(),
        };

        let entries = pipeline.additional_entries();
        assert!(entries.contains_key("splitConfig"));
        let split_map = entries.get("splitConfig").unwrap();
        assert_eq!(split_map.get("testFraction"), Some(&"0.25".to_string()));
    }
}
