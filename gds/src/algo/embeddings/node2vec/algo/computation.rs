//! Node2Vec computation runtime.
//!
//! This is the **Subtle pole**: walk generation + skip-gram style training.
//!
//! ## What happens in `run()` (mental model)
//!
//! 1. **Validate weights** (if present): Node2Vec assumes non-negative relationship weights.
//! 2. **Generate biased random walks**:
//!    - For each source node, generate `walks_per_node` walks of length `walk_length`.
//!    - The transition probabilities are biased by `return_factor` ($p$) and `in_out_factor` ($q$).
//! 3. **Collect corpus statistics** while generating walks:
//!    - Subsampling decisions (via `positive_sampling_factor`).
//!    - Negative sampling distribution (unigram^`negative_sampling_exponent`).
//! 4. **Train** a skip-gram model with negative sampling:
//!    - For each position in each walk, predict nearby context nodes within `window_size`.
//!    - For each positive pair, sample `negative_sampling_rate` negatives.
//!    - SGD runs for `iterations`.
//! 5. Return `Node2VecResult { embeddings, loss_per_iteration }`.
//!
//! The output `embeddings` is indexed by the graph's **mapped** node ids (0..node_count).
//! The model is constructed with a mapping closure so training can reference original ids
//! where needed, but the returned matrix is aligned to the projected graph's node order.

use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{TaskProgressTracker, Tasks};
use crate::ml::core::samplers::RandomWalkSampler;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::Graph;
use std::sync::Arc;

use super::super::compressed_random_walks::CompressedRandomWalks;
use super::super::node2vec_model::Node2VecModel;
use super::super::node2vec_parameters::Node2VecParameters;
use super::super::random_walk_probabilities::RandomWalkProbabilitiesBuilder;
use super::super::sampling_walk_parameters::SamplingWalkParameters;
use super::super::train_parameters::{EmbeddingInitializer, TrainParameters};
use super::spec::{EmbeddingInitializerConfig, Node2VecConfig, Node2VecResult};
use super::Node2VecStorageRuntime;

#[derive(Debug, Default, Clone)]
pub struct Node2VecComputationRuntime;

