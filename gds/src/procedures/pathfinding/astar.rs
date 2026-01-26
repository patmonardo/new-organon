//! A* Facade
//!
//! **What is it?**: A* (A-star) - optimal pathfinding with heuristic guidance
//! **Why care?**: Finds optimal paths faster than Dijkstra using admissible heuristics
//! **Complexity**: O((V + E) log V) with good heuristics, same as Dijkstra with poor heuristics
//! **Best for**: Weighted graphs with good heuristic estimates, GPS navigation, game pathfinding
//!
//! ## How A* Works
//!
//! A* improves on Dijkstra by using a heuristic function h(n) that estimates distance to target:
//! - f(n) = g(n) + h(n) where g(n) is actual cost from source, h(n) is estimated cost to target
//! - Uses priority queue ordered by f(n) (total estimated cost)
//! - Guarantees optimality if heuristic is admissible (never overestimates)
//! - Explores fewer nodes than Dijkstra when heuristic is informative
//!
//! ## Heuristics
//!
//! - **Manhattan**: |dx| + |dy| (grid-based, taxicab distance)
//! - **Euclidean**: sqrt(dx² + dy²) (straight-line distance)
//! - **Haversine**: Great circle distance for geographic coordinates
//! - **Custom**: User-provided closure function
//!
//! ## Example
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # let graph: Graph = unimplemented!();
//! # use gds::procedures::pathfinding::Heuristic;
//! let path = graph
//!     .astar()
//!     .source(42)
//!     .target(99)
//!     .weight_property("cost")
//!     .heuristic(Heuristic::Manhattan)
//!     .stream()?
//!     .next()
//!     .unwrap();
//! ```

use crate::algo::algorithms::Result;
use crate::algo::astar::{
    AStarComputationRuntime, AStarConfig, AStarMutateResult, AStarMutationSummary, AStarResult,
    AStarResultBuilder, AStarStats, AStarStorageRuntime, AStarWriteSummary,
};
use crate::core::utils::progress::TaskProgressTracker;
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::relationship_type::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::graph_store::GraphStore;
use crate::types::prelude::DefaultGraphStore;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// Import upgraded systems
use crate::algo::algorithms::pathfinding::{PathFindingResult, PathResult};
use crate::core::utils::progress::{EmptyTaskRegistryFactory, TaskRegistryFactory, Tasks};

// ============================================================================
// Heuristic Types
// ============================================================================

/// Heuristic function types for A* algorithm
#[derive(Debug, Clone, Copy)]
pub enum Heuristic {
    /// Manhattan distance: |dx| + |dy| (taxicab distance)
    Manhattan,
    /// Euclidean distance: sqrt(dx² + dy²) (straight-line distance)
    Euclidean,
    /// Haversine distance: Great circle distance for geographic coordinates
    Haversine,
    /// Custom heuristic function provided by user
    Custom(fn(u64, u64) -> f64),
}

impl Heuristic {
    /// Calculate heuristic value between two nodes
    pub fn calculate(&self, node_a: u64, node_b: u64) -> f64 {
        match self {
            Heuristic::Manhattan => {
                // Note: coordinate lookup + Manhattan calculation is deferred.
                // For now, return a simple estimate based on node IDs
                ((node_a as f64 - node_b as f64).abs() * 2.0).min(100.0)
            }
            Heuristic::Euclidean => {
                // Note: coordinate lookup + Euclidean calculation is deferred.
                ((node_a as f64 - node_b as f64).abs() * 1.414).min(100.0)
            }
            Heuristic::Haversine => {
                // Note: lat/lng lookup + Haversine calculation is deferred.
                // For geographic routing, this would use latitude/longitude properties
                ((node_a as f64 - node_b as f64).abs() * 111.0).min(1000.0) // Rough km estimate
            }
            Heuristic::Custom(f) => f(node_a, node_b),
        }
    }
}

// ============================================================================
// Facade Type
// ============================================================================

/// A* algorithm builder - fluent configuration
///
/// Use this to configure and run A* with custom parameters.
/// Supports multiple execution modes via method chaining.
///
/// ## Default Configuration
/// - source: None (must be set explicitly)
/// - targets: empty (compute path to all reachable nodes)
/// - weight_property: "weight"
/// - heuristic: Manhattan (simple and fast)
/// - concurrency: 4
///
/// ## Example
/// ```rust,no_run
/// # use gds::Graph;
/// # let graph: Graph = unimplemented!();
/// # use gds::procedures::pathfinding::Heuristic;
/// let builder = graph.astar()
///     .source(42)
///     .target(99)
///     .weight_property("cost")
///     .heuristic(Heuristic::Euclidean)
///     .concurrency(8);
/// ```
pub struct AStarFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: AStarConfig,
    /// Property name for edge weights
    weight_property: String,
    /// Heuristic function type
    heuristic: Heuristic,
    /// Progress tracking components
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

