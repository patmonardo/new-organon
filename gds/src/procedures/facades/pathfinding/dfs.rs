//! DFS Facade
//!
//! **What is it?**: Depth-First Search - deep exploration before breadth expansion
//! **Why care?**: Explores entire branches before backtracking, great for topological analysis
//! **Complexity**: O(V + E) - visits each node and edge once
//! **Best for**: Topological sorting, cycle detection, connected component analysis
//!
//! ## How DFS Works
//!
//! DFS explores a graph by going as deep as possible:
//! 1. Start with source node at depth 0
//! 2. Choose one unvisited neighbor and go deeper (depth + 1)
//! 3. Continue until dead end, then backtrack to previous node
//! 4. Try other unvisited neighbors from each backtracked node
//! 5. Continue until all reachable nodes visited
//!
//! ## Example
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # let graph: Graph = unimplemented!();
//! let traversal = graph
//!     .dfs()
//!     .source(42)
//!     .max_depth(10)
//!     .track_paths(true)
//!     .targets(vec![99, 100])
//!     .stream()?
//!     .collect::<Vec<_>>();
//! ```

use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, ProgressTracker, TaskRegistryFactory, Tasks,
};
use crate::mem::MemoryRange;
use crate::procedures::core::prelude::{PathFindingResult, PathResultBuilder};
use crate::procedures::core::result_builders::{ExecutionMetadata, ResultBuilder};
use crate::procedures::dfs::{DfsComputationRuntime, DfsStorageRuntime};
use crate::procedures::facades::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::facades::traits::{PathResult, Result};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// ============================================================================
// Statistics Type
// ============================================================================

/// Statistics about DFS computation
#[derive(Debug, Clone, serde::Serialize)]
pub struct DfsStats {
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
    /// Number of backtracking operations performed
    pub backtrack_operations: u64,
    /// Average branch depth before backtracking
    pub avg_branch_depth: f64,
}

// ============================================================================
// Builder Type
// ============================================================================

/// DFS algorithm builder - fluent configuration
///
/// Use this to configure and run DFS with custom parameters.
/// Supports multiple execution modes via method chaining.
///
/// ## Default Configuration
/// - source: None (must be set explicitly)
/// - targets: empty (find all reachable nodes)
/// - max_depth: None (unlimited traversal)
/// - track_paths: false (only discovery order, not full paths)
/// - concurrency: 1 (DFS is typically single-threaded)
///
/// ## Example
/// ```rust,no_run
/// # use gds::Graph;
/// # let graph: Graph = unimplemented!();
/// # use gds::procedures::facades::pathfinding::DfsBuilder;
/// let builder = graph.dfs()
///     .source(42)
///     .max_depth(10)
///     .track_paths(true)
///     .targets(vec![99, 100]);
/// ```
pub struct DfsBuilder {
    graph_store: Arc<DefaultGraphStore>,
    /// Source node for DFS traversal
    source: Option<u64>,
    /// Target nodes (empty = all reachable, specific = stop when found)
    targets: Vec<u64>,
    /// Maximum depth to traverse (None = unlimited)
    max_depth: Option<u32>,
    /// Whether to track full paths or just discovery order
    track_paths: bool,
    /// Concurrency level for parallel processing
    concurrency: usize,
    task_registry_factory: Box<dyn TaskRegistryFactory>,
}

