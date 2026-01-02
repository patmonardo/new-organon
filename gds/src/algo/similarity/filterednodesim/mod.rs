use crate::algo::similarity::nodesim::NodeSimilarityMetric;
use crate::algo::similarity::{NodeSimilarityConfig, NodeSimilarityResult};
use crate::types::graph::graph::Graph;
use crate::types::graph::MappedNodeId;
use rayon::prelude::*;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::sync::Arc;

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

    let metric_inner = config.similarity_metric.create(config.similarity_cutoff);
    let metric: Arc<dyn crate::algo::similarity::nodesim::similarity_metric::SimilarityMetric> =
        Arc::from(metric_inner);

    #[derive(PartialEq)]
    struct ScoredNode(f64, u64);

    impl Eq for ScoredNode {}

    impl PartialOrd for ScoredNode {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for ScoredNode {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0
                .partial_cmp(&other.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| self.1.cmp(&other.1))
        }
    }

    sources
        .par_iter()
        .flat_map(|&source| {
            let _metric = Arc::clone(&metric);
            let source_mapped = source as MappedNodeId;
            let source_degree = graph.degree(source_mapped);

            if source_degree == 0 {
                return Vec::new();
            }

            let mut overlaps: std::collections::HashMap<u64, f64> =
                std::collections::HashMap::with_capacity(source_degree);

            let relationships =
                graph.stream_relationships(source_mapped, graph.default_property_value());

            for rel in relationships {
                let neighbor_node = rel.target_id();

                let inverse_relationships = graph.stream_inverse_relationships(
                    neighbor_node,
                    graph.default_property_value(),
                );

                for inv_rel in inverse_relationships {
                    let potential_target = inv_rel.source_id();
                    if potential_target == source_mapped {
                        continue;
                    }

                    if let Some(mask) = &target_mask {
                        let idx = potential_target as usize;
                        if idx >= mask.len() || !mask[idx] {
                            continue;
                        }
                    }

                    let potential_target_u64 = potential_target as u64;

                    if can_enforce_ordering && potential_target_u64 <= source {
                        continue;
                    }

                    *overlaps.entry(potential_target_u64).or_insert(0.0) += 1.0;
                }
            }

            let mut heap = std::collections::BinaryHeap::new();

            for (target, intersection) in overlaps {
                let target_degree = graph.degree(target as MappedNodeId);

                let sim = match config.similarity_metric {
                    NodeSimilarityMetric::Jaccard => {
                        let union = (source_degree as f64 + target_degree as f64) - intersection;
                        if union > 0.0 {
                            intersection / union
                        } else {
                            0.0
                        }
                    }
                    NodeSimilarityMetric::Overlap => {
                        let min_degree = (source_degree as f64).min(target_degree as f64);
                        if min_degree > 0.0 {
                            intersection / min_degree
                        } else {
                            0.0
                        }
                    }
                    NodeSimilarityMetric::Cosine => {
                        let sqrt_prod = ((source_degree as f64) * (target_degree as f64)).sqrt();
                        if sqrt_prod > 0.0 {
                            intersection / sqrt_prod
                        } else {
                            0.0
                        }
                    }
                };

                if sim >= config.similarity_cutoff {
                    let limit = if config.top_k == 0 {
                        usize::MAX
                    } else {
                        config.top_k
                    };

                    let item = Reverse(ScoredNode(sim, target));

                    if heap.len() < limit {
                        heap.push(item);
                    } else if let Some(min_top) = heap.peek() {
                        if sim > min_top.0 .0 {
                            heap.pop();
                            heap.push(item);
                        }
                    }
                }
            }

            heap.into_sorted_vec()
                .into_iter()
                .map(|Reverse(ScoredNode(sim, target))| NodeSimilarityResult {
                    source,
                    target,
                    similarity: sim,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
