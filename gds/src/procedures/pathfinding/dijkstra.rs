//! Dijkstra Facade
//!
//! **What is it?**: Shortest path algorithm using priority queue (Dijkstra's algorithm)
//! **Why care?**: Finds optimal paths by weight, guarantees shortest path in non-negative graphs
//! **Complexity**: O((V + E) log V) with binary heap, O(V²) with simple implementation
//! **Best for**: Weighted graphs with non-negative edge weights
//!
//! ## How Dijkstra Works
//!
//! Dijkstra finds shortest paths from a source node by:
//! 1. Starting with source node distance = 0, others = infinity
//! 2. Using priority queue to always expand lowest-distance node
//! 3. Relaxing edges: `distance[v] = min(distance[v], distance[u] + weight(u,v))`
//! 4. Continuing until target reached or all reachable nodes visited
//!
//! ## Example
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # let graph: Graph = unimplemented!();
//! let paths = graph
//!     .dijkstra()
//!     .source(42)
//!     .target(99)
//!     .weight_property("cost")
//!     .stream()?
//!     .collect::<Vec<_>>();
//! ```

use crate::algo::algorithms::Result;
use crate::algo::dijkstra::{
    create_targets, DijkstraComputationRuntime, DijkstraConfig, DijkstraMutateResult,
    DijkstraMutationSummary, DijkstraResult, DijkstraResultBuilder, DijkstraStats,
    DijkstraStorageRuntime, DijkstraWriteSummary,
};
use crate::mem::MemoryRange;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

// Import upgraded systems
use crate::algo::algorithms::pathfinding::{PathFindingResult, PathResult};
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::{EmptyTaskRegistryFactory, TaskRegistryFactory, Tasks};
use crate::projection::eval::algorithm::AlgorithmError;

// ============================================================================
// Facade Type
// ============================================================================

/// Dijkstra algorithm builder - fluent configuration
///
/// Use this to configure and run Dijkstra with custom parameters.
/// Supports multiple execution modes via method chaining.
///
/// ## Default Configuration
/// - source: 0 (must be set explicitly)
/// - targets: empty (compute all reachable paths)
/// - weight_property: "weight"
/// - direction: "outgoing"
/// - track_relationships: false
/// - concurrency: 4
///
/// ## Example
/// ```rust,no_run
/// # use gds::Graph;
/// # let graph: Graph = unimplemented!();
/// # use gds::procedures::pathfinding::DijkstraBuilder;
/// let builder = graph.dijkstra()
///     .source(42)
///     .target(99)
///     .weight_property("cost")
///     .direction("outgoing");
/// ```
pub struct DijkstraFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: DijkstraConfig,
    /// Property name for edge weights
    weight_property: String,
    /// Progress tracking components
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

/// Backwards-compatible alias (builder-style naming).
pub type DijkstraBuilder = DijkstraFacade;

