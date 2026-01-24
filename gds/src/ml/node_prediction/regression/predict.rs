//! Node regression prediction
//! 1:1 translation of NodeRegressionPredict.java

use crate::collections::HugeDoubleArray;
use crate::concurrency::parallel_util::parallel_for_each_node;
use crate::concurrency::Concurrency;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::LeafTask;
use crate::core::utils::progress::ProgressTracker;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::ml::models::Features;
use crate::ml::models::Regressor;
use std::sync::Arc;
use std::sync::Mutex;

/// Performs regression prediction on nodes
/// 1:1 with NodeRegressionPredict.java
pub struct NodeRegressionPredict {
    regressor: Arc<dyn Regressor>,
    features: Arc<dyn Features>,
    concurrency: Concurrency,
    progress_tracker: TaskProgressTracker,
    termination_flag: TerminationFlag,
}

impl NodeRegressionPredict {
    /// Creates a new regression predictor
    pub fn new(
        regressor: Arc<dyn Regressor>,
        features: Arc<dyn Features>,
        concurrency: Concurrency,
        progress_tracker: TaskProgressTracker,
        termination_flag: TerminationFlag,
    ) -> Self {
        Self {
            regressor,
            features,
            concurrency,
            progress_tracker,
            termination_flag,
        }
    }

    /// Computes predictions for all nodes
    /// 1:1 with compute() in Java
    pub fn compute(&self) -> HugeDoubleArray {
        let mut tracker = self.progress_tracker.clone();
        tracker.begin_subtask_with_description("Predict");

        let predicted_targets = Arc::new(Mutex::new(HugeDoubleArray::new(self.features.size())));
        let predicted_targets_for_task = Arc::clone(&predicted_targets);
        let regressor = Arc::clone(&self.regressor);
        let features = Arc::clone(&self.features);

        parallel_for_each_node(
            self.features.size(),
            self.concurrency,
            &self.termination_flag,
            move |id| {
                let feature_vec = features.get(id);
                let prediction = regressor.predict(feature_vec);
                if let Ok(mut targets) = predicted_targets_for_task.lock() {
                    targets.set(id, prediction);
                }
            },
        );

        let predicted_targets = Arc::try_unwrap(predicted_targets)
            .map(|mutex| {
                mutex
                    .into_inner()
                    .expect("predicted_targets mutex poisoned")
            })
            .unwrap_or_else(|arc| {
                arc.lock()
                    .expect("predicted_targets mutex poisoned")
                    .clone()
            });

        tracker.end_subtask_with_description("Predict");

        predicted_targets
    }

    /// Progress task for regression prediction
    pub fn progress_task(node_count: u64) -> LeafTask {
        Tasks::leaf_with_volume("Predict".to_string(), node_count as usize)
    }
}
