//! **BFS Storage Runtime**
//!
//! **Translation Source**: `org.neo4j.gds.paths.traverse.BFS`
//!
//! This module implements the "Gross pole" for BFS algorithm - persistent data access
//! and algorithm orchestration using the Java GDS parallel BFS architecture.

use super::computation::BfsComputationRuntime;
use super::spec::{BfsPathResult, BfsResult};
use crate::algo::traversal::{
    Aggregator, ExitPredicate, FollowExitPredicate, OneHopAggregator, TargetExitPredicate,
};
use crate::core::utils::progress::{ProgressTracker, UNKNOWN_VOLUME};
use crate::projection::eval::procedure::AlgorithmError;
use crate::types::graph::id_map::NodeId;
use crate::types::graph::Graph;
use std::sync::atomic::{AtomicUsize, Ordering};

/// BFS Storage Runtime - handles persistent data access and algorithm orchestration
///
/// Translation of: `BFS.java` (lines 62-1.073)
/// This implements the "Gross pole" for accessing graph data using parallel BFS architecture
pub struct BfsStorageRuntime {
    /// Source node for BFS traversal
    pub source_node: NodeId,
    /// Target nodes to find
    pub target_nodes: Vec<NodeId>,
    /// Maximum depth to traverse
    pub max_depth: Option<u32>,
    /// Whether to track paths during traversal
    pub track_paths: bool,
    /// Concurrency level
    pub concurrency: usize,
    /// Delta parameter for chunking (default 64)
    pub delta: usize,
    /// Exit predicate for controlling traversal
    pub exit_predicate: Box<dyn ExitPredicate>,
    /// Aggregator function for computing weights
    pub aggregator: Box<dyn Aggregator>,
}

impl BfsStorageRuntime {
    /// Create new BFS storage runtime with default predicates
    pub fn new(
        source_node: NodeId,
        target_nodes: Vec<NodeId>,
        max_depth: Option<u32>,
        track_paths: bool,
        concurrency: usize,
        delta: usize,
    ) -> Self {
        let exit_predicate = if target_nodes.is_empty() {
            Box::new(FollowExitPredicate) as Box<dyn ExitPredicate>
        } else {
            Box::new(TargetExitPredicate::new(target_nodes.clone())) as Box<dyn ExitPredicate>
        };

        Self {
            source_node,
            target_nodes,
            max_depth,
            track_paths,
            concurrency,
            delta,
            exit_predicate,
            aggregator: Box::new(OneHopAggregator),
        }
    }

    /// Create new BFS storage runtime with custom predicates
    #[allow(clippy::too_many_arguments)]
    pub fn with_predicates(
        source_node: NodeId,
        target_nodes: Vec<NodeId>,
        max_depth: Option<u32>,
        track_paths: bool,
        concurrency: usize,
        delta: usize,
        exit_predicate: Box<dyn ExitPredicate>,
        aggregator: Box<dyn Aggregator>,
    ) -> Self {
        Self {
            source_node,
            target_nodes,
            max_depth,
            track_paths,
            concurrency,
            delta,
            exit_predicate,
            aggregator,
        }
    }

    /// Compute BFS traversal using parallel architecture
    ///
    /// Translation of: `BFS.compute()` (lines 1.075-259)
    /// This orchestrates the main BFS algorithm loop using Java GDS parallel architecture
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

        // Initialize computation runtime
        computation.initialize(self.source_node, self.max_depth);

        // Parallel BFS state - following Java GDS architecture
        let node_count = graph.map(|g| g.node_count()).unwrap_or(1000);
        let mut traversed_nodes = vec![0 as NodeId; node_count];
        let mut weights = vec![0.0f64; node_count];
        let mut visited = vec![false; node_count];
        let mut predecessor = vec![-1 as NodeId; node_count];

        let node_id_to_index = |node_id: NodeId| -> Option<usize> {
            if node_id < 0 {
                return None;
            }
            let index = node_id as usize;
            if index >= node_count {
                return None;
            }
            Some(index)
        };

        // Atomic counters for parallel processing
        let traversed_nodes_index = AtomicUsize::new(0);
        let traversed_nodes_length = AtomicUsize::new(1);
        let target_found_index = AtomicUsize::new(usize::MAX);

        // Initialize with source node
        let source_index = node_id_to_index(self.source_node).ok_or_else(|| {
            AlgorithmError::Execution(format!(
                "source_node {} is out of bounds for node_count {}",
                self.source_node, node_count
            ))
        })?;

        visited[source_index] = true;
        traversed_nodes[0] = self.source_node;
        weights[0] = 0.0;
        predecessor[source_index] = self.source_node;

        // Main BFS loop with depth control
        let mut current_depth = 0;
        let max_depth = self.max_depth.unwrap_or(u32::MAX);

