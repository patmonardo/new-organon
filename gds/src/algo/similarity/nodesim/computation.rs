use super::similarity_metric::{NodeSimilarityMetric, SimilarityMetric};
use crate::types::graph::graph::Graph;
use rayon::prelude::*;
use std::cmp::Reverse;
use std::sync::Arc;

pub struct NodeSimilarityComputationResult {
    pub source: u64,
    pub target: u64,
    pub similarity: f64,
}

pub struct NodeSimilarityComputationRuntime {}

// Helper struct for TopK heap
// Note: Using custom Ord/PartialOrd since f64 doesn't implement Ord (NaN handling)
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

impl Default for NodeSimilarityComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeSimilarityComputationRuntime {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compute(
        &self,
        graph: &dyn Graph,
        config: &super::spec::NodeSimilarityConfig,
        _concurrency: usize,
    ) -> Vec<NodeSimilarityComputationResult> {
        let metric_inner = config.similarity_metric.create(config.similarity_cutoff);
        let metric: Arc<dyn SimilarityMetric> = Arc::from(metric_inner);

        // Pre-calculate degrees/weights if needed for metric normalization
        // For unweighted, we just need degrees.
        // Parallelize over source nodes `u`.

        let nodes: Vec<u64> = (0..graph.node_count()).map(|i| i as u64).collect();

        // Use a chunk size for parallel processing? Rayon handles this.
        let _node_count = graph.node_count();

        nodes
            .par_iter()
            .flat_map(|&source| {
                let _metric = Arc::clone(&metric); // Unused in optimized path currently
                let source_mapped = source as usize; // Assuming MappedNodeId is compatible with usize for indexing if needed, but we use Graph API.
                let source_degree =
                    graph.degree(source_mapped as crate::types::graph::MappedNodeId);

                if source_degree == 0 {
                    return Vec::new();
                }

                // Map target -> intersection_count (or overlap)
                let mut overlaps: std::collections::HashMap<u64, f64> =
                    std::collections::HashMap::with_capacity(source_degree); // Heuristic capacity

                // 1. Iterate neighbors of source
                let relationships = graph.stream_relationships(
                    source_mapped as crate::types::graph::MappedNodeId,
                    graph.default_property_value(),
                );

                for rel in relationships {
                    let neighbor_node = rel.target_id();

                    // 2. Iterate INCOMING relationships to neighbor (inverse graph traversal)
                    // We need `stream_inverse_relationships`
                    let inverse_relationships = graph.stream_inverse_relationships(
                        neighbor_node,
                        graph.default_property_value(),
                    );

                    for inv_rel in inverse_relationships {
                        let potential_target = inv_rel.source_id();
                        let potential_target_u64 = potential_target as u64;

                        // Only consider v > u to avoid duplicates and self-loops
                        if potential_target_u64 > source {
                            // Accumulate overlap
                            // Assuming Unweighted Intersection count:
                            *overlaps.entry(potential_target_u64).or_insert(0.0) += 1.0;
                        }
                    }
                }

                // Now we have overlaps for `source`. Calculate similarities and Apply TopK.
                let mut heap = std::collections::BinaryHeap::new();

                for (target, intersection) in overlaps {
                    let target_degree = graph.degree(target as crate::types::graph::MappedNodeId);

                    let sim = match config.similarity_metric {
                        NodeSimilarityMetric::Jaccard => {
                            let union =
                                (source_degree as f64 + target_degree as f64) - intersection;
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
                            let sqrt_prod =
                                ((source_degree as f64) * (target_degree as f64)).sqrt();
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

                // Extract from heap
                heap.into_sorted_vec()
                    .into_iter()
                    .map(
                        |Reverse(ScoredNode(sim, target))| NodeSimilarityComputationResult {
                            source,
                            target,
                            similarity: sim,
                        },
                    )
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
