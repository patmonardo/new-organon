use crate::types::graph::Graph;
use crate::core::utils::progress::ProgressTracker;

use super::computation::LouvainComputationRuntime;
use super::spec::LouvainResult;

pub struct LouvainStorageRuntime {
    #[allow(dead_code)]
    concurrency: usize,
}

impl LouvainStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }

    pub fn compute_louvain(
        &self,
        computation: &mut LouvainComputationRuntime,
        graph: &dyn Graph,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> LouvainResult {
        let node_count = graph.node_count();
        let fallback = graph.default_property_value();

        progress_tracker.begin_subtask_with_volume(node_count);

        let get_neighbors = |node: usize| -> Vec<usize> {
            let id = node as u64;
            let mut out: Vec<usize> = graph
                .stream_relationships(id as i64, fallback)
                .map(|c| c.target_id() as usize)
                .collect();
            let mut inc: Vec<usize> = graph
                .stream_inverse_relationships(id as i64, fallback)
                .map(|c| c.source_id() as usize)
                .collect();
            out.append(&mut inc);
            out
        };

        let result = computation.compute(node_count, get_neighbors);

        // Placeholder implementation: count nodes as the only work unit.
        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        result
    }
}
