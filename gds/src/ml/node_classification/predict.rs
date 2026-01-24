use crate::collections::HugeLongArray;
use crate::collections::HugeObjectArray;
use crate::concurrency::Concurrency;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::LeafTask;
use crate::core::utils::progress::ProgressTracker;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::mem::Estimate;
use crate::mem::MemoryEstimation;
use crate::mem::MemoryEstimations;
use crate::mem::MemoryRange;
use crate::ml::core::batch::compute_batch_size;
use crate::ml::models::Classifier;
use crate::ml::models::ClassifierFactory;
use crate::ml::models::Features;
use crate::ml::models::TrainingMethod;
use std::fmt;
use std::sync::Arc;

/// Result of node classification prediction
/// 1:1 with NodeClassificationResult in Java
pub struct NodeClassificationPredictResult {
    predicted_classes: Arc<HugeLongArray>,
    predicted_probabilities: Option<Arc<HugeObjectArray<Vec<f64>>>>,
}

impl fmt::Debug for NodeClassificationPredictResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeClassificationPredictResult")
            .field("predicted_classes_len", &self.predicted_classes.size())
            .field(
                "probabilities_size",
                &self.predicted_probabilities.as_ref().map(|arr| arr.size()),
            )
            .finish()
    }
}

impl NodeClassificationPredictResult {
    /// Creates a new NodeClassificationPredictResult
    pub fn new(
        predicted_classes: Arc<HugeLongArray>,
        predicted_probabilities: Option<Arc<HugeObjectArray<Vec<f64>>>>,
    ) -> Self {
        Self {
            predicted_classes,
            predicted_probabilities,
        }
    }

    /// Returns the predicted classes
    pub fn predicted_classes(&self) -> &Arc<HugeLongArray> {
        &self.predicted_classes
    }

    /// Returns the predicted probabilities if available
    pub fn predicted_probabilities(&self) -> Option<&Arc<HugeObjectArray<Vec<f64>>>> {
        self.predicted_probabilities.as_ref()
    }
}

/// Node classification prediction algorithm
/// 1:1 translation of NodeClassificationPredict.java
pub struct NodeClassificationPredict {
    classifier: Arc<dyn Classifier>,
    features: Arc<dyn Features>,
    batch_size: usize,
    produce_probabilities: bool,
    concurrency: Concurrency,
    termination_flag: TerminationFlag,
    progress_tracker: TaskProgressTracker,
}

impl NodeClassificationPredict {
    /// Creates a new NodeClassificationPredict
    /// Simplified constructor matching Java's NodeClassificationPredict
    pub fn new(
        classifier: Arc<dyn Classifier>,
        features: Arc<dyn Features>,
        batch_size: usize,
        produce_probabilities: bool,
        concurrency: Concurrency,
        termination_flag: TerminationFlag,
        progress_tracker: TaskProgressTracker,
    ) -> Self {
        Self {
            classifier,
            features,
            batch_size,
            produce_probabilities,
            concurrency,
            termination_flag,
            progress_tracker,
        }
    }

    /// Computes predictions for all nodes
    /// 1:1 with compute() in Java
    pub fn compute(&self) -> NodeClassificationPredictResult {
        let node_count = self.features.size();

        let mut tracker = self.progress_tracker.clone();
        tracker.begin_subtask_with_volume(node_count);
        tracker.set_steps(node_count);

        let parallel_classifier = super::parallel_classifier::ParallelNodeClassifier::new(
            self.classifier.clone(),
            self.features.clone(),
            self.batch_size,
            self.concurrency,
            self.termination_flag.clone(),
            self.progress_tracker.clone(),
        );

        let (predicted_classes, predicted_probabilities) = if self.produce_probabilities {
            let (classes, probabilities) = parallel_classifier.predict_with_probabilities();
            (classes, Some(Arc::new(probabilities)))
        } else {
            (parallel_classifier.predict_all(), None)
        };

        tracker.end_subtask();

        NodeClassificationPredictResult::new(Arc::new(predicted_classes), predicted_probabilities)
    }

    /// Progress task (Java: NodeClassificationPredict.progressTask)
    pub fn progress_task(node_count: u64) -> LeafTask {
        Tasks::leaf_with_volume(
            "Node classification predict".to_string(),
            node_count as usize,
        )
    }
}

/// Memory estimation for node classification prediction
/// 1:1 with memoryEstimation() in Java
pub fn estimate_predict_memory(
    produce_probabilities: bool,
    batch_size: usize,
    feature_count: usize,
    class_count: usize,
) -> Box<dyn MemoryEstimation> {
    let mut builder = MemoryEstimations::builder("NodeClassificationPredict");

    if produce_probabilities {
        builder = builder.range_per_graph_dimension("predicted probabilities", move |dim, _| {
            let node_count = dim.node_count();
            let array_bytes = Estimate::size_of_object_array(node_count);
            let per_row = Estimate::size_of_double_array(class_count);
            MemoryRange::of(array_bytes + per_row.saturating_mul(node_count))
        });
    }

    builder
        .range_per_graph_dimension("predicted classes", move |dim, _| {
            MemoryRange::of(Estimate::size_of_long_array(dim.node_count()))
        })
        .fixed_range(
            "computation graph",
            ClassifierFactory::runtime_overhead_memory_estimation(
                TrainingMethod::LogisticRegression,
                batch_size,
                class_count,
                feature_count,
                false,
            ),
        )
        .build()
}

/// Memory estimation with derived batch size (Java: memoryEstimationWithDerivedBatchSize)
pub fn estimate_predict_memory_with_derived_batch_size(
    method: TrainingMethod,
    produce_probabilities: bool,
    min_batch_size: usize,
    feature_count: usize,
    class_count: usize,
    is_reduced: bool,
) -> Box<dyn MemoryEstimation> {
    let mut builder = MemoryEstimations::builder("NodeClassificationPredict");

    if produce_probabilities {
        builder = builder.range_per_graph_dimension("predicted probabilities", move |dim, _| {
            let node_count = dim.node_count();
            let array_bytes = Estimate::size_of_object_array(node_count);
            let per_row = Estimate::size_of_double_array(class_count);
            MemoryRange::of(array_bytes + per_row.saturating_mul(node_count))
        });
    }

    builder
        .range_per_graph_dimension("predicted classes", move |dim, _| {
            MemoryRange::of(Estimate::size_of_long_array(dim.node_count()))
        })
        .range_per_graph_dimension("classifier runtime", move |dim, threads| {
            let batch_size = compute_batch_size(dim.node_count() as u64, min_batch_size, threads);
            ClassifierFactory::runtime_overhead_memory_estimation(
                method,
                batch_size,
                class_count,
                feature_count,
                is_reduced,
            )
        })
        .build()
}
