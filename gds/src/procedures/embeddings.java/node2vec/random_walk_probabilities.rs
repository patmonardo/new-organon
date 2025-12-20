use crate::collections::{HugeAtomicLongArray, HugeDoubleArray, HugeLongArray};
use crate::concurrency::Concurrency;
use std::sync::atomic::{AtomicU64, Ordering};

/// Random walk derived probabilities for Node2Vec training.
///
/// Java: `RandomWalkProbabilities` value class + Builder.
pub struct RandomWalkProbabilities {
    node_frequencies: HugeAtomicLongArray,
    positive_sampling_probabilities: HugeDoubleArray,
    negative_sampling_distribution: HugeLongArray,
    sample_count: u64,
}

impl RandomWalkProbabilities {
    pub fn node_frequencies(&self) -> &HugeAtomicLongArray {
        &self.node_frequencies
    }

    pub fn positive_sampling_probabilities(&self) -> &HugeDoubleArray {
        &self.positive_sampling_probabilities
    }

    pub fn negative_sampling_distribution(&self) -> &HugeLongArray {
        &self.negative_sampling_distribution
    }

    pub fn sample_count(&self) -> u64 {
        self.sample_count
    }
}

/// Builder (mirrors Java nested `RandomWalkProbabilities.Builder`).
pub struct RandomWalkProbabilitiesBuilder {
    node_count: usize,
    _concurrency: Concurrency,
    positive_sampling_factor: f64,
    negative_sampling_exponent: f64,
    node_frequencies: HugeAtomicLongArray,
    sample_count: AtomicU64,
}

impl RandomWalkProbabilitiesBuilder {
    pub fn new(
        node_count: usize,
        concurrency: Concurrency,
        positive_sampling_factor: f64,
        negative_sampling_exponent: f64,
    ) -> Self {
        Self {
            node_count,
            _concurrency: concurrency,
            positive_sampling_factor,
            negative_sampling_exponent,
            node_frequencies: HugeAtomicLongArray::new(node_count),
            sample_count: AtomicU64::new(0),
        }
    }

    pub fn register_walk(&self, walk: &[i64]) {
        for &node in walk {
            if node >= 0 {
                self.node_frequencies.get_and_add(node as usize, 1);
            }
        }
        self.sample_count
            .fetch_add(walk.len() as u64, Ordering::Relaxed);
    }

    pub fn build(self) -> RandomWalkProbabilities {
        let sum = self.sample_count.load(Ordering::Relaxed);
        let positive = self.compute_positive_sampling_probabilities(sum);
        let negative = self.compute_negative_sampling_distribution();

        RandomWalkProbabilities {
            node_frequencies: self.node_frequencies,
            positive_sampling_probabilities: positive,
            negative_sampling_distribution: negative,
            sample_count: sum,
        }
    }

    fn compute_positive_sampling_probabilities(&self, sum: u64) -> HugeDoubleArray {
        let mut probs = HugeDoubleArray::new(self.node_count);
        if sum == 0 {
            return probs;
        }

        for node_id in 0..self.node_count {
            let freq_count = self.node_frequencies.get(node_id) as f64;
            if freq_count <= 0.0 {
                probs.set(node_id, 0.0);
                continue;
            }

            let frequency = freq_count / sum as f64;
            // Java: (sqrt(frequency / factor) + 1) * (factor / frequency)
            let p = (frequency / self.positive_sampling_factor).sqrt();
            let value = (p + 1.0) * (self.positive_sampling_factor / frequency);
            probs.set(node_id, value);
        }

        probs
    }

    fn compute_negative_sampling_distribution(&self) -> HugeLongArray {
        let mut dist = HugeLongArray::new(self.node_count);
        let mut sum: i64 = 0;
        for i in 0..self.node_count {
            let freq = self.node_frequencies.get(i) as f64;
            let add = freq.powf(self.negative_sampling_exponent) as i64;
            sum = sum.saturating_add(add);
            dist.set(i, sum);
        }
        dist
    }
}


