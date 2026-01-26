use crate::collections::BitSet;
use crate::collections::HugeDoubleArray;
use crate::concurrency::Concurrency;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::tasks::LogLevel;
use crate::core::utils::progress::tasks::ProgressTracker;
use crate::core::utils::progress::tasks::TaskProgressTracker;
use crate::mem::Estimate;
use crate::mem::MemoryEstimation;
use crate::mem::MemoryEstimations;
use crate::mem::MemoryRange;
use crate::ml::decision_tree::DecisionTreeRegressorTrainer;
use crate::ml::decision_tree::DecisionTreeTrainer;
use crate::ml::decision_tree::DecisionTreeTrainerConfig;
use crate::ml::decision_tree::FeatureBagger;
use crate::ml::decision_tree::TreeNode;
use crate::ml::models::random_forest::DatasetBootstrapper;
use crate::ml::models::random_forest::RandomForestRegressor;
use crate::ml::models::random_forest::RandomForestRegressorData;
use crate::ml::models::random_forest::RandomForestRegressorTrainerConfig;
use crate::ml::models::Features;
use crate::ml::models::Regressor;
use crate::ml::models::RegressorTrainer;
use crate::core::LogLevel as ProcedureLogLevel;
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
        _message_log_level: ProcedureLogLevel,
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
            self.termination_flag.assert_running();
            // Bootstrap sample for this tree
            let mut rng: rand::rngs::StdRng = if let Some(seed) = self.random_seed {
                rand::rngs::StdRng::seed_from_u64(seed + tree_idx as u64)
            } else {
                rand::rngs::StdRng::from_entropy()
            };
            let mut bootstrapped_indices = BitSet::new(train_set.len());
            let bootstrap_sample = if self.config.forest.num_samples_ratio == 0.0 {
                for idx in 0..train_set.len() {
                    bootstrapped_indices.set(idx);
                }
                Arc::new(train_set.to_vec())
            } else {
                let train_set_array = Arc::new(train_set.to_vec());
                DatasetBootstrapper::bootstrap(
                    &mut rng,
                    self.config.forest.num_samples_ratio,
                    &train_set_array,
                    &mut bootstrapped_indices,
                )
            };

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
            decision_trees.push(Box::new(tree));

            let mut progress_tracker = self.progress_tracker.clone();
            progress_tracker.log_message(
                LogLevel::Info,
                &format!(
                    "Trained decision tree {} out of {}",
                    tree_idx + 1,
                    number_of_trees
                ),
            );
        }

        RandomForestRegressorData {
            decision_trees,
            num_features: features.feature_dimension(),
        }
    }

    /// Memory estimation for training, mirroring Java's RandomForestRegressorTrainer.memoryEstimation.
    pub fn memory_estimation(
        number_of_training_samples: impl Fn(u64) -> u64 + Send + Sync + 'static,
        feature_dimension: MemoryRange,
        config: &RandomForestRegressorTrainerConfig,
    ) -> Box<dyn MemoryEstimation> {
        let config = Arc::new(config.clone());
        let number_of_training_samples = Arc::new(number_of_training_samples);
        let min_bagged_features = (config.forest.max_features_ratio(feature_dimension.min())
            * feature_dimension.min() as f64)
            .ceil() as usize;
        let max_bagged_features = (config.forest.max_features_ratio(feature_dimension.max())
            * feature_dimension.max() as f64)
            .ceil() as usize;

        let model_data = MemoryEstimations::builder("Decision trees")
            .range_per_graph_dimension("Decision trees", {
                let config = Arc::clone(&config);
                let number_of_training_samples = Arc::clone(&number_of_training_samples);
                move |dim, _| {
                    let tree_bytes =
                        <DecisionTreeRegressorTrainer as DecisionTreeTrainer<f64>>::estimate_tree(
                            &DecisionTreeTrainerConfig::builder()
                                .max_depth(config.forest.max_depth)
                                .min_split_size(config.forest.min_samples_split)
                                .min_leaf_size(config.forest.min_samples_leaf)
                                .build()
                                .expect("Invalid decision tree config"),
                            number_of_training_samples(dim.node_count() as u64) as usize,
                            TreeNode::<f64>::leaf_memory_estimation(),
                            0,
                        );
                    MemoryRange::of(tree_bytes.saturating_mul(config.forest.num_decision_trees))
                }
            })
            .build();

        MemoryEstimations::builder("Training")
            .add(model_data)
            .range_per_graph_dimension("Decision tree training", {
                let config = Arc::clone(&config);
                let number_of_training_samples = Arc::clone(&number_of_training_samples);
                move |dim, conc| {
                    let samples = number_of_training_samples(dim.node_count() as u64) as usize;
                    let used_samples =
                        (config.forest.num_samples_ratio * samples as f64).ceil() as usize;

                    let bootstrapped = MemoryRange::of(
                        Estimate::size_of_long_array(used_samples)
                            + Estimate::size_of_bitset(used_samples),
                    );

                    let bagger_min = FeatureBagger::memory_estimation(min_bagged_features);
                    let bagger_max = FeatureBagger::memory_estimation(max_bagged_features);

                    let per_tree_min = bagger_min.min()
                        + DecisionTreeRegressorTrainer::memory_estimation(
                            &DecisionTreeTrainerConfig::builder()
                                .max_depth(config.forest.max_depth)
                                .min_split_size(config.forest.min_samples_split)
                                .min_leaf_size(config.forest.min_samples_leaf)
                                .build()
                                .expect("Invalid decision tree config"),
                            used_samples,
                        )
                        + bootstrapped.min();

                    let per_tree_max = bagger_max.max()
                        + DecisionTreeRegressorTrainer::memory_estimation(
                            &DecisionTreeTrainerConfig::builder()
                                .max_depth(config.forest.max_depth)
                                .min_split_size(config.forest.min_samples_split)
                                .min_leaf_size(config.forest.min_samples_leaf)
                                .build()
                                .expect("Invalid decision tree config"),
                            used_samples,
                        )
                        + bootstrapped.max();

                    MemoryRange::of_range(per_tree_min, per_tree_max).times(conc)
                }
            })
            .build()
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
