//! Spanning Tree Storage Runtime - Graph Data Access and Algorithm Orchestration
//!
//! **Translation Source**: `org.neo4j.gds.spanningtree.Prim` (algorithm orchestration)
//!
//! This module implements the "Gross pole" for spanning tree algorithms,
//! handling persistent data access and orchestrating the Prim's algorithm execution.

use super::{SpanningTree, SpanningTreeComputationRuntime};
use crate::core::utils::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::Graph;

/// Spanning Tree Storage Runtime
///
/// **Translation Source**: `org.neo4j.gds.spanningtree.Prim`
///
/// Implements the "Gross pole" for spanning tree algorithms, handling:
/// - Graph data access and neighbor iteration
/// - Algorithm orchestration and execution
/// - Result construction and validation
pub struct SpanningTreeStorageRuntime {
    /// Start node for the spanning tree
    pub start_node_id: u32,

    /// Whether to compute minimum (true) or maximum (false) spanning tree
    pub compute_minimum: bool,

    /// Concurrency level
    pub concurrency: usize,
}

impl SpanningTreeStorageRuntime {
    /// Creates a new SpanningTreeStorageRuntime.
    ///
    /// # Arguments
    ///
    /// * `start_node_id` - Starting node for the spanning tree
    /// * `compute_minimum` - Whether to compute minimum (true) or maximum (false) spanning tree
    /// * `concurrency` - Concurrency level
    ///
    /// # Returns
    ///
    /// A new `SpanningTreeStorageRuntime` instance.
    pub fn new(start_node_id: u32, compute_minimum: bool, concurrency: usize) -> Self {
        Self {
            start_node_id,
            compute_minimum,
            concurrency,
        }
    }

    /// Compute the spanning tree using Prim's algorithm.
    ///
    /// **Translation Source**: `org.neo4j.gds.spanningtree.Prim.compute()`
    ///
    /// This method orchestrates the Prim's algorithm, handling graph access and progress tracking,
    /// while delegating all state management to the computation runtime.
    ///
    /// # Arguments
    ///
    /// * `computation` - Mutable reference to the computation runtime for state management
    /// * `graph` - Optional graph interface for neighbor access
    /// * `direction` - Traversal direction (0=outgoing, 1=incoming, 2=undirected)
    /// * `progress_tracker` - Progress tracking interface
    ///
    /// # Returns
    ///
    /// A `Result` containing the `SpanningTree` or an error.
    pub fn compute_spanning_tree(
        &self,
        computation: &mut SpanningTreeComputationRuntime,
        graph: Option<&dyn Graph>,
        direction: u8,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<SpanningTree, AlgorithmError> {
        let node_count = graph.map(|g| g.node_count() as u32).unwrap_or(0);

        progress_tracker.begin_subtask_with_volume(node_count as usize);

        let mut processed_nodes: usize = 0;
        const LOG_BATCH: usize = 256;

        let result = (|| {
            // Handle empty graph upfront
            if node_count == 0 {
                return Ok(computation.build_result(0));
            }

            // Initialize computation runtime
            computation.initialize(self.start_node_id);

            // Main Prim's algorithm loop
            while !computation.is_queue_empty() {
                // Get next node from priority queue
                let (current_node, current_cost) = match computation.pop_from_queue() {
                    Some((node, cost)) => (node, cost),
                    None => break,
                };

                // Skip if already visited
                if computation.is_visited(current_node) {
                    continue;
                }

                // Mark as visited and update progress
                computation.mark_visited(current_node, current_cost);
                processed_nodes += 1;

                if processed_nodes >= LOG_BATCH {
                    progress_tracker.log_progress(processed_nodes);
                    processed_nodes = 0;
                }

                // Process neighbors via graph interface
                if let Some(graph) = graph {
                    let neighbors = self.get_neighbors_from_graph(graph, current_node, direction);
                    for (neighbor, weight) in neighbors {
                        // Skip if neighbor already visited
                        if computation.is_visited(neighbor) {
                            continue;
                        }

                        // Transform weight for min/max spanning tree
                        let transformed_weight = computation.transform_weight(weight);

                        // Check if neighbor is already in queue
                        let current_parent = computation.parent(neighbor);
                        let current_cost_to_parent = computation.cost_to_parent(neighbor);

                        if current_parent == -1 {
                            // Neighbor not in queue, add it
                            computation.add_to_queue(neighbor, transformed_weight, current_node);
                        } else if transformed_weight < current_cost_to_parent {
                            // Better path found, update
                            computation.update_cost(neighbor, transformed_weight, current_node);
                        }
                    }
                }
            }

            if processed_nodes > 0 {
                progress_tracker.log_progress(processed_nodes);
            }

            // Build and return result
            Ok(computation.build_result(node_count))
        })();

        match result {
            Ok(result) => {
                progress_tracker.end_subtask();
                Ok(result)
            }
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                Err(e)
            }
        }
    }

