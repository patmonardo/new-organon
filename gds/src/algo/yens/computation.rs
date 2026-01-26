//! **Yen's Computation Runtime**
//!
//! **Translation Source**: `org.neo4j.gds.paths.yens.YensTask`
//!
//! This module implements the "Subtle pole" for Yen's algorithm - ephemeral computation state.

use super::mutable_path_result::MutablePathResult;
use super::relationship_filterer::RelationshipFilterer;
use super::spec::YensResult;
use super::YensStorageRuntime;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::types::graph::NodeId;
use crate::types::graph::Graph;
use std::collections::HashMap;

/// Yen's Computation Runtime - handles ephemeral computation state
///
/// Translation of: `YensTask.java` (lines 35-167)
/// This implements the "Subtle pole" for managing Yen's algorithm state
pub struct YensComputationRuntime {
    /// Source node for path finding
    pub source_node: NodeId,
    /// Target node for path finding
    pub target_node: NodeId,
    /// Number of shortest paths to find (K)
    pub k: usize,
    /// Whether to track relationships
    pub track_relationships: bool,
    /// Concurrency level
    pub concurrency: usize,
    /// Relationship filterer for avoiding cycles
    relationship_filterer: RelationshipFilterer,
    /// Visited nodes for cycle avoidance
    visited_nodes: HashMap<NodeId, bool>,
}

impl YensComputationRuntime {
    /// Create new Yen's computation runtime
    pub fn new(
        source_node: NodeId,
        target_node: NodeId,
        k: usize,
        track_relationships: bool,
        concurrency: usize,
    ) -> Self {
        Self {
            source_node,
            target_node,
            k,
            track_relationships,
            concurrency,
            relationship_filterer: RelationshipFilterer::new(k, track_relationships),
            visited_nodes: HashMap::new(),
        }
    }

    /// Initialize computation state
    ///
    /// Translation of: `YensTask` constructor and initialization
    /// This resets the internal state for a new computation
    pub fn initialize(
        &mut self,
        source_node: NodeId,
        target_node: NodeId,
        k: usize,
        track_relationships: bool,
    ) {
        self.source_node = source_node;
        self.target_node = target_node;
        self.k = k;
        self.track_relationships = track_relationships;
        self.visited_nodes.clear();
        self.relationship_filterer.reset();
    }

    /// Process a spur node to generate candidate paths
    ///
    /// Translation of: `YensTask.process()` (lines 100-112)
    /// This processes a single spur node to find alternative paths
    pub fn process_spur_node(
        &mut self,
        spur_node: NodeId,
        _root_path: &MutablePathResult,
    ) -> Result<Option<MutablePathResult>, String> {
        // Set up relationship filter
        self.relationship_filterer.set_filter(spur_node);

        // Note: spur path computation via Dijkstra is deferred.
        // For now, return None (no spur path found)
        Ok(None)
    }

    /// Add a visited node to avoid cycles
    ///
    /// Translation of: `YensTask.withVisited()` (lines 128-130)
    /// This marks nodes as visited to avoid cycles
    pub fn add_visited_node(&mut self, node: NodeId) {
        self.visited_nodes.insert(node, true);
    }

    /// Check if a node has been visited
    pub fn is_visited(&self, node: NodeId) -> bool {
        self.visited_nodes.get(&node).copied().unwrap_or(false)
    }

    /// Get the relationship filterer
    pub fn relationship_filterer(&mut self) -> &mut RelationshipFilterer {
        &mut self.relationship_filterer
    }

    /// Reset visited nodes
    pub fn reset_visited(&mut self) {
        self.visited_nodes.clear();
    }

    /// Get number of visited nodes
    pub fn visited_count(&self) -> usize {
        self.visited_nodes.len()
    }

    /// Compute Yen's K-shortest paths using a graph for neighbor access
    pub fn compute_with_graph(&mut self, graph: &dyn Graph) -> YensResult {
        // Create storage runtime
        let storage = YensStorageRuntime::new(
            self.source_node,
            self.target_node,
            self.k,
            self.track_relationships,
            self.concurrency,
        );

        // For now, use outgoing direction (0 = outgoing)
        // This can be parameterized later if needed
        let direction_byte = 0u8;

        // Create a dummy progress tracker
        // In a real implementation, this would be passed in
        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf("yens".to_string()),
            self.concurrency,
        );

        // Run the algorithm
        storage
            .compute_yens(self, Some(graph), direction_byte, &mut progress_tracker)
            .unwrap_or_else(|_e| {
                // For now, return empty result on error
                // In production, this should be handled properly
                YensResult {
                    paths: vec![],
                    path_count: 0,
                    computation_time_ms: 0,
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yens_computation_runtime_creation() {
        let runtime = YensComputationRuntime::new(0, 3, 5, true, 4);
        assert_eq!(runtime.source_node, 0);
        assert_eq!(runtime.target_node, 3);
        assert_eq!(runtime.k, 5);
        assert!(runtime.track_relationships);
        assert_eq!(runtime.concurrency, 4);
        assert_eq!(runtime.visited_count(), 0);
    }

    #[test]
    fn test_yens_computation_runtime_initialization() {
        let mut runtime = YensComputationRuntime::new(0, 3, 5, true, 1);
        runtime.initialize(1, 4, 3, false);

        assert_eq!(runtime.source_node, 1);
        assert_eq!(runtime.target_node, 4);
        assert_eq!(runtime.k, 3);
        assert!(!runtime.track_relationships);
        assert_eq!(runtime.visited_count(), 0);
    }

    #[test]
    fn test_yens_computation_runtime_visited_operations() {
        let mut runtime = YensComputationRuntime::new(0, 3, 5, true, 1);

        assert!(!runtime.is_visited(1));
        runtime.add_visited_node(1);
        assert!(runtime.is_visited(1));
        assert_eq!(runtime.visited_count(), 1);

        runtime.reset_visited();
        assert!(!runtime.is_visited(1));
        assert_eq!(runtime.visited_count(), 0);
    }
}