impl DijkstraFacade {
    /// Create a new Dijkstra builder bound to a live graph store.
    ///
    /// Defaults:
    /// - source: None (must be set)
    /// - targets: empty (compute all reachable paths)
    /// - weight_property: "weight"
    /// - direction: "outgoing"
    /// - track_relationships: false
    /// - concurrency: 4
    /// - progress tracking: None (uses defaults)
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: DijkstraConfig::default(),
            weight_property: "weight".to_string(),
            task_registry_factory: None,
            user_log_registry_factory: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: DijkstraConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
            weight_property: "weight".to_string(),
            task_registry_factory: None,
            user_log_registry_factory: None,
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: DijkstraConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: DijkstraConfig) -> Result<Self> {
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

        if self.weight_property.is_empty() {
            return Err(AlgorithmError::Execution(
                "weight_property cannot be empty".to_string(),
            ));
        }

        // Set up progress tracking
        let _task_registry_factory = self
            .task_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));
        let _user_log_registry_factory = self
            .user_log_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));

        // Progress tracker is best-effort; the driver loop in storage owns begin/log/end.

        let source_node = self.config.source_node;
        let targets = create_targets(self.config.target_nodes.clone());

        let mut storage = DijkstraStorageRuntime::new(
            source_node,
            self.config.track_relationships,
            self.config.concurrency,
            self.config.use_heuristic,
        );

        let mut computation = DijkstraComputationRuntime::new(
            source_node,
            self.config.track_relationships,
            self.config.concurrency,
            self.config.use_heuristic,
        );

        // Use selectors so the algorithm consumes the requested weight property.
        // (If no selector is provided, DefaultGraph may auto-select only when exactly one exists.)
        let rel_types: HashSet<RelationshipType> = if self.config.relationship_types.is_empty() {
            self.graph_store.relationship_types()
        } else {
            RelationshipType::list_of(self.config.relationship_types.clone())
                .into_iter()
                .collect()
        };
        let selectors: HashMap<RelationshipType, String> = rel_types
            .iter()
            .map(|t| (t.clone(), self.weight_property.clone()))
            .collect();
        let (orientation, direction_byte) = match self.config.direction.as_str() {
            "incoming" => (Orientation::Reverse, 1u8),
            "both" => (Orientation::Undirected, 0u8),
            _ => (Orientation::Natural, 0u8),
        };

        let graph_view = self
            .graph_store
            .get_graph_with_types_selectors_and_orientation(&rel_types, &selectors, orientation)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("dijkstra".to_string(), graph_view.relationship_count()),
            self.config.concurrency,
        );

        let start = std::time::Instant::now();
        let result: DijkstraResult = storage.compute_dijkstra(
            &mut computation,
            targets,
            Some(graph_view.as_ref()),
            direction_byte,
            &mut progress_tracker,
        )?;
        DijkstraResultBuilder::result(result, start.elapsed())
    }

    /// Set source node (NodeId)
    pub fn source_node(mut self, source: NodeId) -> Self {
        self.config.source_node = source;
        self
    }

    /// Set source node
    ///
    /// The algorithm starts path computation from this node.
    /// Must be a valid node ID in the graph.
    pub fn source(mut self, source: u64) -> Self {
        self.config.source_node = i64::try_from(source).unwrap_or(-1);
        self
    }

    /// Set single target node
    ///
    /// If specified, algorithm stops when target is reached.
    /// If not specified, computes paths to all reachable nodes.
    pub fn target(mut self, target: u64) -> Self {
        self.config.target_nodes = vec![i64::try_from(target).unwrap_or(-1)];
        self
    }

    /// Set multiple target nodes
    ///
    /// Algorithm computes shortest paths to all specified targets.
    pub fn targets(mut self, targets: Vec<u64>) -> Self {
        self.config.target_nodes = targets
            .into_iter()
            .map(|value| i64::try_from(value).unwrap_or(-1))
            .collect();
        self
    }

    /// Set weight property name
    ///
    /// Property must exist on relationships and contain numeric values.
    /// Default: "weight"
    pub fn weight_property(mut self, property: &str) -> Self {
        self.weight_property = property.to_string();
        self
    }

    /// Set relationship types filter
    pub fn relationship_types(mut self, relationship_types: Vec<String>) -> Self {
        self.config.relationship_types = relationship_types;
        self
    }

    /// Set traversal direction
    ///
    /// Options: "incoming", "outgoing", "both"
    /// Default: "outgoing"
    pub fn direction(mut self, direction: &str) -> Self {
        self.config.direction = direction.to_string();
        self
    }

    /// Enable relationship tracking in results
    ///
    /// When true, results include relationship IDs along paths.
    /// Slightly more memory usage but enables path reconstruction.
    pub fn track_relationships(mut self, track: bool) -> Self {
        self.config.track_relationships = track;
        self
    }

    /// Enable heuristic function (A* behavior)
    pub fn use_heuristic(mut self, enabled: bool) -> Self {
        self.config.use_heuristic = enabled;
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// Higher values = faster on large graphs, but more memory.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    /// Set task registry factory for progress tracking
    pub fn task_registry_factory(mut self, factory: Box<dyn TaskRegistryFactory>) -> Self {
        self.task_registry_factory = Some(factory);
        self
    }

    /// Set user log registry factory for progress tracking
    pub fn user_log_registry_factory(mut self, factory: Box<dyn TaskRegistryFactory>) -> Self {
        self.user_log_registry_factory = Some(factory);
        self
    }

    /// Execute the algorithm and return iterator over path results
    ///
    /// Returns paths from source to target(s) in order of discovery.
    ///
    /// Use this when you want individual path results:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// # use gds::procedures::pathfinding::DijkstraBuilder;
    /// let builder = graph.dijkstra().source(0).target(5);
    /// for path in builder.stream()? {
    ///     println!("Path: {:?}, Cost: {}", path.path, path.cost);
    /// }
    /// ```
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PathResult>>> {
        let result = self.compute()?;
        Ok(Box::new(result.paths.into_iter()))
    }

    /// Stats mode: Get aggregated statistics
    ///
    /// Returns computation statistics without individual paths.
    ///
    /// Use this when you want performance metrics:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// # use gds::procedures::pathfinding::DijkstraBuilder;
    /// let builder = graph.dijkstra().source(0);
    /// let stats = builder.stats()?;
    /// println!("Found {} paths in {}ms", stats.paths_found, stats.execution_time_ms);
    /// ```
    pub fn stats(self) -> Result<DijkstraStats> {
        let has_targets = !self.config.target_nodes.is_empty();
        let result = self.compute()?;
        Ok(DijkstraStats {
            paths_found: result.paths.len() as u64,
            execution_time_ms: result.metadata.execution_time.as_millis() as u64,
            nodes_expanded: 0,   // Note: extract from metadata when available.
            edges_considered: 0, // Note: extract from metadata when available.
            max_queue_size: 0,   // Note: extract from metadata when available.
            target_reached: !result.paths.is_empty() && has_targets,
        })
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores shortest path distances as a node property.
    /// Property contains distance from source to each reachable node.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// # use gds::procedures::pathfinding::DijkstraBuilder;
    /// let builder = graph.dijkstra().source(0);
    /// let result = builder.mutate("distance")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<DijkstraMutateResult> {
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

        let summary = DijkstraMutationSummary {
            nodes_updated: paths.len() as u64,
            property_name: property_name.to_string(),
            execution_time_ms: result.metadata.execution_time.as_millis() as u64,
        };

        Ok(DijkstraMutateResult {
            summary,
            updated_store,
        })
    }

    /// Write mode: Compute and persist to storage
    ///
    /// Persists shortest paths and distances to storage backend.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// # use gds::procedures::pathfinding::DijkstraBuilder;
    /// let builder = graph.dijkstra().source(0);
    /// let result = builder.write("paths")?;
    /// println!("Wrote {} nodes", result.nodes_written);
    /// ```
    pub fn write(self, property_name: &str) -> Result<DijkstraWriteSummary> {
        if property_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "property_name cannot be empty".to_string(),
            ));
        }
        let res = self.mutate(property_name)?;
        Ok(DijkstraWriteSummary {
            nodes_written: res.summary.nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: res.summary.execution_time_ms,
        })
    }

    /// Estimate memory requirements for Dijkstra execution
    ///
    /// Returns a memory range estimate based on:
    /// - Priority queue storage (heap for open set)
    /// - Distance arrays
    /// - Path tracking (if enabled)
    /// - Graph structure overhead
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Priority queue (open set) - worst case: all nodes in queue
        let priority_queue_memory = node_count * 32; // node_id + distance + heap overhead

        // Distance array (8 bytes per node)
        let distance_array_memory = node_count * 8;

        // Path tracking: predecessor array (8 bytes per node)
        let path_tracking_memory = if self.config.track_relationships {
            node_count * 8
        } else {
            0
        };

        // Graph structure overhead
        let avg_degree = 10.0;
        let relationship_count = (node_count as f64 * avg_degree) as usize;
        let graph_overhead = relationship_count * 16;

        let total_memory =
            priority_queue_memory + distance_array_memory + path_tracking_memory + graph_overhead;

        let overhead = total_memory / 5; // 20% overhead
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
            seed: Some(3),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_builder_defaults() {
        let builder = DijkstraBuilder::new(store());
        assert_eq!(builder.config.source_node, 0);
        assert!(builder.config.target_nodes.is_empty());
        assert_eq!(builder.weight_property, "weight");
        assert_eq!(builder.config.direction, "outgoing");
        assert!(!builder.config.track_relationships);
        assert_eq!(builder.config.concurrency, 4);
    }

    #[test]
    fn test_builder_fluent_chain() {
        let builder = DijkstraBuilder::new(store())
            .source(42)
            .target(99)
            .weight_property("cost")
            .direction("incoming")
            .track_relationships(true)
            .concurrency(8);

        assert_eq!(builder.config.source_node, 42);
        assert_eq!(builder.config.target_nodes, vec![99]);
        assert_eq!(builder.weight_property, "cost");
        assert_eq!(builder.config.direction, "incoming");
        assert!(builder.config.track_relationships);
        assert_eq!(builder.config.concurrency, 8);
    }

    #[test]
    fn test_validate_missing_source() {
        let builder = DijkstraBuilder::new(store()).source_node(-1);
        assert!(builder.config.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_concurrency() {
        let builder = DijkstraBuilder::new(store()).source(0).concurrency(0);
        assert!(builder.config.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_direction() {
        let builder = DijkstraBuilder::new(store()).source(0).direction("invalid");
        assert!(builder.config.validate().is_err());
    }

    #[test]
    fn test_validate_empty_weight_property() {
        let builder = DijkstraBuilder::new(store()).source(0).weight_property("");
        assert!(builder.stream().is_err());
    }

    #[test]
    fn test_validate_valid_config() {
        let builder = DijkstraBuilder::new(store())
            .source(0)
            .target(5)
            .weight_property("cost")
            .direction("outgoing");
        assert!(builder.config.validate().is_ok());
    }

    #[test]
    fn test_stream_requires_validation() {
        let builder = DijkstraBuilder::new(store()).source_node(-1);
        assert!(builder.stream().is_err());
    }

    #[test]
    fn test_stats_requires_validation() {
        let builder = DijkstraBuilder::new(store()).direction("invalid");
        assert!(builder.stats().is_err());
    }

    #[test]
    fn test_mutate_requires_validation() {
        let builder = DijkstraBuilder::new(store()).source(0); // Valid config but...
        assert!(builder.mutate("").is_err()); // Empty property name
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let builder = DijkstraBuilder::new(store()).source(0);
        assert!(builder.mutate("distance").is_ok());
    }

    #[test]
    fn test_write_validates_property_name() {
        let builder = DijkstraBuilder::new(store()).source(0);
        assert!(builder.write("paths").is_ok());
    }
}
