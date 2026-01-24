use super::metric::Metric;
use super::metric::MetricComparator;
use crate::collections::HugeLongArray;
use crate::collections::LongMultiSet;
use crate::ml::core::subgraph::LocalIdMap;

/// Out-of-bag error metric for Random Forest models.
///
/// This metric is model-specific and only applicable to Random Forest training.
#[derive(Debug, Clone, Default)]
pub struct OutOfBagError;

impl OutOfBagError {
    pub const NAME: &'static str = "OUT_OF_BAG_ERROR";

    pub fn new() -> Self {
        Self
    }
}

impl Metric for OutOfBagError {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn comparator(&self) -> MetricComparator {
        MetricComparator::Natural
    }

    fn is_model_specific(&self) -> bool {
        true
    }
}

const EPSILON: f64 = 1e-8;

pub trait ClassificationMetric: Metric {
    fn compute(&self, targets: &HugeLongArray, predictions: &HugeLongArray) -> f64;
}

#[derive(Debug, Clone)]
pub struct Accuracy {
    internal_target: i64,
    name: String,
}

impl Accuracy {
    pub const NAME: &'static str = "ACCURACY";

    pub fn new(original_target: i64, internal_target: i64) -> Self {
        Self {
            internal_target,
            name: format!("{}(class={})", Self::NAME, original_target),
        }
    }
}

impl Metric for Accuracy {
    fn name(&self) -> &str {
        &self.name
    }

    fn comparator(&self) -> MetricComparator {
        MetricComparator::Natural
    }

    fn as_classification_metric(&self) -> Option<&dyn ClassificationMetric> {
        Some(self)
    }
}

impl ClassificationMetric for Accuracy {
    fn compute(&self, targets: &HugeLongArray, predictions: &HugeLongArray) -> f64 {
        debug_assert_eq!(targets.size(), predictions.size());

        if targets.size() == 0 {
            return 0.0;
        }

        let mut accurates = 0u64;
        for i in 0..targets.size() {
            let target_class = targets.get(i);
            let predicted_class = predictions.get(i);

            let predicted_is_positive = predicted_class == self.internal_target;
            let target_is_positive = target_class == self.internal_target;

            if predicted_is_positive == target_is_positive {
                accurates += 1;
            }
        }

        accurates as f64 / targets.size() as f64
    }
}

#[derive(Debug, Clone)]
pub struct GlobalAccuracy;

impl Default for GlobalAccuracy {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalAccuracy {
    pub const NAME: &'static str = "ACCURACY";

    pub fn new() -> Self {
        Self
    }
}

impl Metric for GlobalAccuracy {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn comparator(&self) -> MetricComparator {
        MetricComparator::Natural
    }

    fn as_classification_metric(&self) -> Option<&dyn ClassificationMetric> {
        Some(self)
    }
}

impl ClassificationMetric for GlobalAccuracy {
    fn compute(&self, targets: &HugeLongArray, predictions: &HugeLongArray) -> f64 {
        debug_assert_eq!(targets.size(), predictions.size());

        let mut accurate_predictions = 0u64;
        for i in 0..targets.size() {
            if targets.get(i) == predictions.get(i) {
                accurate_predictions += 1;
            }
        }

        if targets.size() == 0 {
            return 0.0;
        }

        round_up(accurate_predictions as f64 / targets.size() as f64, 8)
    }
}

#[derive(Debug, Clone)]
pub struct F1Score {
    internal_target: i64,
    name: String,
}

impl F1Score {
    pub const NAME: &'static str = "F1";

    pub fn new(original_target: i64, internal_target: i64) -> Self {
        Self {
            internal_target,
            name: format!("{}(class={})", Self::NAME, original_target),
        }
    }
}

impl Metric for F1Score {
    fn name(&self) -> &str {
        &self.name
    }

    fn comparator(&self) -> MetricComparator {
        MetricComparator::Natural
    }

    fn as_classification_metric(&self) -> Option<&dyn ClassificationMetric> {
        Some(self)
    }
}

impl ClassificationMetric for F1Score {
    fn compute(&self, targets: &HugeLongArray, predictions: &HugeLongArray) -> f64 {
        debug_assert_eq!(targets.size(), predictions.size());
        let mut true_positives = 0;
        let mut false_positives = 0;
        let mut false_negatives = 0;

        for i in 0..targets.size() {
            let target = targets.get(i);
            let prediction = predictions.get(i);

            if target == self.internal_target {
                if prediction == target {
                    true_positives += 1;
                } else {
                    false_negatives += 1;
                }
            } else if prediction == self.internal_target {
                false_positives += 1;
            }
        }

        let precision =
            true_positives as f64 / ((true_positives + false_positives) as f64 + EPSILON);
        let recall = true_positives as f64 / ((true_positives + false_negatives) as f64 + EPSILON);
        let result = 2.0 * (precision * recall) / (precision + recall + EPSILON);
        debug_assert!(result <= 1.0 + EPSILON);
        result
    }
}

#[derive(Debug, Clone)]
pub struct Precision {
    internal_target: i64,
    name: String,
}

impl Precision {
    pub const NAME: &'static str = "PRECISION";

