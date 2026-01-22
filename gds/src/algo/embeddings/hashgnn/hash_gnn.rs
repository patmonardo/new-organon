use crate::collections::HugeObjectArray;
use crate::concurrency::TerminationFlag;
use crate::core::utils::paged::HugeAtomicBitSet;
use crate::core::utils::partition::{DegreeFunction, DegreePartition, Partition, PartitionUtils};
use crate::core::utils::progress::TaskProgressTracker;
use crate::types::graph::Graph;
use std::collections::HashSet;
use std::sync::Arc;

use super::densify_task::DensifyTask;
use super::embeddings_to_node_property_values::EmbeddingsToNodePropertyValues;
use super::generate_features_task::GenerateFeaturesTask;
use super::hash_gnn_parameters::HashGNNParameters;
use super::hash_gnn_result::HashGNNResult;
use super::hash_task::HashTask;
use super::min_hash_task::MinHashTask;

/// Java: `HashGNN.MinAndArgmin`
#[derive(Debug, Default, Clone, Copy)]
pub struct MinAndArgmin {
    pub min: i32,
    pub arg_min: i32,
}

/// Java: `HashGNN extends Algorithm<HashGNNResult>`
pub struct HashGNN {
    graph: Arc<dyn Graph>,
    parameters: HashGNNParameters,
    _progress_tracker: TaskProgressTracker,
    termination_flag: TerminationFlag,
}

impl HashGNN {
    pub fn new(
        graph: Arc<dyn Graph>,
        parameters: HashGNNParameters,
        progress_tracker: TaskProgressTracker,
        termination_flag: TerminationFlag,
    ) -> Self {
        Self {
            graph,
            parameters,
            _progress_tracker: progress_tracker,
            termination_flag,
        }
    }

