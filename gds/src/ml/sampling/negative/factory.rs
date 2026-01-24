use super::NegativeSampler;
use super::RandomNegativeSampler;
use super::UserInputNegativeSampler;
use crate::projection::NodeLabel;
use crate::projection::RelationshipType;
use crate::types::graph::Graph;
use crate::types::graph::IdMap;
use crate::types::graph_store::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

/// Creates a NegativeSampler based on configuration.
/// 1:1 translation of NegativeSampler.of() from Java GDS.
///
/// If `negative_relationship_type` is provided, uses UserInputNegativeSampler.
/// Otherwise, uses RandomNegativeSampler.
#[allow(clippy::too_many_arguments)]
pub fn create_sampler<GS: GraphStore>(
    graph_store: Arc<GS>,
    graph: Arc<dyn Graph>,
    _source_and_target_node_labels: Vec<NodeLabel>,
    negative_relationship_type: Option<String>,
    negative_sampling_ratio: f64,
    test_positive_count: i64,
    train_positive_count: i64,
    valid_source_nodes: Arc<dyn IdMap>,
    valid_target_nodes: Arc<dyn IdMap>,
    source_labels: Vec<NodeLabel>,
    target_labels: Vec<NodeLabel>,
    random_seed: Option<u64>,
) -> Box<dyn NegativeSampler> {
    if let Some(rel_type_str) = negative_relationship_type {
        let rel_type = RelationshipType::of(rel_type_str.clone());
        let relationship_types = HashSet::from([rel_type]);

        let negative_example_graph = graph_store.get_graph_with_types(&relationship_types)
            .unwrap_or_else(|_| panic!("Failed to create filtered graph for negative sampling with relationship type: {}", rel_type_str));

        let total_positive = test_positive_count + train_positive_count;
        let test_train_fraction = if total_positive > 0 {
            test_positive_count as f64 / total_positive as f64
        } else {
            0.0
        };

        Box::new(UserInputNegativeSampler::new(
            negative_example_graph,
            test_train_fraction,
            random_seed,
            source_labels,
            target_labels,
        ))
    } else {
        let test_sample_count =
            scaled_negative_samples(test_positive_count, negative_sampling_ratio);
        let train_sample_count =
            scaled_negative_samples(train_positive_count, negative_sampling_ratio);

        Box::new(RandomNegativeSampler::new(
            graph,
            test_sample_count,
            train_sample_count,
            valid_source_nodes,
            valid_target_nodes,
            random_seed,
        ))
    }
}

fn scaled_negative_samples(count: i64, ratio: f64) -> usize {
    if count <= 0 {
        return 0;
    }
    let scaled = (count as f64 * ratio).max(0.0);
    scaled.floor() as usize
}
