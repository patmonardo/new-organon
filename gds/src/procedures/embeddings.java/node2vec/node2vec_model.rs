use super::compressed_random_walks::CompressedRandomWalks;
use super::negative_sample_producer::NegativeSampleProducer;
use super::node2vec_result::Node2VecResult;
use super::positive_sample_producer::PositiveSampleProducer;
use super::random_walk_probabilities::RandomWalkProbabilities;
use super::train_parameters::{EmbeddingInitializer, TrainParameters};
use super::training_task::TrainingTask;
use crate::collections::HugeObjectArray;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::ProgressTracker;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub const EPSILON: f64 = 1e-10;

/// Node2Vec model (skip-gram + negative sampling) trained over random walks.
///
/// Java: `Node2VecModel`
pub struct Node2VecModel {
    center_embeddings: HugeObjectArray<Vec<f32>>,
    context_embeddings: HugeObjectArray<Vec<f32>>,

    initial_learning_rate: f64,
    min_learning_rate: f64,
    iterations: usize,
    embedding_dimension: usize,
    window_size: usize,
    negative_sampling_rate: usize,
    embedding_initializer: EmbeddingInitializer,
    concurrency: Concurrency,

    walks: CompressedRandomWalks,
    random_walk_probabilities: RandomWalkProbabilities,
    progress_tracker: ProgressTracker,
    random_seed: u64,
    termination_flag: TerminationFlag,
}

impl Node2VecModel {
    pub fn new(
        to_original_id: impl Fn(i64) -> i64,
        node_count: usize,
        train_parameters: TrainParameters,
        concurrency: Concurrency,
        maybe_random_seed: Option<u64>,
        walks: CompressedRandomWalks,
        random_walk_probabilities: RandomWalkProbabilities,
        progress_tracker: ProgressTracker,
        termination_flag: TerminationFlag,
    ) -> Self {
        let random_seed = maybe_random_seed.unwrap_or(42);

        let mut center_embeddings = initialize_embeddings(
            node_count,
            train_parameters.embedding_dimension,
            &to_original_id,
            random_seed,
            train_parameters.embedding_initializer,
        );
        let context_embeddings =
            initialize_embeddings(node_count, train_parameters.embedding_dimension, &to_original_id, random_seed, train_parameters.embedding_initializer);

        // Ensure all slots are initialized (HugeObjectArray defaults to empty Vec otherwise).
        // This is already handled by initialize_embeddings, but keep the binding mutable for future refinements.
        let _ = &mut center_embeddings;

        Self {
            center_embeddings,
            context_embeddings,
            initial_learning_rate: train_parameters.initial_learning_rate,
            min_learning_rate: train_parameters.min_learning_rate,
            iterations: train_parameters.iterations,
            embedding_dimension: train_parameters.embedding_dimension,
            window_size: train_parameters.window_size,
            negative_sampling_rate: train_parameters.negative_sampling_rate,
            embedding_initializer: train_parameters.embedding_initializer,
            concurrency,
            walks,
            random_walk_probabilities,
            progress_tracker,
            random_seed,
            termination_flag,
        }
    }

    pub fn train(mut self) -> Node2VecResult {
        let alpha = if self.iterations == 0 {
            0.0
        } else {
            (self.initial_learning_rate - self.min_learning_rate) / self.iterations as f64
        };

        let mut loss_per_iteration = Vec::with_capacity(self.iterations);

        for iteration in 0..self.iterations {
            self.termination_flag.assert_running();
            let lr = (self.initial_learning_rate - (iteration as f64) * alpha)
                .max(self.min_learning_rate) as f32;

            if self.walks.size() == 0 {
                loss_per_iteration.push(0.0);
                continue;
            }

            // NOTE: For now this is a single sequential task (safe mutation).
            // We’ll introduce Hogwild-style parallelism later once we have
            // an ergonomically safe embedding store abstraction.
            let walks_iter = self.walks.iterator(0, self.walks.size());
            let positive = PositiveSampleProducer::new(
                walks_iter,
                self.random_walk_probabilities.positive_sampling_probabilities(),
                self.window_size,
                self.random_seed + iteration as u64,
                self.progress_tracker.clone(),
            );
            let negative = NegativeSampleProducer::new(
                self.random_walk_probabilities.negative_sampling_distribution().clone(),
                self.random_seed + 10_000 + iteration as u64,
            );

            let mut task = TrainingTask::new(
                &mut self.center_embeddings,
                &mut self.context_embeddings,
                positive,
                negative,
                lr,
                self.negative_sampling_rate,
                self.embedding_dimension,
            );
            task.run();
            loss_per_iteration.push(task.loss_sum());
        }

        Node2VecResult {
            embeddings: self.center_embeddings,
            loss_per_iteration,
        }
    }
}

fn initialize_embeddings(
    node_count: usize,
    embedding_dimensions: usize,
    to_original_id: &impl Fn(i64) -> i64,
    random_seed: u64,
    embedding_initializer: EmbeddingInitializer,
) -> HugeObjectArray<Vec<f32>> {
    let bound = match embedding_initializer {
        EmbeddingInitializer::Uniform => 1.0,
        EmbeddingInitializer::Normalized => 0.5 / embedding_dimensions as f64,
    } as f32;

    let mut embeddings: HugeObjectArray<Vec<f32>> = HugeObjectArray::new(node_count);

    for i in 0..node_count {
        let original = to_original_id(i as i64) as u64;
        let mut rng = ChaCha8Rng::seed_from_u64(original.wrapping_add(random_seed));
        let mut data = Vec::with_capacity(embedding_dimensions);
        for _ in 0..embedding_dimensions {
            let v = rng.gen_range(-bound..bound);
            data.push(v);
        }
        embeddings.set(i, data);
    }

    embeddings
}


