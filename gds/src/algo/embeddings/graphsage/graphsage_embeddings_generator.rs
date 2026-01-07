//! Java: `GraphSageEmbeddingsGenerator`.

use crate::collections::HugeObjectArray;
use crate::concurrency::virtual_threads::RunWithConcurrency;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::partition::PartitionUtils;
use crate::core::utils::progress::TaskProgressTracker;
use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::tensor::Matrix;
use crate::types::graph::Graph;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::sync::Arc;

use super::feature_function::FeatureFunction;
use super::graphsage_helper;
use super::layer::Layer;

#[allow(dead_code)]
pub struct GraphSageEmbeddingsGenerator {
    layers: Vec<Arc<dyn Layer>>,
    batch_size: usize,
    concurrency: Concurrency,
    feature_function: Arc<dyn FeatureFunction>,
    random_seed: u64,
    progress_tracker: TaskProgressTracker,
    termination_flag: TerminationFlag,
}

impl GraphSageEmbeddingsGenerator {
    pub fn new(
        layers: Vec<Arc<dyn Layer>>,
        batch_size: usize,
        concurrency: Concurrency,
        feature_function: Arc<dyn FeatureFunction>,
        random_seed: Option<u64>,
        progress_tracker: TaskProgressTracker,
        termination_flag: TerminationFlag,
    ) -> Self {
        let seed = random_seed.unwrap_or_else(|| {
            // stable fallback; Java uses ThreadLocalRandom; here we derive from RNG.
            ChaCha8Rng::from_entropy().gen()
        });
        Self {
            layers,
            batch_size,
            concurrency,
            feature_function,
            random_seed: seed,
            progress_tracker,
            termination_flag,
        }
    }

