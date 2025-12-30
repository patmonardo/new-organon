use crate::{
    collections::HugeLongArray,
    ml::{
        metrics::classification::ClassificationMetric,
        models::{Classifier, Features},
    },
};
use rayon::prelude::*;
use std::sync::Arc;

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
    ) -> Self {
        // Predict classes for evaluation set
        let predictor = ParallelNodeClassifier::new(classifier, features, 100);

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
}

/// Parallel classifier implementation with batch processing
/// 1:1 translation of ParallelNodeClassifier.java
struct ParallelNodeClassifier {
    classifier: Arc<dyn Classifier>,
    features: Arc<dyn Features>,
    batch_size: usize,
}

impl ParallelNodeClassifier {
    fn new(
        classifier: Arc<dyn Classifier>,
        features: Arc<dyn Features>,
        batch_size: usize,
    ) -> Self {
        Self {
            classifier,
            features,
            batch_size,
        }
    }

    fn predict(&self, evaluation_set: &[u64]) -> HugeLongArray {
        let mut predictions = HugeLongArray::new(evaluation_set.len());

        // Process in batches for better performance
        // 1:1 with Java's BatchQueue.consecutive().parallelConsume()
        let batch_predictions: Vec<(usize, i64)> = evaluation_set
            .par_chunks(self.batch_size)
            .enumerate()
            .flat_map(|(batch_idx, batch)| {
                self.process_batch(batch, batch_idx * self.batch_size)
            })
            .collect();

        // Set predictions in the array
        for (idx, prediction) in batch_predictions {
            predictions.set(idx, prediction);
        }

        predictions
    }

    /// Process a batch of node IDs and return (index, prediction) pairs
    /// 1:1 with NodeClassificationPredictConsumer.accept() in Java
    fn process_batch(&self, batch: &[u64], offset: usize) -> Vec<(usize, i64)> {
        batch
            .iter()
            .enumerate()
            .map(|(local_idx, &node_id)| {
                let global_idx = offset + local_idx;

                // Get feature vector for this node
                let feature_vec = self.features.get(node_id as usize);

                // Predict probabilities
                let probs = self.classifier.predict_probabilities(feature_vec);

                // Find class with maximum probability
                // 1:1 with Java's argmax logic in NodeClassificationPredictConsumer
                let predicted_class = probs
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .map(|(idx, _)| idx as i64)
                    .unwrap_or(0);

                (global_idx, predicted_class)
            })
            .collect()
    }
}