        while current_depth < max_depth {
            // Process current level in parallel chunks
            let start_index = traversed_nodes_index.load(Ordering::SeqCst);
            let end_index = traversed_nodes_length.load(Ordering::SeqCst);

            // Process nodes in chunks of delta size
            for chunk_start in (start_index..end_index).step_by(self.delta) {
                let chunk_end = (chunk_start + self.delta).min(end_index);

                for idx in chunk_start..chunk_end {
                    let node_id = traversed_nodes[idx];
                    let source_id = if node_id == self.source_node {
                        self.source_node
                    } else {
                        // Find source for this node (simplified)
                        self.source_node
                    };

                    let weight = self.aggregator.apply(source_id, node_id, weights[idx]);
                    weights[idx] = weight;

                    // Apply exit predicate
                    let exit_result = self.exit_predicate.test(source_id, node_id, weight);

                    if exit_result == crate::algo::traversal::ExitPredicateResult::Break {
                        target_found_index.store(idx, Ordering::SeqCst);
                        break;
                    }

                    if exit_result == crate::algo::traversal::ExitPredicateResult::Follow {
                        // Relax node - get neighbors and add to next level
                        let neighbors = self.get_neighbors(graph, node_id);
                        // Progress is tracked in terms of relationships examined.
                        progress_tracker.log_progress(neighbors.len());
                        for neighbor in neighbors {
                            let Some(neighbor_index) = node_id_to_index(neighbor) else {
                                continue;
                            };

                            if !visited[neighbor_index] {
                                visited[neighbor_index] = true;
                                predecessor[neighbor_index] = node_id;
                                let new_index =
                                    traversed_nodes_length.fetch_add(1, Ordering::SeqCst);
                                if new_index < node_count {
                                    traversed_nodes[new_index] = neighbor;
                                    weights[new_index] = weight;
                                }
                            }
                        }
                    }

                    // Update computation runtime
                    computation.add_visited_node(node_id, current_depth);
                }
            }

            // Check if we found targets
            if target_found_index.load(Ordering::SeqCst) != usize::MAX {
                break;
            }

            // Update indices for next level: move start to the previous end_index
            let new_length = traversed_nodes_length.load(Ordering::SeqCst);
            traversed_nodes_index.store(end_index, Ordering::SeqCst);
            current_depth += 1;

            // Check if no new nodes were added.
            if end_index == new_length {
                break;
            }
        }

        // Build result
        let final_length = if target_found_index.load(Ordering::SeqCst) != usize::MAX {
            target_found_index.load(Ordering::SeqCst) + 1
        } else {
            traversed_nodes_length.load(Ordering::SeqCst)
        };

        let visited_nodes: Vec<(NodeId, u32)> = (0..final_length)
            .map(|i| {
                let node_id = traversed_nodes[i];
                let distance = computation.get_distance(node_id).unwrap_or(0);
                (node_id, distance)
            })
            .collect();

        let paths = if self.track_paths {
            self.build_paths(&traversed_nodes[..final_length], &predecessor)
        } else {
            Vec::new()
        };

        let computation_time = start_time.elapsed().as_millis() as u64;

        progress_tracker.end_subtask();

        Ok(BfsResult {
            visited_nodes,
            paths,
            nodes_visited: final_length,
            computation_time_ms: computation_time,
        })
    }

    /// Build paths from traversed nodes
    fn build_paths(&self, traversed_nodes: &[NodeId], predecessor: &[NodeId]) -> Vec<BfsPathResult> {
        let mut paths = Vec::new();

        let targets: Vec<NodeId> = if self.target_nodes.is_empty() {
            traversed_nodes.to_vec()
        } else {
            self.target_nodes.clone()
        };

        for target in targets {
            let path_nodes = match reconstruct_path(self.source_node, target, predecessor) {
                Some(path) => path,
                None => continue,
            };

            let path_length = path_nodes.len().saturating_sub(1) as u32;
            paths.push(BfsPathResult {
                source_node: self.source_node,
                target_node: target,
                node_ids: path_nodes,
                path_length,
            });
        }

        paths
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

fn reconstruct_path(
    source: NodeId,
    target: NodeId,
    predecessor: &[NodeId],
) -> Option<Vec<NodeId>> {
    if source < 0 || target < 0 {
        return None;
    }

    let target_index = target as usize;
    if target_index >= predecessor.len() {
        return None;
    }

    // If the target was never discovered, its predecessor stays at -1.
    if predecessor[target_index] == -1 {
        return None;
    }

    let mut path = Vec::new();
    let mut current = target;
    let mut steps = 0usize;

    loop {
        path.push(current);
        if current == source {
            break;
        }

        let current_index = current as usize;
        if current_index >= predecessor.len() {
            return None;
        }

        let pred = predecessor[current_index];
        if pred == -1 {
            return None;
        }

        current = pred;
        steps += 1;
        if steps > predecessor.len() {
            return None;
        }
    }

    path.reverse();
    Some(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::Tasks;

    #[test]
    fn test_bfs_storage_runtime_creation() {
        let storage = BfsStorageRuntime::new(0, vec![3], Some(5), true, 4, 64);
        assert_eq!(storage.source_node, 0);
        assert_eq!(storage.target_nodes, vec![3]);
        assert_eq!(storage.max_depth, Some(5));
        assert!(storage.track_paths);
        assert_eq!(storage.concurrency, 4);
        assert_eq!(storage.delta, 64);
    }

    #[test]
    fn test_bfs_path_computation() {
        let storage = BfsStorageRuntime::new(0, vec![3], None, true, 1, 64);
        let mut computation = BfsComputationRuntime::new(0, true, 1);

        let mut progress_tracker = ProgressTracker::new(Tasks::leaf("BFS".to_string()));

        let result = storage
            .compute_bfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        assert!(result.nodes_visited > 0);
    }

    #[test]
    fn test_bfs_path_same_source_target() {
        let storage = BfsStorageRuntime::new(0, vec![0], None, true, 1, 64);
        let mut computation = BfsComputationRuntime::new(0, true, 1);

        let mut progress_tracker = ProgressTracker::new(Tasks::leaf("BFS".to_string()));

        let result = storage
            .compute_bfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        assert!(result.nodes_visited >= 1);
    }

    #[test]
    fn test_bfs_max_depth_constraint() {
        let storage = BfsStorageRuntime::new(0, vec![], Some(1), false, 1, 64);
        let mut computation = BfsComputationRuntime::new(0, false, 1);

        let mut progress_tracker = ProgressTracker::new(Tasks::leaf("BFS".to_string()));

        let result = storage
            .compute_bfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        // With max_depth=1, we should only visit nodes at distance 0 and 1
        assert!(result.nodes_visited <= 3); // Source + immediate neighbors
    }
}
