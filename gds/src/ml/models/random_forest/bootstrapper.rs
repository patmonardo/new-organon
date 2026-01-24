use crate::collections::BitSet;
use rand::prelude::*;
use std::sync::Arc;

/// ReadOnlyHugeLongArray type alias for shared read-only arrays
type ReadOnlyHugeLongArray = Arc<Vec<u64>>;

/// Utility for bootstrapping training datasets in random forests.
/// 1:1 translation of DatasetBootstrapper.java from Java GDS.
pub struct DatasetBootstrapper;

impl DatasetBootstrapper {
    /// Bootstrap a training set by sampling with replacement.
    /// 1:1 translation of DatasetBootstrapper.bootstrap() from Java GDS.
    ///
    /// # Arguments
    /// * `rng` - Random number generator
    /// * `num_feature_vectors_ratio` - Ratio of feature vectors to sample (0.0-1.0)
    /// * `train_set` - Original training set (node IDs)
    /// * `bootstrapped_train_set_indices` - BitSet to track which indices were sampled
    ///
    /// # Returns
    /// A new ReadOnlyHugeLongArray containing the bootstrapped sample indices
    pub fn bootstrap(
        rng: &mut impl Rng,
        num_feature_vectors_ratio: f64,
        train_set: &ReadOnlyHugeLongArray,
        bootstrapped_train_set_indices: &mut BitSet,
    ) -> ReadOnlyHugeLongArray {
        assert!(
            (0.0..=1.0).contains(&num_feature_vectors_ratio),
            "num_feature_vectors_ratio must be between 0.0 and 1.0"
        );

        let num_vectors = (num_feature_vectors_ratio * train_set.len() as f64).ceil() as usize;
        let mut bootstrapped_vectors = Vec::with_capacity(num_vectors);

        for _ in 0..num_vectors {
            // Sample with replacement
            let sampled_idx = rng.gen_range(0..train_set.len());

            // Store the sampled node id (translate from train set idx)
            bootstrapped_vectors.push(train_set[sampled_idx]);

            // Track which indices were sampled
            bootstrapped_train_set_indices.set(sampled_idx);
        }

        Arc::new(bootstrapped_vectors)
    }
}
