//! Int uniform sampler with retries for ML in GDS.
//!
//! Translated from Java GDS ml-core IntUniformSamplerWithRetries.java.
//! This is a literal 1:1 translation following repository translation policy.

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::collections::HashSet;

use crate::mem::{Estimate, MemoryRange};

/// Samples with retries until the desired number of unique samples are obtained.
///
/// WARNING: There is no maximum number of retries, so can take a long while if the number
/// of possible samples are close to the number of desired samples.
#[derive(Clone)]
pub struct IntUniformSamplerWithRetries {
    rng: StdRng,
    sampled_values_cache: HashSet<i32>,
}

impl IntUniformSamplerWithRetries {
    /// Create a new sampler with the given random seed.
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            sampled_values_cache: HashSet::new(),
        }
    }

    /// Sample unique values from the range [inclusive_min, exclusive_max).
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

        let mut samples = Vec::with_capacity(number_of_samples);
        self.sampled_values_cache.clear();

        while samples.len() < number_of_samples {
            let sample = self.rng.gen_range(inclusive_min..exclusive_max);

            if is_invalid_sample(sample) {
                continue;
            }

            if !self.sampled_values_cache.insert(sample) {
                continue;
            }

            samples.push(sample);
        }

        samples
    }

    /// Estimate memory usage for this sampler.
    pub fn memory_estimation(number_of_samples: usize) -> MemoryRange {
        MemoryRange::of(
            Estimate::size_of_instance("IntUniformSamplerWithRetries")
                + Estimate::size_of_long_hash_set(number_of_samples)
                + Estimate::size_of_int_array(number_of_samples),
        )
    }
}
