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

use crate::algo::algorithms::pathfinding::PathResult;
use crate::algo::algorithms::ConfigValidator;
use crate::algo::algorithms::Result;
use crate::algo::bfs::{
    BfsComputationRuntime, BfsConfig, BfsMutateResult, BfsResult, BfsResultBuilder, BfsStats,
    BfsStorageRuntime, BfsWriteSummary,
};
use crate::core::utils::progress::{TaskProgressTracker, Tasks};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;

// ============================================================================
// Facade Type
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
pub struct BfsFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: BfsConfig,
}

/// Backwards-compatible alias (builder-style naming).
pub type BfsBuilder = BfsFacade;

impl BfsFacade {
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
            config: BfsConfig::default(),
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: BfsConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: BfsConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: BfsConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
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
    /// BFS is typically single-threaded but can benefit from parallelism.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    /// Set delta parameter for chunking
    ///
    /// Affects internal chunking strategy.
    /// Higher values = larger chunks, better for large graphs.
    pub fn delta(mut self, delta: usize) -> Self {
        self.config.delta = delta;
        self
    }

    fn compute(self) -> Result<(BfsResult, std::time::Duration)> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        // Create progress tracker for BFS execution.
        // We track progress in terms of relationships examined.
        let task = Tasks::leaf("BFS".to_string());
        let mut progress_tracker =
            TaskProgressTracker::with_concurrency(task, self.config.concurrency);

        let source_node = self.config.source_node;
        let target_nodes = self.config.target_nodes.clone();

        let storage = BfsStorageRuntime::new(
            source_node,
            target_nodes.clone(),
            self.config.max_depth,
            self.config.track_paths,
        );

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let node_count = graph_view.node_count() as usize;
        let mut computation = BfsComputationRuntime::new(
            source_node,
            self.config.track_paths,
            self.config.concurrency,
            node_count,
        );

        let start = std::time::Instant::now();
        let result: BfsResult = storage.compute_bfs(
            &mut computation,
            Some(graph_view.as_ref()),
            &mut progress_tracker,
        )?;
        Ok((result, start.elapsed()))
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
        let source_node = self.config.source_node;
        let target_count = self.config.target_nodes.len();
        let (result, elapsed) = self.compute()?;
        let paths = BfsResultBuilder::new(result, elapsed, source_node, target_count).paths();
        Ok(Box::new(paths.into_iter()))
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
        let source_node = self.config.source_node;
        let target_count = self.config.target_nodes.len();
        let (result, elapsed) = self.compute()?;
        Ok(BfsResultBuilder::new(result, elapsed, source_node, target_count).stats())
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
    pub fn mutate(self, property_name: &str) -> Result<BfsMutateResult> {
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let graph_store = Arc::clone(&self.graph_store);
        let source_node = self.config.source_node;
        let target_count = self.config.target_nodes.len();
        let (result, elapsed) = self.compute()?;
        let builder = BfsResultBuilder::new(result, elapsed, source_node, target_count);
        let paths = builder.paths();

        let updated_store = crate::algo::algorithms::pathfinding::build_path_relationship_store(
            graph_store.as_ref(),
            property_name,
            &paths,
        )?;

        let summary = builder.mutation_summary(property_name, paths.len() as u64);

        Ok(BfsMutateResult {
            summary,
            updated_store,
        })
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
    pub fn write(self, property_name: &str) -> Result<BfsWriteSummary> {
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let res = self.mutate(property_name)?;
        Ok(BfsWriteSummary {
            nodes_written: res.summary.nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: res.summary.execution_time_ms,
        })
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
        let path_memory = if self.config.track_paths {
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
        assert_eq!(builder.config.source_node, 0);
        assert!(builder.config.target_nodes.is_empty());
        assert!(builder.config.max_depth.is_none());
        assert!(!builder.config.track_paths);
        assert_eq!(builder.config.concurrency, 1);
        assert_eq!(builder.config.delta, 64);
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

        assert_eq!(builder.config.source_node, 42);
        assert_eq!(builder.config.target_nodes, vec![99, 100]);
        assert_eq!(builder.config.max_depth, Some(5));
        assert!(builder.config.track_paths);
        assert_eq!(builder.config.concurrency, 4);
        assert_eq!(builder.config.delta, 128);
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
        let builder = BfsBuilder::new(store).source_node(-1);
        assert!(builder.config.validate().is_err());
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
        assert!(builder.config.validate().is_err());
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
        assert!(builder.config.validate().is_err());
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
        assert!(builder.config.validate().is_ok());
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
        let builder = BfsBuilder::new(store).source_node(-1);
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
        assert!(builder.mutate("distance").is_ok());
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
        assert!(builder.write("bfs_results").is_ok());
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
