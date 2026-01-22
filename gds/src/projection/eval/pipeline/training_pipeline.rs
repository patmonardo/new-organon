// Translated from Neo4j Graph Data Science:
// https://github.com/neo4j/graph-data-science
// pipeline/src/main/java/org/neo4j/gds/ml/pipeline/TrainingPipeline.java

use std::collections::HashMap;
use std::error::Error as StdError;

use dyn_clone::DynClone;

use crate::projection::eval::pipeline::{AutoTuningConfig, ExecutableNodePropertyStep, Pipeline};

/// Training type: classification or regression.
///
/// Java source: `TrainingPipeline.TrainingType` enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrainingType {
    Classification,
    Regression,
}

impl TrainingType {
    /// Get supported training methods for this training type.
    ///
    /// Java: `abstract List<TrainingMethod> supportedMethods()`
    pub fn supported_methods(&self) -> Vec<TrainingMethod> {
        match self {
            TrainingType::Classification => vec![
                TrainingMethod::LogisticRegression,
                TrainingMethod::RandomForestClassification,
                TrainingMethod::MLPClassification,
            ],
            TrainingType::Regression => vec![
                TrainingMethod::LinearRegression,
                TrainingMethod::RandomForestRegression,
            ],
        }
    }
}

/// Training method types supported by pipelines.
///
/// Java GDS defines these in TrainingMethod enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrainingMethod {
    LogisticRegression,
    RandomForestClassification,
    MLPClassification,
    LinearRegression,
    RandomForestRegression,
}

impl std::fmt::Display for TrainingMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LogisticRegression => write!(f, "LogisticRegression"),
            Self::RandomForestClassification => write!(f, "RandomForestClassification"),
            Self::MLPClassification => write!(f, "MLPClassification"),
            Self::LinearRegression => write!(f, "LinearRegression"),
            Self::RandomForestRegression => write!(f, "RandomForestRegression"),
        }
    }
}

/// Tunable trainer configuration for hyperparameter search.
///
/// This represents a single model candidate in the training parameter space.
/// Can be concrete (fixed parameters) or tunable (parameter ranges for AutoML).
pub trait TunableTrainerConfig: Send + Sync + DynClone {
    /// Get the training method for this config.
    fn training_method(&self) -> TrainingMethod;

    /// Check if this config is concrete (all parameters fixed).
    ///
    /// Concrete configs are used directly without AutoML tuning.
    fn is_concrete(&self) -> bool;

    /// Convert to map for serialization.
    fn to_map(&self) -> HashMap<String, serde_json::Value>;
}

dyn_clone::clone_trait_object!(TunableTrainerConfig);

/// Training pipeline that supports model selection and hyperparameter tuning.
///
/// Extends the base Pipeline trait with training-specific features:
/// - Training parameter space (multiple model candidates)
/// - AutoML configuration
/// - Model selection trials
///
/// # Java Source (TrainingPipeline.java)
/// ```java
/// public abstract class TrainingPipeline<FEATURE_STEP extends FeatureStep>
///     implements Pipeline<FEATURE_STEP> {
///     protected final List<ExecutableNodePropertyStep> nodePropertySteps;
///     protected final List<FEATURE_STEP> featureSteps;
///     private final ZonedDateTime creationTime;
///     protected Map<TrainingMethod, List<TunableTrainerConfig>> trainingParameterSpace;
///     protected AutoTuningConfig autoTuningConfig;
///
///     public abstract String type();
///     public int numberOfModelSelectionTrials() { /* ... */ }
///     public void addTrainerConfig(TunableTrainerConfig) { /* ... */ }
///     public void validateTrainingParameterSpace() { /* ... */ }
/// }
/// ```
pub trait TrainingPipeline: Pipeline {
    /// Get the pipeline type (e.g., "Node classification training pipeline").
    fn pipeline_type(&self) -> &str;

    /// Get the training parameter space.
    ///
    /// This is a map from training method to list of model candidates.
    /// Each candidate can be concrete or tunable.
    ///
    /// Java: `Map<TrainingMethod, List<TunableTrainerConfig>> trainingParameterSpace()`
    fn training_parameter_space(
        &self,
    ) -> &HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>>;

    /// Get the training parameter space (mutable).
    fn training_parameter_space_mut(
        &mut self,
    ) -> &mut HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>>;

    /// Get the AutoML tuning configuration.
    ///
    /// Java: `AutoTuningConfig autoTuningConfig()`
    fn auto_tuning_config(&self) -> &AutoTuningConfig;

    /// Set the AutoML tuning configuration.
    ///
    /// Java: `void setAutoTuningConfig(AutoTuningConfig)`
    fn set_auto_tuning_config(&mut self, config: AutoTuningConfig);

    /// Add a trainer configuration to the parameter space.
    ///
    /// Java: `void addTrainerConfig(TunableTrainerConfig)`
    fn add_trainer_config(&mut self, config: Box<dyn TunableTrainerConfig>) {
        let method = config.training_method();
        self.training_parameter_space_mut()
            .entry(method)
            .or_default()
            .push(config);
    }

