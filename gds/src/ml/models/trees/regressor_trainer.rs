//! Random Forest Regressor Trainer implementation.
//!
//! 1:1 translation of RandomForestRegressorTrainer.java from Java GDS.

use crate::collections::HugeDoubleArray;
use crate::concurrency::Concurrency;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::TaskProgressTracker;
use crate::ml::decision_tree::{
    DecisionTreeRegressorTrainer, DecisionTreeTrainer, DecisionTreeTrainerConfig, FeatureBagger,
};
use crate::ml::models::trees::{
    DatasetBootstrapper, RandomForestRegressor, RandomForestRegressorData,
    RandomForestRegressorTrainerConfig,
};
use crate::ml::models::{Features, Regressor, RegressorTrainer};
use rand::SeedableRng;
use std::sync::Arc;

/// Random Forest Regressor Trainer.
/// 1:1 translation of RandomForestRegressorTrainer.java from Java GDS.
#[allow(dead_code)]
pub struct RandomForestRegressorTrainer {
    config: RandomForestRegressorTrainerConfig,
    concurrency: Concurrency,
    random_seed: Option<u64>,
    progress_tracker: TaskProgressTracker,
    termination_flag: TerminationFlag,
}

impl RandomForestRegressorTrainer {
    /// Create a new Random Forest Regressor Trainer.
    /// 1:1 with RandomForestRegressorTrainer constructor in Java.
    pub fn new(
        concurrency: Concurrency,
        config: RandomForestRegressorTrainerConfig,
        random_seed: Option<u64>,
        termination_flag: TerminationFlag,
        progress_tracker: TaskProgressTracker,
        _message_log_level: crate::projection::eval::procedure::LogLevel,
    ) -> Self {
        Self {
            config,
            concurrency,
            random_seed,
            progress_tracker,
            termination_flag,
        }
    }

    /// Train the random forest regressor.
    /// 1:1 with train() method in Java.
    fn train_internal(
        &self,
        features: &dyn Features,
        targets: &HugeDoubleArray,
        train_set: &[u64],
    ) -> RandomForestRegressorData {
        let number_of_trees = self.config.forest.num_decision_trees;
        let mut decision_trees = Vec::with_capacity(number_of_trees);

        // Train each decision tree
        for tree_idx in 0..number_of_trees {
            // Bootstrap sample for this tree
            let mut rng: rand::rngs::StdRng = if let Some(seed) = self.random_seed {
                rand::rngs::StdRng::seed_from_u64(seed + tree_idx as u64)
            } else {
                rand::rngs::StdRng::from_entropy()
            };
            let mut bootstrapped_indices = crate::collections::BitSet::new(train_set.len());
            let train_set_array = Arc::new(train_set.to_vec());
            let bootstrap_sample = DatasetBootstrapper::bootstrap(
                &mut rng,
                self.config.forest.num_samples_ratio,
                &train_set_array,
                &mut bootstrapped_indices,
            );

            // Create decision tree trainer config from random forest config
            let tree_config = DecisionTreeTrainerConfig::builder()
                .max_depth(self.config.forest.max_depth)
                .min_split_size(self.config.forest.min_samples_split)
                .min_leaf_size(self.config.forest.min_samples_leaf)
                .build()
                .expect("Invalid decision tree config");

            // Create feature bagger
            let feature_bagger = FeatureBagger::new(
                self.random_seed
                    .map(|s| s + tree_idx as u64)
                    .unwrap_or(tree_idx as u64),
                features.feature_dimension(),
                self.config
                    .forest
                    .max_features_ratio(features.feature_dimension()),
            );

            // Create and train decision tree
            let mut tree_trainer = DecisionTreeRegressorTrainer::new(
                targets.clone(),
                features,
                tree_config,
                feature_bagger,
            );

            let tree = tree_trainer.train(
                &bootstrap_sample
                    .iter()
                    .map(|&x| x as i64)
                    .collect::<Vec<_>>(),
            );
            decision_trees.push(
                Box::new(tree) as Box<dyn crate::ml::models::trees::DecisionTreePredictor<f64>>
            );
        }

        RandomForestRegressorData {
            decision_trees,
            num_features: features.feature_dimension(),
        }
    }
}

impl RegressorTrainer for RandomForestRegressorTrainer {
    /// Train a random forest regressor.
    /// 1:1 with RegressorTrainer.train() in Java.
    fn train(
        &self,
        features: &dyn Features,
        targets: &HugeDoubleArray,
        train_set: &Arc<Vec<u64>>,
    ) -> Box<dyn Regressor> {
        let data = self.train_internal(features, targets, train_set);
        Box::new(RandomForestRegressor::new(data))
    }
}
