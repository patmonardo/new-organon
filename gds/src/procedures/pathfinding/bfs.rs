//! BFS Facade
//!
//! **What is it?**: Breadth-First Search - level-by-level graph traversal
//! **Why care?**: Finds shortest paths in unweighted graphs, explores systematically
//! **Complexity**: O(V + E) - visits each node and edge once
//! **Best for**: Unweighted graphs, shortest paths by edge count, connectivity analysis
//!
//! ## How BFS Works
//!
//! BFS explores a graph level by level:
//! 1. Start with source node at distance 0
//! 2. Visit all neighbors at distance 1
//! 3. Visit all neighbors of those at distance 2
//! 4. Continue until target found or all reachable nodes visited
//!
//! ## Example
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # let graph: Graph = unimplemented!();
//! let traversal = graph
//!     .bfs()
//!     .source(42)
//!     .max_depth(5)
//!     .track_paths(true)
//!     .stream()?
//!     .collect::<Vec<_>>();
//! ```

use crate::core::utils::progress::{ProgressTracker, Tasks};
use crate::mem::MemoryRange;
use crate::algo::bfs::{BfsComputationRuntime, BfsStorageRuntime};
use crate::algo::core::prelude::{PathFindingResult, PathResultBuilder};
use crate::algo::core::result_builders::{ExecutionMetadata, ResultBuilder};
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::{PathResult, Result};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// ============================================================================
// Statistics Type
// ============================================================================

/// Statistics about BFS computation
#[derive(Debug, Clone, serde::Serialize)]
pub struct BfsStats {
    /// Number of nodes visited during traversal
    pub nodes_visited: u64,
    /// Maximum depth reached during traversal
    pub max_depth_reached: u64,
    /// Total computation time in milliseconds
    pub execution_time_ms: u64,
    /// Number of target nodes found (if any specified)
    pub targets_found: u64,
    /// Whether all targets were reached
    pub all_targets_reached: bool,
    /// Average branching factor (neighbors per node)
    pub avg_branching_factor: f64,
}

// ============================================================================
// Builder Type
// ============================================================================

/// BFS algorithm builder - fluent configuration
///
/// Use this to configure and run BFS with custom parameters.
/// Supports multiple execution modes via method chaining.
///
/// ## Default Configuration
/// - source: None (must be set explicitly)
/// - targets: empty (find all reachable nodes)
/// - max_depth: None (unlimited traversal)
/// - track_paths: false (only distances, not full paths)
/// - concurrency: 1 (BFS is typically single-threaded)
///
/// ## Example
/// ```rust,no_run
/// # use gds::Graph;
/// # let graph: Graph = unimplemented!();
/// # use gds::procedures::pathfinding::BfsBuilder;
/// let builder = graph.bfs()
///     .source(42)
///     .max_depth(5)
///     .track_paths(true)
///     .targets(vec![99, 100]);
/// ```
pub struct BfsBuilder {
    graph_store: Arc<DefaultGraphStore>,
    /// Source node for BFS traversal
    source: Option<u64>,
    /// Target nodes (empty = all reachable, specific = stop when found)
    targets: Vec<u64>,
    /// Maximum depth to traverse (None = unlimited)
    max_depth: Option<u32>,
    /// Whether to track full paths or just distances
    track_paths: bool,
    /// Concurrency level for parallel processing
    concurrency: usize,
    /// Delta parameter for chunking (affects performance)
    delta: usize,
}