    pub fn compute(self) -> HashGNNResult {
        let node_count = self.graph.node_count();
        let concurrency = self.parameters.concurrency;
        let random_seed = self.parameters.random_seed.unwrap_or(42);

        // Degree + range partitions (close to Java; we use our core PartitionUtils).
        let decreased_concurrency = (concurrency.value() * 4).min(node_count).max(1);

        let degree_partitions: Vec<DegreePartition> = PartitionUtils::degree_partition(
            node_count,
            self.graph.relationship_count(),
            Box::new(GraphDegrees {
                graph: Arc::clone(&self.graph),
            }),
            decreased_concurrency,
            |p| p,
            Some(1),
        );

        let range_partitions: Vec<Partition> =
            PartitionUtils::range_partition(concurrency.value(), node_count, |p| p, Some(1));

        // Build filtered graphs list for heterogeneous mode.
        let graphs: Vec<Arc<dyn Graph>> = if self.parameters.heterogeneous {
            let schema_types = self.graph.schema().relationship_schema().available_types();
            schema_types
                .into_iter()
                .map(|rt| {
                    let set: HashSet<_> = vec![rt].into_iter().collect();
                    self.graph
                        .relationship_type_filtered_graph(&set)
                        .expect("relationship type filtered graph")
                })
                .collect()
        } else {
            vec![<dyn Graph as Graph>::concurrent_copy(&*self.graph)]
        };

        // Construct input embeddings.
        let (embeddings_b, mut current_total_feature_count) =
            self.construct_input_embeddings(&range_partitions, random_seed);

        let embedding_dimension = embeddings_b
            .get(0)
            .as_ref()
            .expect("embeddings not initialized")
            .size();

        // Second cache (A) with same dimension, initialized to empty bitsets.
        let mut embeddings_a: HugeObjectArray<Option<Arc<HugeAtomicBitSet>>> =
            HugeObjectArray::new(node_count);
        embeddings_a.set_all(|_| Some(Arc::new(HugeAtomicBitSet::new(embedding_dimension))));

        // Upper bound expectation used for scaling neighbor influence (from Java).
        let avg_degree = if node_count == 0 {
            0.0
        } else {
            self.graph.relationship_count() as f64 / node_count as f64
        };
        let upper_bound_neighbor_expected_bits = if embedding_dimension == 0 {
            1.0
        } else {
            embedding_dimension as f64
                * (1.0f64 - (1.0f64 - (1.0f64 / embedding_dimension as f64)).powf(avg_degree))
        };

        for iteration in 0..self.parameters.iterations {
            self.termination_flag.assert_running();

            let (current_embeddings, previous_embeddings) = if iteration % 2 == 0 {
                (&embeddings_a, &embeddings_b)
            } else {
                (&embeddings_b, &embeddings_a)
            };

            // Clear current embeddings.
            for i in 0..node_count {
                current_embeddings
                    .get(i)
                    .as_ref()
                    .expect("current embedding not initialized")
                    .clear();
            }

            let scaled_neighbor_influence = if self.graph.relationship_count() == 0 {
                1.0
            } else {
                (current_total_feature_count as f64 / node_count.max(1) as f64)
                    * self.parameters.neighbor_influence
                    / upper_bound_neighbor_expected_bits
            };
            current_total_feature_count = 0;

            let hashes = HashTask::compute(
                embedding_dimension,
                scaled_neighbor_influence,
                graphs.len(),
                concurrency,
                self.parameters.embedding_density,
                random_seed + (self.parameters.embedding_density as u64) * iteration as u64,
                &self.termination_flag,
            );

            let added = MinHashTask::compute(
                &degree_partitions,
                &graphs,
                concurrency,
                self.parameters.embedding_density,
                embedding_dimension,
                current_embeddings,
                previous_embeddings,
                &hashes,
                &self.termination_flag,
            );
            current_total_feature_count += added;
        }

        let binary_output_vectors =
            if (self.parameters.iterations.saturating_sub(1)).is_multiple_of(2) {
                embeddings_a
            } else {
                embeddings_b
            };

        let embeddings = if let Some(out_dim) = self.parameters.output_dimension {
            let dense = DensifyTask::compute(
                range_partitions,
                out_dim,
                random_seed,
                &binary_output_vectors,
            );
            EmbeddingsToNodePropertyValues::from_dense(dense)
        } else {
            EmbeddingsToNodePropertyValues::from_binary(binary_output_vectors, embedding_dimension)
        };

        HashGNNResult { embeddings }
    }

