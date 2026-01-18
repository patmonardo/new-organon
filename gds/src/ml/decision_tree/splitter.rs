//! Splitter for finding optimal decision tree splits.
//!
//! Translated from Java GDS ml-algo Splitter.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::collections::HugeLongArray;
use crate::core::utils::paged::HugeSerialIndirectMergeSort;
use crate::mem::Estimate;
use crate::ml::decision_tree::{
    FeatureBagger, Group, Groups, ImpurityCriterion, ImpurityData, Split,
};
use crate::ml::models::Features;

pub struct Splitter<'a> {
    impurity_criterion: Box<dyn ImpurityCriterion>,
    features: &'a dyn Features,
    feature_bagger: FeatureBagger,
    min_leaf_size: usize,
    sort_cache: HugeLongArray,
    right_impurity_data: Box<dyn ImpurityData>,
}

impl<'a> Splitter<'a> {
    pub fn new(
        train_set_size: usize,
        impurity_criterion: Box<dyn ImpurityCriterion>,
        feature_bagger: FeatureBagger,
        features: &'a dyn Features,
        min_leaf_size: usize,
    ) -> Self {
        let sort_cache = HugeLongArray::new(train_set_size);
        let right_impurity_data = impurity_criterion.group_impurity(&HugeLongArray::new(0), 0, 0);

        Self {
            impurity_criterion,
            features,
            feature_bagger,
            min_leaf_size,
            sort_cache,
            right_impurity_data,
        }
    }

    pub fn memory_estimation(
        number_of_training_samples: usize,
        size_of_impurity_data: usize,
    ) -> usize {
        Estimate::size_of_instance("Splitter")
            + Estimate::size_of_long_array(number_of_training_samples) // sort_cache
            + 4 * size_of_impurity_data
            + 4 * Estimate::size_of_long_array(number_of_training_samples) // child arrays
    }

    pub fn find_best_split(&mut self, group: &Group) -> Split {
        let mut best_idx = -1i32;
        let mut best_value = f64::MAX;
        let mut best_impurity = f64::MAX;
        let mut best_left_group_size = 0usize;

        let mut left_child_array = HugeLongArray::new(group.size());
        let mut right_child_array = HugeLongArray::new(group.size());
        let mut best_left_child_array = HugeLongArray::new(group.size());
        let mut best_right_child_array = HugeLongArray::new(group.size());

        let mut best_left_impurity_data =
            self.impurity_criterion
                .group_impurity(&HugeLongArray::new(0), 0, 0);
        let mut best_right_impurity_data =
            self.impurity_criterion
                .group_impurity(&HugeLongArray::new(0), 0, 0);

        // Initialize right_child_array with all group indices
        for idx in 0..group.size() {
            right_child_array.set(idx, group.array().get(group.start_idx() + idx));
        }
        right_child_array.copy_to(&mut best_right_child_array, group.size());

        let feature_bag = self.feature_bagger.sample();

        for &feature_idx in &feature_bag {
            // Sort by current feature
            self.sort_by_feature(&mut right_child_array, group.size(), feature_idx);

            group
                .impurity_data()
                .copy_to(self.right_impurity_data.as_mut());

            // Move vectors to left until reaching min_leaf_size
            for left_group_size in 1..self.min_leaf_size {
                let splitting_feature_vector_idx =
                    right_child_array.get(left_group_size - 1) as usize;
                left_child_array.set(left_group_size - 1, splitting_feature_vector_idx as i64);
                self.impurity_criterion.decremental_impurity(
                    splitting_feature_vector_idx,
                    self.right_impurity_data.as_mut(),
                );
            }

            let mut left_impurity_data = self.impurity_criterion.group_impurity(
                &left_child_array,
                0,
                self.min_leaf_size - 1,
            );
            let mut found_improvement_with_idx = false;

            // Continue moving and compute combined impurity
            for left_group_size in self.min_leaf_size..=(group.size() - self.min_leaf_size) {
                let splitting_feature_vector_idx =
                    right_child_array.get(left_group_size - 1) as usize;
                left_child_array.set(left_group_size - 1, splitting_feature_vector_idx as i64);

                self.impurity_criterion.incremental_impurity(
                    splitting_feature_vector_idx,
                    left_impurity_data.as_mut(),
                );
                self.impurity_criterion.decremental_impurity(
                    splitting_feature_vector_idx,
                    self.right_impurity_data.as_mut(),
                );

                let combined_impurity = self.impurity_criterion.combined_impurity(
                    left_impurity_data.as_ref(),
                    self.right_impurity_data.as_ref(),
                );

                if combined_impurity < best_impurity {
                    found_improvement_with_idx = true;
                    best_idx = feature_idx as i32;
                    best_value = self.features.get(splitting_feature_vector_idx)[feature_idx];
                    best_impurity = combined_impurity;
                    best_left_group_size = left_group_size;
                    left_impurity_data.copy_to(best_left_impurity_data.as_mut());
                    self.right_impurity_data
                        .copy_to(best_right_impurity_data.as_mut());
                }
            }

            if found_improvement_with_idx {
                std::mem::swap(&mut best_right_child_array, &mut right_child_array);
                std::mem::swap(&mut best_left_child_array, &mut left_child_array);
            }
        }

        Split::new(
            best_idx as usize,
            best_value,
            Groups::new(
                Group::new(
                    best_left_child_array,
                    0,
                    best_left_group_size,
                    best_left_impurity_data,
                ),
                Group::new(
                    best_right_child_array,
                    best_left_group_size,
                    group.size() - best_left_group_size,
                    best_right_impurity_data,
                ),
            ),
        )
    }

    fn sort_by_feature(&mut self, array: &mut HugeLongArray, size: usize, feature_idx: usize) {
        // Use indirect merge sort to sort indices by feature values
        // We need to limit the sort to the actual size we're working with
        let sort_size = size.min(array.size());
        HugeSerialIndirectMergeSort::sort_with_buffer(
            array,
            sort_size,
            |idx| self.features.get(idx as usize)[feature_idx],
            &mut self.sort_cache,
        );
    }
}