    pub fn make_embeddings(
        &self,
        graph: Arc<dyn Graph>,
        features: Arc<HugeObjectArray<Vec<f64>>>,
    ) -> HugeObjectArray<Vec<f64>> {
        // Progress tracker integration is deferred in Rust GDS; keep the value live for parity.
        let _ = &self.progress_tracker;

        let mut result: HugeObjectArray<Vec<f64>> = HugeObjectArray::new(graph.node_count());

        // initialize rows to correct length using embedding dim from the last layer weight rows
        // (falls back to 0 if layers empty).
        let embedding_dim = self
            .layers
            .first()
            .map(|_| {
                // Infer from first aggregator's weights (rows).
                // In Java the weights are (rows=embeddingDim, cols=inputDim).
                // We'll infer at runtime from computed embeddings instead; initialize empty here.
                0usize
            })
            .unwrap_or(0);
        if embedding_dim > 0 {
            result.set_all(|_| vec![0.0; embedding_dim]);
        }

        let result = Arc::new(parking_lot::Mutex::new(result));

        let layers = self.layers.clone();
        let tasks: Vec<Box<dyn FnOnce() + Send>> = PartitionUtils::range_partition_with_batch_size(
            graph.node_count(),
            self.batch_size,
            |partition| {
                let graph = <dyn Graph as Graph>::concurrent_copy(&*graph);
                let layers = layers.clone();
                // We cannot clone trait objects; we re-borrow from &self inside the closure by cloning Arc pointers.
                let feature_function = Arc::clone(&self.feature_function);
                let features = Arc::clone(&features);
                let result = Arc::clone(&result);
                let termination_flag = self.termination_flag.clone();
                let random_seed = self.random_seed;

                Box::new(move || {
                    termination_flag.assert_running();

                    let node_ids: Vec<u64> = partition.iter().map(|n| n as u64).collect();
                    let sub_graphs = graphsage_helper::sub_graphs_per_layer(
                        Arc::clone(&graph),
                        &node_ids,
                        &layers,
                        random_seed,
                    );

                    let deepest = sub_graphs
                        .last()
                        .map(|s| s.original_node_ids())
                        .unwrap_or(&node_ids);

                    let batched_features =
                        feature_function.apply(Arc::clone(&graph), deepest, Arc::clone(&features));

                    let embedding_var = graphsage_helper::embeddings_computation_graph(
                        &sub_graphs,
                        &layers,
                        batched_features,
                    );

                    let embeddings_tensor =
                        ComputationContext::new().forward(embedding_var.as_ref());
                    let embeddings = embeddings_tensor
                        .as_any()
                        .downcast_ref::<Matrix>()
                        .expect("expected Matrix embeddings");

                    // write back in partition order
                    let mut out = result.lock();
                    let start = partition.start_node();
                    for (i, node_id) in (start..start + partition.node_count()).enumerate() {
                        out.set(node_id, embeddings.get_row(i).to_vec());
                    }
                }) as Box<dyn FnOnce() + Send>
            },
        );

        RunWithConcurrency::builder()
            .concurrency(self.concurrency)
            .tasks(tasks)
            .termination_flag(self.termination_flag.clone())
            .run()
            .expect("run with concurrency");

        match Arc::try_unwrap(result) {
            Ok(mutex) => mutex.into_inner(),
            Err(_) => panic!("result still referenced"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algo::embeddings::graphsage::graphsage_helper;
    use crate::algo::embeddings::graphsage::layer_factory::generate_weights;
    use crate::algo::embeddings::graphsage::layer_factory::LayerFactory;
    use crate::algo::embeddings::graphsage::multi_label_feature_function::MultiLabelFeatureFunction;
    use crate::algo::embeddings::graphsage::single_label_feature_function::SingleLabelFeatureFunction;
    use crate::algo::embeddings::graphsage::types::{
        ActivationFunctionType, AggregatorType, LayerConfig,
    };
    use crate::collections::backends::vec::VecDouble;
    use crate::core::utils::progress::{TaskProgressTracker, Tasks};
    use crate::types::graph_store::{DefaultGraphStore, GraphStore};
    use crate::types::properties::node::DefaultDoubleNodePropertyValues;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};
    use crate::types::schema::NodeLabel;
    use std::collections::HashMap;
    use std::collections::HashSet;

    #[test]
    fn graphsage_smoke_single_label_embeddings() {
        let config = RandomGraphConfig {
            graph_name: "graphsage".into(),
            database_name: "in-memory".into(),
            node_count: 30,
            node_labels: vec!["N".into()],
            relationships: vec![RandomRelationshipConfig::new("R", 0.2)],
            directed: true,
            inverse_indexed: false,
            seed: Some(42),
        };
        let mut store = DefaultGraphStore::random(&config).unwrap();

        // Add a numeric node feature property.
        let values: Vec<f64> = (0..config.node_count).map(|i| i as f64).collect();
        let prop = DefaultDoubleNodePropertyValues::from_collection(
            VecDouble::from(values),
            config.node_count,
        );
        let mut labels = HashSet::new();
        labels.insert(NodeLabel::of("N"));
        store
            .add_node_property(labels, "x", Arc::new(prop))
            .unwrap();

        let graph = store.graph();
        let feature_properties = vec!["x".to_string()];
        let features = Arc::new(graphsage_helper::initialize_single_label_features(
            graph.as_ref(),
            &feature_properties,
        ));

        // One mean layer.
        let embedding_dim = 8;
        let layer_cfg = LayerConfig {
            rows: embedding_dim,
            cols: 1,
            sample_size: 5,
            random_seed: 7,
            bias: None,
            aggregator_type: AggregatorType::Mean,
            activation_function: ActivationFunctionType::Relu,
        };
        let layers = vec![LayerFactory::create_layer(&layer_cfg)];

        let gen = GraphSageEmbeddingsGenerator::new(
            layers,
            10,
            Concurrency::of(1),
            Arc::new(SingleLabelFeatureFunction),
            Some(123),
            TaskProgressTracker::new(Tasks::leaf_with_volume("GraphSage".to_string(), 1)),
            TerminationFlag::default(),
        );

        let embeddings = gen.make_embeddings(graph, features);
        assert_eq!(embeddings.size(), config.node_count);
        assert_eq!(embeddings.get(0).len(), embedding_dim);
    }

    #[test]
    fn graphsage_smoke_multi_label_embeddings() {
        let config = RandomGraphConfig {
            graph_name: "graphsage-mlabel".into(),
            database_name: "in-memory".into(),
            node_count: 40,
            node_labels: vec!["A".into(), "B".into()],
            relationships: vec![RandomRelationshipConfig::new("R", 0.2)],
            directed: true,
            inverse_indexed: false,
            seed: Some(42),
        };
        let store = DefaultGraphStore::random(&config).unwrap();
        let graph = store.graph();

        // Use per-label schema property (random graph generator adds `random_score` to each label).
        let feature_properties = vec!["random_score".to_string()];
        let features = Arc::new(graphsage_helper::initialize_multi_label_features(
            graph.as_ref(),
            &feature_properties,
        ));

        // Multi-label feature projection output dimension.
        let projected_dim = 4;

        // Each label gets its own projection weights (projected_dim x (features+bias)).
        let feature_dim_with_bias = 2; // random_score + bias
        let mut weights_by_label = HashMap::new();
        weights_by_label.insert(
            NodeLabel::of("A"),
            Arc::new(generate_weights(
                projected_dim,
                feature_dim_with_bias,
                0.5,
                7,
            )),
        );
        weights_by_label.insert(
            NodeLabel::of("B"),
            Arc::new(generate_weights(
                projected_dim,
                feature_dim_with_bias,
                0.5,
                11,
            )),
        );

        let feature_fn = Arc::new(MultiLabelFeatureFunction::new(
            weights_by_label,
            projected_dim,
        ));

        // One mean layer that maps projected_dim -> embedding_dim.
        let embedding_dim = 8;
        let layer_cfg = LayerConfig {
            rows: embedding_dim,
            cols: projected_dim,
            sample_size: 5,
            random_seed: 19,
            bias: None,
            aggregator_type: AggregatorType::Mean,
            activation_function: ActivationFunctionType::Relu,
        };
        let layers = vec![LayerFactory::create_layer(&layer_cfg)];

        let gen = GraphSageEmbeddingsGenerator::new(
            layers,
            10,
            Concurrency::of(1),
            feature_fn,
            Some(123),
            TaskProgressTracker::new(Tasks::leaf_with_volume("GraphSage".to_string(), 1)),
            TerminationFlag::default(),
        );

        let embeddings = gen.make_embeddings(graph, features);
        assert_eq!(embeddings.size(), config.node_count);
        assert_eq!(embeddings.get(0).len(), embedding_dim);
    }
}