    /// Get the number of model selection trials.
    ///
    /// This accounts for:
    /// - Concrete configs (evaluated directly)
    /// - Tunable configs (AutoML trials)
    ///
    /// Java: `int numberOfModelSelectionTrials()`
    fn number_of_model_selection_trials(&self) -> usize {
        let total_configs = self.number_of_trainer_configs();
        let concrete_configs = self.concrete_trainer_configs_count();

        if concrete_configs == total_configs {
            // All configs are concrete, no AutoML needed
            total_configs
        } else {
            // Mix of concrete and tunable: concrete + AutoML trials
            concrete_configs + self.auto_tuning_config().max_trials()
        }
    }

    /// Validate the training parameter space.
    ///
    /// Ensures at least one model candidate exists for training.
    ///
    /// Java: `void validateTrainingParameterSpace()`
    fn validate_training_parameter_space(&self) -> Result<(), Box<dyn StdError + Send + Sync>> {
        if self.number_of_model_selection_trials() == 0 {
            return Err("Need at least one model candidate for training.".into());
        }
        Ok(())
    }

    /// Validate unique mutate properties across node property steps.
    ///
    /// Ensures no duplicate property names are created by steps.
    ///
    /// Java: `private void validateUniqueMutateProperty(ExecutableNodePropertyStep)`
    fn validate_unique_mutate_property(
        &self,
        step: &dyn ExecutableNodePropertyStep,
    ) -> Result<(), Box<dyn StdError + Send + Sync>> {
        for existing_step in self.node_property_steps() {
            let new_property = step.mutate_node_property();
            let existing_property = existing_step.mutate_node_property();

            if new_property == existing_property {
                return Err(format!(
                    "The value of `mutateProperty` is expected to be unique, but {} was already \
                     specified in the {} procedure.",
                    new_property,
                    existing_step.proc_name()
                )
                .into());
            }
        }
        Ok(())
    }

    /// Convert training parameter space to map for serialization.
    ///
    /// Java: `static Map<String, List<Map<String, Object>>> toMapParameterSpace(...)`
    fn parameter_space_to_map(&self) -> HashMap<String, Vec<HashMap<String, serde_json::Value>>> {
        self.training_parameter_space()
            .iter()
            .map(|(method, configs)| {
                let method_str = method.to_string();
                let configs_vec = configs.iter().map(|c| c.to_map()).collect();
                (method_str, configs_vec)
            })
            .collect()
    }

    // Helper methods

    /// Get total number of trainer configs.
    fn number_of_trainer_configs(&self) -> usize {
        self.training_parameter_space()
            .values()
            .map(|configs| configs.len())
            .sum()
    }

