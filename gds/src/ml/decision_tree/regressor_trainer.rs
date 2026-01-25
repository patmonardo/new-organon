//! Decision tree regressor trainer.
//!
//! Note: `max_depth == 0` is treated as "unlimited" (no explicit depth cap). This
//! matches the convention used by `RandomForest` configs and is used by the
//! estimation and training logic.
//!
//! Translated from Java GDS ml-algo DecisionTreeRegressorTrainer.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::collections::HugeDoubleArray;
use crate::mem::Estimate;
use crate::ml::decision_tree::{
    DecisionTreeTrainer, DecisionTreeTrainerConfig, FeatureBagger, Group, ImpurityCriterion,
    MSEImpurityData, SplitMeanSquaredError, TreeNode,
};
use crate::ml::decision_tree::{Splitter, StackRecord};
use crate::ml::models::Features;
use std::sync::Arc;

pub struct DecisionTreeRegressorTrainer<'a> {
    targets: Arc<HugeDoubleArray>,
    features: &'a dyn Features,
    config: DecisionTreeTrainerConfig,
    feature_bagger: FeatureBagger,
}

impl<'a> DecisionTreeRegressorTrainer<'a> {
    pub fn new(
        targets: HugeDoubleArray,
        features: &'a dyn Features,
        config: DecisionTreeTrainerConfig,
        feature_bagger: FeatureBagger,
    ) -> Self {
        assert_eq!(targets.size(), features.size());

        Self {
            targets: Arc::new(targets),
            features,
            config,
            feature_bagger,
        }
    }

    pub fn memory_estimation(
        config: &DecisionTreeTrainerConfig,
        number_of_training_samples: usize,
    ) -> usize {
        std::mem::size_of::<Self>()
            + DecisionTreeRegressorTrainer::estimate_tree_static(
                config,
                number_of_training_samples,
                TreeNode::<f64>::leaf_memory_estimation(),
                MSEImpurityData::memory_estimation(),
            )
    }

    // Static helper for memory estimation (trait method can't be called without self)
    fn estimate_tree_static(
        config: &DecisionTreeTrainerConfig,
        number_of_training_samples: usize,
        leaf_node_size_in_bytes: usize,
        size_of_impurity_data: usize,
    ) -> usize {
        let predictor_estimation = Self::estimate_tree_memory_static(
            config,
            number_of_training_samples,
            leaf_node_size_in_bytes,
        );

        let normalized_max_depth =
            if crate::ml::decision_tree::is_unlimited_depth(config.max_depth()) {
                1.max(number_of_training_samples.saturating_sub(config.min_split_size()) + 2)
            } else {
                config.max_depth().min(
                    1.max(number_of_training_samples.saturating_sub(config.min_split_size()) + 2),
                )
            };
        let max_items_on_stack = 2 * normalized_max_depth;
        let max_stack_size = Estimate::size_of_instance("VecDeque")
            + std::mem::size_of::<StackRecord<f64>>() * max_items_on_stack
            + (Estimate::size_of_long_array(number_of_training_samples) / max_items_on_stack)
                * max_items_on_stack;

        let splitter_estimation =
            Splitter::memory_estimation(number_of_training_samples, size_of_impurity_data);

        predictor_estimation + max_stack_size + splitter_estimation
    }

    fn estimate_tree_memory_static(
        config: &DecisionTreeTrainerConfig,
        number_of_training_samples: usize,
        leaf_node_size_in_bytes: usize,
    ) -> usize {
        if number_of_training_samples == 0 {
            return 0;
        }

        let pow_depth = if crate::ml::decision_tree::is_unlimited_depth(config.max_depth()) {
            f64::INFINITY
        } else {
            2.0_f64.powi(config.max_depth() as i32)
        };
        let max_num_leaf_nodes = pow_depth
            .min((number_of_training_samples as f64) / (config.min_leaf_size() as f64))
            .min(2.0 * (number_of_training_samples as f64) / (config.min_split_size() as f64))
            .ceil() as usize;

        Estimate::size_of_instance("DecisionTreePredictor")
            + (1..=max_num_leaf_nodes).sum::<usize>() * leaf_node_size_in_bytes
            + (0..max_num_leaf_nodes.saturating_sub(1)).sum::<usize>()
                * TreeNode::<f64>::split_memory_estimation()
    }
}

impl<'a> DecisionTreeTrainer<f64> for DecisionTreeRegressorTrainer<'a> {
    fn impurity_criterion(&self) -> Box<dyn ImpurityCriterion> {
        Box::new(SplitMeanSquaredError::new(self.targets.clone()))
    }

    fn features(&self) -> &dyn Features {
        self.features
    }

    fn config(&self) -> &DecisionTreeTrainerConfig {
        &self.config
    }

    fn feature_bagger(&self) -> &FeatureBagger {
        &self.feature_bagger
    }

    fn to_terminal(&self, group: &Group) -> f64 {
        let array = group.array();
        let mut sum = 0.0;
        for i in group.start_idx()..(group.start_idx() + group.size()) {
            sum += self.targets.get(array.get(i) as usize);
        }
        sum / group.size() as f64
    }
}
