use super::compressed_random_walks::CompressedRandomWalks;
use super::node2vec_model::Node2VecModel;
use super::node2vec_parameters::Node2VecParameters;
use super::node2vec_result::Node2VecResult;
use super::random_walk_probabilities::RandomWalkProbabilitiesBuilder;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::ProgressTracker;
use crate::ml::core::samplers::RandomWalkSampler;
use crate::types::graph::Graph;
use std::sync::Arc;

/// Node2Vec algorithm wrapper.
///
/// Java: `Node2Vec extends Algorithm<Node2VecResult>`
pub struct Node2Vec {
    graph: Arc<dyn Graph>,
    concurrency: Concurrency,
    source_nodes: Vec<i64>,
    maybe_random_seed: Option<u64>,
    walk_buffer_size: usize,
    parameters: Node2VecParameters,
    progress_tracker: ProgressTracker,
    termination_flag: TerminationFlag,
}

impl Node2Vec {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        graph: Arc<dyn Graph>,
        concurrency: Concurrency,
        source_nodes: Vec<i64>,
        maybe_random_seed: Option<u64>,
        walk_buffer_size: usize,
        parameters: Node2VecParameters,
        progress_tracker: ProgressTracker,
        termination_flag: TerminationFlag,
    ) -> Self {
        Self {
            graph,
            concurrency,
            source_nodes,
            maybe_random_seed,
            walk_buffer_size,
            parameters,
            progress_tracker,
            termination_flag,
        }
    }

    pub fn compute(self) -> Node2VecResult {
        // Weight validation (minimal): ensure non-negative weights if present.
        if self.graph.has_relationship_property() {
            let fallback = self.graph.default_property_value();
            for node in 0..self.graph.node_count() {
                for cursor in self.graph.stream_relationships_weighted(node as i64, fallback) {
                    if cursor.weight() < 0.0 {
                        panic!("Node2Vec only supports non-negative weights.");
                    }
                }
            }
        }

        let sampling = &self.parameters.sampling_walk_parameters;
        let node_count = self.graph.node_count();

        let probabilities_builder = RandomWalkProbabilitiesBuilder::new(
            node_count,
            self.concurrency,
            sampling.positive_sampling_factor,
            sampling.negative_sampling_exponent,
        );

        let max_walk_count = node_count.saturating_mul(sampling.walks_per_node);
        let mut walks = CompressedRandomWalks::new(max_walk_count);

        let random_seed = self.maybe_random_seed.unwrap_or(42);

        // Build a cumulative-weight supplier by summing outgoing relationship weights.
        let graph_for_weight = Arc::clone(&self.graph);
        let weight_fn = move |node_id: u64| -> f64 {
            let mut sum = 0.0;
            for cursor in graph_for_weight.stream_relationships(node_id as i64, 1.0) {
                sum += cursor.property();
            }
            sum
        };

        let nodes: Vec<i64> = if self.source_nodes.is_empty() {
            (0..node_count as i64).collect()
        } else {
            self.source_nodes.clone()
        };

        let mut sampler = RandomWalkSampler::create(
            Arc::clone(&self.graph),
            weight_fn,
            sampling.walk_length,
            sampling.return_factor,
            sampling.in_out_factor,
            random_seed,
        );

        let mut used_walks = 0usize;
        let mut max_len = 0usize;
        let mut produced_since_check = 0usize;
        let check_every = self.walk_buffer_size.max(1);

        for &node_id in &nodes {
            if produced_since_check >= check_every {
                self.termination_flag.assert_running();
                produced_since_check = 0;
            }

            if self.graph.degree(node_id) == 0 {
                continue;
            }

            sampler.prepare_for_new_node(node_id as u64);
            for _ in 0..sampling.walks_per_node {
                self.termination_flag.assert_running();
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

        let probabilities = probabilities_builder.build();

        let to_original = move |mapped: i64| {
            self.graph
                .to_original_node_id(mapped)
                .unwrap_or(mapped)
        };

        let model = Node2VecModel::new(
            to_original,
            node_count,
            self.parameters.train_parameters.clone(),
            self.concurrency,
            self.maybe_random_seed,
            walks,
            probabilities,
            self.progress_tracker,
            self.termination_flag,
        );

        model.train()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::{ProgressTracker, Tasks};
    use crate::procedures::embeddings::node2vec::sampling_walk_parameters::SamplingWalkParameters;
    use crate::procedures::embeddings::node2vec::train_parameters::{
        EmbeddingInitializer, TrainParameters,
    };
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

        let params = Node2VecParameters {
            sampling_walk_parameters: SamplingWalkParameters {
                walks_per_node: 2,
                walk_length: 6,
                return_factor: 1.0,
                in_out_factor: 1.0,
                positive_sampling_factor: 0.001,
                negative_sampling_exponent: 0.75,
            },
            train_parameters: TrainParameters {
                initial_learning_rate: 0.05,
                min_learning_rate: 0.01,
                iterations: 1,
                window_size: 3,
                negative_sampling_rate: 1,
                embedding_dimension: 8,
                embedding_initializer: EmbeddingInitializer::Uniform,
            },
        };

        let algo = Node2Vec::new(
            graph,
            Concurrency::of(1),
            vec![],
            Some(7),
            8,
            params,
            ProgressTracker::new(Tasks::Leaf("Node2Vec".to_string(), 1)),
            TerminationFlag::default(),
        );

        let result = algo.compute();
        assert_eq!(result.loss_per_iteration.len(), 1);
        assert_eq!(result.embeddings.size(), config.node_count);
        assert_eq!(result.embeddings.get(0).len(), 8);
    }
}


