//! Random Forest Classifier Trainer implementation.
//!
//! 1:1 translation of RandomForestClassifierTrainer.java from Java GDS.

use crate::collections::HugeIntArray;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::TaskProgressTracker;
use crate::ml::decision_tree::{
    DecisionTreeClassifierTrainer, DecisionTreeTrainer, DecisionTreeTrainerConfig, FeatureBagger,
    GiniIndex,
};
use crate::ml::metrics::ModelSpecificMetricsHandler;
use crate::ml::models::trees::{
    DatasetBootstrapper, RandomForestClassifier, RandomForestClassifierData,
    RandomForestClassifierTrainerConfig,
};
use crate::ml::models::{Classifier, ClassifierTrainer, Features};
use rand::SeedableRng;
use std::sync::Arc;

/// Random Forest Classifier Trainer.
/// 1:1 translation of RandomForestClassifierTrainer.java from Java GDS.
#[allow(dead_code)]
pub struct RandomForestClassifierTrainer {
    number_of_classes: usize,
    config: RandomForestClassifierTrainerConfig,
    concurrency: Concurrency,
    random_seed: Option<u64>,
    progress_tracker: TaskProgressTracker,
    termination_flag: TerminationFlag,
    metrics_handler: Arc<ModelSpecificMetricsHandler>,
}

impl RandomForestClassifierTrainer {
    /// Create a new Random Forest Classifier Trainer.
    /// 1:1 with RandomForestClassifierTrainer constructor in Java.
    pub fn new(
        concurrency: Concurrency,
        number_of_classes: usize,
        config: RandomForestClassifierTrainerConfig,
        random_seed: Option<u64>,
        progress_tracker: TaskProgressTracker,
        termination_flag: TerminationFlag,
        metrics_handler: Arc<ModelSpecificMetricsHandler>,
    ) -> Self {
        Self {
            number_of_classes,
            config,
            concurrency,
            random_seed,
            progress_tracker,
            termination_flag,
            metrics_handler,
        }
    }

    /// Train the random forest classifier.
    /// 1:1 with train() method in Java.
    fn train_internal(
        &self,
        features: &dyn Features,
        labels: &HugeIntArray,
        train_set: &[u64],
    ) -> RandomForestClassifierData {
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

            // Create Gini impurity criterion
            let impurity_criterion = Box::new(GiniIndex::new(
                Arc::new(labels.clone()),
                self.number_of_classes,
            ));

            // Create and train decision tree
            let mut tree_trainer = DecisionTreeClassifierTrainer::new(
                impurity_criterion,
                features,
                Arc::new(labels.clone()),
                self.number_of_classes,
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
                Box::new(tree) as Box<dyn crate::ml::models::trees::DecisionTreePredictor<usize>>
            );
        }

        RandomForestClassifierData {
            decision_trees,
            num_classes: self.number_of_classes,
            num_features: features.feature_dimension(),
        }
    }
}

impl ClassifierTrainer for RandomForestClassifierTrainer {
    /// Train a random forest classifier.
    /// 1:1 with ClassifierTrainer.train() in Java.
    fn train(
        &self,
        features: &dyn Features,
        labels: &HugeIntArray,
        train_set: &Arc<Vec<u64>>,
    ) -> Box<dyn Classifier> {
        let data = self.train_internal(features, labels, train_set);
        Box::new(RandomForestClassifier::new(data))
    }
}
