use crate::concurrency::Concurrency;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::{ProgressTracker, TaskProgressTracker, Tasks};
use crate::ml::metrics::ModelSpecificMetricsHandler;
use crate::ml::models::logistic_regression::{
    LogisticRegressionTrainConfig, LogisticRegressionTrainer,
};
use crate::ml::models::mlp::{MLPClassifierTrainConfig, MLPClassifierTrainer};
use crate::ml::models::random_forest::{
    RandomForestClassifierTrainer, RandomForestClassifierTrainerConfig,
};
use crate::ml::models::{base::TrainerConfigTrait, ClassifierTrainer, TrainingMethod};
use parking_lot::RwLock;
use std::sync::Arc;

/// Factory for creating classifier trainers from configuration.
/// 1:1 translation of ClassifierTrainerFactory.java from Java GDS.
pub struct ClassifierTrainerFactory;

impl ClassifierTrainerFactory {
    /// Create a classifier trainer from configuration.
    /// 1:1 with ClassifierTrainerFactory.create() in Java
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        config: &dyn TrainerConfigTrait,
        number_of_classes: usize,
        _termination_flag: &TerminationFlag,
        _progress_tracker: &dyn ProgressTracker,
        concurrency: &Concurrency,
        random_seed: Option<u64>,
        _reduce_class_count: bool,
        metrics_handler: &ModelSpecificMetricsHandler,
    ) -> Box<dyn ClassifierTrainer> {
        match config.method() {
            TrainingMethod::LogisticRegression => {
                let lr_config = (config as &dyn std::any::Any)
                    .downcast_ref::<LogisticRegressionTrainConfig>()
                    .expect("Invalid config type for LogisticRegression");
                let dummy_termination = Arc::new(RwLock::new(false));
                Box::new(LogisticRegressionTrainer::new(
                    lr_config.clone(),
                    number_of_classes,
                    _reduce_class_count,
                    dummy_termination,
                    concurrency.value(),
                ))
            }
            TrainingMethod::RandomForestClassification => {
                // In Java: new RandomForestClassifierTrainer(...)
                let rf_config = (config as &dyn std::any::Any)
                    .downcast_ref::<RandomForestClassifierTrainerConfig>()
                    .expect("Invalid config type for RandomForestClassification");

                let base_task = Tasks::leaf("RandomForestClassifierTrainer".to_string());
                let progress_tracker =
                    TaskProgressTracker::with_concurrency(base_task, concurrency.value());

                Box::new(RandomForestClassifierTrainer::new(
                    *concurrency,
                    number_of_classes,
                    rf_config.clone(),
                    random_seed,
                    progress_tracker,
                    _termination_flag.clone(),
                    Arc::new(metrics_handler.clone()),
                ))
            }
            TrainingMethod::MLPClassification => {
                // In Java: new MLPClassifierTrainer(numberOfClasses, (MLPClassifierTrainConfig) config, randomSeed, ...)
                let mlp_config = (config as &dyn std::any::Any)
                    .downcast_ref::<MLPClassifierTrainConfig>()
                    .expect("Invalid config type for MLPClassification");
                Box::new(MLPClassifierTrainer::new(
                    number_of_classes,
                    mlp_config.clone(),
                    random_seed,
                    concurrency.value(),
                ))
            }
            _ => panic!(
                "No such training method for classifier: {:?}",
                config.method()
            ),
        }
    }
}
