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

use crate::mem::MemoryRange;
use crate::algo::dijkstra::targets::create_targets;
use crate::algo::dijkstra::{DijkstraComputationRuntime, DijkstraStorageRuntime};
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::{PathResult, Result};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

// Import upgraded systems
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, TaskRegistryFactory, Tasks,
};
use crate::algo::common::prelude::{PathFindingResult, PathResultBuilder};
use crate::algo::common::result_builders::{ExecutionMetadata, ResultBuilder};

// ============================================================================
// Statistics Type
// ============================================================================

/// Statistics about Dijkstra computation
#[derive(Debug, Clone, serde::Serialize)]
pub struct DijkstraStats {
    /// Number of paths found
    pub paths_found: u64,
    /// Total computation time in milliseconds
    pub execution_time_ms: u64,
    /// Number of nodes expanded during search
    pub nodes_expanded: u64,
    /// Number of edges considered
    pub edges_considered: u64,
    /// Maximum queue size during execution
    pub max_queue_size: u64,
    /// Whether search reached target(s)
    pub target_reached: bool,
}

// ============================================================================
// Builder Type
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
pub struct DijkstraBuilder {
    graph_store: Arc<DefaultGraphStore>,
    /// Source node for path computation
    source: Option<u64>,
    /// Target nodes (empty = all targets, single = specific target)
    targets: Vec<u64>,
    /// Property name for edge weights
    weight_property: String,
    /// Direction of traversal ("incoming", "outgoing", "both")
    direction: String,
    /// Whether to track relationship IDs in results
    track_relationships: bool,
    /// Concurrency level for parallel processing
    concurrency: usize,
    /// Progress tracking components
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

impl DijkstraBuilder {
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
            source: None,
            targets: vec![],
            weight_property: "weight".to_string(),
            direction: "outgoing".to_string(),
            track_relationships: false,
            concurrency: 4,
            task_registry_factory: None,
            user_log_registry_factory: None,
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
        let _task_registry_factory = self
            .task_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));
        let _user_log_registry_factory = self
            .user_log_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));

        // Progress tracker is best-effort; the driver loop in storage owns begin/log/end.

        let source_u64 = self.source.expect("validate() ensures source is set");
        let source_node = Self::checked_node_id(source_u64, "source")?;
        let target_nodes: Vec<NodeId> = self
            .targets
            .iter()
            .map(|&value| Self::checked_node_id(value, "targets"))
            .collect::<Result<Vec<_>>>()?;

        let targets = create_targets(target_nodes.clone());

        let mut storage = DijkstraStorageRuntime::new(
            source_node,
            self.track_relationships,
            self.concurrency,
            false,
        );

        let mut computation = DijkstraComputationRuntime::new(
            source_node,
            self.track_relationships,
            self.concurrency,
            false,
        );

        // Use selectors so the algorithm consumes the requested weight property.
        // (If no selector is provided, DefaultGraph may auto-select only when exactly one exists.)
        let rel_types: HashSet<RelationshipType> = self.graph_store.relationship_types();
        let selectors: HashMap<RelationshipType, String> = rel_types
            .iter()
            .map(|t| (t.clone(), self.weight_property.clone()))
            .collect();
        let (orientation, direction_byte) = match self.direction.as_str() {
            "incoming" => (Orientation::Reverse, 1u8),
            "both" => (Orientation::Undirected, 0u8),
            _ => (Orientation::Natural, 0u8),
        };

        let graph_view = self
            .graph_store
            .get_graph_with_types_selectors_and_orientation(&rel_types, &selectors, orientation)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("dijkstra".to_string(), graph_view.relationship_count()),
            self.concurrency,
        );

        let result = storage.compute_dijkstra(
            &mut computation,
            targets,
            Some(graph_view.as_ref()),
            direction_byte,
            &mut progress_tracker,
        )?;

        let paths: Vec<crate::algo::common::result_builders::PathResult> = result
            .path_finding_result
            .paths()
            .filter(|p| p.source_node >= 0 && p.target_node >= 0)
            .map(|p| crate::algo::common::result_builders::PathResult {
                source: p.source_node as u64,
                target: p.target_node as u64,
                path: p
                    .node_ids
                    .iter()
                    .copied()
                    .filter(|node_id| *node_id >= 0)
                    .map(|node_id| node_id as u64)
                    .collect(),
                cost: p.total_cost(),
            })
            .collect();

        // Create execution metadata
        let metadata = ExecutionMetadata {
            execution_time: std::time::Duration::from_millis(result.computation_time_ms),
            iterations: None,
            converged: None,
            additional: std::collections::HashMap::new(),
        };

        // Build result using upgraded result builder
        let path_result = PathResultBuilder::new()
            .with_paths(paths)
            .with_metadata(metadata)
            .build()
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Execution(e.to_string())
            })?;

        Ok(path_result)
    }

    /// Set source node
    ///
    /// The algorithm starts path computation from this node.
    /// Must be a valid node ID in the graph.
    pub fn source(mut self, source: u64) -> Self {
        self.source = Some(source);
        self
    }

    /// Set single target node
    ///
    /// If specified, algorithm stops when target is reached.
    /// If not specified, computes paths to all reachable nodes.
    pub fn target(mut self, target: u64) -> Self {
        self.targets = vec![target];
        self
    }

    /// Set multiple target nodes
    ///
    /// Algorithm computes shortest paths to all specified targets.
    pub fn targets(mut self, targets: Vec<u64>) -> Self {
        self.targets = targets;
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

    /// Set traversal direction
    ///
    /// Options: "incoming", "outgoing", "both"
    /// Default: "outgoing"
    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = direction.to_string();
        self
    }

    /// Enable relationship tracking in results
    ///
    /// When true, results include relationship IDs along paths.
    /// Slightly more memory usage but enables path reconstruction.
    pub fn track_relationships(mut self, track: bool) -> Self {
        self.track_relationships = track;
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// Higher values = faster on large graphs, but more memory.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
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

        if !["incoming", "outgoing", "both"].contains(&self.direction.as_str()) {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                    "direction must be 'incoming', 'outgoing', or 'both', got '{}'",
                    self.direction
                )),
            );
        }

        ConfigValidator::non_empty_string(&self.weight_property, "weight_property")?;

        Ok(())
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
        let has_targets = !self.targets.is_empty();
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
    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "Dijkstra mutate/write is not implemented yet".to_string(),
            ),
        )
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
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "Dijkstra mutate/write is not implemented yet".to_string(),
            ),
        )
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
        let path_tracking_memory = if self.track_relationships {
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
        assert_eq!(builder.source, None);
        assert!(builder.targets.is_empty());
        assert_eq!(builder.weight_property, "weight");
        assert_eq!(builder.direction, "outgoing");
        assert!(!builder.track_relationships);
        assert_eq!(builder.concurrency, 4);
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

        assert_eq!(builder.source, Some(42));
        assert_eq!(builder.targets, vec![99]);
        assert_eq!(builder.weight_property, "cost");
        assert_eq!(builder.direction, "incoming");
        assert!(builder.track_relationships);
        assert_eq!(builder.concurrency, 8);
    }

    #[test]
    fn test_validate_missing_source() {
        let builder = DijkstraBuilder::new(store());
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_concurrency() {
        let builder = DijkstraBuilder::new(store()).source(0).concurrency(0);
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_direction() {
        let builder = DijkstraBuilder::new(store()).source(0).direction("invalid");
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_empty_weight_property() {
        let builder = DijkstraBuilder::new(store()).source(0).weight_property("");
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_valid_config() {
        let builder = DijkstraBuilder::new(store())
            .source(0)
            .target(5)
            .weight_property("cost")
            .direction("outgoing");
        assert!(builder.validate().is_ok());
    }

    #[test]
    fn test_stream_requires_validation() {
        let builder = DijkstraBuilder::new(store()); // Missing source
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
        assert!(builder.mutate("distance").is_err());
    }

    #[test]
    fn test_write_validates_property_name() {
        let builder = DijkstraBuilder::new(store()).source(0);
        assert!(builder.write("paths").is_err());
    }
}
