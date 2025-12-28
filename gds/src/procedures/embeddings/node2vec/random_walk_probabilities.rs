//! Builder for random walk probabilities used in negative sampling.

use crate::concurrency::Concurrency;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RandomWalkProbabilities {
    // Simplified: just store node frequencies for negative sampling
    pub node_frequencies: HashMap<i64, f64>,
    pub total_frequency: f64,
}

#[derive(Debug)]
pub struct RandomWalkProbabilitiesBuilder {
    node_count: usize,
    _concurrency: Concurrency,
    positive_sampling_factor: f64,
    negative_sampling_exponent: f64,
    node_frequencies: HashMap<i64, f64>,
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
            node_frequencies: HashMap::new(),
        }
    }

    pub fn register_walk(&mut self, walk: &[i64]) {
        for &node_id in walk {
            *self.node_frequencies.entry(node_id).or_insert(0.0) += 1.0;
        }
    }

    pub fn build(self) -> RandomWalkProbabilities {
        let total_frequency = self.node_frequencies.values().sum();
        RandomWalkProbabilities {
            node_frequencies: self.node_frequencies,
            total_frequency,
        }
    }
}