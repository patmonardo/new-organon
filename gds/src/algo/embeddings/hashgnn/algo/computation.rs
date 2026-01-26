//! HashGNN computation runtime.
//!
//! This is the **Subtle pole**: ephemeral computation state for HashGNN.

use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{TaskProgressTracker, Tasks};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::Graph;
use std::sync::Arc;

use super::spec::{HashGNNConfig, HashGNNEmbeddings, HashGNNResult};
use super::HashGNNStorageRuntime;

/// Runs the HashGNN algorithm.
#[derive(Debug, Default, Clone)]
pub struct HashGNNComputationRuntime;

impl HashGNNComputationRuntime {
    pub fn run(
        graph: Arc<dyn Graph>,
        config: &HashGNNConfig,
        storage: &HashGNNStorageRuntime,
    ) -> Result<HashGNNResult, AlgorithmError> {
        if !config.feature_properties.is_empty() {
            storage
                .validate_feature_properties(graph.as_ref(), &config.feature_properties)
                .map_err(AlgorithmError::Execution)?;
        }

        let params = super::super::hash_gnn_parameters::HashGNNParameters {
            concurrency: Concurrency::of(config.concurrency.max(1)),
            iterations: config.iterations,
            embedding_density: config.embedding_density,
            neighbor_influence: config.neighbor_influence,
            feature_properties: config.feature_properties.clone(),
            heterogeneous: config.heterogeneous,
            output_dimension: config.output_dimension,
            binarize_features: config.binarize_features.clone().map(|cfg| {
                super::super::hash_gnn_parameters::BinarizeFeaturesConfig {
                    dimension: cfg.dimension,
                    threshold: cfg.threshold,
                }
            }),
            generate_features: config.generate_features.clone().map(|cfg| {
                super::super::hash_gnn_parameters::GenerateFeaturesConfig {
                    dimension: cfg.dimension,
                    density_level: cfg.density_level,
                }
            }),
            random_seed: config.random_seed,
        };

        let algo = super::super::hash_gnn::HashGNN::new(
            graph,
            params,
            TaskProgressTracker::new(Tasks::leaf_with_volume("HashGNN".to_string(), 1)),
            TerminationFlag::default(),
        );

        let raw = algo.compute();

        let embeddings = match raw.embeddings {
            super::super::hash_gnn_result::HashGNNEmbeddings::Binary {
                embeddings,
                embedding_dimension,
            } => {
                let node_count = embeddings.size();
                let mut out: Vec<Vec<u32>> = Vec::with_capacity(node_count);
                for i in 0..node_count {
                    let mut indices: Vec<u32> = Vec::new();
                    if let Some(bitset) = embeddings.get(i).as_ref() {
                        bitset.for_each_set_bit(|bit| indices.push(bit as u32));
                    }
                    out.push(indices);
                }

                HashGNNEmbeddings::BinaryIndices {
                    embedding_dimension,
                    embeddings: out,
                }
            }
            super::super::hash_gnn_result::HashGNNEmbeddings::Dense { embeddings } => {
                let node_count = embeddings.size();
                let mut out: Vec<Vec<f32>> = Vec::with_capacity(node_count);
                for i in 0..node_count {
                    let row = embeddings.get(i);
                    out.push(row.iter().map(|v| *v as f32).collect());
                }
                HashGNNEmbeddings::Dense { embeddings: out }
            }
        };

        Ok(HashGNNResult { embeddings })
    }
}
