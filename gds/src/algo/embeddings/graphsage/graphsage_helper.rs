//! Java: `GraphSageHelper`.

use crate::collections::HugeObjectArray;
use crate::ml::core::features;
use crate::ml::core::features::BiasFeature;
use crate::ml::core::features::HugeObjectArrayFeatureConsumer;
use crate::ml::core::functions::NormalizeRows;
use crate::ml::core::neighborhood_function::NeighborhoodFunction;
use crate::ml::core::subgraph::{NeighborhoodSampler, SubGraph};
use crate::ml::core::variable::VariableRef;
use crate::types::graph::Graph;
use crate::types::schema::NodeLabel;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use super::layer::Layer;
use super::types::{ActivationFunctionType, AggregatorType, LayerConfig};

/// Java: `GraphSageHelper.embeddingsComputationGraph(...)`
pub fn embeddings_computation_graph(
    sub_graphs: &[SubGraph],
    layers: &[Arc<dyn Layer>],
    batched_features_extractor: VariableRef,
) -> VariableRef {
    let mut previous_layer = batched_features_extractor;

    // Java:
    // for (int layerNr = layers.length - 1; layerNr >= 0; layerNr--) {
    //   Layer layer = layers[layers.length - layerNr - 1];
    //   previous = layer.aggregator().aggregate(previous, subGraphs.get(layerNr));
    // }
    for layer_nr in (0..layers.len()).rev() {
        let layer_idx = layers.len() - layer_nr - 1;
        let layer = &layers[layer_idx];
        previous_layer = layer
            .aggregator()
            .aggregate(previous_layer, &sub_graphs[layer_nr]);
    }

    Arc::new(NormalizeRows::new_ref(previous_layer))
}

/// Java: `GraphSageHelper.subGraphsPerLayer(...)`
pub fn sub_graphs_per_layer(
    graph: Arc<dyn Graph>,
    node_ids: &[u64],
    layers: &[Arc<dyn Layer>],
    random_seed: u64,
) -> Vec<SubGraph> {
    let mut rng = ChaCha8Rng::seed_from_u64(random_seed);

    let mut samplers: Vec<Arc<dyn NeighborhoodFunction>> = layers
        .iter()
        .map(|layer| {
            let sampler = NeighborhoodSampler::new(rng.gen());
            Arc::new(GraphNeighborhoodFunction {
                graph: Arc::clone(&graph),
                sampler,
                sample_size: layer.sample_size(),
            }) as Arc<dyn NeighborhoodFunction>
        })
        .collect();

    samplers.reverse();

    let (weight_function, weighted) = SubGraph::relationship_weight_function(graph.as_ref());
    SubGraph::build_sub_graphs(node_ids, &samplers, weight_function, weighted)
}

/// Java: `GraphSageHelper.initializeSingleLabelFeatures(...)`
pub fn initialize_single_label_features(
    graph: &dyn Graph,
    feature_properties: &[String],
) -> HugeObjectArray<Vec<f64>> {
    let features = HugeObjectArray::new(graph.node_count());
    let extractors = features::property_extractors(graph, feature_properties);
    features::extract_graph(graph, &extractors, features)
}

/// Java: `GraphSageHelper.multiLabelFeatureExtractors(...)`
pub struct MultiLabelFeatureExtractors {
    pub feature_count_per_label: HashMap<NodeLabel, usize>,
    pub extractors_per_label: HashMap<NodeLabel, Vec<features::AnyFeatureExtractor>>,
}

/// Java: `GraphSageHelper.initializeMultiLabelFeatures(...)`
pub fn initialize_multi_label_features(
    graph: &dyn Graph,
    feature_properties: &[String],
) -> HugeObjectArray<Vec<f64>> {
    let multi = multi_label_feature_extractors(graph, feature_properties);

    let features_out: HugeObjectArray<Vec<f64>> = HugeObjectArray::new(graph.node_count());
    let mut consumer = HugeObjectArrayFeatureConsumer::new(features_out);

    for node_id in 0..graph.node_count() {
        let node_label = label_of(graph, node_id as u64);
        let extractors = multi
            .extractors_per_label
            .get(&node_label)
            .unwrap_or_else(|| panic!("missing extractors for label `{}`", node_label.name()));
        let feature_count = *multi
            .feature_count_per_label
            .get(&node_label)
            .unwrap_or_else(|| panic!("missing featureCount for label `{}`", node_label.name()));

        consumer
            .features_mut()
            .set(node_id, vec![0.0f64; feature_count]);
        features::extract(node_id as u64, node_id as u64, extractors, &mut consumer);
    }

    consumer.into_inner()
}

