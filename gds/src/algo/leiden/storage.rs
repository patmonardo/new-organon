//! Leiden storage/runtime adapter.
//!
//! Responsible for:
//! - obtaining the correct graph projection from `GraphStore`
//! - converting that projection into the computation-friendly representation

use super::computation::{AdjacencyGraph, LeidenComputationRuntime};
use super::spec::{LeidenConfig, LeidenResult};
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;

pub struct LeidenStorageRuntime {}

impl LeidenStorageRuntime {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compute_leiden(
        &self,
        computation: &mut LeidenComputationRuntime,
        graph_store: &impl GraphStore,
        config: &LeidenConfig,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<LeidenResult, String> {
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| format!("failed to build graph view: {e}"))?;

        let node_count = graph_view.node_count();
        let mut adj: Vec<Vec<(usize, f64)>> = vec![Vec::new(); node_count];

        // Weight fallback matches the procedures layer behavior: unweighted edges => 1.0.
        let weight_fallback = 1.0;

        progress_tracker.begin_subtask_with_volume(node_count);
        for node_id in 0..node_count {
            termination_flag.assert_running();
            let stream = graph_view.stream_relationships_weighted(node_id as i64, weight_fallback);
            for cursor in stream {
                let t = cursor.target_id();
                if t >= 0 {
                    adj[node_id].push((t as usize, cursor.weight()));
                }
            }
            progress_tracker.log_progress(1);
        }
        progress_tracker.end_subtask();

        let input = AdjacencyGraph::new(node_count, adj);
        let result = computation
            .compute(&input, config, termination_flag)
            .map_err(|e| format!("leiden compute failed: {e}"))?;

        Ok(LeidenComputationRuntime::into_result(result))
    }
}

impl Default for LeidenStorageRuntime {
    fn default() -> Self {
        Self::new()
    }
}
