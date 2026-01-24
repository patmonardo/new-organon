use crate::collections::HugeLongArray;
use crate::concurrency::Concurrency;
use crate::core::utils::paged::HugeMergeSort;
use crate::core::utils::progress::ProgressTracker;
use crate::core::utils::shuffle::ShuffleUtil;
use crate::ml::splitting::FractionSplitter;
use crate::ml::splitting::TrainingExamplesSplit;
use std::sync::Arc;

/// Result of splitting nodes into training and test sets
/// 1:1 with NodeSplits interface in Java
#[derive(Debug, Clone)]
pub struct NodeSplits {
    all_training_examples: Arc<Vec<i64>>,
    outer_split: TrainingExamplesSplit,
}

impl NodeSplits {
    pub fn new(all_training_examples: Arc<Vec<i64>>, outer_split: TrainingExamplesSplit) -> Self {
        Self {
            all_training_examples,
            outer_split,
        }
    }

    pub fn all_training_examples(&self) -> &Arc<Vec<i64>> {
        &self.all_training_examples
    }

    pub fn outer_split(&self) -> &TrainingExamplesSplit {
        &self.outer_split
    }
}

/// Splits nodes into training and test sets
/// 1:1 with NodeSplitter.java
pub struct NodeSplitter {
    concurrency: Concurrency,
    number_of_examples: usize,
    to_original_id: Arc<dyn Fn(usize) -> i64 + Send + Sync>,
    to_mapped_id: Arc<dyn Fn(i64) -> usize + Send + Sync>,
}

impl NodeSplitter {
    /// Creates a new node splitter
    pub fn new(
        concurrency: Concurrency,
        number_of_examples: usize,
        to_original_id: Arc<dyn Fn(usize) -> i64 + Send + Sync>,
        to_mapped_id: Arc<dyn Fn(i64) -> usize + Send + Sync>,
    ) -> Self {
        Self {
            concurrency,
            number_of_examples,
            to_original_id,
            to_mapped_id,
        }
    }

    /// Splits nodes into train/test sets
    /// 1:1 with split() in Java
    pub fn split(
        &self,
        test_fraction: f64,
        validation_folds: usize,
        random_seed: Option<u64>,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> NodeSplits {
        let mut all_training_examples = HugeLongArray::new(self.number_of_examples);

        // Sort by original IDs for deterministic projections (matches Java)
        all_training_examples.set_all(|i| (self.to_original_id)(i));

        HugeMergeSort::sort(&mut all_training_examples, self.concurrency);

        for i in 0..self.number_of_examples {
            let original_id = all_training_examples.get(i);
            let mapped = (self.to_mapped_id)(original_id) as i64;
            all_training_examples.set(i, mapped);
        }

        // Shuffle with seed if provided (matches Java ShuffleUtil)
        let mut rng = ShuffleUtil::create_random_data_generator(random_seed);
        ShuffleUtil::shuffle_array(&mut all_training_examples, &mut rng);

        let all_examples = Arc::new(
            (0..self.number_of_examples)
                .map(|i| all_training_examples.get(i))
                .collect::<Vec<_>>(),
        );

        let outer_split = FractionSplitter::split(all_examples.clone(), 1.0 - test_fraction);

        // Warn for small node sets (matches Java)
        self.warn_for_small_node_sets(
            outer_split.train_set().len(),
            outer_split.test_set().len(),
            validation_folds,
            progress_tracker,
        );

        NodeSplits::new(all_examples, outer_split)
    }

    /// Warns if training or test sets are too small
    /// 1:1 with warnForSmallNodeSets in Java
    fn warn_for_small_node_sets(
        &self,
        train_size: usize,
        test_size: usize,
        validation_folds: usize,
        progress_tracker: &mut dyn ProgressTracker,
    ) {
        if train_size < 500 || test_size < 100 {
            progress_tracker.log_warning(&format!(
                "Warning: Small node sets detected: training={}, test={}. Consider adjusting split fractions.",
                train_size, test_size
            ));
        }

        if validation_folds > 0 && train_size / validation_folds < 100 {
            progress_tracker.log_warning(&format!(
                "Warning: Small validation fold size: {} nodes per fold. Consider reducing validation_folds.",
                train_size / validation_folds
            ));
        }
    }
}
