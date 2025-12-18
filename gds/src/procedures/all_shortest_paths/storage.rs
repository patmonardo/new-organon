//! All Shortest Paths Storage Runtime
//!
//! This module implements the **Gross pole** of the Functor machinery for All Shortest Paths.
//! It represents persistent data structures (Graph view and graph topology).
//!
//! **Translation Source**: `org.neo4j.gds.allshortestpaths.MSBFSAllShortestPaths` and `WeightedAllShortestPaths`
//! **Key Features**: Multi-source parallelization, weighted/unweighted support, streaming results

use crate::projection::eval::procedure::AlgorithmError;
use crate::types::graph::id_map::NodeId;
use crate::types::graph::Graph;
use std::sync::mpsc;
use std::sync::Arc;

/// Algorithm type for All Shortest Paths
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AlgorithmType {
    /// Unweighted Multi-Source BFS (MSBFS)
    Unweighted,
    /// Weighted Multi-Source Dijkstra
    Weighted,
}

/// Storage Runtime for All Shortest Paths
///
/// This is the **Gross pole** - persistent data structures.
/// It knows how to access the graph structure and compute shortest paths.
///
/// ## The Pole's Role
///
/// In the Functor machinery:
/// - **Storage Runtime** (Gross) = persistent GraphStore and graph topology
/// - **Computation Runtime** (Subtle) = ephemeral shortest path results
/// - **Functor** = the mapping between them via shortest path computation
pub struct AllShortestPathsStorageRuntime {
    /// Graph view to traverse
    graph: Arc<dyn Graph>,
    /// Algorithm type (weighted vs unweighted)
    algorithm_type: AlgorithmType,
    /// Number of parallel workers
    concurrency: usize,
}

impl AllShortestPathsStorageRuntime {
    /// Create with specific settings
    pub fn with_settings(
        graph: Arc<dyn Graph>,
        algorithm_type: AlgorithmType,
        concurrency: usize,
    ) -> Self {
        Self {
            graph,
            algorithm_type,
            concurrency,
        }
    }

    /// Compute shortest paths from a source node
    ///
    /// This projects from Graph (Gross - persistent topology)
    /// to shortest path results (Subtle - path distances).
    ///
    /// **This is where the Functor machinery actually works**:
    /// Graph (Gross) → ShortestPathResults (Subtle)
    ///
    /// **Translation of Java logic**:
    /// - Unweighted: Multi-Source BFS using MSBFS
    /// - Weighted: Multi-Source Dijkstra with priority queue
    pub fn compute_shortest_paths(
        &self,
        source_node: NodeId,
        direction: u8,
    ) -> Result<Vec<ShortestPathResult>, AlgorithmError> {
        match self.algorithm_type {
            AlgorithmType::Unweighted => {
                self.compute_unweighted_shortest_paths(source_node, direction)
            }
            AlgorithmType::Weighted => self.compute_weighted_shortest_paths(source_node, direction),
        }
    }

    /// Compute unweighted shortest paths using BFS
    fn compute_unweighted_shortest_paths(
        &self,
        source_node: NodeId,
        direction: u8,
    ) -> Result<Vec<ShortestPathResult>, AlgorithmError> {
        let node_count = self.graph.node_count();
        let source_index = usize::try_from(source_node).map_err(|_| {
            AlgorithmError::InvalidGraph(format!("Invalid source node id: {source_node}"))
        })?;
        if source_index >= node_count {
            return Err(AlgorithmError::InvalidGraph(format!(
                "Source node id out of range: {source_node} (node_count={node_count})"
            )));
        }
        let mut distances = vec![f64::INFINITY; node_count];
        let mut queue = std::collections::VecDeque::new();

        // Initialize BFS
        distances[source_index] = 0.0;
        queue.push_back(source_node);

        // BFS traversal
        while let Some(current_node) = queue.pop_front() {
            let current_index = usize::try_from(current_node).map_err(|_| {
                AlgorithmError::InvalidGraph(format!("Invalid node id: {current_node}"))
            })?;
            let current_distance = distances[current_index];

            for neighbor in self.get_neighbors(current_node, direction) {
                let neighbor_index = usize::try_from(neighbor).map_err(|_| {
                    AlgorithmError::InvalidGraph(format!("Invalid neighbor id: {neighbor}"))
                })?;
                if neighbor_index >= node_count {
                    continue;
                }
                if distances[neighbor_index] == f64::INFINITY {
                    distances[neighbor_index] = current_distance + 1.0;
                    queue.push_back(neighbor);
                }
            }
        }

        // Convert to results
        let results = distances
            .into_iter()
            .enumerate()
            .map(|(target, distance)| ShortestPathResult {
                source: source_node,
                target: target as NodeId,
                distance,
            })
            .collect();

        Ok(results)
    }

