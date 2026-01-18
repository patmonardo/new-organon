//! Gini index impurity criterion for classification.
//!
//! Translated from Java GDS ml-algo GiniIndex.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::collections::{HugeIntArray, HugeLongArray};
use crate::mem::Estimate;
use crate::ml::decision_tree::{ImpurityCriterion, ImpurityData, ImpurityDataAny};
use std::any::Any;
use std::sync::Arc;

#[derive(Clone)]
pub struct GiniIndex {
    expected_mapped_labels: Arc<HugeIntArray>,
    number_of_classes: usize,
}

impl GiniIndex {
    pub fn new(expected_mapped_labels: Arc<HugeIntArray>, number_of_classes: usize) -> Self {
        Self {
            expected_mapped_labels,
            number_of_classes,
        }
    }

    pub fn memory_estimation(number_of_training_samples: usize) -> usize {
        HugeIntArray::memory_estimation(number_of_training_samples)
            + Estimate::size_of_instance("GiniIndex")
    }
}

impl ImpurityCriterion for GiniIndex {
    fn group_impurity(
        &self,
        group: &HugeLongArray,
        start_index: usize,
        size: usize,
    ) -> Box<dyn ImpurityData> {
        if size == 0 {
            return Box::new(GiniImpurityData::new(
                0.0,
                vec![0; self.number_of_classes],
                size,
            ));
        }

        let mut group_class_counts = vec![0i64; self.number_of_classes];
        for i in start_index..(start_index + size) {
            let expected_label = self.expected_mapped_labels.get(group.get(i) as usize) as usize;
            group_class_counts[expected_label] += 1;
        }

        let mut sum_of_squares = 0i64;
        for &count in &group_class_counts {
            sum_of_squares += count * count;
        }
        let impurity = 1.0 - (sum_of_squares as f64) / ((size * size) as f64);

        Box::new(GiniImpurityData::new(impurity, group_class_counts, size))
    }

    fn incremental_impurity(
        &self,
        feature_vector_idx: usize,
        impurity_data: &mut dyn ImpurityData,
    ) {
        let gini_impurity_data = impurity_data
            .as_any_mut()
            .downcast_mut::<GiniImpurityData>()
            .expect("Expected GiniImpurityData");

        let label = self.expected_mapped_labels.get(feature_vector_idx) as usize;
        let new_class_count = gini_impurity_data.class_counts[label] + 1;
        let new_group_size = gini_impurity_data.group_size + 1;

        Self::update_impurity_data(label, new_group_size, new_class_count, gini_impurity_data);
    }

    fn decremental_impurity(
        &self,
        feature_vector_idx: usize,
        impurity_data: &mut dyn ImpurityData,
    ) {
        let gini_impurity_data = impurity_data
            .as_any_mut()
            .downcast_mut::<GiniImpurityData>()
            .expect("Expected GiniImpurityData");

        let label = self.expected_mapped_labels.get(feature_vector_idx) as usize;
        let new_class_count = gini_impurity_data.class_counts[label] - 1;
        let new_group_size = gini_impurity_data.group_size - 1;

        Self::update_impurity_data(label, new_group_size, new_class_count, gini_impurity_data);
    }

    fn clone_box(&self) -> Box<dyn ImpurityCriterion> {
        Box::new(self.clone())
    }
}

impl GiniIndex {
    fn update_impurity_data(
        label: usize,
        new_group_size: usize,
        new_class_count: i64,
        impurity_data: &mut GiniImpurityData,
    ) {
        if new_group_size == 0 {
            impurity_data.class_counts[label] = new_class_count;
            impurity_data.group_size = 0;
            impurity_data.impurity = 0.0;
            return;
        }

        let group_size_squared = (impurity_data.group_size * impurity_data.group_size) as f64;
        let new_group_size_squared = (new_group_size * new_group_size) as f64;
        let prev_class_count = impurity_data.class_counts[label];

        let mut new_impurity = impurity_data.impurity;
        new_impurity *= group_size_squared / new_group_size_squared;
        new_impurity += 1.0 - (group_size_squared / new_group_size_squared);
        new_impurity += (prev_class_count * prev_class_count) as f64 / new_group_size_squared;
        new_impurity -= (new_class_count * new_class_count) as f64 / new_group_size_squared;

        impurity_data.class_counts[label] = new_class_count;
        impurity_data.group_size = new_group_size;
        impurity_data.impurity = new_impurity;
    }
}

pub struct GiniImpurityData {
    impurity: f64,
    class_counts: Vec<i64>,
    group_size: usize,
}

impl GiniImpurityData {
    pub fn new(impurity: f64, class_counts: Vec<i64>, group_size: usize) -> Self {
        Self {
            impurity,
            class_counts,
            group_size,
        }
    }

    pub fn memory_estimation(number_of_classes: usize) -> usize {
        Estimate::size_of_instance("GiniImpurityData")
            + Estimate::size_of_long_array(number_of_classes)
    }
}

impl ImpurityData for GiniImpurityData {
    fn impurity(&self) -> f64 {
        self.impurity
    }

    fn group_size(&self) -> usize {
        self.group_size
    }

    fn copy_to(&self, target: &mut dyn ImpurityData) {
        let gini_target = target
            .as_any_mut()
            .downcast_mut::<GiniImpurityData>()
            .expect("Expected GiniImpurityData");
        gini_target.impurity = self.impurity;
        gini_target.group_size = self.group_size;
        gini_target.class_counts.copy_from_slice(&self.class_counts);
    }
}

impl ImpurityDataAny for GiniImpurityData {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