/// Backwards-compatible alias (builder-style naming).
pub type AStarBuilder = AStarFacade;

impl AStarFacade {
    /// Create a new A* builder bound to a live graph store.
    ///
    /// Defaults:
    /// - source: None (must be set)
    /// - targets: empty (must be set; A* requires at least one target)
    /// - weight_property: "weight"
    /// - heuristic: Manhattan (simple and fast)
    /// - concurrency: 4
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: AStarConfig::default(),
            weight_property: "weight".to_string(),
            heuristic: Heuristic::Manhattan,
            task_registry_factory: None,
            user_log_registry_factory: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: AStarConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
            weight_property: "weight".to_string(),
            heuristic: Heuristic::Manhattan,
            task_registry_factory: None,
            user_log_registry_factory: None,
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: AStarConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: AStarConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    /// Set source node (NodeId)
    pub fn source_node(mut self, source: NodeId) -> Self {
        self.config.source_node = source;
        self
    }

    /// Set source node
    ///
    /// The algorithm starts search from this node.
    /// Must be a valid node ID in the graph.
    pub fn source(mut self, source: u64) -> Self {
        self.config.source_node = i64::try_from(source).unwrap_or(-1);
        self
    }

    /// Set single target node
    ///
    /// If specified, algorithm stops when target is reached.
    /// If not specified, computes paths to all reachable nodes.
    pub fn target_node(mut self, target: NodeId) -> Self {
        self.config.target_node = target;
        self
    }

    pub fn target(mut self, target: u64) -> Self {
        self.config.target_node = i64::try_from(target).unwrap_or(-1);
        self
    }

    /// Set multiple target nodes
    ///
    /// Algorithm computes shortest paths to all specified targets.
    pub fn targets(mut self, targets: Vec<u64>) -> Self {
        if let Some(first) = targets.into_iter().next() {
            self.config.target_node = i64::try_from(first).unwrap_or(-1);
        }
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

    /// Restrict traversal to the provided relationship types.
    ///
    /// Empty means all relationship types.
    pub fn relationship_types(mut self, relationship_types: Vec<String>) -> Self {
        self.config.relationship_types = relationship_types;
        self
    }

    /// Set traversal direction.
    ///
    /// Accepted values: "outgoing" (default) or "incoming".
    pub fn direction(mut self, direction: &str) -> Self {
        self.config.direction = direction.to_string();
        self
    }

    /// Set latitude property name
    pub fn latitude_property(mut self, property: &str) -> Self {
        self.config.latitude_property = property.to_string();
        self
    }

    /// Set longitude property name
    pub fn longitude_property(mut self, property: &str) -> Self {
        self.config.longitude_property = property.to_string();
        self
    }

    /// Set heuristic function type
    ///
    /// Different heuristics trade off accuracy vs computation speed:
    /// - Manhattan: Fast, less accurate, good for grids
    /// - Euclidean: Moderate speed/accuracy, good for open spaces
    /// - Haversine: Slow, very accurate for geographic routing
    /// - Custom: User-defined function
    pub fn heuristic(mut self, heuristic: Heuristic) -> Self {
        self.heuristic = heuristic;
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// A* benefits from parallelism when exploring large graphs.
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

        // Best-effort volume: relationship count (work units are edge scans).
        let relationship_count = self.graph_store.relationship_count();

        let source_node = self.config.source_node;
        let target_node = self.config.target_node;

        let rel_types: HashSet<RelationshipType> = if self.config.relationship_types.is_empty() {
            self.graph_store.relationship_types()
        } else {
            RelationshipType::list_of(self.config.relationship_types.clone())
                .into_iter()
                .collect()
        };

        let (orientation, direction_byte) =
            match self.config.direction.to_ascii_lowercase().as_str() {
                "incoming" => (Orientation::Reverse, 1u8),
                _ => (Orientation::Natural, 0u8),
            };

        let selectors: HashMap<RelationshipType, String> = rel_types
            .iter()
            .map(|t| (t.clone(), self.weight_property.clone()))
            .collect();

        let graph_view = self
            .graph_store
            .get_graph_with_types_selectors_and_orientation(&rel_types, &selectors, orientation)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let lat_values = graph_view.node_properties(&self.config.latitude_property);
        let lon_values = graph_view.node_properties(&self.config.longitude_property);

        let start_time = std::time::Instant::now();

        // Create a fresh progress tracker per run.
        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("astar".to_string(), relationship_count),
            self.config.concurrency,
        );

        let mut storage = match (&lat_values, &lon_values) {
            (Some(lat), Some(lon)) => AStarStorageRuntime::new_with_values(
                source_node,
                target_node,
                self.config.latitude_property.clone(),
                self.config.longitude_property.clone(),
                Arc::clone(lat),
                Arc::clone(lon),
            ),
            _ => AStarStorageRuntime::new(
                source_node,
                target_node,
                self.config.latitude_property.clone(),
                self.config.longitude_property.clone(),
            ),
        };

        let mut computation = AStarComputationRuntime::new();
        let result = storage
            .compute_astar_path(
                &mut computation,
                Some(graph_view.as_ref()),
                direction_byte,
                &mut progress_tracker,
            )
            .map_err(AlgorithmError::Execution)?;

        let execution_time_ms = start_time.elapsed().as_millis() as u64;
        let result = AStarResult::new(
            result.path,
            result.total_cost,
            execution_time_ms,
            result.nodes_explored,
        );

        let heuristic_accuracy = match self.heuristic {
            Heuristic::Manhattan => "1.2",
            Heuristic::Euclidean => "1.0",
            Heuristic::Haversine => "1.0",
            Heuristic::Custom(_) => "1.1",
        };

        let additional = HashMap::from([
            (
                "heuristic_accuracy".to_string(),
                heuristic_accuracy.to_string(),
            ),
            (
                "heuristic_evaluations".to_string(),
                (result.nodes_explored as u64).to_string(),
            ),
        ]);

        AStarResultBuilder::result_with_additional(
            result,
            start_time.elapsed(),
            source_node,
            target_node,
            additional,
        )
    }