    pub fn new(original_target: i64, internal_target: i64) -> Self {
        Self {
            internal_target,
            name: format!("{}(class={})", Self::NAME, original_target),
        }
    }
}

impl Metric for Precision {
    fn name(&self) -> &str {
        &self.name
    }

    fn comparator(&self) -> MetricComparator {
        MetricComparator::Natural
    }

    fn as_classification_metric(&self) -> Option<&dyn ClassificationMetric> {
        Some(self)
    }
}

impl ClassificationMetric for Precision {
    fn compute(&self, targets: &HugeLongArray, predictions: &HugeLongArray) -> f64 {
        debug_assert_eq!(targets.size(), predictions.size());
        let mut true_positives = 0;
        let mut false_positives = 0;

        for i in 0..targets.size() {
            let target_class = targets.get(i);
            let predicted_class = predictions.get(i);
            let predicted_is_positive = predicted_class == self.internal_target;
            if !predicted_is_positive {
                continue;
            }

            let target_is_positive = target_class == self.internal_target;
            if target_is_positive {
                true_positives += 1;
            } else {
                false_positives += 1;
            }
        }

        let result = true_positives as f64 / ((true_positives + false_positives) as f64 + EPSILON);
        debug_assert!(result <= 1.0 + EPSILON);
        result
    }
}

#[derive(Debug, Clone)]
pub struct Recall {
    internal_target: i64,
    name: String,
}

impl Recall {
    pub const NAME: &'static str = "RECALL";

    pub fn new(original_target: i64, internal_target: i64) -> Self {
        Self {
            internal_target,
            name: format!("{}(class={})", Self::NAME, original_target),
        }
    }
}

impl Metric for Recall {
    fn name(&self) -> &str {
        &self.name
    }

    fn comparator(&self) -> MetricComparator {
        MetricComparator::Natural
    }

    fn as_classification_metric(&self) -> Option<&dyn ClassificationMetric> {
        Some(self)
    }
}

impl ClassificationMetric for Recall {
    fn compute(&self, targets: &HugeLongArray, predictions: &HugeLongArray) -> f64 {
        debug_assert_eq!(targets.size(), predictions.size());
        let mut true_positives = 0;
        let mut false_negatives = 0;

        for i in 0..targets.size() {
            let target_class = targets.get(i);
            let predicted_class = predictions.get(i);

            let predicted_is_positive = predicted_class == self.internal_target;
            let target_is_positive = target_class == self.internal_target;
            let predicted_is_negative = !predicted_is_positive;

            if predicted_is_positive && target_is_positive {
                true_positives += 1;
            }

            if predicted_is_negative && target_is_positive {
                false_negatives += 1;
            }
        }

        true_positives as f64 / ((true_positives + false_negatives) as f64 + EPSILON)
    }
}

#[derive(Debug, Clone)]
pub struct F1Macro {
    class_id_map: LocalIdMap,
}

impl F1Macro {
    pub const NAME: &'static str = "F1_MACRO";

    pub fn new(class_id_map: LocalIdMap) -> Self {
        Self { class_id_map }
    }
}

impl Metric for F1Macro {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn comparator(&self) -> MetricComparator {
        MetricComparator::Natural
    }

    fn as_classification_metric(&self) -> Option<&dyn ClassificationMetric> {
        Some(self)
    }
}

impl ClassificationMetric for F1Macro {
    fn compute(&self, targets: &HugeLongArray, predictions: &HugeLongArray) -> f64 {
        let mut sum = 0.0;
        let mut count = 0usize;
        for (original, internal) in self.class_id_map.mappings() {
            let metric = F1Score::new(original as i64, internal as i64);
            sum += metric.compute(targets, predictions);
            count += 1;
        }

        if count == 0 {
            -1.0
        } else {
            sum / count as f64
        }
    }
}

#[derive(Debug, Clone)]
pub struct F1Weighted {
    class_id_map: LocalIdMap,
    global_class_counts: LongMultiSet,
}

impl F1Weighted {
    pub const NAME: &'static str = "F1_WEIGHTED";

    pub fn new(class_id_map: LocalIdMap, global_class_counts: LongMultiSet) -> Self {
        Self {
            class_id_map,
            global_class_counts,
        }
    }
}

impl Metric for F1Weighted {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn comparator(&self) -> MetricComparator {
        MetricComparator::Natural
    }

    fn as_classification_metric(&self) -> Option<&dyn ClassificationMetric> {
        Some(self)
    }
}

impl ClassificationMetric for F1Weighted {
    fn compute(&self, targets: &HugeLongArray, predictions: &HugeLongArray) -> f64 {
        if self.global_class_counts.size() == 0 {
            return 0.0;
        }

        let mut weighted_sum = 0.0;
        for (original, internal) in self.class_id_map.mappings() {
            let weight = self.global_class_counts.count(original as i64) as f64;
            let score =
                F1Score::new(original as i64, internal as i64).compute(targets, predictions);
            weighted_sum += weight * score;
        }

        weighted_sum / self.global_class_counts.sum() as f64
    }
}

fn round_up(value: f64, decimals: u32) -> f64 {
    if decimals == 0 {
        return value.ceil();
    }

    let factor = 10_f64.powi(decimals as i32);
    (value * factor).ceil() / factor
}
