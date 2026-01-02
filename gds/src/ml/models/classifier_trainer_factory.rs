//! Classifier Trainer Factory - 1:1 translation of ClassifierTrainerFactory.java from Java GDS

use crate::concurrency::Concurrency;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::ml::metrics::ModelSpecificMetricsHandler;
use crate::ml::models::neural::{MLPClassifierTrainConfig, MLPClassifierTrainer};
use crate::ml::models::{base::TrainerConfigTrait, ClassifierTrainer, TrainingMethod};

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
        _progress_tracker: &ProgressTracker,
        concurrency: &Concurrency,
        random_seed: Option<u64>,
        _reduce_class_count: bool,
        _metrics_handler: &ModelSpecificMetricsHandler,
    ) -> Box<dyn ClassifierTrainer> {
        match config.method() {
            TrainingMethod::LogisticRegression => {
                panic!("ClassifierTrainerFactory::create: LogisticRegression trainer not yet implemented")
            }
            TrainingMethod::RandomForestClassification => {
                // In Java: new RandomForestClassifierTrainer(...)
                panic!("ClassifierTrainerFactory::create: RandomForestClassification trainer not yet implemented")
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