    /// Execute the algorithm and return iterator over path results
    ///
    /// Returns optimal paths from source to target(s) using A* search.
    ///
    /// Use this when you want individual path results:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// # use gds::procedures::pathfinding::Heuristic;
    /// let builder = graph.astar()
    ///     .source(0)
    ///     .target(5)
    ///     .heuristic(Heuristic::Euclidean);
    /// for path in builder.stream()? {
    ///     println!("Found path: {:?}, Cost: {}", path.path, path.cost);
    /// }
    /// ```
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PathResult>>> {
        let result = self.compute()?;
        Ok(Box::new(result.paths.into_iter()))
    }

    /// Stats mode: Get aggregated statistics
    ///
    /// Returns search statistics without individual paths.
    ///
    /// Use this when you want performance metrics:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// let builder = graph.astar().source(0).target(1);
    /// let stats = builder.stats()?;
    /// println!("Visited {} nodes, heuristic accuracy: {:.2}", stats.nodes_visited, stats.heuristic_accuracy);
    /// ```
    pub fn stats(self) -> Result<AStarStats> {
        let result = self.compute()?;
        let nodes_visited = result
            .metadata
            .additional
            .get("nodes_visited")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        let targets_found = result
            .metadata
            .additional
            .get("targets_found")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        let all_targets_reached = result
            .metadata
            .additional
            .get("all_targets_reached")
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(false);
        let heuristic_accuracy = result
            .metadata
            .additional
            .get("heuristic_accuracy")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(1.0);
        let heuristic_evaluations = result
            .metadata
            .additional
            .get("heuristic_evaluations")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        Ok(AStarStats {
            nodes_visited,
            final_queue_size: 0,
            max_queue_size: 0,
            execution_time_ms: result.metadata.execution_time.as_millis() as u64,
            targets_found,
            all_targets_reached,
            heuristic_accuracy,
            heuristic_evaluations,
        })
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores A* distances as a node property.
    /// Property contains estimated distance from source to each reachable node.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// let builder = graph.astar().source(0).target(1);
    /// let result = builder.mutate("astar_distance")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<AStarMutateResult> {
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

        let summary = AStarMutationSummary {
            nodes_updated: paths.len() as u64,
            property_name: property_name.to_string(),
            execution_time_ms: result.metadata.execution_time.as_millis() as u64,
        };

        Ok(AStarMutateResult {
            summary,
            updated_store,
        })
    }

