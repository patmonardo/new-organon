use crate::collections::HugeLongArray;
use crate::collections::HugeObjectArray;
use crate::concurrency::virtual_threads::RunWithConcurrency;
use crate::concurrency::Concurrency;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::TaskProgressTracker;
use crate::ml::core::batch::BatchTransformer;
use crate::ml::core::batch::IdentityBatchTransformer;
use crate::ml::core::batch::RangeBatch;
use crate::ml::models::Classifier;
use crate::ml::models::Features;
use std::sync::Arc;
use std::sync::Mutex;

use super::predict_consumer::NodeClassificationPredictConsumer;

/// Parallel node classifier that can process predictions in batches
/// 1:1 translation of ParallelNodeClassifier.java
pub struct ParallelNodeClassifier {
    classifier: Arc<dyn Classifier>,
    features: Arc<dyn Features>,
    batch_size: usize,
    concurrency: Concurrency,
    termination_flag: TerminationFlag,
    progress_tracker: TaskProgressTracker,
}

impl ParallelNodeClassifier {
    /// Creates a new ParallelNodeClassifier
    /// Simplified constructor matching Java's ParallelNodeClassifier
    pub fn new(
        classifier: Arc<dyn Classifier>,
        features: Arc<dyn Features>,
        batch_size: usize,
        concurrency: Concurrency,
        termination_flag: TerminationFlag,
        progress_tracker: TaskProgressTracker,
    ) -> Self {
        Self {
            classifier,
            features,
            batch_size,
            concurrency,
            termination_flag,
            progress_tracker,
        }
    }

    /// Predicts classes for an evaluation set
    /// 1:1 with predict(ReadOnlyHugeLongArray) in Java
    pub fn predict(&self, evaluation_set: &[u64]) -> HugeLongArray {
        let transformer = Arc::new(EvaluationSetTransformer::new(Arc::new(
            evaluation_set.to_vec(),
        )));
        self.predict_internal(evaluation_set.len(), transformer, None)
            .0
    }

    /// Predicts with optional probabilities output
    /// 1:1 with predict(HugeObjectArray<double[]>) in Java
    pub fn predict_with_probabilities(&self) -> (HugeLongArray, HugeObjectArray<Vec<f64>>) {
        let size = self.features.size();
        let transformer: Arc<dyn BatchTransformer + Send + Sync> =
            Arc::new(IdentityBatchTransformer);
        let probabilities = HugeObjectArray::new(size);
        let (classes, probabilities) =
            self.predict_internal(size, transformer, Some(probabilities));
        (
            classes,
            probabilities.expect("probabilities must be present"),
        )
    }

    /// Predict classes for all nodes without probabilities (identity mapping).
    pub fn predict_all(&self) -> HugeLongArray {
        let size = self.features.size();
        let transformer: Arc<dyn BatchTransformer + Send + Sync> =
            Arc::new(IdentityBatchTransformer);
        self.predict_internal(size, transformer, None).0
    }

    /// Internal prediction method
    /// 1:1 with private predict() in Java
    fn predict_internal(
        &self,
        evaluation_set_size: usize,
        node_id_mapper: Arc<dyn BatchTransformer + Send + Sync>,
        predicted_probabilities: Option<HugeObjectArray<Vec<f64>>>,
    ) -> (HugeLongArray, Option<HugeObjectArray<Vec<f64>>>) {
        if evaluation_set_size == 0 {
            return (HugeLongArray::new(0), predicted_probabilities);
        }

        let predicted_classes = Arc::new(Mutex::new(HugeLongArray::new(evaluation_set_size)));
        let predicted_probabilities = predicted_probabilities.map(|arr| Arc::new(Mutex::new(arr)));

        let consumer = Arc::new(NodeClassificationPredictConsumer::new(
            Arc::clone(&self.features),
            Arc::clone(&self.classifier),
            predicted_probabilities.clone(),
            Arc::clone(&predicted_classes),
            self.progress_tracker.clone(),
        ));

        let tasks: Vec<Box<dyn FnOnce() + Send>> = (0..evaluation_set_size)
            .step_by(self.batch_size)
            .map(|start| {
                let batch_size = self.batch_size;
                let node_id_mapper = Arc::clone(&node_id_mapper);
                let consumer = Arc::clone(&consumer);
                let termination_flag = self.termination_flag.clone();
                Box::new(move || {
                    if !termination_flag.running() {
                        return;
                    }
                    let batch =
                        RangeBatch::new(start as u64, batch_size, evaluation_set_size as u64);
                    consumer.accept(&batch, node_id_mapper.as_ref());
                }) as Box<dyn FnOnce() + Send>
            })
            .collect();

        let _ = RunWithConcurrency::builder()
            .concurrency(self.concurrency)
            .termination_flag(self.termination_flag.clone())
            .tasks(tasks)
            .run();

        let predicted_classes = Arc::try_unwrap(predicted_classes)
            .map(|mutex| {
                mutex
                    .into_inner()
                    .expect("predicted_classes mutex poisoned")
            })
            .unwrap_or_else(|arc| {
                arc.lock()
                    .expect("predicted_classes mutex poisoned")
                    .clone()
            });

        let predicted_probabilities = predicted_probabilities.and_then(|probabilities| {
            Arc::try_unwrap(probabilities)
                .ok()
                .and_then(|mutex| mutex.into_inner().ok())
        });

        (predicted_classes, predicted_probabilities)
    }
}

struct EvaluationSetTransformer {
    evaluation_set: Arc<Vec<u64>>,
}

impl EvaluationSetTransformer {
    fn new(evaluation_set: Arc<Vec<u64>>) -> Self {
        Self { evaluation_set }
    }
}

impl BatchTransformer for EvaluationSetTransformer {
    fn apply(&self, index: u64) -> u64 {
        self.evaluation_set[index as usize]
    }
}
