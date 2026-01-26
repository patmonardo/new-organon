use super::spec::{TriangleConfig, TriangleResult};
use super::TriangleComputationRuntime;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;

pub struct TriangleStorageRuntime {}

impl TriangleStorageRuntime {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compute(
        &self,
        computation: &mut TriangleComputationRuntime,
        graph_store: &impl GraphStore,
        config: &TriangleConfig,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<TriangleResult, String> {
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| format!("failed to build graph view: {e}"))?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok(TriangleResult {
                local_triangles: Vec::new(),
                global_triangles: 0,
                node_count: 0,
                execution_time: std::time::Duration::default(),
            });
        }

        // For triangle enumeration we only need topology, not weights.
        let fallback = graph_view.default_property_value();

        progress_tracker.begin_subtask_with_volume(node_count);

        // Build sorted/deduped adjacency lists once (storage responsibility).
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); node_count];
        for node in 0..node_count {
            termination_flag.assert_running();

            let mut neighbors: Vec<usize> = graph_view
                .stream_relationships(node as i64, fallback)
                .map(|cursor| cursor.target_id())
                .filter(|t| *t >= 0)
                .map(|t| t as usize)
                .collect();

            neighbors.sort_unstable();
            neighbors.dedup();
            adj[node] = neighbors;

            progress_tracker.log_progress(1);
        }

        progress_tracker.end_subtask();

        let max_degree = config.max_degree;
        let get_neighbors = |node: usize| -> Vec<usize> { adj[node].clone() };
        Ok(computation.compute_with_max_degree(node_count, get_neighbors, max_degree))
    }
}

impl Default for TriangleStorageRuntime {
    fn default() -> Self {
        Self::new()
    }
}
