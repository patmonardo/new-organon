use super::hyperparameter::ConcreteParameter;
use super::hyperparameter::DoubleRangeParameter;
use super::hyperparameter::IntegerRangeParameter;
use super::hyperparameter::NumericalRangeParameter;
use super::parameter_parser::parse_concrete_parameters;
use super::parameter_parser::parse_range_parameters;
use crate::ml::decision_tree::ClassifierImpurityCriterionType;
use crate::ml::gradient_descent::GradientDescentConfig;
use crate::ml::models::linear_regression::LinearRegressionTrainConfig;
use crate::ml::models::logistic_regression::LogisticRegressionTrainConfig;
use crate::ml::models::mlp::MLPClassifierTrainConfig;
use crate::ml::models::random_forest::{
    RandomForestClassifierTrainerConfig, RandomForestConfig, RandomForestRegressorTrainerConfig,
};
use crate::ml::models::{TrainerConfig, TrainingMethod};
use std::collections::{HashMap, HashSet};

const EPSILON: f64 = 1e-8;
pub const LOG_SCALE_PARAMETERS: &[&str] =
    &["penalty", "learningRate", "learning_rate", "tolerance"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonNumericParameterType {
    String,
    List,
}

pub fn is_log_scale_parameter(key: &str) -> bool {
    LOG_SCALE_PARAMETERS.contains(&key)
}

pub fn non_numeric_parameter_type(key: &str) -> Option<NonNumericParameterType> {
    match key {
        "criterion" => Some(NonNumericParameterType::String),
        "hiddenLayerSizes" | "hidden_layer_sizes" => Some(NonNumericParameterType::List),
        "classWeights" | "class_weights" => Some(NonNumericParameterType::List),
        _ => None,
    }
}

#[derive(Clone)]
pub struct TunableTrainerConfig {
    concrete_parameters: HashMap<String, Box<dyn ConcreteParameter>>,
    double_ranges: HashMap<String, DoubleRangeParameter>,
    integer_ranges: HashMap<String, IntegerRangeParameter>,
    method: TrainingMethod,
}

impl TunableTrainerConfig {
    pub fn of(
        user_input: &HashMap<String, serde_json::Value>,
        method: TrainingMethod,
    ) -> Result<Self, String> {
        let range_parameters = parse_range_parameters(user_input)?;
        let defaults = create_trainer_config_from_map(HashMap::new(), method).to_map();
        let input_with_defaults = fill_defaults(user_input, &defaults);
        let concrete_parameters = parse_concrete_parameters(&input_with_defaults)?;
        let config = Self {
            concrete_parameters,
            double_ranges: range_parameters.double_ranges().clone(),
            integer_ranges: range_parameters.integer_ranges().clone(),
            method,
        };

        // triggers validation for combinations of end endpoints of each range.
        let _ = config.stream_corner_case_configs();
        Ok(config)
    }

    pub fn concrete_parameters(&self) -> &HashMap<String, Box<dyn ConcreteParameter>> {
        &self.concrete_parameters
    }

    pub fn double_ranges(&self) -> &HashMap<String, DoubleRangeParameter> {
        &self.double_ranges
    }

    pub fn integer_ranges(&self) -> &HashMap<String, IntegerRangeParameter> {
        &self.integer_ranges
    }

    pub fn training_method(&self) -> TrainingMethod {
        self.method
    }

    pub fn is_concrete(&self) -> bool {
        self.double_ranges.is_empty() && self.integer_ranges.is_empty()
    }

    pub fn materialize(
        &self,
        hyper_parameter_values: HashMap<String, serde_json::Value>,
    ) -> Box<dyn TrainerConfig> {
        let mut materialized_map = HashMap::new();
        self.concrete_parameters.iter().for_each(|(key, value)| {
            materialized_map.insert(key.clone(), value.value().to_json());
        });
        materialized_map.extend(hyper_parameter_values);
        create_trainer_config_from_map(materialized_map, self.method)
    }

    pub fn stream_corner_case_configs(&self) -> Vec<Box<dyn TrainerConfig>> {
        enum RangeKind {
            Double(DoubleRangeParameter),
            Integer(IntegerRangeParameter),
        }

        let mut range_parameters: Vec<(String, RangeKind)> = Vec::new();
        for (key, value) in &self.double_ranges {
            range_parameters.push((key.clone(), RangeKind::Double(value.clone())));
        }
        for (key, value) in &self.integer_ranges {
            range_parameters.push((key.clone(), RangeKind::Integer(value.clone())));
        }

        let number_of_hyper_parameters = range_parameters.len();
        if number_of_hyper_parameters > 20 {
            panic!("Currently at most 20 hyperparameters are supported");
        }

        let mut configs = Vec::new();
        for mask in 0..(1usize << number_of_hyper_parameters) {
            let mut hyper_parameter_values = HashMap::new();
            for (idx, (key, range)) in range_parameters.iter().enumerate() {
                let use_min = (mask >> idx) & 1 == 0;
                let value = match range {
                    RangeKind::Double(range) => {
                        let value = if use_min {
                            range.min() + EPSILON
                        } else {
                            range.max() - EPSILON
                        };
                        serde_json::Value::Number(serde_json::Number::from_f64(value).unwrap())
                    }
                    RangeKind::Integer(range) => serde_json::Value::Number(
                        if use_min { range.min() } else { range.max() }.into(),
                    ),
                };
                hyper_parameter_values.insert(key.clone(), value);
            }
            configs.push(self.materialize(hyper_parameter_values));
        }

        configs
    }

    pub fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut result = HashMap::new();
        self.concrete_parameters.iter().for_each(|(key, value)| {
            result.insert(key.clone(), value.value().to_json());
        });
        self.double_ranges.iter().for_each(|(key, value)| {
            result.insert(
                key.clone(),
                serde_json::Value::Object(value.to_map().into_iter().collect()),
            );
        });
        self.integer_ranges.iter().for_each(|(key, value)| {
            result.insert(
                key.clone(),
                serde_json::Value::Object(value.to_map().into_iter().collect()),
            );
        });
        result.insert(
            "methodName".to_string(),
            serde_json::Value::String(training_method_name(self.method).to_string()),
        );
        result
    }
}

