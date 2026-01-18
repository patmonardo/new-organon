//! Int uniform sampler from range for ML in GDS.
//!
//! Translated from Java GDS ml-core IntUniformSamplerFromRange.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::int_uniform_sampler_by_exclusion::IntUniformSamplerByExclusion;
use super::int_uniform_sampler_with_retries::IntUniformSamplerWithRetries;
use crate::mem::{Estimate, MemoryRange};

/// Threshold for choosing retry vs exclusion strategy.
const RETRY_SAMPLING_RATIO: f64 = 0.6;

/// Adaptive uniform sampler that chooses between retry and exclusion strategies.
#[derive(Clone)]
pub struct IntUniformSamplerFromRange {
    retry_based_sampler: IntUniformSamplerWithRetries,
    exclusion_based_sampler: IntUniformSamplerByExclusion,
}

impl IntUniformSamplerFromRange {
    /// Create a new adaptive sampler with the given random seed.
    pub fn new(seed: u64) -> Self {
        Self {
            retry_based_sampler: IntUniformSamplerWithRetries::new(seed),
            exclusion_based_sampler: IntUniformSamplerByExclusion::new(seed),
        }
    }

    /// Sample unique values from the range using the optimal strategy.
    pub fn sample<F>(
        &mut self,
        inclusive_min: i32,
        exclusive_max: i32,
        lower_bound_on_valid_samples: usize,
        number_of_samples: usize,
        is_invalid_sample: F,
    ) -> Vec<i32>
    where
        F: Fn(i32) -> bool,
    {
        let sampling_ratio = number_of_samples as f64 / lower_bound_on_valid_samples as f64;

        if sampling_ratio < RETRY_SAMPLING_RATIO {
            self.retry_based_sampler.sample(
                inclusive_min,
                exclusive_max,
                lower_bound_on_valid_samples,
                number_of_samples,
                is_invalid_sample,
            )
        } else {
            self.exclusion_based_sampler.sample(
                inclusive_min,
                exclusive_max,
                lower_bound_on_valid_samples,
                number_of_samples,
                is_invalid_sample,
            )
        }
    }

    /// Estimate memory usage for this sampler.
    pub fn memory_estimation(number_of_samples: usize) -> MemoryRange {
        let retry_estimation = IntUniformSamplerWithRetries::memory_estimation(number_of_samples);

        let exclusion_estimation = IntUniformSamplerByExclusion::memory_estimation(
            number_of_samples,
            ((number_of_samples as f64) / RETRY_SAMPLING_RATIO).ceil() as usize,
        );

        retry_estimation
            .add(&exclusion_estimation)
            .add(&MemoryRange::of(Estimate::size_of_instance(
                "IntUniformSamplerFromRange",
            )))
            .element_wise_subtract(&MemoryRange::of(Estimate::size_of_int_array(
                number_of_samples,
            )))
    }
}
