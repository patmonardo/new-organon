//! Int uniform sampler by exclusion for ML in GDS.
//!
//! Translated from Java GDS ml-core IntUniformSamplerByExclusion.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::int_uniform_sampler_with_retries::IntUniformSamplerWithRetries;
use crate::mem::{Estimate, MemoryRange};

/// Sample numbers by excluding from the given range.
///
/// Appropriate when the desired sample size is close to the valid sample count.
#[derive(Clone)]
pub struct IntUniformSamplerByExclusion {
    sampler_with_retries: IntUniformSamplerWithRetries,
}

impl IntUniformSamplerByExclusion {
    /// Create a new exclusion-based sampler with the given random seed.
    pub fn new(seed: u64) -> Self {
        Self {
            sampler_with_retries: IntUniformSamplerWithRetries::new(seed),
        }
    }

    /// Sample unique values from the range by building the valid space and excluding samples.
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
        if number_of_samples >= lower_bound_on_valid_samples {
            return (inclusive_min..exclusive_max)
                .filter(|&x| !is_invalid_sample(x))
                .collect();
        }

        let valid_sample_space: Vec<i32> = (inclusive_min..exclusive_max)
            .filter(|&x| !is_invalid_sample(x))
            .collect();

        assert!(
            valid_sample_space.len() >= number_of_samples,
            "Valid sample space {} must be >= number_of_samples {}",
            valid_sample_space.len(),
            number_of_samples
        );

        let num_to_remove = valid_sample_space.len() - number_of_samples;
        let mut samples_to_remove = self.sampler_with_retries.sample(
            0,
            valid_sample_space.len() as i32,
            valid_sample_space.len(),
            num_to_remove,
            |_| false,
        );

        samples_to_remove.sort_unstable();

        let mut samples = Vec::with_capacity(number_of_samples);
        let mut next_idx_to_keep = 0usize;

        for &next_idx_to_remove in &samples_to_remove {
            let next_idx_to_remove = next_idx_to_remove as usize;
            samples.extend_from_slice(&valid_sample_space[next_idx_to_keep..next_idx_to_remove]);
            next_idx_to_keep = next_idx_to_remove + 1;
        }

        samples.extend_from_slice(&valid_sample_space[next_idx_to_keep..]);

        samples
    }

    /// Estimate memory usage for this sampler.
    pub fn memory_estimation(
        number_of_samples: usize,
        max_lower_bound_on_valid_samples: usize,
    ) -> MemoryRange {
        let sampler_with_retries_estimation = IntUniformSamplerWithRetries::memory_estimation(
            number_of_samples.min(max_lower_bound_on_valid_samples - number_of_samples),
        )
        .union(&IntUniformSamplerWithRetries::memory_estimation(0));

        let base = Estimate::size_of_instance("IntUniformSamplerByExclusion")
            + Estimate::size_of_int_array(number_of_samples);

        sampler_with_retries_estimation.add(&MemoryRange::of_range(
            base + Estimate::size_of_int_array_list(0),
            base + Estimate::size_of_int_array_list(max_lower_bound_on_valid_samples),
        ))
    }
}