    /// Get number of concrete trainer configs.
    fn concrete_trainer_configs_count(&self) -> usize {
        self.training_parameter_space()
            .values()
            .flat_map(|configs| configs.iter())
            .filter(|config| config.is_concrete())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::pipeline::FeatureStep;
    use crate::projection::eval::pipeline::PipelineValidationError;
    use crate::types::graph_store::DefaultGraphStore;

    #[test]
    fn test_training_method_display() {
        assert_eq!(
            TrainingMethod::LogisticRegression.to_string(),
            "LogisticRegression"
        );
        assert_eq!(
            TrainingMethod::RandomForestClassification.to_string(),
            "RandomForestClassification"
        );
    }

    #[test]
    fn test_training_method_equality() {
        assert_eq!(
            TrainingMethod::LogisticRegression,
            TrainingMethod::LogisticRegression
        );
        assert_ne!(
            TrainingMethod::LogisticRegression,
            TrainingMethod::LinearRegression
        );
    }

    #[test]
    fn test_training_type_classification_methods() {
        let methods = TrainingType::Classification.supported_methods();
        assert_eq!(methods.len(), 3);
        assert!(methods.contains(&TrainingMethod::LogisticRegression));
        assert!(methods.contains(&TrainingMethod::RandomForestClassification));
        assert!(methods.contains(&TrainingMethod::MLPClassification));
    }

    #[test]
    fn test_training_type_regression_methods() {
        let methods = TrainingType::Regression.supported_methods();
        assert_eq!(methods.len(), 2);
        assert!(methods.contains(&TrainingMethod::LinearRegression));
        assert!(methods.contains(&TrainingMethod::RandomForestRegression));
    }

    // Mock implementations for testing
    #[derive(Clone)]
    struct MockTrainerConfig {
        method: TrainingMethod,
        concrete: bool,
    }

    impl TunableTrainerConfig for MockTrainerConfig {
        fn training_method(&self) -> TrainingMethod {
            self.method
        }

        fn is_concrete(&self) -> bool {
            self.concrete
        }

        fn to_map(&self) -> HashMap<String, serde_json::Value> {
            let mut map = HashMap::new();
            map.insert(
                "method".to_string(),
                serde_json::json!(self.method.to_string()),
            );
            map.insert("concrete".to_string(), serde_json::json!(self.concrete));
            map
        }
    }

    #[test]
    fn test_tunable_trainer_config() {
        let config = MockTrainerConfig {
            method: TrainingMethod::LogisticRegression,
            concrete: true,
        };

        assert_eq!(config.training_method(), TrainingMethod::LogisticRegression);
        assert!(config.is_concrete());

        let map = config.to_map();
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_parameter_space_to_map() {
        let mut training_parameter_space: HashMap<
            TrainingMethod,
            Vec<Box<dyn TunableTrainerConfig>>,
        > = HashMap::new();

        training_parameter_space.insert(
            TrainingMethod::LogisticRegression,
            vec![Box::new(MockTrainerConfig {
                method: TrainingMethod::LogisticRegression,
                concrete: true,
            })],
        );

        let pipeline = MockTrainingPipeline {
            node_property_steps: vec![],
            feature_steps: vec![],
            training_parameter_space,
            auto_tuning_config: AutoTuningConfig::default(),
        };

        let map = pipeline.parameter_space_to_map();
        assert!(map.contains_key("LogisticRegression"));
        assert_eq!(map["LogisticRegression"].len(), 1);
    }

    #[derive(Clone, Debug)]
    struct MockStep {
        name: String,
        mutate: String,
    }

    impl ExecutableNodePropertyStep for MockStep {
        fn execute(
            &self,
            _graph_store: &mut DefaultGraphStore,
            _node_labels: &[String],
            _relationship_types: &[String],
            _concurrency: usize,
        ) -> Result<(), Box<dyn StdError + Send + Sync>> {
            Ok(())
        }

        fn config(&self) -> &HashMap<String, serde_json::Value> {
            static EMPTY: std::sync::OnceLock<HashMap<String, serde_json::Value>> =
                std::sync::OnceLock::new();
            EMPTY.get_or_init(HashMap::new)
        }

        fn proc_name(&self) -> &str {
            &self.name
        }

        fn mutate_node_property(&self) -> &str {
            &self.mutate
        }
    }

    #[derive(Clone, Debug)]
    struct MockFeatureStep {
        input: Vec<String>,
        name: String,
    }

    impl FeatureStep for MockFeatureStep {
        fn input_node_properties(&self) -> &[String] {
            &self.input
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn configuration(&self) -> &HashMap<String, serde_json::Value> {
            static EMPTY: std::sync::OnceLock<HashMap<String, serde_json::Value>> =
                std::sync::OnceLock::new();
            EMPTY.get_or_init(HashMap::new)
        }

        fn to_map(&self) -> HashMap<String, serde_json::Value> {
            HashMap::new()
        }
    }

    struct MockTrainingPipeline {
        node_property_steps: Vec<Box<dyn ExecutableNodePropertyStep>>,
        feature_steps: Vec<MockFeatureStep>,
        training_parameter_space: HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>>,
        auto_tuning_config: AutoTuningConfig,
    }

    impl Pipeline for MockTrainingPipeline {
        type FeatureStep = MockFeatureStep;

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

        fn to_map(&self) -> HashMap<String, serde_json::Value> {
            HashMap::new()
        }
    }

    impl TrainingPipeline for MockTrainingPipeline {
        fn pipeline_type(&self) -> &str {
            "mock-training-pipeline"
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

    #[test]
    fn test_number_of_model_selection_trials_mixed_configs() {
        let mut training_parameter_space: HashMap<
            TrainingMethod,
            Vec<Box<dyn TunableTrainerConfig>>,
        > = HashMap::new();

        training_parameter_space.insert(
            TrainingMethod::LogisticRegression,
            vec![
                Box::new(MockTrainerConfig {
                    method: TrainingMethod::LogisticRegression,
                    concrete: true,
                }),
                Box::new(MockTrainerConfig {
                    method: TrainingMethod::LogisticRegression,
                    concrete: false,
                }),
            ],
        );

        let pipeline = MockTrainingPipeline {
            node_property_steps: vec![],
            feature_steps: vec![],
            training_parameter_space,
            auto_tuning_config: AutoTuningConfig::new(5).unwrap(),
        };

        // One concrete + AutoML trials
        assert_eq!(pipeline.number_of_model_selection_trials(), 1 + 5);
    }

    #[test]
    fn test_validate_training_parameter_space_requires_candidates() {
        let pipeline = MockTrainingPipeline {
            node_property_steps: vec![],
            feature_steps: vec![],
            training_parameter_space: HashMap::new(),
            auto_tuning_config: AutoTuningConfig::default(),
        };

        let result = pipeline.validate_training_parameter_space();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Need at least one model candidate"));
    }

    #[test]
    fn test_validate_unique_mutate_property() {
        let pipeline = MockTrainingPipeline {
            node_property_steps: vec![Box::new(MockStep {
                name: "gds.step.a".to_string(),
                mutate: "dup".to_string(),
            })],
            feature_steps: vec![],
            training_parameter_space: HashMap::new(),
            auto_tuning_config: AutoTuningConfig::default(),
        };

        let duplicate_step = MockStep {
            name: "gds.step.b".to_string(),
            mutate: "dup".to_string(),
        };

        let err = pipeline
            .validate_unique_mutate_property(&duplicate_step)
            .unwrap_err();

        assert!(err.to_string().contains("mutateProperty"));
    }
}