fn fill_defaults(
    user_input: &HashMap<String, serde_json::Value>,
    defaults: &HashMap<String, serde_json::Value>,
) -> HashMap<String, serde_json::Value> {
    let mut keys: HashSet<String> = defaults.keys().cloned().collect();
    keys.extend(user_input.keys().cloned());

    keys.into_iter()
        .filter(|key| key != "methodName")
        .map(|key| {
            let value = user_input
                .get(&key)
                .cloned()
                .or_else(|| defaults.get(&key).cloned());
            (key, value.unwrap_or(serde_json::Value::Null))
        })
        .collect()
}

fn training_method_name(method: TrainingMethod) -> &'static str {
    match method {
        TrainingMethod::LogisticRegression => "LogisticRegression",
        TrainingMethod::RandomForestClassification => "RandomForestClassification",
        TrainingMethod::MLPClassification => "MLPClassification",
        TrainingMethod::LinearRegression => "LinearRegression",
        TrainingMethod::RandomForestRegression => "RandomForestRegression",
    }
}

pub fn create_trainer_config_from_map(
    config_map: HashMap<String, serde_json::Value>,
    method: TrainingMethod,
) -> Box<dyn TrainerConfig> {
    match method {
        TrainingMethod::LogisticRegression => Box::new(logistic_regression_from_map(&config_map)),
        TrainingMethod::RandomForestClassification => {
            Box::new(random_forest_classifier_from_map(&config_map))
        }
        TrainingMethod::MLPClassification => Box::new(mlp_classifier_from_map(&config_map)),
        TrainingMethod::LinearRegression => Box::new(linear_regression_from_map(&config_map)),
        TrainingMethod::RandomForestRegression => {
            Box::new(random_forest_regressor_from_map(&config_map))
        }
    }
}

