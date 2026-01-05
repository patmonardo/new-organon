use crate::core::utils::progress::ProgressTracker;
use crate::types::graph::Graph;

use crate::algo::modularity_optimization::ModularityOptimizationInput;
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
        // For Louvain, treat unweighted relationships as weight=1.0 (matches other procedures).
        let weight_fallback = 1.0;

        progress_tracker.begin_subtask_with_volume(node_count);

        let mut adj: Vec<Vec<(usize, f64)>> = vec![Vec::new(); node_count];
        for node_id in 0..node_count {
            let stream = graph.stream_relationships(node_id as i64, weight_fallback);
            for cursor in stream {
                let t = cursor.target_id();
                if t >= 0 {
                    adj[node_id].push((t as usize, cursor.property()));
                }
            }
            progress_tracker.log_progress(1);
        }

        let input = ModularityOptimizationInput::new(node_count, adj);
        let result = computation.compute(&input);

        progress_tracker.end_subtask();

        result
    }
}
