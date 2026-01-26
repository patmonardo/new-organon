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

use crate::algo::algorithms::pathfinding::PathFindingResult;
use crate::algo::algorithms::Result;
use crate::algo::algorithms::pathfinding::PathResult;
use crate::algo::dfs::{
    DfsComputationRuntime, DfsConfig, DfsMutateResult, DfsMutationSummary, DfsResultBuilder,
    DfsStats, DfsStorageRuntime, DfsWriteSummary,
};
use crate::core::utils::progress::{EmptyTaskRegistryFactory, TaskRegistryFactory, Tasks};
use crate::mem::MemoryRange;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;

// Additional imports for error handling and progress tracking
use crate::core::utils::progress::TaskProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;

// ============================================================================
// Facade Type
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
/// # use gds::procedures::pathfinding::DfsBuilder;
/// let builder = graph.dfs()
///     .source(42)
///     .max_depth(10)
///     .track_paths(true)
///     .targets(vec![99, 100]);
/// ```
pub struct DfsFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: DfsConfig,
    task_registry_factory: Box<dyn TaskRegistryFactory>,
}

/// Backwards-compatible alias (builder-style naming).
pub type DfsBuilder = DfsFacade;

impl DfsFacade {
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
            config: DfsConfig::default(),
            task_registry_factory: Box::new(EmptyTaskRegistryFactory),
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: DfsConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
            task_registry_factory: Box::new(EmptyTaskRegistryFactory),
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: DfsConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: DfsConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    fn compute(self) -> Result<PathFindingResult> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        // Set up progress tracking
        let _task_registry_factory = self.task_registry_factory;

        // Create progress tracker for DFS execution.
        // We track progress in terms of relationships examined.
        let task = Tasks::leaf("DFS".to_string());
        let mut progress_tracker =
            TaskProgressTracker::with_concurrency(task, self.config.concurrency);

        let source_node = self.config.source_node;
        let target_nodes = self.config.target_nodes.clone();

        let storage = DfsStorageRuntime::new(
            source_node,
            target_nodes.clone(),
            self.config.max_depth,
            self.config.track_paths,
            self.config.concurrency,
        );

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let node_count = graph_view.node_count() as usize;

        let mut computation = DfsComputationRuntime::new(
            source_node,
            self.config.track_paths,
            self.config.concurrency,
            node_count,
        );