fn logistic_regression_from_map(
    map: &HashMap<String, serde_json::Value>,
) -> LogisticRegressionTrainConfig {
    let mut config = LogisticRegressionTrainConfig::default();
    if let Some(value) = get_f64(map, &["penalty"]) {
        config.penalty = value;
    }
    if let Some(value) = get_usize(map, &["batchSize", "batch_size"]) {
        config.batch_size = value;
    }
    if let Some(value) = get_f64(map, &["learningRate", "learning_rate"]) {
        config.learning_rate = value;
    }
    if let Some(value) = get_usize(map, &["maxEpochs", "max_epochs"]) {
        config.max_epochs = value;
    }
    if let Some(value) = get_f64(map, &["tolerance"]) {
        config.tolerance = value;
    }
    if let Some(value) = get_f64(map, &["focusWeight", "focus_weight"]) {
        config.focus_weight = value;
    }
    if let Some(values) = get_vec_f64(map, &["classWeights", "class_weights"]) {
        config.class_weights = Some(values);
    }
    config
}

fn linear_regression_from_map(
    map: &HashMap<String, serde_json::Value>,
) -> LinearRegressionTrainConfig {
    let mut builder = GradientDescentConfig::builder();
    if let Some(value) = get_usize(map, &["batchSize", "batch_size"]) {
        builder.batch_size(value);
    }
    if let Some(value) = get_usize(map, &["minEpochs", "min_epochs"]) {
        builder.min_epochs(value);
    }
    if let Some(value) = get_usize(map, &["patience"]) {
        builder.patience(value);
    }
    if let Some(value) = get_usize(map, &["maxEpochs", "max_epochs"]) {
        builder.max_epochs(value);
    }
    if let Some(value) = get_f64(map, &["tolerance"]) {
        builder.tolerance(value);
    }
    if let Some(value) = get_f64(map, &["learningRate", "learning_rate"]) {
        builder.learning_rate(value);
    }

    let gradient = builder
        .build()
        .expect("GradientDescentConfig builder must be valid");
    let mut config = LinearRegressionTrainConfig::new(gradient, 0.0);
    if let Some(value) = get_f64(map, &["penalty"]) {
        config.set_penalty(value);
    }
    config
}

fn mlp_classifier_from_map(map: &HashMap<String, serde_json::Value>) -> MLPClassifierTrainConfig {
    let mut config = MLPClassifierTrainConfig::default();
    if let Some(value) = get_usize(map, &["batchSize", "batch_size"]) {
        config.batch_size = value;
    }
    if let Some(value) = get_usize(map, &["minEpochs", "min_epochs"]) {
        config.min_epochs = value;
    }
    if let Some(value) = get_usize(map, &["patience"]) {
        config.patience = value;
    }
    if let Some(value) = get_usize(map, &["maxEpochs", "max_epochs"]) {
        config.max_epochs = value;
    }
    if let Some(value) = get_f64(map, &["tolerance"]) {
        config.tolerance = value;
    }
    if let Some(value) = get_f64(map, &["learningRate", "learning_rate"]) {
        config.learning_rate = value;
    }
    if let Some(value) = get_f64(map, &["penalty"]) {
        config.penalty = value;
    }
    if let Some(value) = get_f64(map, &["focusWeight", "focus_weight"]) {
        config.focus_weight = value;
    }
    if let Some(values) = get_vec_f64(map, &["classWeights", "class_weights"]) {
        config.class_weights = values;
    }
    if let Some(values) = get_vec_usize(map, &["hiddenLayerSizes", "hidden_layer_sizes"]) {
        config.hidden_layer_sizes = values;
    }
    config
}

