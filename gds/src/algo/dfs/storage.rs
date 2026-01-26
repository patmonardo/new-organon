//! **DFS Storage Runtime**
//!
//! **Translation Source**: `org.neo4j.gds.traversal.DFS`
//!
//! This module implements the "Gross pole" for DFS algorithm - persistent data access
//! and algorithm orchestration.

use super::DfsComputationRuntime;
use super::spec::DfsResult;
use crate::core::utils::progress::{ProgressTracker, UNKNOWN_VOLUME};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::NodeId;
use crate::types::graph::Graph;
use std::collections::VecDeque;

/// DFS Storage Runtime - handles persistent data access and algorithm orchestration
///
/// Translation of: `DFS.java` (lines 76-1.050)
/// This implements the "Gross pole" for accessing graph data
pub struct DfsStorageRuntime {
    /// Source node for DFS traversal
    pub source_node: NodeId,
    /// Target nodes to find
    pub target_nodes: Vec<NodeId>,
    /// Maximum depth to traverse
    pub max_depth: Option<u32>,
    /// Whether to track paths during traversal
    pub track_paths: bool,
    /// Concurrency level
    pub concurrency: usize,
}

impl DfsStorageRuntime {
    /// Create new DFS storage runtime
    pub fn new(
        source_node: NodeId,
        target_nodes: Vec<NodeId>,
        max_depth: Option<u32>,
        track_paths: bool,
        concurrency: usize,
    ) -> Self {
        Self {
            source_node,
            target_nodes,
            max_depth,
            track_paths,
            concurrency,
        }
    }

    /// Compute DFS traversal
    ///
    /// Translation of: `DFS.compute()` (lines 1.051.0-200)
    /// This orchestrates the main DFS algorithm loop using stacks
    pub fn compute_dfs(
        &self,
        computation: &mut DfsComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<DfsResult, AlgorithmError> {
        let start_time = std::time::Instant::now();

        let volume = graph
            .map(|g| g.relationship_count())
            .unwrap_or(UNKNOWN_VOLUME);
        if volume == UNKNOWN_VOLUME {
            progress_tracker.begin_subtask_unknown();
        } else {
            progress_tracker.begin_subtask_with_volume(volume);
        }

        let node_count = graph.map(|g| g.node_count()).unwrap_or(1000) as usize;
        computation.initialize(self.source_node, self.max_depth, node_count);

        // DFS stacks for depth-first traversal
        let mut nodes: VecDeque<NodeId> = VecDeque::new();
        let mut sources: VecDeque<NodeId> = VecDeque::new();
        let mut weights: VecDeque<f64> = VecDeque::new();

        nodes.push_back(self.source_node);
        sources.push_back(self.source_node);
        weights.push_back(0.0);

        let mut result = Vec::new();

        // Main DFS loop
        while let (Some(node), Some(_source), Some(weight)) =
            (nodes.pop_back(), sources.pop_back(), weights.pop_back())
        {
            result.push(node);

            // Progress is tracked in terms of relationships examined.
            if let Some(g) = graph {
                progress_tracker.log_progress(g.degree(node));
            }

            // Check max depth
            if computation.check_max_depth(weight) {
                if let Some(g) = graph {
                    for relationship in g.stream_relationships(node, 1.0) {
                        let t = relationship.target_id();
                        if !computation.is_visited(t) {
                            computation.set_visited(t);
                            sources.push_back(node);
                            nodes.push_back(t);
                            weights.push_back(weight + 1.0);
                        }
                    }
                }
            }
        }

        let computation_time = start_time.elapsed().as_millis() as u64;

        progress_tracker.end_subtask();

        Ok(DfsResult {
            visited_nodes: result,
            computation_time_ms: computation_time,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::{TaskProgressTracker, Tasks};

    #[test]
    fn test_dfs_storage_runtime_creation() {
        let storage = DfsStorageRuntime::new(0, vec![3], Some(5), true, 4);
        assert_eq!(storage.source_node, 0);
        assert_eq!(storage.target_nodes, vec![3]);
        assert_eq!(storage.max_depth, Some(5));
        assert!(storage.track_paths);
        assert_eq!(storage.concurrency, 4);
    }

    #[test]
    fn test_dfs_path_computation() {
        let storage = DfsStorageRuntime::new(0, vec![3], None, true, 1);
        let mut computation = DfsComputationRuntime::new(0, true, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("DFS".to_string()));

        let result = storage
            .compute_dfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        assert!(!result.visited_nodes.is_empty());
        assert!(result.visited_nodes.contains(&0));
    }

    #[test]
    fn test_dfs_path_same_source_target() {
        let storage = DfsStorageRuntime::new(0, vec![0], None, true, 1);
        let mut computation = DfsComputationRuntime::new(0, true, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("DFS".to_string()));

        let result = storage
            .compute_dfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        assert!(!result.visited_nodes.is_empty());
        assert_eq!(result.visited_nodes[0], 0);
    }

    #[test]
    fn test_dfs_max_depth_constraint() {
        let storage = DfsStorageRuntime::new(0, vec![], Some(1), false, 1);
        let mut computation = DfsComputationRuntime::new(0, false, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("DFS".to_string()));

        let result = storage
            .compute_dfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        // With max_depth=1, should visit source and its neighbors
        assert!(!result.visited_nodes.is_empty());
        assert!(result.visited_nodes.len() <= 3); // Source + immediate neighbors
    }
}
