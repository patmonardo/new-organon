//! Decision tree classifier trainer.
//!
//! Translated from Java GDS ml-algo DecisionTreeClassifierTrainer.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::collections::HugeIntArray;
use crate::mem::Estimate;
use crate::ml::decision_tree::{
    DecisionTreeTrainer, DecisionTreeTrainerConfig, FeatureBagger, GiniImpurityData, Group,
    ImpurityCriterion, TreeNode,
};
use crate::ml::models::Features;
use std::sync::Arc;

pub struct DecisionTreeClassifierTrainer<'a> {
    impurity_criterion: Box<dyn ImpurityCriterion>,
    features: &'a dyn Features,
    all_labels: Arc<HugeIntArray>,
    number_of_classes: usize,
    config: DecisionTreeTrainerConfig,
    feature_bagger: FeatureBagger,
}

impl<'a> DecisionTreeClassifierTrainer<'a> {
    pub fn new(
        impurity_criterion: Box<dyn ImpurityCriterion>,
        features: &'a dyn Features,
        labels: Arc<HugeIntArray>,
        number_of_classes: usize,
        config: DecisionTreeTrainerConfig,
        feature_bagger: FeatureBagger,
    ) -> Self {
        assert_eq!(labels.size(), features.size());
        Self {
            impurity_criterion,
            features,
            all_labels: labels,
            number_of_classes,
            config,
            feature_bagger,
        }
    }

    pub fn memory_estimation(
        config: &DecisionTreeTrainerConfig,
        number_of_training_samples: usize,
        number_of_classes: usize,
    ) -> usize {
        Estimate::size_of_instance("DecisionTreeClassifierTrainer")
            + Self::estimate_tree(
                config,
                number_of_training_samples,
                TreeNode::<usize>::leaf_memory_estimation(),
                GiniImpurityData::memory_estimation(number_of_classes),
            )
            + Estimate::size_of_long_array(number_of_classes)
    }

    fn to_terminal(&self, group: &Group) -> usize {
        let mut classes_in_group = vec![0i64; self.number_of_classes];
        let array = group.array();

        for i in group.start_idx()..(group.start_idx() + group.size()) {
            let label = self.all_labels.get(array.get(i) as usize) as usize;
            classes_in_group[label] += 1;
        }

        let mut max_class_count_in_group = -1i64;
        let mut max_mapped_class = 0;
        for (i, &count) in classes_in_group.iter().enumerate() {
            if count <= max_class_count_in_group {
                continue;
            }
            max_class_count_in_group = count;
            max_mapped_class = i;
        }

        max_mapped_class
    }
}

impl<'a> DecisionTreeTrainer<usize> for DecisionTreeClassifierTrainer<'a> {
    fn impurity_criterion(&self) -> Box<dyn ImpurityCriterion> {
        self.impurity_criterion.clone()
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

    fn to_terminal(&self, group: &Group) -> usize {
        self.to_terminal(group)
    }
}