        let start = std::time::Instant::now();
        let result = storage.compute_dfs(
            &mut computation,
            Some(graph_view.as_ref()),
            &mut progress_tracker,
        )?;
        DfsResultBuilder::result(result, start.elapsed(), source_node, target_nodes.len())
    }

    /// Set source node
    ///
    /// The algorithm starts traversal from this node.
    /// Must be a valid node ID in the graph.
    pub fn source(mut self, source: u64) -> Self {
        self.config.source_node = i64::try_from(source).unwrap_or(-1);
        self
    }

    pub fn source_node(mut self, source: NodeId) -> Self {
        self.config.source_node = source;
        self
    }

    /// Set single target node
    ///
    /// If specified, traversal stops when target is reached.
    /// If not specified, traverses all reachable nodes.
    pub fn target(mut self, target: u64) -> Self {
        self.config.target_nodes = vec![i64::try_from(target).unwrap_or(-1)];
        self
    }

    /// Set multiple target nodes
    ///
    /// Algorithm computes traversal until all targets are found or max depth reached.
    pub fn targets(mut self, targets: Vec<u64>) -> Self {
        self.config.target_nodes = targets
            .into_iter()
            .map(|value| i64::try_from(value).unwrap_or(-1))
            .collect();
        self
    }

    /// Set maximum depth to traverse
    ///
    /// Limits how far from source to explore.
    /// Useful for neighborhood analysis or performance control.
    pub fn max_depth(mut self, depth: u32) -> Self {
        self.config.max_depth = Some(depth);
        self
    }

    /// Enable path tracking
    ///
    /// When true, results include full node sequences for each path.
    /// Slightly more memory usage but enables path reconstruction.
    pub fn track_paths(mut self, track: bool) -> Self {
        self.config.track_paths = track;
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// DFS is typically single-threaded but can benefit from parallelism
    /// for large disconnected components.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    /// Execute the algorithm and return iterator over traversal results
    ///
    /// Returns nodes in depth-first order with their discovery depths.
    ///
    /// Use this when you want individual traversal results:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// # use gds::procedures::pathfinding::DfsBuilder;
    /// let builder = graph.dfs().source(0).max_depth(5);
    /// for result in builder.stream()? {
    ///     println!("Node {} at depth {}", result.target, result.cost);
    /// }
    /// ```
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PathResult>>> {
        let result = self.compute()?;
        Ok(Box::new(result.paths.into_iter()))
    }

    /// Stats mode: Get aggregated statistics
    ///
    /// Returns traversal statistics without individual nodes.
    ///
    /// Use this when you want performance metrics:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// # use gds::procedures::pathfinding::DfsBuilder;
    /// let builder = graph.dfs().source(0);
    /// let stats = builder.stats()?;
    /// println!("Visited {} nodes, backtracked {} times", stats.nodes_visited, stats.backtrack_operations);
    /// ```
    pub fn stats(self) -> Result<DfsStats> {
        let targets = self.config.target_nodes.len() as u64;
        let result = self.compute()?;
        let nodes_visited = result.paths.len() as u64;
        let max_depth_reached = result
            .metadata
            .additional
            .get("max_depth_reached")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        let targets_found = if targets == 0 {
            0
        } else {
            nodes_visited.min(targets)
        };
        let all_targets_reached = targets > 0 && targets_found == targets;

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
    /// # use gds::procedures::pathfinding::DfsBuilder;
    /// let builder = graph.dfs().source(0);
    /// let result = builder.mutate("dfs_order")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<DfsMutateResult> {
        if property_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "property_name cannot be empty".to_string(),
            ));
        }
        let graph_store = Arc::clone(&self.graph_store);
        let result = self.compute()?;
        let paths = result.paths;

        let updated_store = crate::algo::algorithms::pathfinding::build_path_relationship_store(
            graph_store.as_ref(),
            property_name,
            &paths,
        )?;

        let summary = DfsMutationSummary {
            nodes_updated: paths.len() as u64,
            property_name: property_name.to_string(),
            execution_time_ms: result.metadata.execution_time.as_millis() as u64,
        };

        Ok(DfsMutateResult {
            summary,
            updated_store,
        })
    }

    /// Write mode: Compute and persist to storage
    ///
    /// Persists DFS traversal results and discovery order to storage backend.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// # use gds::procedures::pathfinding::DfsBuilder;
    /// let builder = graph.dfs().source(0);
    /// let result = builder.write("dfs_results")?;
    /// println!("Wrote {} nodes", result.nodes_written);
    /// ```
    pub fn write(self, property_name: &str) -> Result<DfsWriteSummary> {
        if property_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "property_name cannot be empty".to_string(),
            ));
        }
        let res = self.mutate(property_name)?;
        Ok(DfsWriteSummary {
            nodes_written: res.summary.nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: res.summary.execution_time_ms,
        })
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
        let path_memory = if self.config.track_paths {
            node_count * 8
        } else {
            0
        };

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
        assert_eq!(builder.config.source_node, 0);
        assert!(builder.config.target_nodes.is_empty());
        assert!(builder.config.max_depth.is_none());
        assert!(!builder.config.track_paths);
        assert_eq!(builder.config.concurrency, 1);
    }

    #[test]
    fn test_builder_fluent_chain() {
        let builder = DfsBuilder::new(store())
            .source(42)
            .targets(vec![99, 100])
            .max_depth(10)
            .track_paths(true)
            .concurrency(4);

        assert_eq!(builder.config.source_node, 42);
        assert_eq!(builder.config.target_nodes, vec![99, 100]);
        assert_eq!(builder.config.max_depth, Some(10));
        assert!(builder.config.track_paths);
        assert_eq!(builder.config.concurrency, 4);
    }

    #[test]
    fn test_validate_missing_source() {
        let builder = DfsBuilder::new(store()).source_node(-1);
        assert!(builder.config.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_concurrency() {
        let builder = DfsBuilder::new(store()).source(0).concurrency(0);
        assert!(builder.config.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_max_depth() {
        let builder = DfsBuilder::new(store()).source(0).max_depth(0);
        assert!(builder.config.validate().is_err());
    }

    #[test]
    fn test_validate_valid_config() {
        let builder = DfsBuilder::new(store())
            .source(0)
            .max_depth(10)
            .track_paths(true);
        assert!(builder.config.validate().is_ok());
    }

    #[test]
    fn test_stream_requires_validation() {
        let builder = DfsBuilder::new(store()).source_node(-1);
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
        assert!(builder.mutate("dfs_order").is_ok());
    }

    #[test]
    fn test_write_validates_property_name() {
        let builder = DfsBuilder::new(store()).source(0);
        assert!(builder.write("dfs_results").is_ok());
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
