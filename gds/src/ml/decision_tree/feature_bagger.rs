//! Feature bagger for random feature selection in decision trees.
//!
//! Translated from Java GDS ml-algo FeatureBagger.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::mem::{Estimate, MemoryRange};
use crate::ml::core::samplers::IntUniformSamplerFromRange;

/// Samples a subset of features for each split (feature bagging).
///
/// NOTE: This struct is not thread-safe.
#[derive(Clone)]
pub struct FeatureBagger {
    sampler: IntUniformSamplerFromRange,
    total_number_of_features: usize,
    number_of_samples: usize,
}

impl FeatureBagger {
    pub fn memory_estimation(number_of_samples: usize) -> MemoryRange {
        IntUniformSamplerFromRange::memory_estimation(number_of_samples).add(&MemoryRange::of(
            Estimate::size_of_instance("FeatureBagger"),
        ))
    }

    pub fn new(seed: u64, total_number_of_features: usize, max_features_ratio: f64) -> Self {
        assert!(
            max_features_ratio != 0.0,
            "Invalid maxFeaturesRatio: {}",
            max_features_ratio
        );

        let number_of_samples =
            (max_features_ratio * total_number_of_features as f64).ceil() as usize;
        Self {
            sampler: IntUniformSamplerFromRange::new(seed),
            total_number_of_features,
            number_of_samples,
        }
    }

    pub fn sample(&mut self) -> Vec<usize> {
        self.sampler
            .sample(
                0,
                self.total_number_of_features as i32,
                self.total_number_of_features,
                self.number_of_samples,
                |_| false,
            )
            .into_iter()
            .map(|v| v as usize)
            .collect()
    }
}