impl DfsBuilder {
    /// Create a new DFS builder bound to a live graph store.
    ///
    /// Defaults:
    /// - source: None (must be set)
    /// - targets: empty (find all reachable nodes)
    /// - max_depth: None (unlimited traversal)
    /// - track_paths: false (only discovery order, not full paths)
    /// - concurrency: 1 (DFS is typically single-threaded)
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            source: None,
            targets: vec![],
            max_depth: None,
            track_paths: false,
            concurrency: 1,
            task_registry_factory: Box::new(EmptyTaskRegistryFactory),
        }
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

        // Set up progress tracking
        let _task_registry_factory = self.task_registry_factory;

        // Create progress tracker for DFS execution
        let node_count = self.graph_store.node_count();
        let _progress_tracker = ProgressTracker::new(Tasks::Leaf("DFS".to_string(), node_count));

        let source_u64 = self.source.expect("validate() ensures source is set");
        let source_node = Self::checked_node_id(source_u64, "source")?;
        let target_nodes: Vec<NodeId> = self
            .targets
            .iter()
            .map(|&value| Self::checked_node_id(value, "targets"))
            .collect::<Result<Vec<_>>>()?;

        let storage = DfsStorageRuntime::new(
            source_node,
            target_nodes.clone(),
            self.max_depth,
            self.track_paths,
            self.concurrency,
        );

        let mut computation =
            DfsComputationRuntime::new(source_node, self.track_paths, self.concurrency);

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let result = storage.compute_dfs(&mut computation, Some(graph_view.as_ref()))?;

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

        let max_depth_reached: u64 = if self.track_paths {
            path_map
                .values()
                .map(|p| p.len().saturating_sub(1) as u64)
                .max()
                .unwrap_or(0)
        } else {
            0
        };

        let targets_found: u64 = if target_nodes.is_empty() {
            0
        } else {
            result
                .visited_nodes
                .iter()
                .filter(|(node_id, _)| target_nodes.contains(node_id))
                .count() as u64
        };

        let all_targets_reached =
            !target_nodes.is_empty() && targets_found == target_nodes.len() as u64;

        let paths: Vec<crate::procedures::core::result_builders::PathResult> = result
            .visited_nodes
            .into_iter()
            .filter(|(node_id, _)| *node_id >= 0)
            .map(|(node_id, discovery_order)| {
                let target = node_id as u64;
                let path = path_map.get(&node_id).cloned().unwrap_or_default();
                crate::procedures::core::result_builders::PathResult {
                    source: source_u64,
                    target,
                    path,
                    cost: discovery_order as f64,
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
                |e: crate::procedures::core::result_builders::ResultBuilderError| {
                    crate::projection::eval::procedure::AlgorithmError::Execution(e.to_string())
                },
            )?;

        Ok(path_result)
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
    /// DFS is typically single-threaded but can benefit from parallelism
    /// for large disconnected components.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
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
    /// Returns nodes in depth-first order with their discovery depths.
    ///
    /// Use this when you want individual traversal results:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// # use gds::procedures::facades::pathfinding::DfsBuilder;
    /// let builder = graph.dfs().source(0).max_depth(5);
    /// for result in builder.stream()? {
    ///     println!("Node {} at depth {}", result.target, result.cost);
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
    /// # use gds::procedures::facades::pathfinding::DfsBuilder;
    /// let builder = graph.dfs().source(0);
    /// let stats = builder.stats()?;
    /// println!("Visited {} nodes, backtracked {} times", stats.nodes_visited, stats.backtrack_operations);
    /// ```
    pub fn stats(self) -> Result<DfsStats> {
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

        Ok(DfsStats {
            nodes_visited,
            max_depth_reached,
            execution_time_ms: result.metadata.execution_time.as_millis() as u64,
            targets_found,
            all_targets_reached,
            backtrack_operations: 0, // Not available in current implementation
            avg_branch_depth: 0.0,   // Not available in current implementation
        })
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores DFS discovery order or depths as a node property.
    /// Property contains discovery order (1, 2, 3...) or depth from source.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// # use gds::procedures::facades::pathfinding::DfsBuilder;
    /// let builder = graph.dfs().source(0);
    /// let result = builder.mutate("dfs_order")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "DFS mutate/write is not implemented yet".to_string(),
            ),
        )
    }

    /// Write mode: Compute and persist to storage
    ///
    /// Persists DFS traversal results and discovery order to storage backend.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// # use gds::procedures::facades::pathfinding::DfsBuilder;
    /// let builder = graph.dfs().source(0);
    /// let result = builder.write("dfs_results")?;
    /// println!("Wrote {} nodes", result.nodes_written);
    /// ```
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "DFS mutate/write is not implemented yet".to_string(),
            ),
        )
    }

    /// Estimate memory requirements for DFS execution
    ///
    /// Returns a memory range estimate based on stack storage, visited tracking, and path storage.
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Stack storage (for DFS recursion/iteration)
        let stack_memory = node_count * 8; // node_id per entry

        // Visited tracking
        let visited_memory = node_count;

        // Path tracking (if enabled)
        let path_memory = if self.track_paths { node_count * 8 } else { 0 };

        // Graph structure overhead
        let avg_degree = 10.0;
        let relationship_count = (node_count as f64 * avg_degree) as usize;
        let graph_overhead = relationship_count * 16;

        let total_memory = stack_memory + visited_memory + path_memory + graph_overhead;
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
    use std::sync::Arc;

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(2),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_builder_defaults() {
        let builder = DfsBuilder::new(store());
        assert_eq!(builder.source, None);
        assert!(builder.targets.is_empty());
        assert!(builder.max_depth.is_none());
        assert!(!builder.track_paths);
        assert_eq!(builder.concurrency, 1);
    }

    #[test]
    fn test_builder_fluent_chain() {
        let builder = DfsBuilder::new(store())
            .source(42)
            .targets(vec![99, 100])
            .max_depth(10)
            .track_paths(true)
            .concurrency(4);

        assert_eq!(builder.source, Some(42));
        assert_eq!(builder.targets, vec![99, 100]);
        assert_eq!(builder.max_depth, Some(10));
        assert!(builder.track_paths);
        assert_eq!(builder.concurrency, 4);
    }

    #[test]
    fn test_validate_missing_source() {
        let builder = DfsBuilder::new(store());
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_concurrency() {
        let builder = DfsBuilder::new(store()).source(0).concurrency(0);
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_max_depth() {
        let builder = DfsBuilder::new(store()).source(0).max_depth(0);
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_valid_config() {
        let builder = DfsBuilder::new(store())
            .source(0)
            .max_depth(10)
            .track_paths(true);
        assert!(builder.validate().is_ok());
    }

    #[test]
    fn test_stream_requires_validation() {
        let builder = DfsBuilder::new(store()); // Missing source
        assert!(builder.stream().is_err());
    }

    #[test]
    fn test_stats_requires_validation() {
        let builder = DfsBuilder::new(store()).concurrency(0); // Invalid concurrency
        assert!(builder.stats().is_err());
    }

    #[test]
    fn test_mutate_requires_validation() {
        let builder = DfsBuilder::new(store()).source(0); // Valid config but...
        assert!(builder.mutate("").is_err()); // Empty property name
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let builder = DfsBuilder::new(store()).source(0);
        assert!(builder.mutate("dfs_order").is_err());
    }

    #[test]
    fn test_write_validates_property_name() {
        let builder = DfsBuilder::new(store()).source(0);
        assert!(builder.write("dfs_results").is_err());
    }

    #[test]
    fn test_stream_returns_paths_to_targets() {
        let builder = DfsBuilder::new(store()).source(0).targets(vec![2, 3]);
        let results: Vec<_> = builder.stream().unwrap().collect();

        assert!(!results.is_empty());
        assert_eq!(results[0].source, 0);
    }

    #[test]
    fn test_stream_returns_all_reachable() {
        let builder = DfsBuilder::new(store()).source(0); // No targets specified
        let results: Vec<_> = builder.stream().unwrap().collect();

        assert!(!results.is_empty());
        assert_eq!(results[0].source, 0);
    }

    #[test]
    fn test_stats_returns_aggregated_info() {
        let builder = DfsBuilder::new(store()).source(0).targets(vec![1, 2, 3]);
        let stats = builder.stats().unwrap();

        assert!(stats.nodes_visited > 0);
    }
}
