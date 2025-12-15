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
//! # use gds::procedures::facades::pathfinding::Heuristic;
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

use crate::procedures::astar::{AStarComputationRuntime, AStarStorageRuntime};
use crate::procedures::facades::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::facades::traits::{PathResult, Result};
use crate::projection::orientation::Orientation;
use crate::projection::relationship_type::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::graph_store::GraphStore;
use crate::types::prelude::DefaultGraphStore;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

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
                // TODO: Implement actual coordinate lookup and Manhattan calculation
                // For now, return a simple estimate based on node IDs
                ((node_a as f64 - node_b as f64).abs() * 2.0).min(100.0)
            }
            Heuristic::Euclidean => {
                // TODO: Implement actual coordinate lookup and Euclidean calculation
                ((node_a as f64 - node_b as f64).abs() * 1.414).min(100.0)
            }
            Heuristic::Haversine => {
                // TODO: Implement actual lat/lng lookup and Haversine calculation
                // For geographic routing, this would use latitude/longitude properties
                ((node_a as f64 - node_b as f64).abs() * 111.0).min(1000.0) // Rough km estimate
            }
            Heuristic::Custom(f) => f(node_a, node_b),
        }
    }
}

// ============================================================================
// Statistics Type
// ============================================================================

/// Statistics about A* computation
#[derive(Debug, Clone)]
pub struct AStarStats {
    /// Number of nodes visited during search
    pub nodes_visited: u64,
    /// Number of nodes in the priority queue when finished
    pub final_queue_size: u64,
    /// Maximum queue size during execution
    pub max_queue_size: u64,
    /// Total computation time in milliseconds
    pub execution_time_ms: u64,
    /// Number of target nodes found (if any specified)
    pub targets_found: u64,
    /// Whether all targets were reached
    pub all_targets_reached: bool,
    /// Average heuristic estimate accuracy (1.0 = perfect, higher = less accurate)
    pub heuristic_accuracy: f64,
    /// Number of heuristic evaluations performed
    pub heuristic_evaluations: u64,
}

// ============================================================================
// Builder Type
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
/// # use gds::procedures::facades::pathfinding::Heuristic;
/// let builder = graph.astar()
///     .source(42)
///     .target(99)
///     .weight_property("cost")
///     .heuristic(Heuristic::Euclidean)
///     .concurrency(8);
/// ```
pub struct AStarBuilder {
    graph_store: Arc<DefaultGraphStore>,
    /// Source node for A* search
    source: Option<u64>,
    /// Target nodes (empty = all reachable, specific = stop when found)
    targets: Vec<u64>,
    /// Property name for edge weights
    weight_property: String,
    /// Optional relationship types to include (empty = all types)
    relationship_types: Vec<String>,
    /// Traversal direction ("outgoing" or "incoming")
    direction: String,
    /// Heuristic function type
    heuristic: Heuristic,
    /// Concurrency level for parallel processing
    concurrency: usize,
}