fn random_forest_classifier_from_map(
    map: &HashMap<String, serde_json::Value>,
) -> RandomForestClassifierTrainerConfig {
    let mut config = RandomForestClassifierTrainerConfig {
        forest: random_forest_config_from_map(map),
        criterion: ClassifierImpurityCriterionType::Gini,
    };
    if let Some(value) = get_string(map, &["criterion"]) {
        if let Ok(parsed) = ClassifierImpurityCriterionType::parse(&value) {
            config.criterion = parsed;
        }
    }
    config
}

fn random_forest_regressor_from_map(
    map: &HashMap<String, serde_json::Value>,
) -> RandomForestRegressorTrainerConfig {
    RandomForestRegressorTrainerConfig {
        forest: random_forest_config_from_map(map),
    }
}

fn random_forest_config_from_map(map: &HashMap<String, serde_json::Value>) -> RandomForestConfig {
    let mut config = RandomForestConfig::default();
    if let Some(value) = get_f64(map, &["maxFeaturesRatio", "max_features_ratio"]) {
        config.max_features_ratio = Some(value);
    }
    if let Some(value) = get_f64(map, &["numberOfSamplesRatio", "num_samples_ratio"]) {
        config.num_samples_ratio = value;
    }
    if let Some(value) = get_usize(map, &["numberOfDecisionTrees", "num_decision_trees"]) {
        config.num_decision_trees = value;
    }
    if let Some(value) = get_usize(map, &["maxDepth", "max_depth"]) {
        config.max_depth = value;
    }
    if let Some(value) = get_usize(map, &["minSamplesSplit", "min_samples_split"]) {
        config.min_samples_split = value;
    }
    if let Some(value) = get_usize(map, &["minSamplesLeaf", "min_samples_leaf"]) {
        config.min_samples_leaf = value;
    }
    config
}

fn get_f64(map: &HashMap<String, serde_json::Value>, keys: &[&str]) -> Option<f64> {
    keys.iter().find_map(|key| {
        map.get(*key).and_then(|value| match value {
            serde_json::Value::Number(num) => {
                num.as_f64().or_else(|| num.as_i64().map(|v| v as f64))
            }
            _ => None,
        })
    })
}

fn get_usize(map: &HashMap<String, serde_json::Value>, keys: &[&str]) -> Option<usize> {
    keys.iter().find_map(|key| {
        map.get(*key).and_then(|value| match value {
            serde_json::Value::Number(num) => num
                .as_u64()
                .map(|v| v as usize)
                .or_else(|| num.as_i64().map(|v| v as usize)),
            _ => None,
        })
    })
}

fn get_string(map: &HashMap<String, serde_json::Value>, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| {
        map.get(*key).and_then(|value| match value {
            serde_json::Value::String(text) => Some(text.clone()),
            _ => None,
        })
    })
}

fn get_vec_f64(map: &HashMap<String, serde_json::Value>, keys: &[&str]) -> Option<Vec<f64>> {
    keys.iter().find_map(|key| {
        map.get(*key).and_then(|value| match value {
            serde_json::Value::Array(values) => {
                let mut out = Vec::new();
                for value in values {
                    if let Some(number) =
                        value.as_f64().or_else(|| value.as_i64().map(|v| v as f64))
                    {
                        out.push(number);
                    } else {
                        return None;
                    }
                }
                Some(out)
            }
            _ => None,
        })
    })
}

fn get_vec_usize(map: &HashMap<String, serde_json::Value>, keys: &[&str]) -> Option<Vec<usize>> {
    keys.iter().find_map(|key| {
        map.get(*key).and_then(|value| match value {
            serde_json::Value::Array(values) => {
                let mut out = Vec::new();
                for value in values {
                    if let Some(number) =
                        value.as_u64().or_else(|| value.as_i64().map(|v| v as u64))
                    {
                        out.push(number as usize);
                    } else {
                        return None;
                    }
                }
                Some(out)
            }
            _ => None,
        })
    })
}
