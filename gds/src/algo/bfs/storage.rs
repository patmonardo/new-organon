//! **BFS Storage Runtime**
//!
//! **Translation Source**: `org.neo4j.gds.paths.traverse.BFS`
//!
//! This module implements the "Gross pole" for BFS algorithm - persistent data access
//! and algorithm orchestration using the Java GDS parallel BFS architecture.

use super::spec::BfsResult;
use super::BfsComputationRuntime;
use crate::core::utils::progress::{ProgressTracker, UNKNOWN_VOLUME};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::NodeId;
use crate::types::graph::Graph;

/// BFS Storage Runtime - handles persistent data access and algorithm orchestration
///
/// Translation of: `BFS.java` (lines 62-1.073)
/// This implements the "Gross pole" for accessing graph data using simplified BFS architecture
pub struct BfsStorageRuntime {
    /// Source node for BFS traversal
    pub source_node: NodeId,
    /// Target nodes to find
    pub target_nodes: Vec<NodeId>,
    /// Maximum depth to traverse
    pub max_depth: Option<u32>,
    /// Whether to track paths during traversal
    pub track_paths: bool,
}

impl BfsStorageRuntime {
    /// Create new BFS storage runtime
    pub fn new(
        source_node: NodeId,
        target_nodes: Vec<NodeId>,
        max_depth: Option<u32>,
        track_paths: bool,
    ) -> Self {
        Self {
            source_node,
            target_nodes,
            max_depth,
            track_paths,
        }
    }

    /// Compute BFS traversal
    ///
    /// Translation of: `BFS.compute()` (lines 1.075-259)
    /// This orchestrates the main BFS algorithm loop
    pub fn compute_bfs(
        &self,
        computation: &mut BfsComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<BfsResult, AlgorithmError> {
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

        // BFS queue (node, depth)
        let mut queue: std::collections::VecDeque<(NodeId, u32)> =
            std::collections::VecDeque::new();
        computation.set_visited(self.source_node);
        queue.push_back((self.source_node, 0));

        let mut result = Vec::new();

        while let Some((current_node, depth)) = queue.pop_front() {
            result.push(current_node);

            // Get neighbors
            let neighbors = self.get_neighbors(graph, current_node);
            progress_tracker.log_progress(neighbors.len());

            // Respect max depth: only expand when `depth < max_depth`
            if computation.check_max_depth(depth as f64) {
                for neighbor in neighbors {
                    if !computation.is_visited(neighbor) {
                        computation.set_visited(neighbor);
                        queue.push_back((neighbor, depth.saturating_add(1)));
                    }
                }
            }
        }

        let computation_time = start_time.elapsed().as_millis() as u64;

        progress_tracker.end_subtask();

        Ok(BfsResult {
            visited_nodes: result,
            computation_time_ms: computation_time,
        })
    }

    /// Get neighbors of a node (graph-backed when available; mock fallback)
    fn get_neighbors(&self, graph: Option<&dyn Graph>, node: NodeId) -> Vec<NodeId> {
        if let Some(g) = graph {
            let fallback: f64 = 1.0;
            let stream = g.stream_relationships(node, fallback);
            stream.into_iter().map(|c| c.target_id()).collect()
        } else {
            match node {
                0 => vec![1, 2],
                1 => vec![0, 3],
                2 => vec![0, 3],
                3 => vec![1, 2],
                _ => vec![],
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::{TaskProgressTracker, Tasks};

    #[test]
    fn test_bfs_storage_runtime_creation() {
        let storage = BfsStorageRuntime::new(0, vec![3], Some(5), true);
        assert_eq!(storage.source_node, 0);
        assert_eq!(storage.target_nodes, vec![3]);
        assert_eq!(storage.max_depth, Some(5));
        assert!(storage.track_paths);
    }

    #[test]
    fn test_bfs_path_computation() {
        let storage = BfsStorageRuntime::new(0, vec![3], None, true);
        let mut computation = BfsComputationRuntime::new(0, true, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("BFS".to_string()));

        let result = storage
            .compute_bfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        assert!(!result.visited_nodes.is_empty());
    }

    #[test]
    fn test_bfs_path_same_source_target() {
        let storage = BfsStorageRuntime::new(0, vec![0], None, true);
        let mut computation = BfsComputationRuntime::new(0, true, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("BFS".to_string()));

        let result = storage
            .compute_bfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        assert!(result.visited_nodes.len() >= 1);
    }

    #[test]
    fn test_bfs_max_depth_constraint() {
        let storage = BfsStorageRuntime::new(0, vec![], Some(1), false);
        let mut computation = BfsComputationRuntime::new(0, false, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("BFS".to_string()));

        let result = storage
            .compute_bfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        // With max_depth=1, we should only visit nodes at distance 0 and 1
        assert!(result.visited_nodes.len() <= 3); // Source + immediate neighbors
    }
}
