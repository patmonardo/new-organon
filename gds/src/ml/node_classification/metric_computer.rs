use crate::collections::HugeLongArray;
use crate::concurrency::Concurrency;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::TaskProgressTracker;
use crate::mem::Estimate;
use crate::mem::MemoryEstimation;
use crate::mem::MemoryEstimations;
use crate::mem::MemoryRange;
use crate::ml::core::batch::DEFAULT_BATCH_SIZE;
use crate::ml::metrics::classification::ClassificationMetric;
use crate::ml::models::Classifier;
use crate::ml::models::ClassifierFactory;
use crate::ml::models::Features;
use crate::ml::models::TrainerConfig;
use std::sync::Arc;

use super::parallel_classifier::ParallelNodeClassifier;

/// Computer for classification metrics
/// 1:1 translation of ClassificationMetricComputer.java
pub struct ClassificationMetricComputer {
    predicted_classes: Arc<HugeLongArray>,
    labels: Arc<HugeLongArray>,
}

impl ClassificationMetricComputer {
    /// Creates a new instance from predicted classes and actual labels
    pub fn new(predicted_classes: Arc<HugeLongArray>, labels: Arc<HugeLongArray>) -> Self {
        Self {
            predicted_classes,
            labels,
        }
    }

    /// Creates a new instance for evaluating metrics on a validation set
    /// 1:1 with ClassificationMetricComputer.forEvaluationSet() in Java
    pub fn for_evaluation_set(
        features: Arc<dyn Features>,
        labels: Arc<HugeLongArray>,
        evaluation_set: Arc<Vec<u64>>, // ReadOnlyHugeLongArray
        classifier: Arc<dyn Classifier>,
        concurrency: Concurrency,
        termination_flag: TerminationFlag,
        progress_tracker: TaskProgressTracker,
    ) -> Self {
        // Predict classes for evaluation set
        let predictor = ParallelNodeClassifier::new(
            classifier,
            features,
            DEFAULT_BATCH_SIZE,
            concurrency,
            termination_flag,
            progress_tracker,
        );

        let predicted_classes = predictor.predict(&evaluation_set);
        let local_labels = Self::make_local_targets(&evaluation_set, &labels);

        Self {
            predicted_classes: Arc::new(predicted_classes),
            labels: Arc::new(local_labels),
        }
    }

    /// Computes a score using the given metric
    /// 1:1 with score() in Java
    pub fn score(&self, metric: &dyn ClassificationMetric) -> f64 {
        // Compute metric directly on predicted vs actual
        metric.compute(&self.labels, &self.predicted_classes)
    }

    /// Make local targets array aligned with evaluation set
    /// 1:1 with makeLocalTargets() in Java
    fn make_local_targets(node_ids: &[u64], targets: &HugeLongArray) -> HugeLongArray {
        let mut local_targets = HugeLongArray::new(node_ids.len());
        for (i, &node_id) in node_ids.iter().enumerate() {
            local_targets.set(i, targets.get(node_id as usize));
        }
        local_targets
    }

    /// Memory estimation for evaluation (Java: estimateEvaluation)
    #[allow(clippy::too_many_arguments)]
    pub fn estimate_evaluation(
        config: &dyn TrainerConfig,
        batch_size: usize,
        train_set_size: impl Fn(u64) -> u64 + Send + Sync + 'static,
        test_set_size: impl Fn(u64) -> u64 + Send + Sync + 'static,
        fudged_class_count: usize,
        fudged_feature_count: usize,
        is_reduced: bool,
    ) -> Box<dyn MemoryEstimation> {
        let train_set_size = Arc::new(train_set_size);
        let test_set_size = Arc::new(test_set_size);
        let method = config.method();

        MemoryEstimations::builder("computing metrics")
            .range_per_graph_dimension("local targets", {
                let test_set_size = Arc::clone(&test_set_size);
                move |dim, _| {
                    let size = test_set_size(dim.node_count() as u64) as usize;
                    MemoryRange::of(Estimate::size_of_long_array(size))
                }
            })
            .range_per_graph_dimension("predicted classes", {
                let test_set_size = Arc::clone(&test_set_size);
                move |dim, _| {
                    let size = test_set_size(dim.node_count() as u64) as usize;
                    MemoryRange::of(Estimate::size_of_long_array(size))
                }
            })
            .add(ClassifierFactory::data_memory_estimation(
                config,
                {
                    let train_set_size = Arc::clone(&train_set_size);
                    move |node_count| (train_set_size)(node_count)
                },
                fudged_class_count,
                MemoryRange::of(fudged_feature_count),
                is_reduced,
            ))
            .range_per_graph_dimension("classifier runtime", move |_, _| {
                ClassifierFactory::runtime_overhead_memory_estimation(
                    method,
                    batch_size,
                    fudged_class_count,
                    fudged_feature_count,
                    is_reduced,
                )
            })
            .build()
    }
}