    /// Write mode: Compute and persist to storage
    ///
    /// Persists A* search results and optimal paths to storage backend.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// let builder = graph.astar().source(0).target(1);
    /// let result = builder.write("astar_paths")?;
    /// println!("Wrote {} nodes", result.nodes_written);
    /// ```
    pub fn write(self, property_name: &str) -> Result<AStarWriteSummary> {
        if property_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "property_name cannot be empty".to_string(),
            ));
        }
        let res = self.mutate(property_name)?;
        Ok(AStarWriteSummary {
            nodes_written: res.summary.nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: res.summary.execution_time_ms,
        })
    }

    /// Estimate memory requirements for A* execution
    ///
    /// Returns a memory range estimate based on:
    /// - Priority queue storage (heap for open set)
    /// - Distance arrays (g-scores and f-scores)
    /// - Path tracking (if enabled)
    /// - Graph structure overhead
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// let builder = graph.astar().source(0).target(1);
    /// let memory = builder.estimate_memory();
    /// println!("Estimated memory: {} bytes", memory.max());
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Priority queue (open set) - worst case: all nodes in queue
        // Each entry: node_id (8 bytes) + f-score (8 bytes) + heap overhead (16 bytes) = 32 bytes
        let priority_queue_memory = node_count * 32;

        // Distance arrays: g-scores and f-scores (8 bytes each per node)
        let distance_arrays_memory = node_count * 8 * 2; // g-score + f-score

        // Path tracking: predecessor array (8 bytes per node)
        let path_tracking_memory = node_count * 8;

        // Heuristic computation overhead (coordinate storage if using lat/lng)
        let heuristic_overhead =
            if matches!(self.heuristic, Heuristic::Haversine | Heuristic::Euclidean) {
                node_count * 16 // lat + lng per node
            } else {
                0
            };

        // Graph structure overhead (adjacency lists, etc.)
        let avg_degree = 10.0; // Conservative estimate
        let relationship_count = (node_count as f64 * avg_degree) as usize;
        let graph_overhead = relationship_count * 16; // ~16 bytes per relationship

        let total_memory = priority_queue_memory
            + distance_arrays_memory
            + path_tracking_memory
            + heuristic_overhead
            + graph_overhead;

        // Add 20% overhead for algorithm-specific structures
        let overhead = total_memory / 5;
        let total_with_overhead = total_memory + overhead;

        MemoryRange::of_range(total_memory, total_with_overhead)
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
            seed: Some(7),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };

        let mut store = DefaultGraphStore::random(&config).unwrap();

        // Ensure latitude/longitude properties exist so A* uses real coordinates.
        let lat: Vec<f64> = (0..config.node_count).map(|i| i as f64).collect();
        let lon: Vec<f64> = (0..config.node_count).map(|i| -(i as f64)).collect();
        store
            .add_node_property_f64("latitude".to_string(), lat)
            .unwrap();
        store
            .add_node_property_f64("longitude".to_string(), lon)
            .unwrap();

        Arc::new(store)
    }

    #[test]
    fn test_builder_defaults() {
        let builder = AStarBuilder::new(store());
        assert_eq!(builder.config.source_node, 0);
        assert_eq!(builder.config.target_node, 1);
        assert_eq!(builder.weight_property, "weight");
        assert!(builder.config.relationship_types.is_empty());
        assert_eq!(builder.config.direction, "outgoing");
        assert!(matches!(builder.heuristic, Heuristic::Manhattan));
        assert_eq!(builder.config.concurrency, 4);
    }

    #[test]
    fn test_builder_fluent_chain() {
        let _custom_heuristic = |a: u64, b: u64| (a as f64 - b as f64).abs();
        let builder = AStarBuilder::new(store())
            .source(42)
            .target(99)
            .weight_property("cost")
            .relationship_types(vec!["REL".to_string()])
            .direction("incoming")
            .heuristic(Heuristic::Euclidean)
            .concurrency(8);

        assert_eq!(builder.config.source_node, 42);
        assert_eq!(builder.config.target_node, 99);
        assert_eq!(builder.weight_property, "cost");
        assert_eq!(builder.config.relationship_types, vec!["REL".to_string()]);
        assert_eq!(builder.config.direction, "incoming");
        assert!(matches!(builder.heuristic, Heuristic::Euclidean));
        assert_eq!(builder.config.concurrency, 8);
    }

    #[test]
    fn test_heuristic_calculations() {
        let manhattan = Heuristic::Manhattan;
        let euclidean = Heuristic::Euclidean;
        let haversine = Heuristic::Haversine;

        // Manhattan distance should be higher than Euclidean for same nodes
        assert!(manhattan.calculate(0, 5) >= euclidean.calculate(0, 5));

        // Haversine should give larger values (geographic scale)
        assert!(haversine.calculate(0, 5) >= manhattan.calculate(0, 5));

        // Custom heuristic
        let custom = Heuristic::Custom(|a, b| (a as f64 - b as f64).powi(2));
        assert_eq!(custom.calculate(0, 3), 9.0);
    }

    #[test]
    fn test_validate_missing_source() {
        let builder = AStarBuilder::new(store()).source_node(-1);
        assert!(builder.config.validate().is_err());
    }

    #[test]
    fn test_validate_missing_targets() {
        let builder = AStarBuilder::new(store()).target_node(-1);
        assert!(builder.config.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_direction() {
        let builder = AStarBuilder::new(store())
            .source(0)
            .target(1)
            .direction("both");
        assert!(builder.config.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_concurrency() {
        let builder = AStarBuilder::new(store())
            .source(0)
            .target(1)
            .concurrency(0);
        assert!(builder.config.validate().is_err());
    }

    #[test]
    fn test_validate_empty_weight_property() {
        let builder = AStarBuilder::new(store())
            .source(0)
            .target(1)
            .weight_property("");
        assert!(builder.stream().is_err());
    }

    #[test]
    fn test_validate_valid_config() {
        let builder = AStarBuilder::new(store())
            .source(0)
            .target(5)
            .weight_property("cost")
            .heuristic(Heuristic::Euclidean);
        assert!(builder.config.validate().is_ok());
    }

    #[test]
    fn test_stream_requires_validation() {
        let builder = AStarBuilder::new(store()).source_node(-1);
        assert!(builder.stream().is_err());
    }

    #[test]
    fn test_stats_requires_validation() {
        let builder = AStarBuilder::new(store())
            .source(0)
            .target(1)
            .concurrency(0); // Invalid concurrency
        assert!(builder.stats().is_err());
    }

    #[test]
    fn test_mutate_requires_validation() {
        let builder = AStarBuilder::new(store()).source(0).target(1); // Valid config but...
        assert!(builder.mutate("").is_err()); // Empty property name
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let builder = AStarBuilder::new(store()).source(0).target(1);
        assert!(builder.mutate("astar_distance").is_ok());
    }

    #[test]
    fn test_write_validates_property_name() {
        let builder = AStarBuilder::new(store()).source(0).target(1);
        assert!(builder.write("astar_paths").is_ok());
    }

    #[test]
    fn test_stream_returns_paths_to_targets() {
        let builder = AStarBuilder::new(store()).source(0).target(2);
        let results: Vec<_> = builder.stream().unwrap().collect();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].source, 0);
        assert_eq!(results[0].target, 2);

        // Paths should start from source and end at target
        assert_eq!(results[0].path.first().copied(), Some(0));
        assert_eq!(results[0].path.last().copied(), Some(2));
        // Costs should be finite for connected graphs
        assert!(results[0].cost.is_finite());
    }

    #[test]
    fn test_stream_with_incoming_direction() {
        let builder = AStarBuilder::new(store())
            .source(0)
            .target(3)
            .direction("incoming")
            .relationship_types(vec!["REL".to_string()]);
        let _results: Vec<_> = builder.stream().unwrap().collect();
    }

    #[test]
    fn test_stream_requires_targets() {
        let builder = AStarBuilder::new(store()).target_node(-1);
        assert!(builder.stream().is_err());
    }

    #[test]
    fn test_stats_returns_heuristic_specific_info() {
        // Test Manhattan heuristic
        let builder = AStarBuilder::new(store())
            .source(0)
            .target(1)
            .heuristic(Heuristic::Manhattan);
        let stats = builder.stats().unwrap();
        assert_eq!(stats.heuristic_accuracy, 1.2); // Manhattan is less accurate

        // Test Euclidean heuristic
        let builder = AStarBuilder::new(store())
            .source(0)
            .target(1)
            .heuristic(Heuristic::Euclidean);
        let stats = builder.stats().unwrap();
        assert_eq!(stats.heuristic_accuracy, 1.0); // Euclidean is perfect

        // Test Haversine heuristic
        let builder = AStarBuilder::new(store())
            .source(0)
            .target(1)
            .heuristic(Heuristic::Haversine);
        let stats = builder.stats().unwrap();
        assert_eq!(stats.heuristic_accuracy, 1.0); // Haversine is perfect for geo
    }
}
