use crate::concurrency::Concurrency;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::TaskProgressTracker;
use crate::ml::models::linear_regression::{LinearRegressionTrainConfig, LinearRegressionTrainer};
use crate::ml::models::random_forest::{
    RandomForestRegressorTrainer, RandomForestRegressorTrainerConfig,
};
use crate::ml::models::{base::TrainerConfigTrait, RegressorTrainer, TrainingMethod};
use crate::core::LogLevel as ProcedureLogLevel;
use parking_lot::RwLock;
use std::sync::Arc;

/// Factory for creating regression trainers from configuration.
/// 1:1 translation of RegressionTrainerFactory.java from Java GDS.
pub struct RegressionTrainerFactory;

impl RegressionTrainerFactory {
    /// Create a regression trainer from configuration.
    /// 1:1 with RegressionTrainerFactory.create() in Java
    pub fn create(
        config: &dyn TrainerConfigTrait,
        termination_flag: &TerminationFlag,
        progress_tracker: TaskProgressTracker,
        concurrency: &Concurrency,
        random_seed: Option<u64>,
    ) -> Box<dyn RegressorTrainer> {
        match config.method() {
            TrainingMethod::LinearRegression => {
                // Downcast to LinearRegressionTrainConfig
                let linear_config = (config as &dyn std::any::Any)
                    .downcast_ref::<LinearRegressionTrainConfig>()
                    .expect("Invalid config type for LinearRegression");
                // Create dummy termination flag for now (not used in training)
                let dummy_termination = Arc::new(RwLock::new(false));
                Box::new(LinearRegressionTrainer::new(
                    concurrency.value(),
                    linear_config.clone(),
                    dummy_termination,
                ))
            }
            TrainingMethod::RandomForestRegression => {
                // Downcast to RandomForestRegressorTrainerConfig
                let rf_config = (config as &dyn std::any::Any)
                    .downcast_ref::<RandomForestRegressorTrainerConfig>()
                    .expect("Invalid config type for RandomForestRegression");
                Box::new(RandomForestRegressorTrainer::new(
                    *concurrency,
                    rf_config.clone(),
                    random_seed,
                    termination_flag.clone(),
                    progress_tracker.clone(),
                    ProcedureLogLevel::Info, // Default log level
                ))
            }
            _ => panic!(
                "No such training method for regression: {:?}",
                config.method()
            ),
        }
    }
}
