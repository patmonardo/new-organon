use crate::collections::HugeLongArray;
use crate::collections::HugeObjectArray;
use crate::core::utils::progress::ProgressTracker;
use crate::core::utils::progress::TaskProgressTracker;
use crate::ml::core::batch::Batch;
use crate::ml::core::batch::BatchTransformer;
use crate::ml::models::Classifier;
use crate::ml::models::Features;
use std::sync::Arc;
use std::sync::Mutex;

/// Consumer for node classification predictions
/// 1:1 translation of NodeClassificationPredictConsumer.java
pub struct NodeClassificationPredictConsumer {
    features: Arc<dyn Features>,
    classifier: Arc<dyn Classifier>,
    predicted_probabilities: Option<Arc<Mutex<HugeObjectArray<Vec<f64>>>>>,
    predicted_classes: Arc<Mutex<HugeLongArray>>,
    progress_tracker: TaskProgressTracker,
}

impl NodeClassificationPredictConsumer {
    /// Creates a new NodeClassificationPredictConsumer
    pub fn new(
        features: Arc<dyn Features>,
        classifier: Arc<dyn Classifier>,
        predicted_probabilities: Option<Arc<Mutex<HugeObjectArray<Vec<f64>>>>>,
        predicted_classes: Arc<Mutex<HugeLongArray>>,
        progress_tracker: TaskProgressTracker,
    ) -> Self {
        Self {
            features,
            classifier,
            predicted_probabilities,
            predicted_classes,
            progress_tracker,
        }
    }

    /// Accepts a batch for processing
    /// 1:1 with accept(Batch) in Java
    pub fn accept<B: Batch>(&self, batch: &B, node_ids: &dyn BatchTransformer) {
        let number_of_classes = self.classifier.number_of_classes();

        let element_ids: Vec<u64> = batch.element_ids().collect();
        let mapped_ids: Vec<usize> = element_ids
            .iter()
            .map(|id| node_ids.apply(*id) as usize)
            .collect();

        let probability_matrix = self
            .classifier
            .predict_probabilities_batch(&mapped_ids, &*self.features);

        for (row, &element_id) in element_ids.iter().enumerate() {
            let mut best_class = 0;
            let mut max_prob = probability_matrix[(row, 0)];

            for class in 1..number_of_classes {
                let prob = probability_matrix[(row, class)];
                if prob > max_prob {
                    max_prob = prob;
                    best_class = class;
                }
            }

            let mut predicted_classes = self
                .predicted_classes
                .lock()
                .expect("predicted_classes mutex poisoned");
            predicted_classes.set(element_id as usize, best_class as i64);

            if let Some(ref probs) = self.predicted_probabilities {
                let mut class_probs = vec![0.0; number_of_classes];
                for class in 0..number_of_classes {
                    class_probs[class] = probability_matrix[(row, class)];
                }
                let mut probabilities = probs
                    .lock()
                    .expect("predicted_probabilities mutex poisoned");
                probabilities.set(element_id as usize, class_probs);
            }
        }

        let mut tracker = self.progress_tracker.clone();
        tracker.log_steps(batch.size());
    }
}