    fn construct_input_embeddings(
        &self,
        partitions: &[Partition],
        random_seed: u64,
    ) -> (HugeObjectArray<Option<Arc<HugeAtomicBitSet>>>, u64) {
        // Translation note: FeatureProperties path depends on FeatureExtraction plumbing.
        if !self.parameters.feature_properties.is_empty() {
            if self.parameters.binarize_features.is_some() {
                let cfg = self.parameters.binarize_features.clone().unwrap();
                return super::binarize_task::BinarizeTask::compute(
                    cfg,
                    Arc::clone(&self.graph),
                    partitions.to_vec(),
                    self.parameters.feature_properties.clone(),
                    random_seed,
                    &self.termination_flag,
                );
            } else {
                return super::raw_features_task::RawFeaturesTask::compute(
                    Arc::clone(&self.graph),
                    partitions.to_vec(),
                    self.parameters.feature_properties.clone(),
                    &self.termination_flag,
                );
            }
        }

        // GenerateFeatures must be present when featureProperties is empty.
        let cfg = self
            .parameters
            .generate_features
            .clone()
            .expect("generate_features must be provided when feature_properties is empty");

        GenerateFeaturesTask::compute(
            cfg,
            Arc::clone(&self.graph),
            partitions.to_vec(),
            random_seed,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::hash_gnn_parameters::GenerateFeaturesConfig;
    use super::*;
    use crate::collections::backends::vec::VecLong;
    use crate::concurrency::Concurrency;
    use crate::core::utils::progress::{TaskProgressTracker, Tasks};
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::graph_store::GraphStore;
    use crate::types::properties::node::DefaultLongNodePropertyValues;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};
    use crate::types::schema::NodeLabel;
    use std::collections::HashSet;

    #[test]
    fn hashgnn_smoke_generate_features_binary_output() {
        let config = RandomGraphConfig {
            graph_name: "hgnn".into(),
            database_name: "in-memory".into(),
            node_count: 20,
            node_labels: vec!["N".into()],
            relationships: vec![RandomRelationshipConfig::new("R", 0.3)],
            directed: true,
            inverse_indexed: false,
            seed: Some(42),
        };
        let store = DefaultGraphStore::random(&config).unwrap();
        let graph = store.graph();

        let params = super::super::hash_gnn_parameters::HashGNNParameters {
            concurrency: Concurrency::of(1),
            iterations: 2,
            embedding_density: 4,
            neighbor_influence: 1.0,
            feature_properties: vec![],
            heterogeneous: false,
            output_dimension: None,
            binarize_features: None,
            generate_features: Some(GenerateFeaturesConfig {
                dimension: 64,
                density_level: 3,
            }),
            random_seed: Some(7),
        };

        let algo = HashGNN::new(
            graph,
            params,
            TaskProgressTracker::new(Tasks::leaf_with_volume("HashGNN".to_string(), 1)),
            TerminationFlag::default(),
        );

        let result = algo.compute();
        match result.embeddings {
            super::super::hash_gnn_result::HashGNNEmbeddings::Binary {
                embeddings,
                embedding_dimension,
            } => {
                assert_eq!(embeddings.size(), config.node_count);
                assert_eq!(embedding_dimension, 64);
                // At least the structures exist; density depends on random + propagation.
                assert_eq!(
                    embeddings.get(0).as_ref().expect("bitset missing").size(),
                    64
                );
            }
            _ => panic!("expected binary output"),
        }
    }

    #[test]
    fn hashgnn_smoke_feature_properties_raw_binary_output() {
        let config = RandomGraphConfig {
            graph_name: "hgnn-props".into(),
            database_name: "in-memory".into(),
            node_count: 20,
            node_labels: vec!["N".into()],
            relationships: vec![RandomRelationshipConfig::new("R", 0.3)],
            directed: true,
            inverse_indexed: false,
            seed: Some(42),
        };
        let mut store = DefaultGraphStore::random(&config).unwrap();

        // Add a simple 0/1 feature property (required by RawFeaturesTask).
        let data: Vec<i64> = (0..config.node_count)
            .map(|i| if i % 2 == 0 { 1 } else { 0 })
            .collect();
        let values =
            DefaultLongNodePropertyValues::from_collection(VecLong::from(data), config.node_count);
        let mut labels = HashSet::new();
        labels.insert(NodeLabel::of("N"));
        store
            .add_node_property(labels, "feat", Arc::new(values))
            .unwrap();

        let graph = store.graph();

        let params = super::super::hash_gnn_parameters::HashGNNParameters {
            concurrency: Concurrency::of(1),
            iterations: 1,
            embedding_density: 4,
            neighbor_influence: 1.0,
            feature_properties: vec!["feat".to_string()],
            heterogeneous: false,
            output_dimension: None,
            binarize_features: None,
            generate_features: None,
            random_seed: Some(7),
        };

        let algo = HashGNN::new(
            graph,
            params,
            TaskProgressTracker::new(Tasks::leaf_with_volume("HashGNN".to_string(), 1)),
            TerminationFlag::default(),
        );

        let result = algo.compute();
        match result.embeddings {
            super::super::hash_gnn_result::HashGNNEmbeddings::Binary { embeddings, .. } => {
                // Just ensure we have an embedding for every node.
                assert_eq!(embeddings.size(), config.node_count);
                for i in 0..config.node_count {
                    assert!(embeddings.get(i).is_some());
                }
            }
            _ => panic!("expected binary embeddings"),
        }
    }
}

struct GraphDegrees {
    graph: Arc<dyn Graph>,
}

impl DegreeFunction for GraphDegrees {
    fn degree(&self, node_id: usize) -> usize {
        self.graph.degree(node_id as i64)
    }
}