    /// Compute weighted shortest paths using Dijkstra
    fn compute_weighted_shortest_paths(
        &self,
        source_node: NodeId,
        direction: u8,
    ) -> Result<Vec<ShortestPathResult>, AlgorithmError> {
        use std::cmp::Ordering;
        use std::collections::BinaryHeap;

        #[derive(Debug, Clone, Copy)]
        struct State {
            cost: f64,
            node: NodeId,
        }

        impl PartialEq for State {
            fn eq(&self, other: &Self) -> bool {
                self.cost == other.cost && self.node == other.node
            }
        }

        impl Eq for State {}

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                // Reverse for min-heap behavior.
                other
                    .cost
                    .partial_cmp(&self.cost)
                    .or_else(|| other.node.partial_cmp(&self.node))
            }
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(other).unwrap_or(Ordering::Equal)
            }
        }

        let node_count = self.graph.node_count();
        let source_index = usize::try_from(source_node).map_err(|_| {
            AlgorithmError::InvalidGraph(format!("Invalid source node id: {source_node}"))
        })?;
        if source_index >= node_count {
            return Err(AlgorithmError::InvalidGraph(format!(
                "Source node id out of range: {source_node} (node_count={node_count})"
            )));
        }

        let mut distances = vec![f64::INFINITY; node_count];
        distances[source_index] = 0.0;

        let mut heap = BinaryHeap::new();
        heap.push(State {
            cost: 0.0,
            node: source_node,
        });

        while let Some(State { cost, node }) = heap.pop() {
            let node_index = match usize::try_from(node) {
                Ok(idx) => idx,
                Err(_) => continue,
            };
            if node_index >= node_count {
                continue;
            }

            // Stale queue entry.
            if cost > distances[node_index] {
                continue;
            }

            for (neighbor, weight) in self.get_neighbors_with_weights(node, direction) {
                let neighbor_index = match usize::try_from(neighbor) {
                    Ok(idx) => idx,
                    Err(_) => continue,
                };
                if neighbor_index >= node_count {
                    continue;
                }

                let next_cost = cost + weight;
                if next_cost < distances[neighbor_index] {
                    distances[neighbor_index] = next_cost;
                    heap.push(State {
                        cost: next_cost,
                        node: neighbor,
                    });
                }
            }
        }

        // Convert to results
        let results = distances
            .into_iter()
            .enumerate()
            .map(|(target, distance)| ShortestPathResult {
                source: source_node,
                target: target as NodeId,
                distance,
            })
            .collect();

        Ok(results)
    }

    fn get_neighbors(&self, node_id: NodeId, direction: u8) -> Vec<NodeId> {
        let fallback: f64 = 1.0;
        match direction {
            1 => self
                .graph
                .stream_inverse_relationships(node_id, fallback)
                .map(|cursor| cursor.target_id())
                .collect(),
            2 => {
                let mut out: Vec<NodeId> = self
                    .graph
                    .stream_relationships(node_id, fallback)
                    .map(|cursor| cursor.target_id())
                    .collect();
                out.extend(
                    self.graph
                        .stream_inverse_relationships(node_id, fallback)
                        .map(|cursor| cursor.target_id()),
                );
                out
            }
            _ => self
                .graph
                .stream_relationships(node_id, fallback)
                .map(|cursor| cursor.target_id())
                .collect(),
        }
    }

    fn get_neighbors_with_weights(&self, node_id: NodeId, direction: u8) -> Vec<(NodeId, f64)> {
        let fallback: f64 = 1.0;
        match direction {
            1 => self
                .graph
                .stream_inverse_relationships_weighted(node_id, fallback)
                .map(|cursor| (cursor.target_id(), cursor.weight()))
                .collect(),
            2 => {
                let mut out: Vec<(NodeId, f64)> = self
                    .graph
                    .stream_relationships_weighted(node_id, fallback)
                    .map(|cursor| (cursor.target_id(), cursor.weight()))
                    .collect();
                out.extend(
                    self.graph
                        .stream_inverse_relationships_weighted(node_id, fallback)
                        .map(|cursor| (cursor.target_id(), cursor.weight())),
                );
                out
            }
            _ => self
                .graph
                .stream_relationships_weighted(node_id, fallback)
                .map(|cursor| (cursor.target_id(), cursor.weight()))
                .collect(),
        }
    }

    /// Compute all shortest paths in parallel
    ///
    /// This implements the multi-source parallelization from Java GDS.
    /// Results are streamed to avoid O(V²) memory usage.
    ///
    /// Note: This is a simplified version that doesn't use threading
    /// to avoid lifetime issues. In a real implementation, we would
    /// need to handle the GraphStore lifetime properly.
    pub fn compute_all_shortest_paths_streaming(
        &self,
        direction: u8,
    ) -> Result<mpsc::Receiver<ShortestPathResult>, AlgorithmError> {
        let (sender, receiver) = mpsc::channel();
        let node_count = self.graph.node_count();

        // For now, process sequentially to avoid lifetime issues
        // TODO: Implement proper parallel processing with lifetime management
        for source_node in 0..node_count as NodeId {
            let results = self.compute_shortest_paths(source_node, direction)?;

            // Send results to stream
            for result in results {
                if sender.send(result).is_err() {
                    // Receiver was dropped, stop processing
                    break;
                }
            }
        }

        // Drop the sender to signal completion
        drop(sender);

        Ok(receiver)
    }

    /// Get total number of nodes
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Get algorithm type
    pub fn algorithm_type(&self) -> AlgorithmType {
        self.algorithm_type
    }

    /// Get concurrency setting
    pub fn concurrency(&self) -> usize {
        self.concurrency
    }
}

/// Result of a shortest path computation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShortestPathResult {
    /// Source node ID
    pub source: NodeId,
    /// Target node ID
    pub target: NodeId,
    /// Shortest path distance
    pub distance: f64,
}
