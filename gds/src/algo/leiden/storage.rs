//! Leiden storage/runtime adapter.
//!
//! Controller responsibilities:
//! - obtain the undirected projection from `GraphStore`
//! - build adjacency lists (weight fallback 1.0) with progress/termination
//! - call the pure computation runtime

use super::spec::{LeidenConfig, LeidenResult};
use super::{AdjacencyGraph, LeidenComputationRuntime};
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

use crate::types::graph::Graph;

#[derive(Clone)]
pub struct LeidenStorageRuntime {
    graph: Arc<dyn Graph>,
}

impl LeidenStorageRuntime {
    pub fn new<G: GraphStore>(graph_store: &G) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;
        Ok(Self { graph })
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count() as usize
    }

    pub fn compute_leiden(
        &self,
        computation: &mut LeidenComputationRuntime,
        config: &LeidenConfig,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<LeidenResult, AlgorithmError> {
        let node_count = self.graph.node_count();
        if node_count == 0 {
            return Ok(LeidenResult {
                communities: Vec::new(),
                community_count: 0,
                modularity: 0.0,
                levels: 0,
                converged: true,
                node_count: 0,
                execution_time: std::time::Duration::default(),
            });
        }
        let mut adj: Vec<Vec<(usize, f64)>> = vec![Vec::new(); node_count];

        // Weight fallback matches the procedures layer behavior: unweighted edges => 1.0.
        let weight_fallback = 1.0;

        // Single subtask covering adjacency build + iteration progress (levels).
        progress_tracker
            .begin_subtask_with_volume(node_count.saturating_add(config.max_iterations));
        for node_id in 0..node_count {
            termination_flag.assert_running();
            let stream = self
                .graph
                .stream_relationships_weighted(node_id as i64, weight_fallback);
            for cursor in stream {
                let t = cursor.target_id();
                if t >= 0 {
                    adj[node_id].push((t as usize, cursor.weight()));
                }
            }
            progress_tracker.log_progress(1);
        }

        let input = AdjacencyGraph::new(node_count, adj);
        let result = computation
            .compute(&input, config, termination_flag)
            .map_err(|e| AlgorithmError::Execution(format!("leiden compute failed: {e}")))?;

        progress_tracker.log_progress(result.levels);
        progress_tracker.end_subtask();

        Ok(LeidenComputationRuntime::into_result(result))
    }
}
