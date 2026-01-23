//! Machine Learning models module
//!
//! This module contains core ML model traits and implementations.
//! The hierarchy is:
//!
//! - Base traits (Classifier, Regressor, BaseModelData, ClassifierData, RegressorData)
//! - Training method enum
//! - Feature system (Features trait and implementations)
//! - Trainer config system
//! - Model implementations by type:
//!   - Linear models (linear/logistic regression)
//!   - Tree-based models (random forests)
//!   - Neural networks (MLP)
//! - AutoML system for hyperparameter optimization

pub mod automl;
pub mod base;
pub mod classifier_factory;
pub mod classifier_trainer_factory;
pub mod config;
pub mod features;
pub mod linear_regression;
pub mod logistic_regression;
pub mod mlp;
pub mod regression_trainer_factory;
pub mod training_method;
pub mod trees;

// Core traits - 1:1 with Java GDS
pub use base::{
    BaseModelData, Classifier, ClassifierData, ClassifierTrainer, Features, ModelData, Regressor,
    RegressorData, RegressorTrainer,
};
pub use classifier_factory::ClassifierFactory;
pub use classifier_trainer_factory::ClassifierTrainerFactory;
pub use config::{BaseTrainerConfig, ClassAwareTrainerConfig, PenaltyConfig, TrainerConfig};
pub use features::{DenseFeatures, FeaturesFactory, LazyFeatures};
pub use regression_trainer_factory::RegressionTrainerFactory;
pub use training_method::TrainingMethod;

// Model implementations
pub use automl::*;
pub use linear_regression::{LinearRegressionData, LinearRegressor};
pub use logistic_regression::{LogisticRegressionClassifier, LogisticRegressionData};
pub use mlp::{
    MLPClassifier, MLPClassifierData, MLPClassifierObjective, MLPClassifierTrainConfig,
    MLPClassifierTrainer,
};
pub use trees::{
    DatasetBootstrapper, DecisionTreePredictor, RandomForestClassifier,
    RandomForestClassifierConfig, RandomForestClassifierData, RandomForestClassifierTrainer,
    RandomForestClassifierTrainerConfig, RandomForestConfig, RandomForestRegressor,
    RandomForestRegressorConfig, RandomForestRegressorData, RandomForestRegressorTrainer,
    RandomForestRegressorTrainerConfig,
};
