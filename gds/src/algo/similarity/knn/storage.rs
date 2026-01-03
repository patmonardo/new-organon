use super::computation::{KnnComputationResult, KnnComputationRuntime};
use super::metrics::{KnnNodePropertySpec, SimilarityComputer, SimilarityMetric};
use crate::projection::eval::procedure::AlgorithmError;
use crate::core::utils::progress::ProgressTracker;
use crate::types::graph_store::GraphStore;
use std::sync::Arc;

pub struct KnnStorageRuntime {
    _concurrency: usize,
}

impl KnnStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self {
            _concurrency: concurrency,
        }
    }

    pub fn compute_single(
        &self,
        computation: &KnnComputationRuntime,
        graph_store: &impl GraphStore,
        node_property: &str,
        k: usize,
        similarity_cutoff: f64,
        metric: SimilarityMetric,
        progress_tracker: &mut ProgressTracker,
    ) -> Result<Vec<KnnComputationResult>, AlgorithmError> {
        let node_count = graph_store.node_count();
        progress_tracker.begin_subtask(node_count);

        let result = (|| {
            let values = graph_store
                .node_property_values(node_property)
                .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?;

            let similarity =
                <dyn SimilarityComputer>::of_property_values(node_property, values, metric)
                    .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?;

            Ok(computation.compute(node_count, k, similarity_cutoff, similarity))
        })();

        match result {
            Ok(value) => {
                progress_tracker.log_progress(node_count);
                progress_tracker.end_subtask();
                Ok(value)
            }
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                Err(e)
            }
        }
    }

    pub fn compute_multi(
        &self,
        computation: &KnnComputationRuntime,
        graph_store: &impl GraphStore,
        node_properties: &[KnnNodePropertySpec],
        k: usize,
        similarity_cutoff: f64,
        progress_tracker: &mut ProgressTracker,
    ) -> Result<Vec<KnnComputationResult>, AlgorithmError> {
        let node_count = graph_store.node_count();
        progress_tracker.begin_subtask(node_count);

        let result = (|| {
            if node_properties.is_empty() {
                return Err(AlgorithmError::InvalidGraph(
                    "Missing `node_properties`".to_string(),
                ));
            }

            let mut props: Vec<(
                String,
                Arc<dyn crate::types::properties::node::NodePropertyValues>,
                SimilarityMetric,
            )> = Vec::with_capacity(node_properties.len());

            for spec in node_properties {
                let name = spec.name.trim();
                if name.is_empty() {
                    return Err(AlgorithmError::InvalidGraph(
                        "`node_properties` contains an empty property name".to_string(),
                    ));
                }
                let values = graph_store
                    .node_property_values(name)
                    .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?;
                props.push((name.to_string(), values, spec.metric));
            }

            let similarity = <dyn SimilarityComputer>::of_properties(props)
                .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?;

            Ok(computation.compute(node_count, k, similarity_cutoff, similarity))
        })();

        match result {
            Ok(value) => {
                progress_tracker.log_progress(node_count);
                progress_tracker.end_subtask();
                Ok(value)
            }
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                Err(e)
            }
        }
    }
}
