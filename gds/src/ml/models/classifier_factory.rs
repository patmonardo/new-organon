use crate::mem::{Estimate, MemoryEstimation, MemoryEstimations, MemoryRange};
use crate::ml::decision_tree::DecisionTreeClassifierTrainer;
use crate::ml::decision_tree::DecisionTreeTrainerConfig;
use crate::ml::models::base::TrainerConfigTrait;
use crate::ml::models::logistic_regression::{
    LogisticRegressionClassifier, LogisticRegressionData,
};
use crate::ml::models::mlp::MLPClassifierTrainConfig;
use crate::ml::models::mlp::{MLPClassifier, MLPClassifierData};
use crate::ml::models::random_forest::RandomForestClassifierTrainerConfig;
use crate::ml::models::random_forest::{RandomForestClassifier, RandomForestClassifierData};
use crate::ml::models::{Classifier, ClassifierData, TrainingMethod};

/// Factory for creating classifiers from trained model data.
/// 1:1 translation of ClassifierFactory.java from Java GDS.
pub struct ClassifierFactory;

impl ClassifierFactory {
    /// Create a classifier from trained model data.
    /// 1:1 with ClassifierFactory.create(Classifier.ClassifierData) in Java
    pub fn create(classifier_data: &dyn ClassifierData) -> Box<dyn Classifier> {
        match classifier_data.trainer_method() {
            TrainingMethod::LogisticRegression => {
                // In Java: LogisticRegressionClassifier.from((LogisticRegressionData) classifierData)
                let lr_data = classifier_data
                    .as_any()
                    .downcast_ref::<LogisticRegressionData>()
                    .expect("Invalid ClassifierData type for LogisticRegression");
                Box::new(LogisticRegressionClassifier::from(lr_data.clone()))
            }
            TrainingMethod::RandomForestClassification => {
                // In Java: new RandomForestClassifier((RandomForestClassifierData) classifierData)
                let rf_data = classifier_data
                    .as_any()
                    .downcast_ref::<RandomForestClassifierData>()
                    .expect("Invalid ClassifierData type for RandomForestClassification");
                Box::new(RandomForestClassifier::new(rf_data.clone()))
            }
            TrainingMethod::MLPClassification => {
                // In Java: new MLPClassifier((MLPClassifierData) classifierData)
                let mlp_data = classifier_data
                    .as_any()
                    .downcast_ref::<MLPClassifierData>()
                    .expect("Invalid ClassifierData type for MLPClassification");
                Box::new(MLPClassifier::new(mlp_data.clone()))
            }
            _ => panic!(
                "No such classifier for training method: {:?}",
                classifier_data.trainer_method()
            ),
        }
    }

    /// Estimate runtime memory overhead for predictions.
    /// 1:1 with ClassifierFactory.runtimeOverheadMemoryEstimation() in Java
    pub fn runtime_overhead_memory_estimation(
        method: TrainingMethod,
        _batch_size: usize,
        _number_of_classes: usize,
        _feature_dimension: usize,
        _is_reduced: bool,
    ) -> MemoryRange {
        match method {
            TrainingMethod::LogisticRegression => {
                let rows = if _is_reduced {
                    _number_of_classes.saturating_sub(1)
                } else {
                    _number_of_classes
                };
                let batch_features =
                    Estimate::size_of_double_array(_batch_size * _feature_dimension);
                let logits = Estimate::size_of_double_array(_batch_size * rows);
                let probs = Estimate::size_of_double_array(_batch_size * _number_of_classes.max(1));
                MemoryRange::of(batch_features + logits + probs)
            }
            TrainingMethod::RandomForestClassification => {
                let probs = Estimate::size_of_double_array(_batch_size * _number_of_classes.max(1));
                MemoryRange::of(probs)
            }
            TrainingMethod::MLPClassification => {
                let probs = Estimate::size_of_double_array(_batch_size * _number_of_classes.max(1));
                MemoryRange::of(probs)
            }
            _ => panic!("No such classifier for training method: {:?}", method),
        }
    }

