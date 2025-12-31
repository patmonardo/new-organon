//! Node2Vec model implementation with skip-gram training.

use super::compressed_random_walks::CompressedRandomWalks;
use super::train_parameters::TrainParameters;
use crate::concurrency::{Concurrency, TerminationFlag};
use rand::prelude::*;

#[derive(Debug)]
pub struct TrainedNode2Vec {
    pub embeddings: Vec<Vec<f64>>,
    pub loss_per_iteration: Vec<f64>,
}

pub struct Node2VecModel {
    node_count: usize,
    train_params: TrainParameters,
    _concurrency: Concurrency,
    random_seed: Option<u64>,
    walks: CompressedRandomWalks,
    termination_flag: TerminationFlag,
}

impl Node2VecModel {
    pub fn new(
        node_count: usize,
        train_params: TrainParameters,
        concurrency: Concurrency,
        random_seed: Option<u64>,
        walks: CompressedRandomWalks,
        termination_flag: TerminationFlag,
    ) -> Self {
        Self {
            node_count,
            train_params,
            _concurrency: concurrency,
            random_seed,
            walks,
            termination_flag,
        }
    }

    pub fn train(self) -> TrainedNode2Vec {
        let mut rng = match self.random_seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        // Initialize embeddings
        let mut embeddings =
            vec![vec![0.0; self.train_params.embedding_dimension]; self.node_count];
        for embedding in &mut embeddings {
            for val in embedding {
                *val = rng.gen_range(-0.1..0.1);
            }
        }

        let mut loss_per_iteration = Vec::new();

        for _iteration in 0..self.train_params.iterations {
            self.termination_flag.assert_running();

            let mut total_loss = 0.0;
            let mut sample_count = 0;

            // Simple skip-gram training over walks
            for walk in self.walks.walks() {
                for (i, &center_node) in walk.iter().enumerate() {
                    if center_node as usize >= self.node_count {
                        continue;
                    }

                    let start = i.saturating_sub(self.train_params.window_size);
                    let end = (i + self.train_params.window_size + 1).min(walk.len());

                    for (j, &context_node) in walk.iter().enumerate().take(end).skip(start) {
                        if i == j {
                            continue;
                        }
                        if context_node as usize >= self.node_count {
                            continue;
                        }

                        // Skip-gram: predict context from center
                        let loss = self.train_pair(
                            center_node as usize,
                            context_node as usize,
                            &mut embeddings,
                            &mut rng,
                        );
                        total_loss += loss;
                        sample_count += 1;
                    }
                }
            }

            let avg_loss = if sample_count > 0 {
                total_loss / sample_count as f64
            } else {
                0.0
            };
            loss_per_iteration.push(avg_loss);

            // Progress tracking is a no-op for now
        }

        TrainedNode2Vec {
            embeddings,
            loss_per_iteration,
        }
    }

    fn train_pair(
        &self,
        center_idx: usize,
        context_idx: usize,
        embeddings: &mut [Vec<f64>],
        rng: &mut impl Rng,
    ) -> f64 {
        // Get embeddings without borrowing issues
        let center_emb = embeddings[center_idx].clone();
        let context_emb = embeddings[context_idx].clone();

        let mut dot_product = 0.0;
        for i in 0..center_emb.len() {
            dot_product += center_emb[i] * context_emb[i];
        }

        // Sigmoid for positive pair
        let prob = 1.0 / (1.0 + (-dot_product).exp());
        let mut loss = -(prob.ln());

        // Negative sampling
        for _ in 0..self.train_params.negative_sampling_rate {
            // Sample negative from unigram distribution
            let neg_idx = self.sample_negative(center_idx, rng);
            let neg_emb = embeddings[neg_idx].clone();

            let mut neg_dot = 0.0;
            for i in 0..center_emb.len() {
                neg_dot += center_emb[i] * neg_emb[i];
            }

            let neg_prob = 1.0 / (1.0 + (-neg_dot).exp());
            loss += -((1.0 - neg_prob).ln());
        }

        // Simple gradient descent update (simplified)
        let learning_rate = self.train_params.initial_learning_rate;
        let gradient = prob - 1.0; // For positive pair

        for i in 0..center_emb.len() {
            embeddings[center_idx][i] -= learning_rate * gradient * context_emb[i];
            embeddings[context_idx][i] -= learning_rate * gradient * center_emb[i];
        }

        loss
    }

    fn sample_negative(&self, exclude_idx: usize, rng: &mut impl Rng) -> usize {
        loop {
            // Simple uniform sampling for now (should use unigram distribution)
            let idx = rng.gen_range(0..self.node_count);
            if idx != exclude_idx {
                return idx;
            }
        }
    }
}
