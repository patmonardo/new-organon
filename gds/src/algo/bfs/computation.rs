//! **BFS Computation Runtime**
//!
//! **Translation Source**: `org.neo4j.gds.traversal.BFS`
//!
//! This module implements the "Subtle pole" for BFS algorithm - ephemeral computation state.

use crate::types::graph::NodeId;

/// BFS Computation Runtime - handles ephemeral computation state
///
/// Translation of: `BFSComputation.java` (lines 32-75)
/// This implements the "Subtle pole" for accumulating traversal state
pub struct BfsComputationRuntime {
    /// Source node for traversal
    pub source_node: NodeId,
    /// Whether to track paths
    pub track_paths: bool,
    /// Concurrency level
    pub concurrency: usize,
    /// Visited nodes (BitSet equivalent)
    visited: Vec<bool>,
    /// Maximum depth constraint
    max_depth: Option<u32>,
}

impl BfsComputationRuntime {
    /// Create new BFS computation runtime
    pub fn new(
        source_node: NodeId,
        track_paths: bool,
        concurrency: usize,
        node_count: usize,
    ) -> Self {
        Self {
            source_node,
            track_paths,
            concurrency,
            visited: vec![false; node_count],
            max_depth: None,
        }
    }

    /// Initialize computation state
    ///
    /// Translation of: `BFSComputation.initialize()` (lines 76-100)
    /// This resets the internal state for a new traversal
    pub fn initialize(&mut self, source_node: NodeId, max_depth: Option<u32>, node_count: usize) {
        self.source_node = source_node;
        self.max_depth = max_depth;
        self.visited = vec![false; node_count];
    }

    /// Check if a node has been visited
    ///
    /// Translation of: `BFSComputation.isVisited()` (lines 126-140)
    /// This checks the visited state of a node
    pub fn is_visited(&self, node: NodeId) -> bool {
        (node as usize) < self.visited.len() && self.visited[node as usize]
    }

    /// Set a node as visited
    pub fn set_visited(&mut self, node: NodeId) {
        if (node as usize) < self.visited.len() {
            self.visited[node as usize] = true;
        }
    }

    /// Get total number of visited nodes
    ///
    /// Translation of: `BFSComputation.getVisitedCount()` (lines 156-170)
    /// This returns the count of visited nodes
    pub fn visited_count(&self) -> usize {
        self.visited.iter().filter(|&&v| v).count()
    }

    /// Check if max depth constraint is satisfied
    ///
    /// Translation of: `BFSComputation.checkMaxDepth()` (lines 186-200)
    /// This validates depth constraints during traversal
    pub fn check_max_depth(&self, current_depth: f64) -> bool {
        match self.max_depth {
            Some(max_depth) => current_depth < max_depth as f64,
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_computation_runtime_creation() {
        let runtime = BfsComputationRuntime::new(0, true, 4, 10);
        assert_eq!(runtime.source_node, 0);
        assert!(runtime.track_paths);
        assert_eq!(runtime.concurrency, 4);
        assert_eq!(runtime.visited_count(), 0);
    }

    #[test]
    fn test_bfs_computation_runtime_initialization() {
        let mut runtime = BfsComputationRuntime::new(0, true, 1, 10);
        runtime.initialize(5, Some(10), 10);

        assert_eq!(runtime.source_node, 5);
        assert_eq!(runtime.max_depth, Some(10));
        assert_eq!(runtime.visited_count(), 0);
        assert!(!runtime.is_visited(5));
    }

    #[test]
    fn test_bfs_computation_runtime_visited_operations() {
        let mut runtime = BfsComputationRuntime::new(0, false, 1, 10);
        runtime.initialize(0, None, 10);

        assert!(!runtime.is_visited(1));

        runtime.set_visited(1);
        assert!(runtime.is_visited(1));
        assert_eq!(runtime.visited_count(), 1);
    }

    #[test]
    fn test_bfs_computation_runtime_max_depth_check() {
        let mut runtime = BfsComputationRuntime::new(0, false, 1, 10);
        runtime.initialize(0, Some(3), 10);

        assert!(runtime.check_max_depth(0.0));
        assert!(runtime.check_max_depth(1.0));
        assert!(!runtime.check_max_depth(3.0));
        assert!(!runtime.check_max_depth(4.0));

        runtime.initialize(0, None, 10);
        assert!(runtime.check_max_depth(100.0)); // No limit
    }
}
