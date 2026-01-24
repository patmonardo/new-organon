use crate::collections::BitSet;
use crate::collections::HugeIntArray;
use crate::concurrency::Concurrency;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::tasks::LogLevel;
use crate::core::utils::progress::tasks::ProgressTracker;
use crate::core::utils::progress::tasks::TaskProgressTracker;
use crate::mem::Estimate;
use crate::mem::MemoryEstimation;
use crate::mem::MemoryEstimations;
use crate::mem::MemoryRange;
use crate::ml::decision_tree::ClassifierImpurityCriterionType;
use crate::ml::decision_tree::DecisionTreeClassifierTrainer;
use crate::ml::decision_tree::DecisionTreeTrainer;
use crate::ml::decision_tree::DecisionTreeTrainerConfig;
use crate::ml::decision_tree::Entropy;
use crate::ml::decision_tree::FeatureBagger;
use crate::ml::decision_tree::GiniIndex;
use crate::ml::decision_tree::ImpurityCriterion;
use crate::ml::decision_tree::TreeNode;
use crate::ml::metrics::ModelSpecificMetricsHandler;
use crate::ml::metrics::OutOfBagError;
use crate::ml::models::random_forest::DatasetBootstrapper;
use crate::ml::models::random_forest::RandomForestClassifier;
use crate::ml::models::random_forest::RandomForestClassifierData;
use crate::ml::models::random_forest::RandomForestClassifierTrainerConfig;
use crate::ml::models::Classifier;
use crate::ml::models::ClassifierTrainer;
use crate::ml::models::Features;
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

    /// Memory estimation for training, mirroring Java's RandomForestClassifierTrainer.memoryEstimation.
    pub fn memory_estimation(
        number_of_training_samples: impl Fn(u64) -> u64 + Send + Sync + 'static,
        number_of_classes: usize,
        feature_dimension: MemoryRange,
        config: &RandomForestClassifierTrainerConfig,
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
                    let tree_bytes = <DecisionTreeClassifierTrainer as DecisionTreeTrainer<
                        usize,
                    >>::estimate_tree(
                        &DecisionTreeTrainerConfig::builder()
                            .max_depth(config.forest.max_depth)
                            .min_split_size(config.forest.min_samples_split)
                            .min_leaf_size(config.forest.min_samples_leaf)
                            .build()
                            .expect("Invalid decision tree config"),
                        number_of_training_samples(dim.node_count() as u64) as usize,
                        TreeNode::<usize>::leaf_memory_estimation(),
                        match config.criterion {
                            ClassifierImpurityCriterionType::Gini => GiniIndex::memory_estimation(
                                number_of_training_samples(dim.node_count() as u64) as usize,
                            ),
                            ClassifierImpurityCriterionType::Entropy => Entropy::memory_estimation(
                                number_of_training_samples(dim.node_count() as u64) as usize,
                            ),
                        },
                    );
                    MemoryRange::of(tree_bytes.saturating_mul(config.forest.num_decision_trees))
                }
            })
            .build();

        MemoryEstimations::builder("Training")
            .add(model_data)
            .range_per_graph_dimension("Impurity computation data", {
                let config = Arc::clone(&config);
                let number_of_training_samples = Arc::clone(&number_of_training_samples);
                move |dim, _| {
                    let samples = number_of_training_samples(dim.node_count() as u64) as usize;
                    let bytes = match config.criterion {
                        ClassifierImpurityCriterionType::Gini => {
                            GiniIndex::memory_estimation(samples)
                        }
                        ClassifierImpurityCriterionType::Entropy => {
                            Entropy::memory_estimation(samples)
                        }
                    };
                    MemoryRange::of(bytes)
                }
            })
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
                        + DecisionTreeClassifierTrainer::memory_estimation(
                            &DecisionTreeTrainerConfig::builder()
                                .max_depth(config.forest.max_depth)
                                .min_split_size(config.forest.min_samples_split)
                                .min_leaf_size(config.forest.min_samples_leaf)
                                .build()
                                .expect("Invalid decision tree config"),
                            used_samples,
                            number_of_classes,
                        )
                        + bootstrapped.min();

                    let per_tree_max = bagger_max.max()
                        + DecisionTreeClassifierTrainer::memory_estimation(
                            &DecisionTreeTrainerConfig::builder()
                                .max_depth(config.forest.max_depth)
                                .min_split_size(config.forest.min_samples_split)
                                .min_leaf_size(config.forest.min_samples_leaf)
                                .build()
                                .expect("Invalid decision tree config"),
                            used_samples,
                            number_of_classes,
                        )
                        + bootstrapped.max();

                    MemoryRange::of_range(per_tree_min, per_tree_max).times(conc)
                }
            })
            .build()
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
        let oob_metric = OutOfBagError::new();
        let track_oob = self.metrics_handler.is_requested(&oob_metric);
        let mut oob_votes = if track_oob {
            vec![0u32; train_set.len() * self.number_of_classes]
        } else {
            Vec::new()
        };
        let mut oob_counts = if track_oob {
            vec![0u32; train_set.len()]
        } else {
            Vec::new()
        };

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

            // Create impurity criterion
            let impurity_criterion = match self.config.criterion {
                ClassifierImpurityCriterionType::Gini => Box::new(GiniIndex::new(
                    Arc::new(labels.clone()),
                    self.number_of_classes,
                ))
                    as Box<dyn ImpurityCriterion>,
                ClassifierImpurityCriterionType::Entropy => Box::new(Entropy::new(
                    Arc::new(labels.clone()),
                    self.number_of_classes,
                ))
                    as Box<dyn ImpurityCriterion>,
            };

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
            if track_oob {
                for (train_idx, &node_id) in train_set.iter().enumerate() {
                    if !bootstrapped_indices.get(train_idx) {
                        let predicted_class = *tree.predict(features.get(node_id as usize));
                        if predicted_class < self.number_of_classes {
                            let vote_index = train_idx * self.number_of_classes + predicted_class;
                            oob_votes[vote_index] += 1;
                            oob_counts[train_idx] += 1;
                        }
                    }
                }
            }
            decision_trees.push(Arc::new(tree));

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

        if track_oob {
            let mut oob_samples = 0usize;
            let mut oob_errors = 0usize;
            for (train_idx, &count) in oob_counts.iter().enumerate() {
                if count == 0 {
                    continue;
                }
                oob_samples += 1;
                let base = train_idx * self.number_of_classes;
                let mut best_class = 0usize;
                let mut best_votes = oob_votes[base];
                for class in 1..self.number_of_classes {
                    let votes = oob_votes[base + class];
                    if votes > best_votes {
                        best_votes = votes;
                        best_class = class;
                    }
                }

                let node_id = train_set[train_idx] as usize;
                let target_class = labels.get(node_id) as usize;
                if best_class != target_class {
                    oob_errors += 1;
                }
            }

            let oob_error = if oob_samples == 0 {
                0.0
            } else {
                oob_errors as f64 / oob_samples as f64
            };
            self.metrics_handler.handle(&oob_metric, oob_error);
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