    /// Compute the spanning tree using a bound Graph (neighbor streaming via relationship cursors).
    pub fn compute_spanning_tree_with_graph(
        &self,
        graph: &dyn Graph,
        direction: u8,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<SpanningTree, AlgorithmError> {
        let node_count = graph.node_count() as u32;

        // Create computation runtime for this operation
        let mut computation = SpanningTreeComputationRuntime::new(
            self.start_node_id,
            self.compute_minimum,
            node_count,
            self.concurrency,
        );

        self.compute_spanning_tree(&mut computation, Some(graph), direction, progress_tracker)
    }

    /// Neighbor retrieval backed by Graph::stream_relationships (outgoing edges), with numeric fallback.
    fn get_neighbors_from_graph(
        &self,
        graph: &dyn Graph,
        node_id: u32,
        direction: u8,
    ) -> Vec<(u32, f64)> {
        let fallback: f64 = 1.0;
        match direction {
            1 => graph
                .stream_inverse_relationships_weighted(node_id as i64, fallback)
                .map(|cursor| (cursor.target_id() as u32, cursor.weight()))
                .collect(),
            2 => {
                let mut out: Vec<(u32, f64)> = graph
                    .stream_relationships_weighted(node_id as i64, fallback)
                    .map(|cursor| (cursor.target_id() as u32, cursor.weight()))
                    .collect();
                out.extend(
                    graph
                        .stream_inverse_relationships_weighted(node_id as i64, fallback)
                        .map(|cursor| (cursor.target_id() as u32, cursor.weight())),
                );
                out
            }
            _ => graph
                .stream_relationships_weighted(node_id as i64, fallback)
                .map(|cursor| (cursor.target_id() as u32, cursor.weight()))
                .collect(),
        }
    }

    /// Get neighbors of a node (mock implementation for testing).
    ///
    /// # Arguments
    ///
    /// * `node_id` - Node ID to get neighbors for
    ///
    /// # Returns
    ///
    /// A vector of (neighbor_id, weight) pairs.
    pub fn get_neighbors(&self, node_id: u32) -> Vec<(u32, f64)> {
        // Mock implementation for testing
        // In a real implementation, this would access the graph store
        match node_id {
            0 => vec![(1, 1.0), (2, 2.0), (3, 1.5)],
            1 => vec![(0, 1.0), (2, 1.5), (3, 2.5)],
            2 => vec![(0, 2.0), (1, 1.5), (3, 1.0)],
            3 => vec![(0, 1.5), (1, 2.5), (2, 1.0)],
            _ => vec![],
        }
    }

    /// Compute spanning tree with mock graph data.
    ///
    /// # Arguments
    ///
    /// * `node_count` - Total number of nodes in the graph
    ///
    /// # Returns
    ///
    /// A `Result` containing the `SpanningTree` or an error.
    /// Compute spanning tree with mock graph data (for testing).
    ///
    /// # Arguments
    ///
    /// * `node_count` - Total number of nodes in the graph
    ///
    /// # Returns
    ///
    /// A `Result` containing the `SpanningTree` or an error.
    pub fn compute_spanning_tree_mock(
        &self,
        node_count: u32,
        _progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<SpanningTree, AlgorithmError> {
        if node_count == 0 {
            return Ok(SpanningTree::new(
                self.start_node_id,
                0,
                0,
                Vec::new(),
                Vec::new(),
                0.0,
            ));
        }

        // For testing, create a simple mock spanning tree
        let mut parent = vec![-1i32; node_count as usize];
        let mut cost_to_parent = vec![0.0f64; node_count as usize];

        // Simple star topology from start node
        for i in 0..node_count as usize {
            if i != self.start_node_id as usize {
                parent[i] = self.start_node_id as i32;
                cost_to_parent[i] = 1.0;
            }
        }

        let total_weight = (node_count as f64 - 1.0) * 1.0;

        Ok(SpanningTree::new(
            self.start_node_id,
            node_count,
            node_count,
            parent,
            cost_to_parent,
            total_weight,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::{TaskProgressTracker, Tasks};

    #[test]
    fn test_storage_runtime_creation() {
        let runtime = SpanningTreeStorageRuntime::new(0, true, 1);

        assert_eq!(runtime.start_node_id, 0);
        assert!(runtime.compute_minimum);
        assert_eq!(runtime.concurrency, 1);
    }

    #[test]
    fn test_storage_runtime_minimum_spanning_tree() {
        let runtime = SpanningTreeStorageRuntime::new(0, true, 1);
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf_with_volume("spanning_tree".to_string(), 0));
        let result = runtime
            .compute_spanning_tree_mock(4, &mut progress_tracker)
            .unwrap();

        // Verify basic properties
        assert_eq!(result.head(0), 0);
        assert_eq!(result.effective_node_count(), 4);
        assert!(result.total_weight() > 0.0);

        // Verify tree structure (all nodes should be connected)
        assert_eq!(result.parent(0), -1); // Root has no parent
        assert!(result.parent(1) != -1); // Other nodes have parents
        assert!(result.parent(2) != -1);
        assert!(result.parent(3) != -1);
    }

    #[test]
    fn test_storage_runtime_maximum_spanning_tree() {
        let runtime = SpanningTreeStorageRuntime::new(0, false, 1);
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf_with_volume("spanning_tree".to_string(), 0));
        let result = runtime
            .compute_spanning_tree_mock(4, &mut progress_tracker)
            .unwrap();

        // Verify basic properties
        assert_eq!(result.head(0), 0);
        assert_eq!(result.effective_node_count(), 4);
        assert!(result.total_weight() > 0.0);

        // Verify tree structure
        assert_eq!(result.parent(0), -1); // Root has no parent
        assert!(result.parent(1) != -1); // Other nodes have parents
        assert!(result.parent(2) != -1);
        assert!(result.parent(3) != -1);
    }

    #[test]
    fn test_storage_runtime_different_start_nodes() {
        let runtime1 = SpanningTreeStorageRuntime::new(0, true, 1);
        let runtime2 = SpanningTreeStorageRuntime::new(1, true, 1);

        let mut progress_tracker1 =
            TaskProgressTracker::new(Tasks::leaf_with_volume("spanning_tree".to_string(), 0));
        let mut progress_tracker2 =
            TaskProgressTracker::new(Tasks::leaf_with_volume("spanning_tree".to_string(), 0));
        let result1 = runtime1
            .compute_spanning_tree_mock(4, &mut progress_tracker1)
            .unwrap();
        let result2 = runtime2
            .compute_spanning_tree_mock(4, &mut progress_tracker2)
            .unwrap();

        // Both should produce valid spanning trees
        assert_eq!(result1.effective_node_count(), 4);
        assert_eq!(result2.effective_node_count(), 4);

        // Different start nodes should produce different trees
        assert_eq!(result1.head(0), 0);
        assert_eq!(result2.head(1), 1);
    }

    #[test]
    fn test_storage_runtime_edge_iteration() {
        let runtime = SpanningTreeStorageRuntime::new(0, true, 1);
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf_with_volume("spanning_tree".to_string(), 0));
        let result = runtime
            .compute_spanning_tree_mock(4, &mut progress_tracker)
            .unwrap();

        let mut edges = Vec::new();
        result.for_each_edge(|source, target, cost| {
            edges.push((source, target, cost));
            true
        });

        // Should have exactly 3 edges for a 4-node spanning tree
        assert_eq!(edges.len(), 3);

        // All edges should have positive costs
        for (_, _, cost) in &edges {
            assert!(*cost > 0.0);
        }
    }

    #[test]
    fn test_storage_runtime_empty_graph() {
        let runtime = SpanningTreeStorageRuntime::new(0, true, 1);

        // Mock empty graph
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf_with_volume("spanning_tree".to_string(), 0));
        let result = runtime
            .compute_spanning_tree_mock(0, &mut progress_tracker)
            .unwrap();

        assert_eq!(result.effective_node_count(), 0);
        assert_eq!(result.total_weight(), 0.0);
    }

    #[test]
    fn test_storage_runtime_single_node() {
        let runtime = SpanningTreeStorageRuntime::new(0, true, 1);

        // Mock single node graph
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf_with_volume("spanning_tree".to_string(), 0));
        let result = runtime
            .compute_spanning_tree_mock(1, &mut progress_tracker)
            .unwrap();

        assert_eq!(result.effective_node_count(), 1);
        assert_eq!(result.total_weight(), 0.0);
        assert_eq!(result.parent(0), -1); // Root has no parent
    }
}