impl BfsBuilder {
    /// Create a new BFS builder bound to a live graph store.
    ///
    /// Defaults:
    /// - source: None (must be set)
    /// - targets: empty (find all reachable nodes)
    /// - max_depth: None (unlimited traversal)
    /// - track_paths: false (only distances, not full paths)
    /// - concurrency: 1 (BFS is typically single-threaded)
    /// - delta: 64 (chunking parameter)
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            source: None,
            targets: vec![],
            max_depth: None,
            track_paths: false,
            concurrency: 1,
            delta: 64,
        }
    }

    /// Set source node
    ///
    /// The algorithm starts traversal from this node.
    /// Must be a valid node ID in the graph.
    pub fn source(mut self, source: u64) -> Self {
        self.source = Some(source);
        self
    }

    /// Set single target node
    ///
    /// If specified, traversal stops when target is reached.
    /// If not specified, traverses all reachable nodes.
    pub fn target(mut self, target: u64) -> Self {
        self.targets = vec![target];
        self
    }

    /// Set multiple target nodes
    ///
    /// Algorithm computes traversal until all targets are found or max depth reached.
    pub fn targets(mut self, targets: Vec<u64>) -> Self {
        self.targets = targets;
        self
    }

    /// Set maximum depth to traverse
    ///
    /// Limits how far from source to explore.
    /// Useful for neighborhood analysis or performance control.
    pub fn max_depth(mut self, depth: u32) -> Self {
        self.max_depth = Some(depth);
        self
    }

    /// Enable path tracking
    ///
    /// When true, results include full node sequences for each path.
    /// Slightly more memory usage but enables path reconstruction.
    pub fn track_paths(mut self, track: bool) -> Self {
        self.track_paths = track;
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// BFS is typically single-threaded but can benefit from parallelism.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Set delta parameter for chunking
    ///
    /// Affects internal chunking strategy.
    /// Higher values = larger chunks, better for large graphs.
    pub fn delta(mut self, delta: usize) -> Self {
        self.delta = delta;
        self
    }

    fn checked_node_id(value: u64, field: &str) -> Result<NodeId> {
        NodeId::try_from(value).map_err(|_| {
            crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                "{} must fit into i64 (got {})",
                field, value
            ))
        })
    }

    fn compute(self) -> Result<PathFindingResult> {
        self.validate()?;

        // Create progress tracker for BFS execution
        let node_count = self.graph_store.node_count();
        let _progress_tracker = ProgressTracker::new(Tasks::Leaf("BFS".to_string(), node_count));

        let source_u64 = self.source.expect("validate() ensures source is set");
        let source_node = Self::checked_node_id(source_u64, "source")?;
        let target_nodes: Vec<NodeId> = self
            .targets
            .iter()
            .map(|&value| Self::checked_node_id(value, "targets"))
            .collect::<Result<Vec<_>>>()?;

        let storage = BfsStorageRuntime::new(
            source_node,
            target_nodes.clone(),
            self.max_depth,
            self.track_paths,
            self.concurrency,
            self.delta,
        );

        let mut computation =
            BfsComputationRuntime::new(source_node, self.track_paths, self.concurrency);

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let result = storage.compute_bfs(&mut computation, Some(graph_view.as_ref()))?;

        let mut path_map: HashMap<NodeId, Vec<u64>> = HashMap::new();
        if self.track_paths {
            for path in result.paths {
                let node_path: Vec<u64> = path
                    .node_ids
                    .into_iter()
                    .filter(|node_id| *node_id >= 0)
                    .map(|node_id| node_id as u64)
                    .collect();
                path_map.insert(path.target_node, node_path);
            }
        }

        let mut max_depth_reached: u64 = 0;
        let mut targets_found: u64 = 0;
        let mut degree_sum: u64 = 0;

        let visited_nodes: Vec<(NodeId, u32)> = result.visited_nodes;
        for (node_id, depth) in &visited_nodes {
            max_depth_reached = max_depth_reached.max(*depth as u64);
            if !target_nodes.is_empty() && target_nodes.contains(node_id) {
                targets_found += 1;
            }
            if *node_id >= 0 {
                degree_sum += graph_view.degree(*node_id) as u64;
            }
        }

        let avg_branching_factor = if visited_nodes.is_empty() {
            0.0
        } else {
            degree_sum as f64 / visited_nodes.len() as f64
        };

        let all_targets_reached =
            !target_nodes.is_empty() && targets_found == target_nodes.len() as u64;

        let paths: Vec<crate::algo::core::result_builders::PathResult> = visited_nodes
            .into_iter()
            .filter(|(node_id, _)| *node_id >= 0)
            .map(|(node_id, depth)| {
                let target = node_id as u64;
                let path = path_map.get(&node_id).cloned().unwrap_or_default();
                crate::algo::core::result_builders::PathResult {
                    source: source_u64,
                    target,
                    path,
                    cost: depth as f64,
                }
            })
            .collect();

        // Create execution metadata
        let mut additional = std::collections::HashMap::new();
        additional.insert(
            "nodes_visited".to_string(),
            result.nodes_visited.to_string(),
        );
        additional.insert(
            "max_depth_reached".to_string(),
            max_depth_reached.to_string(),
        );
        additional.insert("targets_found".to_string(), targets_found.to_string());
        additional.insert(
            "all_targets_reached".to_string(),
            all_targets_reached.to_string(),
        );
        additional.insert(
            "avg_branching_factor".to_string(),
            avg_branching_factor.to_string(),
        );
        additional.insert("track_paths".to_string(), self.track_paths.to_string());
        additional.insert(
            "max_depth".to_string(),
            self.max_depth
                .map_or("unlimited".to_string(), |d| d.to_string()),
        );

        let metadata = ExecutionMetadata {
            execution_time: std::time::Duration::from_millis(result.computation_time_ms),
            iterations: None,
            converged: None,
            additional,
        };

        // Build result using upgraded result builder
        let path_result = PathResultBuilder::new()
            .with_paths(paths)
            .with_metadata(metadata)
            .build()
            .map_err(
                |e: crate::algo::core::result_builders::ResultBuilderError| {
                    crate::projection::eval::procedure::AlgorithmError::Execution(e.to_string())
                },
            )?;

        Ok(path_result)
    }

    /// Validate configuration before execution
    fn validate(&self) -> Result<()> {
        match self.source {
            None => {
                return Err(
                    crate::projection::eval::procedure::AlgorithmError::Execution(
                        "source node must be specified".to_string(),
                    ),
                )
            }
            Some(id) if id == u64::MAX => {
                return Err(
                    crate::projection::eval::procedure::AlgorithmError::Execution(
                        "source node ID cannot be u64::MAX".to_string(),
                    ),
                )
            }
            _ => {}
        }

        if self.concurrency == 0 {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "concurrency must be > 0".to_string(),
                ),
            );
        }

        if let Some(depth) = self.max_depth {
            if depth == 0 {
                return Err(
                    crate::projection::eval::procedure::AlgorithmError::Execution(
                        "max_depth must be > 0 or None".to_string(),
                    ),
                );
            }
        }

        Ok(())
    }

    /// Execute the algorithm and return iterator over traversal results
    ///
    /// Returns nodes in breadth-first order with their distances from source.
    ///
    /// Use this when you want individual traversal results:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// # use gds::procedures::pathfinding::BfsBuilder;
    /// let builder = graph.bfs().source(0).max_depth(3);
    /// for result in builder.stream()? {
    ///     println!("Node {} at distance {}", result.target, result.cost);
    /// }
    /// ```
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PathResult>>> {
        let result = self.compute()?;
        let paths = result.paths.into_iter().map(|p| PathResult {
            source: p.source,
            target: p.target,
            path: p.path,
            cost: p.cost,
        });
        Ok(Box::new(paths))
    }

    /// Stats mode: Get aggregated statistics
    ///
    /// Returns traversal statistics without individual nodes.
    ///
    /// Use this when you want performance metrics:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// # use gds::procedures::pathfinding::BfsBuilder;
    /// let builder = graph.bfs().source(0);
    /// let stats = builder.stats()?;
    /// println!("Visited {} nodes in {}ms", stats.nodes_visited, stats.execution_time_ms);
    /// ```
    pub fn stats(self) -> Result<BfsStats> {
        let result = self.compute()?;
        let nodes_visited = result
            .metadata
            .additional
            .get("nodes_visited")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let max_depth_reached = result
            .metadata
            .additional
            .get("max_depth_reached")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let targets_found = result
            .metadata
            .additional
            .get("targets_found")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let all_targets_reached = result
            .metadata
            .additional
            .get("all_targets_reached")
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(false);
        let avg_branching_factor = result
            .metadata
            .additional
            .get("avg_branching_factor")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0);

        Ok(BfsStats {
            nodes_visited,
            max_depth_reached,
            execution_time_ms: result.metadata.execution_time.as_millis() as u64,
            targets_found,
            all_targets_reached,
            avg_branching_factor,
        })
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores BFS distances as a node property.
    /// Property contains distance from source to each reachable node.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// # use gds::procedures::pathfinding::BfsBuilder;
    /// let builder = graph.bfs().source(0);
    /// let result = builder.mutate("distance")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "BFS mutate/write is not implemented yet".to_string(),
            ),
        )
    }

    /// Write mode: Compute and persist to storage
    ///
    /// Persists BFS traversal results and distances to storage backend.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// # use gds::procedures::pathfinding::BfsBuilder;
    /// let builder = graph.bfs().source(0);
    /// let result = builder.write("bfs_results")?;
    /// println!("Wrote {} nodes", result.nodes_written);
    /// ```
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "BFS mutate/write is not implemented yet".to_string(),
            ),
        )
    }

    /// Estimate memory requirements for BFS execution
    ///
    /// Returns a memory range estimate based on queue storage, visited tracking, and path storage.
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Queue storage (FIFO queue for BFS)
        let queue_memory = node_count * 8; // node_id per entry

        // Visited tracking (boolean array)
        let visited_memory = node_count;

        // Path tracking (if enabled)
        let path_memory = if self.track_paths {
            node_count * 8 // predecessor array
        } else {
            0
        };

        // Graph structure overhead
        let avg_degree = 10.0;
        let relationship_count = (node_count as f64 * avg_degree) as usize;
        let graph_overhead = relationship_count * 16;

        let total_memory = queue_memory + visited_memory + path_memory + graph_overhead;
        let overhead = total_memory / 5;
        MemoryRange::of_range(total_memory, total_memory + overhead)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    #[test]
    fn test_builder_defaults() {
        let config = RandomGraphConfig {
            seed: Some(1),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let builder = BfsBuilder::new(store);
        assert_eq!(builder.source, None);
        assert!(builder.targets.is_empty());
        assert!(builder.max_depth.is_none());
        assert!(!builder.track_paths);
        assert_eq!(builder.concurrency, 1);
        assert_eq!(builder.delta, 64);
    }

    #[test]
    fn test_builder_fluent_chain() {
        let config = RandomGraphConfig {
            seed: Some(1),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());

        let builder = BfsBuilder::new(store)
            .source(42)
            .targets(vec![99, 100])
            .max_depth(5)
            .track_paths(true)
            .concurrency(4)
            .delta(128);

        assert_eq!(builder.source, Some(42));
        assert_eq!(builder.targets, vec![99, 100]);
        assert_eq!(builder.max_depth, Some(5));
        assert!(builder.track_paths);
        assert_eq!(builder.concurrency, 4);
        assert_eq!(builder.delta, 128);
    }

    #[test]
    fn test_validate_missing_source() {
        let config = RandomGraphConfig {
            seed: Some(1),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let builder = BfsBuilder::new(store);
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_concurrency() {
        let config = RandomGraphConfig {
            seed: Some(1),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let builder = BfsBuilder::new(store).source(0).concurrency(0);
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_max_depth() {
        let config = RandomGraphConfig {
            seed: Some(1),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let builder = BfsBuilder::new(store).source(0).max_depth(0);
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_valid_config() {
        let config = RandomGraphConfig {
            seed: Some(1),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let builder = BfsBuilder::new(store)
            .source(0)
            .max_depth(5)
            .track_paths(true);
        assert!(builder.validate().is_ok());
    }

    #[test]
    fn test_stream_requires_validation() {
        let config = RandomGraphConfig {
            seed: Some(1),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let builder = BfsBuilder::new(store); // Missing source
        assert!(builder.stream().is_err());
    }

    #[test]
    fn test_stats_requires_validation() {
        let config = RandomGraphConfig {
            seed: Some(1),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let builder = BfsBuilder::new(store).concurrency(0); // Invalid concurrency
        assert!(builder.stats().is_err());
    }

    #[test]
    fn test_mutate_requires_validation() {
        let config = RandomGraphConfig {
            seed: Some(1),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let builder = BfsBuilder::new(store).source(0); // Valid config but...
        assert!(builder.mutate("").is_err()); // Empty property name
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let config = RandomGraphConfig {
            seed: Some(1),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let builder = BfsBuilder::new(store).source(0);
        assert!(builder.mutate("distance").is_err());
    }

    #[test]
    fn test_write_validates_property_name() {
        let config = RandomGraphConfig {
            seed: Some(1),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let builder = BfsBuilder::new(store).source(0);
        assert!(builder.write("bfs_results").is_err());
    }

    #[test]
    fn test_stream_returns_paths_to_targets() {
        let config = RandomGraphConfig {
            seed: Some(1),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let builder = BfsBuilder::new(store).source(0).targets(vec![3]);
        let results: Vec<_> = builder.stream().unwrap().collect();

        assert!(!results.is_empty());
        assert_eq!(results[0].source, 0);
    }

    #[test]
    fn test_stream_returns_all_reachable() {
        let config = RandomGraphConfig {
            seed: Some(1),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let builder = BfsBuilder::new(store).source(0); // No targets specified
        let results: Vec<_> = builder.stream().unwrap().collect();

        assert!(!results.is_empty());
        assert_eq!(results[0].source, 0);
    }

    #[test]
    fn test_stats_returns_aggregated_info() {
        let config = RandomGraphConfig {
            seed: Some(1),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let builder = BfsBuilder::new(store).source(0).targets(vec![1, 2, 3]);
        let stats = builder.stats().unwrap();

        assert!(stats.nodes_visited > 0);
    }
}
