use super::similarity_metric::SimilarityMetric;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::Arc;

pub struct NodeSimilarityComputationResult {
    pub source: u64,
    pub target: u64,
    pub similarity: f64,
}

#[derive(Default)]
pub struct NodeSimilarityComputationRuntime;

// Helper struct for TopK heap
// Note: Using custom Ord/PartialOrd since f64 doesn't implement Ord (NaN handling)
#[derive(PartialEq)]
struct ScoredNode {
    similarity: f64,
    target: u64,
}

impl Eq for ScoredNode {}

impl PartialOrd for ScoredNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScoredNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.similarity
            .partial_cmp(&other.similarity)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| self.target.cmp(&other.target))
    }
}

impl NodeSimilarityComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn score_source_from_candidates(
        &self,
        source: u64,
        candidates: impl IntoIterator<Item = u64>,
        vectors: &[Vec<u64>],
        weights: Option<&[Vec<f64>]>,
        metric: &Arc<dyn SimilarityMetric>,
        top_k: usize,
    ) -> Vec<NodeSimilarityComputationResult> {
        let limit = if top_k == 0 { usize::MAX } else { top_k };

        let mut heap: BinaryHeap<Reverse<ScoredNode>> = BinaryHeap::new();
        let source_vec = &vectors[source as usize];

        if source_vec.is_empty() {
            return Vec::new();
        }

        match weights {
            None => {
                for target in candidates {
                    let target_vec = &vectors[target as usize];
                    if target_vec.is_empty() {
                        continue;
                    }

                    let similarity = metric.compute_similarity(source_vec, target_vec);
                    if similarity.is_nan() {
                        continue;
                    }

                    let item = Reverse(ScoredNode { similarity, target });

                    if heap.len() < limit {
                        heap.push(item);
                    } else if let Some(min_top) = heap.peek() {
                        if similarity > min_top.0.similarity {
                            heap.pop();
                            heap.push(item);
                        }
                    }
                }
            }
            Some(weights) => {
                let source_weights = &weights[source as usize];
                debug_assert_eq!(source_vec.len(), source_weights.len());

                for target in candidates {
                    let target_vec = &vectors[target as usize];
                    if target_vec.is_empty() {
                        continue;
                    }
                    let target_weights = &weights[target as usize];
                    debug_assert_eq!(target_vec.len(), target_weights.len());

                    let similarity = metric.compute_weighted_similarity(
                        source_vec,
                        target_vec,
                        source_weights,
                        target_weights,
                    );
                    if similarity.is_nan() {
                        continue;
                    }

                    let item = Reverse(ScoredNode { similarity, target });

                    if heap.len() < limit {
                        heap.push(item);
                    } else if let Some(min_top) = heap.peek() {
                        if similarity > min_top.0.similarity {
                            heap.pop();
                            heap.push(item);
                        }
                    }
                }
            }
        }

        heap.into_sorted_vec()
            .into_iter()
            .map(
                |Reverse(ScoredNode { similarity, target })| NodeSimilarityComputationResult {
                    source,
                    target,
                    similarity,
                },
            )
            .collect()
    }

    pub fn select_top_n(
        &self,
        results: Vec<NodeSimilarityComputationResult>,
        top_n: usize,
    ) -> Vec<NodeSimilarityComputationResult> {
        if top_n == 0 || results.len() <= top_n {
            return results;
        }

        #[derive(PartialEq)]
        struct ScoredPair {
            similarity: f64,
            source: u64,
            target: u64,
        }

        impl Eq for ScoredPair {}

        impl PartialOrd for ScoredPair {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for ScoredPair {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.similarity
                    .partial_cmp(&other.similarity)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| self.source.cmp(&other.source))
                    .then_with(|| self.target.cmp(&other.target))
            }
        }

        let mut heap: BinaryHeap<Reverse<ScoredPair>> = BinaryHeap::new();

        for r in results {
            let item = Reverse(ScoredPair {
                similarity: r.similarity,
                source: r.source,
                target: r.target,
            });

            if heap.len() < top_n {
                heap.push(item);
            } else if let Some(min_top) = heap.peek() {
                if r.similarity > min_top.0.similarity {
                    heap.pop();
                    heap.push(item);
                }
            }
        }

        heap.into_sorted_vec()
            .into_iter()
            .map(
                |Reverse(ScoredPair {
                     similarity,
                     source,
                     target,
                 })| NodeSimilarityComputationResult {
                    source,
                    target,
                    similarity,
                },
            )
            .collect()
    }
}