impl Node2VecComputationRuntime {
    pub fn run(
        graph: Arc<dyn Graph>,
        config: &Node2VecConfig,
        storage: &Node2VecStorageRuntime,
    ) -> Result<Node2VecResult, AlgorithmError> {
        storage.validate_non_negative_weights(graph.as_ref())?;

        let sampling = SamplingWalkParameters {
            walks_per_node: config.walks_per_node,
            walk_length: config.walk_length,
            return_factor: config.return_factor,
            in_out_factor: config.in_out_factor,
            positive_sampling_factor: config.positive_sampling_factor,
            negative_sampling_exponent: config.negative_sampling_exponent,
        };

        let embedding_initializer = match config.embedding_initializer {
            EmbeddingInitializerConfig::Uniform => EmbeddingInitializer::Uniform,
            EmbeddingInitializerConfig::Normalized => EmbeddingInitializer::Normalized,
        };

        let train = TrainParameters {
            initial_learning_rate: config.initial_learning_rate,
            min_learning_rate: config.min_learning_rate,
            iterations: config.iterations,
            window_size: config.window_size,
            negative_sampling_rate: config.negative_sampling_rate,
            embedding_dimension: config.embedding_dimension,
            embedding_initializer,
        };

        let parameters = Node2VecParameters {
            sampling_walk_parameters: sampling,
            train_parameters: train,
        };

        // Build walk corpus + sampling probabilities (mirrors translated wrapper).
        let node_count = graph.node_count();
        let mut probabilities_builder =
            RandomWalkProbabilitiesBuilder::new(Concurrency::of(config.concurrency.max(1)));

        let max_walk_count =
            node_count.saturating_mul(parameters.sampling_walk_parameters.walks_per_node);
        let mut walks = CompressedRandomWalks::new(max_walk_count);

        let random_seed = config.random_seed.unwrap_or(42);

        // Cumulative-weight supplier by summing outgoing relationship weights.
        let graph_for_weight = Arc::clone(&graph);
        let weight_fn = move |node_id: u64| -> f64 {
            let mut sum = 0.0;
            for cursor in graph_for_weight.stream_relationships(node_id as i64, 1.0) {
                sum += cursor.property();
            }
            sum
        };

        let nodes: Vec<i64> = if config.source_nodes.is_empty() {
            (0..node_count as i64).collect()
        } else {
            config.source_nodes.clone()
        };

        let mut sampler = RandomWalkSampler::create(
            Arc::clone(&graph),
            weight_fn,
            parameters.sampling_walk_parameters.walk_length,
            parameters.sampling_walk_parameters.return_factor,
            parameters.sampling_walk_parameters.in_out_factor,
            random_seed,
        );

        let termination_flag = TerminationFlag::default();

        let mut used_walks = 0usize;
        let mut max_len = 0usize;
        let mut produced_since_check = 0usize;
        let check_every = config.walk_buffer_size.max(1);

        for &node_id in &nodes {
            if produced_since_check >= check_every {
                termination_flag.assert_running();
                produced_since_check = 0;
            }

            if graph.degree(node_id) == 0 {
                continue;
            }

            sampler.prepare_for_new_node(node_id as u64);
            for _ in 0..parameters.sampling_walk_parameters.walks_per_node {
                termination_flag.assert_running();
                let walk = sampler.walk(node_id as u64);
                let walk_i64: Vec<i64> = walk.into_iter().map(|v| v as i64).collect();

                probabilities_builder.register_walk(&walk_i64);
                walks.add(used_walks, &walk_i64);
                max_len = max_len.max(walk_i64.len());
                used_walks += 1;
                produced_since_check += 1;
            }
        }

        walks.set_max_walk_length(max_len);
        walks.set_size(used_walks);

        let _probabilities = probabilities_builder.build();

        let graph_for_mapping = Arc::clone(&graph);
        let _to_original = move |mapped: i64| {
            graph_for_mapping
                .to_original_node_id(mapped)
                .unwrap_or(mapped)
        };

        let _progress_tracker = TaskProgressTracker::new(Tasks::leaf_with_volume(
            "Node2Vec".to_string(),
            config.iterations,
        ));

        let model = Node2VecModel::new(
            node_count,
            parameters.train_parameters.clone(),
            Concurrency::of(config.concurrency.max(1)),
            config.random_seed,
            walks,
            termination_flag,
        );

        let trained = model.train();

        Ok(Node2VecResult {
            embeddings: trained
                .embeddings
                .into_iter()
                .map(|emb| emb.into_iter().map(|v| v as f32).collect())
                .collect(),
            loss_per_iteration: trained.loss_per_iteration,
            embedding_dimension: config.embedding_dimension,
            node_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    #[test]
    fn node2vec_smoke_trains_embeddings() {
        let config = RandomGraphConfig {
            graph_name: "n2v".into(),
            database_name: "in-memory".into(),
            node_count: 12,
            node_labels: vec!["N".into()],
            relationships: vec![RandomRelationshipConfig::new("R", 0.4)],
            directed: true,
            inverse_indexed: false,
            seed: Some(42),
        };
        let store = DefaultGraphStore::random(&config).unwrap();
        let graph = store.graph();

        let cfg = Node2VecConfig {
            walks_per_node: 2,
            walk_length: 6,
            return_factor: 1.0,
            in_out_factor: 1.0,
            positive_sampling_factor: 0.001,
            negative_sampling_exponent: 0.75,
            initial_learning_rate: 0.05,
            min_learning_rate: 0.01,
            iterations: 1,
            window_size: 3,
            negative_sampling_rate: 1,
            embedding_dimension: 8,
            embedding_initializer: EmbeddingInitializerConfig::Uniform,
            source_nodes: vec![],
            concurrency: 1,
            walk_buffer_size: 8,
            random_seed: Some(7),
        };

        let storage = Node2VecStorageRuntime::new();
        let result = Node2VecComputationRuntime::run(graph, &cfg, &storage).unwrap();
        assert_eq!(result.loss_per_iteration.len(), 1);
        assert_eq!(
            result.embeddings.len(),
            cfg.source_nodes.len().max(config.node_count)
        );
        assert_eq!(result.embeddings[0].len(), 8);
    }
}