pub fn multi_label_feature_extractors(
    graph: &dyn Graph,
    feature_properties: &[String],
) -> MultiLabelFeatureExtractors {
    let filtered_keys_per_label = filtered_property_keys_per_node_label(graph, feature_properties);

    let mut feature_count_per_label: HashMap<NodeLabel, usize> = HashMap::new();
    let mut extractors_per_label: HashMap<NodeLabel, Vec<features::AnyFeatureExtractor>> =
        HashMap::new();

    for node_id in 0..graph.node_count() {
        let node_label = label_of(graph, node_id as u64);

        extractors_per_label
            .entry(node_label.clone())
            .or_insert_with(|| {
                let property_keys = filtered_keys_per_label
                    .get(&node_label)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .collect::<Vec<_>>();

                let mut extractors =
                    features::property_extractors_with_init(graph, &property_keys, node_id as u64);
                extractors.push(features::AnyFeatureExtractor::Scalar(Box::new(BiasFeature)));
                extractors
            });

        feature_count_per_label
            .entry(node_label.clone())
            .or_insert_with(|| {
                let xs = extractors_per_label.get(&node_label).unwrap();
                features::feature_count(xs)
            });
    }

    MultiLabelFeatureExtractors {
        feature_count_per_label,
        extractors_per_label,
    }
}

fn filtered_property_keys_per_node_label(
    graph: &dyn Graph,
    feature_properties: &[String],
) -> HashMap<NodeLabel, HashSet<String>> {
    let mut out = HashMap::new();
    for entry in graph.schema().node_schema().entries() {
        let label = NodeLabel::of(entry.identifier().name());
        let available: HashSet<String> = entry.properties().keys().cloned().collect();
        let filtered: HashSet<String> = feature_properties
            .iter()
            .filter(|k| available.contains(*k))
            .cloned()
            .collect();
        out.insert(label, filtered);
    }
    out
}

fn label_of(graph: &dyn Graph, node_id: u64) -> NodeLabel {
    let mut label_ref: Option<NodeLabel> = None;
    let mut count = 0usize;
    graph.for_each_node_label(node_id as i64, &mut |node_label: &NodeLabel| {
        count += 1;
        if count == 1 {
            label_ref = Some(node_label.clone());
            true
        } else {
            false
        }
    });

    if count != 1 {
        panic!(
            "Each node must have exactly one label: nodeId={}, labels={:?}",
            node_id,
            graph.node_labels(node_id as i64)
        );
    }
    label_ref.expect("label missing")
}

/// Java: `GraphSageHelper.layerConfigs(...)`
pub fn layer_configs(
    feature_dimension: usize,
    sample_sizes: &[usize],
    random_seed: Option<u64>,
    aggregator_type: AggregatorType,
    activation_function: ActivationFunctionType,
    embedding_dimension: usize,
) -> Vec<LayerConfig> {
    let mut rng = ChaCha8Rng::seed_from_u64(random_seed.unwrap_or(0));

    let mut result = Vec::with_capacity(sample_sizes.len());
    for (i, &sample_size) in sample_sizes.iter().enumerate() {
        let cols = if i == 0 {
            feature_dimension
        } else {
            embedding_dimension
        };
        result.push(LayerConfig {
            rows: embedding_dimension,
            cols,
            sample_size,
            random_seed: rng.gen(),
            bias: None,
            aggregator_type,
            activation_function,
        });
    }
    result
}

// ---------------------------------------------------------------------------
// Internal helpers (SubGraph builder)
// ---------------------------------------------------------------------------

struct GraphNeighborhoodFunction {
    graph: Arc<dyn Graph>,
    sampler: NeighborhoodSampler,
    sample_size: usize,
}

impl NeighborhoodFunction for GraphNeighborhoodFunction {
    fn sample(&self, node_id: u64) -> Box<dyn Iterator<Item = u64> + '_> {
        let sampled = self
            .sampler
            .sample(self.graph.as_ref(), node_id, self.sample_size);
        Box::new(sampled.into_iter())
    }
}
