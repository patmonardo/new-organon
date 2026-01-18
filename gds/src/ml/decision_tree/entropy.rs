//! Entropy impurity criterion for classification.
//!
//! Translated from Java GDS ml-algo Entropy.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::collections::{HugeIntArray, HugeLongArray};
use crate::mem::Estimate;
use crate::ml::decision_tree::{ImpurityCriterion, ImpurityData, ImpurityDataAny};
use std::any::Any;
use std::sync::Arc;

const LN_2: f64 = std::f64::consts::LN_2;

#[derive(Clone)]
pub struct Entropy {
    expected_mapped_labels: Arc<HugeIntArray>,
    number_of_classes: usize,
}

impl Entropy {
    pub fn new(expected_mapped_labels: Arc<HugeIntArray>, number_of_classes: usize) -> Self {
        Self {
            expected_mapped_labels,
            number_of_classes,
        }
    }

    pub fn memory_estimation(number_of_training_samples: usize) -> usize {
        HugeIntArray::memory_estimation(number_of_training_samples)
            + Estimate::size_of_instance("Entropy")
    }
}

impl ImpurityCriterion for Entropy {
    fn group_impurity(
        &self,
        group: &HugeLongArray,
        start_index: usize,
        size: usize,
    ) -> Box<dyn ImpurityData> {
        if size == 0 {
            return Box::new(EntropyImpurityData::new(
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

        let mut impurity = 0.0;
        for &count in &group_class_counts {
            if count == 0 {
                continue;
            }
            let p = count as f64 / size as f64;
            impurity -= p * p.ln();
        }
        impurity /= LN_2;

        Box::new(EntropyImpurityData::new(impurity, group_class_counts, size))
    }

    fn incremental_impurity(
        &self,
        feature_vector_idx: usize,
        impurity_data: &mut dyn ImpurityData,
    ) {
        let entropy_impurity_data = impurity_data
            .as_any_mut()
            .downcast_mut::<EntropyImpurityData>()
            .expect("Expected EntropyImpurityData");

        let label = self.expected_mapped_labels.get(feature_vector_idx) as usize;
        let new_class_count = entropy_impurity_data.class_counts[label] + 1;
        let new_group_size = entropy_impurity_data.group_size + 1;

        Self::update_impurity_data(
            label,
            new_group_size,
            new_class_count,
            entropy_impurity_data,
        );
    }

    fn decremental_impurity(
        &self,
        feature_vector_idx: usize,
        impurity_data: &mut dyn ImpurityData,
    ) {
        let entropy_impurity_data = impurity_data
            .as_any_mut()
            .downcast_mut::<EntropyImpurityData>()
            .expect("Expected EntropyImpurityData");

        let label = self.expected_mapped_labels.get(feature_vector_idx) as usize;
        let new_class_count = entropy_impurity_data.class_counts[label] - 1;
        let new_group_size = entropy_impurity_data.group_size - 1;

        Self::update_impurity_data(
            label,
            new_group_size,
            new_class_count,
            entropy_impurity_data,
        );
    }

    fn clone_box(&self) -> Box<dyn ImpurityCriterion> {
        Box::new(self.clone())
    }
}

impl Entropy {
    fn update_impurity_data(
        label: usize,
        new_group_size: usize,
        new_class_count: i64,
        impurity_data: &mut EntropyImpurityData,
    ) {
        let prev_class_count = impurity_data.class_counts[label];

        let mut new_impurity = 0.0;
        if new_group_size > 0 {
            new_impurity = impurity_data.impurity * LN_2;
            if impurity_data.group_size > 0 {
                new_impurity -= (impurity_data.group_size as f64).ln();
                new_impurity *= impurity_data.group_size as f64;
            }
            if prev_class_count > 0 {
                new_impurity += prev_class_count as f64 * (prev_class_count as f64).ln();
            }
            if new_class_count > 0 {
                new_impurity -= new_class_count as f64 * (new_class_count as f64).ln();
            }
            new_impurity /= new_group_size as f64;
            new_impurity += (new_group_size as f64).ln();
            new_impurity /= LN_2;
        }

        impurity_data.class_counts[label] = new_class_count;
        impurity_data.group_size = new_group_size;
        impurity_data.impurity = new_impurity;
    }
}

pub struct EntropyImpurityData {
    impurity: f64,
    class_counts: Vec<i64>,
    group_size: usize,
}

impl EntropyImpurityData {
    pub fn new(impurity: f64, class_counts: Vec<i64>, group_size: usize) -> Self {
        Self {
            impurity,
            class_counts,
            group_size,
        }
    }

    pub fn memory_estimation(number_of_classes: usize) -> usize {
        Estimate::size_of_instance("EntropyImpurityData")
            + Estimate::size_of_long_array(number_of_classes)
    }
}

impl ImpurityData for EntropyImpurityData {
    fn impurity(&self) -> f64 {
        self.impurity
    }

    fn group_size(&self) -> usize {
        self.group_size
    }

    fn copy_to(&self, target: &mut dyn ImpurityData) {
        let entropy_target = target
            .as_any_mut()
            .downcast_mut::<EntropyImpurityData>()
            .expect("Expected EntropyImpurityData");
        entropy_target.impurity = self.impurity;
        entropy_target.group_size = self.group_size;
        entropy_target
            .class_counts
            .copy_from_slice(&self.class_counts);
    }
}

impl ImpurityDataAny for EntropyImpurityData {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