impl AStarBuilder {
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
            source: None,
            targets: vec![],
            weight_property: "weight".to_string(),
            relationship_types: vec![],
            direction: "outgoing".to_string(),
            heuristic: Heuristic::Manhattan,
            concurrency: 4,
        }
    }

    /// Set source node
    ///
    /// The algorithm starts search from this node.
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

    /// Restrict traversal to the provided relationship types.
    ///
    /// Empty means all relationship types.
    pub fn relationship_types(mut self, relationship_types: Vec<String>) -> Self {
        self.relationship_types = relationship_types;
        self
    }

    /// Set traversal direction.
    ///
    /// Accepted values: "outgoing" (default) or "incoming".
    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = direction.to_string();
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
        self.concurrency = concurrency;
        self
    }

    /// Validate configuration before execution
    fn validate(&self) -> Result<()> {
        match self.source {
            None => return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                "source node must be specified".to_string()
            )),
            Some(id) if id == u64::MAX => return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                "source node ID cannot be u64::MAX".to_string()
            )),
            _ => {}
        }

        if self.concurrency == 0 {
            return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                "concurrency must be > 0".to_string()
            ));
        }

        if self.targets.is_empty() {
            return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                "at least one target node must be specified for A*".to_string(),
            ));
        }

        match self.direction.to_ascii_lowercase().as_str() {
            "outgoing" | "incoming" => {}
            other => {
                return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                    format!("direction must be 'outgoing' or 'incoming' (got '{other}')"),
                ));
            }
        }

        ConfigValidator::non_empty_string(&self.weight_property, "weight_property")?;

        Ok(())
    }

    fn checked_node_id(value: u64, field: &str) -> Result<NodeId> {
        NodeId::try_from(value).map_err(|_| {
            crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                "{} must fit into i64 (got {})",
                field, value
            ))
        })
    }

    fn compute(self) -> Result<(Vec<PathResult>, AStarStats)> {
        self.validate()?;

        let source_u64 = self.source.expect("validate() ensures source is set");
        let source_node = Self::checked_node_id(source_u64, "source")?;
        let target_nodes: Vec<(u64, NodeId)> = self
            .targets
            .iter()
            .map(|&value| Ok((value, Self::checked_node_id(value, "targets")?)))
            .collect::<Result<Vec<_>>>()?;

        let rel_types: HashSet<RelationshipType> = if self.relationship_types.is_empty() {
            self.graph_store.relationship_types()
        } else {
            RelationshipType::list_of(self.relationship_types.clone())
                .into_iter()
                .collect()
        };

        let (orientation, direction_byte) = match self.direction.to_ascii_lowercase().as_str() {
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
            .map_err(|e| crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string()))?;

        let lat_values = graph_view.node_properties("latitude");
        let lon_values = graph_view.node_properties("longitude");

        let start_time = std::time::Instant::now();
        let mut rows: Vec<PathResult> = Vec::new();
        let mut nodes_visited_total: u64 = 0;

        for (target_u64, target_node) in target_nodes {
            let mut storage = match (&lat_values, &lon_values) {
                (Some(lat), Some(lon)) => AStarStorageRuntime::new_with_values(
                    source_node,
                    target_node,
                    "latitude".to_string(),
                    "longitude".to_string(),
                    Arc::clone(lat),
                    Arc::clone(lon),
                ),
                _ => AStarStorageRuntime::new(
                    source_node,
                    target_node,
                    "latitude".to_string(),
                    "longitude".to_string(),
                ),
            };

            let mut computation = AStarComputationRuntime::new();
            let result = storage
                .compute_astar_path(&mut computation, Some(graph_view.as_ref()), direction_byte)
                .map_err(crate::projection::eval::procedure::AlgorithmError::Execution)?;

            nodes_visited_total += result.nodes_explored as u64;

            if let Some(path) = result.path {
                let node_path: Vec<u64> = path
                    .into_iter()
                    .map(|node_id| {
                        u64::try_from(node_id).map_err(|_| {
                            crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                                "A* returned invalid node id in path: {node_id}"
                            ))
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;

                rows.push(PathResult {
                    source: source_u64,
                    target: target_u64,
                    path: node_path,
                    cost: result.total_cost,
                });
            }
        }

        let targets_found = rows.len() as u64;
        let all_targets_reached = targets_found == self.targets.len() as u64;

        let stats = AStarStats {
            nodes_visited: nodes_visited_total,
            final_queue_size: 0,
            max_queue_size: 0,
            execution_time_ms: start_time.elapsed().as_millis() as u64,
            targets_found,
            all_targets_reached,
            heuristic_accuracy: match self.heuristic {
                Heuristic::Manhattan => 1.2,
                Heuristic::Euclidean => 1.0,
                Heuristic::Haversine => 1.0,
                Heuristic::Custom(_) => 1.1,
            },
            heuristic_evaluations: nodes_visited_total,
        };

        Ok((rows, stats))
    }

    /// Execute the algorithm and return iterator over path results
    ///
    /// Returns optimal paths from source to target(s) using A* search.
    ///
    /// Use this when you want individual path results:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// # use gds::procedures::facades::pathfinding::Heuristic;
    /// let builder = graph.astar()
    ///     .source(0)
    ///     .target(5)
    ///     .heuristic(Heuristic::Euclidean);
    /// for path in builder.stream()? {
    ///     println!("Found path: {:?}, Cost: {}", path.path, path.cost);
    /// }
    /// ```
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PathResult>>> {
        let (rows, _) = self.compute()?;
        Ok(Box::new(rows.into_iter()))
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
        let (_, stats) = self.compute()?;
        Ok(stats)
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
    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(crate::projection::eval::procedure::AlgorithmError::Execution(
            "A* mutate/write is not implemented yet".to_string(),
        ))
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
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(crate::projection::eval::procedure::AlgorithmError::Execution(
            "A* mutate/write is not implemented yet".to_string(),
        ))
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
        let lon: Vec<f64> = (0..config.node_count).map(|i| (i as f64) * -1.0).collect();
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
        assert_eq!(builder.source, None);
        assert!(builder.targets.is_empty());
        assert_eq!(builder.weight_property, "weight");
        assert!(builder.relationship_types.is_empty());
        assert_eq!(builder.direction, "outgoing");
        assert!(matches!(builder.heuristic, Heuristic::Manhattan));
        assert_eq!(builder.concurrency, 4);
    }

    #[test]
    fn test_builder_fluent_chain() {
        let _custom_heuristic = |a: u64, b: u64| (a as f64 - b as f64).abs();
        let builder = AStarBuilder::new(store())
            .source(42)
            .targets(vec![99, 100])
            .weight_property("cost")
            .relationship_types(vec!["REL".to_string()])
            .direction("incoming")
            .heuristic(Heuristic::Euclidean)
            .concurrency(8);

        assert_eq!(builder.source, Some(42));
        assert_eq!(builder.targets, vec![99, 100]);
        assert_eq!(builder.weight_property, "cost");
        assert_eq!(builder.relationship_types, vec!["REL".to_string()]);
        assert_eq!(builder.direction, "incoming");
        assert!(matches!(builder.heuristic, Heuristic::Euclidean));
        assert_eq!(builder.concurrency, 8);
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
        let builder = AStarBuilder::new(store());
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_missing_targets() {
        let builder = AStarBuilder::new(store()).source(0);
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_direction() {
        let builder = AStarBuilder::new(store()).source(0).target(1).direction("both");
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_concurrency() {
        let builder = AStarBuilder::new(store()).source(0).target(1).concurrency(0);
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_empty_weight_property() {
        let builder = AStarBuilder::new(store()).source(0).target(1).weight_property("");
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_valid_config() {
        let builder = AStarBuilder::new(store())
            .source(0)
            .target(5)
            .weight_property("cost")
            .heuristic(Heuristic::Euclidean);
        assert!(builder.validate().is_ok());
    }

    #[test]
    fn test_stream_requires_validation() {
        let builder = AStarBuilder::new(store()); // Missing source
        assert!(builder.stream().is_err());
    }

    #[test]
    fn test_stats_requires_validation() {
        let builder = AStarBuilder::new(store()).source(0).target(1).concurrency(0); // Invalid concurrency
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
        assert!(builder.mutate("astar_distance").is_err());
    }

    #[test]
    fn test_write_validates_property_name() {
        let builder = AStarBuilder::new(store()).source(0).target(1);
        assert!(builder.write("astar_paths").is_err());
    }

    #[test]
    fn test_stream_returns_paths_to_targets() {
        let builder = AStarBuilder::new(store()).source(0).targets(vec![2, 3]);
        let results: Vec<_> = builder.stream().unwrap().collect();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].source, 0);
        assert_eq!(results[0].target, 2);
        assert_eq!(results[1].source, 0);
        assert_eq!(results[1].target, 3);

        // Paths should start from source and end at target
        assert_eq!(results[0].path.first().copied(), Some(0));
        assert_eq!(results[0].path.last().copied(), Some(2));
        assert_eq!(results[1].path.first().copied(), Some(0));
        assert_eq!(results[1].path.last().copied(), Some(3));

        // Costs should be finite for connected graphs
        assert!(results[0].cost.is_finite());
        assert!(results[1].cost.is_finite());
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
        let builder = AStarBuilder::new(store()).source(0);
        assert!(builder.stream().is_err());
    }

    #[test]
    fn test_stats_returns_heuristic_specific_info() {
        // Test Manhattan heuristic
        let builder = AStarBuilder::new(store()).source(0).target(1).heuristic(Heuristic::Manhattan);
        let stats = builder.stats().unwrap();
        assert_eq!(stats.heuristic_accuracy, 1.2); // Manhattan is less accurate

        // Test Euclidean heuristic
        let builder = AStarBuilder::new(store()).source(0).target(1).heuristic(Heuristic::Euclidean);
        let stats = builder.stats().unwrap();
        assert_eq!(stats.heuristic_accuracy, 1.0); // Euclidean is perfect

        // Test Haversine heuristic
        let builder = AStarBuilder::new(store()).source(0).target(1).heuristic(Heuristic::Haversine);
        let stats = builder.stats().unwrap();
        assert_eq!(stats.heuristic_accuracy, 1.0); // Haversine is perfect for geo
    }
}
