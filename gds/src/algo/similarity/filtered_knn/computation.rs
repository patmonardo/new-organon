use crate::algo::similarity::knn::computation::KnnNnDescentConfig;
use crate::algo::similarity::knn::metrics::SimilarityComputer;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FilteredKnnComputationResult {
    pub source: u64,
    pub target: u64,
    pub similarity: f64,
}

#[derive(Default)]
pub struct FilteredKnnComputationRuntime;

impl FilteredKnnComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn compute_nn_descent(
        &self,
        node_count: usize,
        initial_neighbors: Vec<Vec<u64>>,
        cfg: KnnNnDescentConfig,
        similarity: Arc<dyn SimilarityComputer>,
        source_allowed: Option<Arc<Vec<bool>>>,
        target_allowed: Option<Arc<Vec<bool>>>,
    ) -> Vec<FilteredKnnComputationResult> {
        let engine = crate::algo::similarity::knn::KnnComputationRuntime::new();
        let (rows, _stats) = engine.compute_nn_descent(
            node_count,
            initial_neighbors,
            cfg,
            similarity,
            source_allowed,
            target_allowed,
        );

        rows.into_iter()
            .map(|r| FilteredKnnComputationResult {
                source: r.source,
                target: r.target,
                similarity: r.similarity,
            })
            .collect()
    }
}
