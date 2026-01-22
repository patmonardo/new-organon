use crate::algo::similarity::{
    NodeSimilarityComputationRuntime, NodeSimilarityConfig, NodeSimilarityResult,
    NodeSimilarityStorageRuntime,
};
use crate::types::graph::graph::Graph;
use crate::types::graph::MappedNodeId;
use std::collections::HashSet;

pub fn compute_filtered_node_similarity(
    graph: &dyn Graph,
    config: &NodeSimilarityConfig,
    source_nodes: Option<&HashSet<MappedNodeId>>,
    target_nodes: Option<&HashSet<MappedNodeId>>,
) -> Vec<NodeSimilarityResult> {
    let node_count = graph.node_count();

    let target_mask: Option<Vec<bool>> = target_nodes.map(|targets| {
        let mut mask = vec![false; node_count];
        for &node_id in targets {
            let idx = node_id as usize;
            if idx < node_count {
                mask[idx] = true;
            }
        }
        mask
    });

    let sources: Vec<u64> = match source_nodes {
        Some(sources) => {
            let mut v: Vec<u64> = sources
                .iter()
                .copied()
                .filter(|&id| (id as usize) < node_count)
                .map(|id| id as u64)
                .collect();
            v.sort_unstable();
            v
        }
        None => (0..node_count as u64).collect(),
    };

    let can_enforce_ordering = match (source_nodes, target_nodes) {
        (Some(sources), Some(targets)) => sources.len() == targets.len() && sources == targets,
        _ => target_nodes.is_none(),
    };

    let storage = NodeSimilarityStorageRuntime::new(config.concurrency);
    let computation = NodeSimilarityComputationRuntime::new();

    let computed = storage.compute_with_filters(
        &computation,
        graph,
        config,
        sources,
        target_mask,
        can_enforce_ordering,
    );

    computed
        .into_iter()
        .map(NodeSimilarityResult::from)
        .collect()
}