    /// Estimate memory for trained model data.
    /// 1:1 with ClassifierFactory.dataMemoryEstimation() in Java
    pub fn data_memory_estimation(
        trainer_config: &dyn TrainerConfigTrait,
        _number_of_training_samples: impl Fn(u64) -> u64 + Send + Sync + 'static,
        _number_of_classes: usize,
        _feature_dimension: MemoryRange,
        _is_reduced: bool,
    ) -> Box<dyn MemoryEstimation> {
        match trainer_config.method() {
            TrainingMethod::LogisticRegression => {
                let rows = if _is_reduced {
                    _number_of_classes.saturating_sub(1)
                } else {
                    _number_of_classes
                };
                let min_weights =
                    Estimate::size_of_double_array(rows.saturating_mul(_feature_dimension.min()));
                let max_weights =
                    Estimate::size_of_double_array(rows.saturating_mul(_feature_dimension.max()));
                let bias = Estimate::size_of_double_array(rows);
                MemoryEstimations::of_range(
                    "LogisticRegressionData",
                    MemoryRange::of_range(min_weights + bias, max_weights + bias),
                )
            }
            TrainingMethod::RandomForestClassification => {
                let rf_config = (trainer_config as &dyn std::any::Any)
                    .downcast_ref::<RandomForestClassifierTrainerConfig>()
                    .expect("Invalid config type for RandomForestClassification");

                let tree_config = DecisionTreeTrainerConfig::builder()
                    .max_depth(rf_config.forest.max_depth)
                    .min_split_size(rf_config.forest.min_samples_split)
                    .min_leaf_size(rf_config.forest.min_samples_leaf)
                    .build()
                    .expect("Invalid decision tree config");

                let num_trees = rf_config.forest.num_decision_trees;
                let tree_config = std::sync::Arc::new(tree_config);

                MemoryEstimations::of_resident("RandomForestClassifierData", move |dim, _| {
                    let training_samples = _number_of_training_samples(dim.node_count() as u64);
                    let per_tree = DecisionTreeClassifierTrainer::memory_estimation(
                        tree_config.as_ref(),
                        training_samples as usize,
                        _number_of_classes,
                    );
                    MemoryRange::of(per_tree.saturating_mul(num_trees))
                })
            }
            TrainingMethod::MLPClassification => {
                let mlp_config = (trainer_config as &dyn std::any::Any)
                    .downcast_ref::<MLPClassifierTrainConfig>()
                    .expect("Invalid config type for MLPClassification");

                let hidden = &mlp_config.hidden_layer_sizes;
                if hidden.is_empty() {
                    return MemoryEstimations::empty();
                }

                let min_feature_dim = _feature_dimension.min();
                let max_feature_dim = _feature_dimension.max();

                let first_layer_min = Estimate::size_of_double_array(hidden[0] * min_feature_dim);
                let first_layer_max = Estimate::size_of_double_array(hidden[0] * max_feature_dim);
                let first_bias = Estimate::size_of_double_array(hidden[0]);

                let mut fixed_bytes = first_bias;
                for i in 0..hidden.len().saturating_sub(1) {
                    fixed_bytes += Estimate::size_of_double_array(hidden[i + 1] * hidden[i]);
                    fixed_bytes += Estimate::size_of_double_array(hidden[i + 1]);
                }

                let output_weights =
                    Estimate::size_of_double_array(_number_of_classes * hidden[hidden.len() - 1]);
                let output_bias = Estimate::size_of_double_array(_number_of_classes);
                fixed_bytes = fixed_bytes.saturating_add(output_weights + output_bias);

                MemoryEstimations::of_range(
                    "MLPClassifierData",
                    MemoryRange::of_range(
                        first_layer_min.saturating_add(fixed_bytes),
                        first_layer_max.saturating_add(fixed_bytes),
                    ),
                )
            }
            _ => panic!(
                "No such classifier for training method: {:?}",
                trainer_config.method()
            ),
        }
    }
}
