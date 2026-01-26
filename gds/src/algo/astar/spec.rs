//! A* Algorithm Specification
//!
//! **Translation Source**: `org.neo4j.gds.paths.astar.AStar`
//!
//! This module defines the A* algorithm specification using focused macros.

use crate::algo::algorithms::pathfinding::{
    PathFindingResult, PathFindingResultBuilder, PathResult,
};
use crate::algo::algorithms::{ExecutionMetadata, ResultBuilder};
use crate::config::validation::ConfigError;
use crate::core::utils::progress::TaskProgressTracker;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::relationship_type::RelationshipType;
use crate::types::graph::NodeId;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::Duration;

/// A* algorithm configuration
///
/// Translation of: `org.neo4j.gds.paths.astar.config.ShortestPathAStarBaseConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AStarConfig {
    /// Source node ID
    pub source_node: NodeId,
    /// Target node ID
    pub target_node: NodeId,
    /// Latitude property name
    pub latitude_property: String,
    /// Longitude property name
    pub longitude_property: String,
    /// Concurrency level
    pub concurrency: usize,
    /// Optional relationship types to include (empty means all types)
    #[serde(default)]
    pub relationship_types: Vec<String>,
    /// Direction for traversal ("outgoing" or "incoming")
    #[serde(default = "AStarDirection::default_as_str")]
    pub direction: String,
}

impl Default for AStarConfig {
    fn default() -> Self {
        Self {
            source_node: 0,
            target_node: 1,
            latitude_property: "latitude".to_string(),
            longitude_property: "longitude".to_string(),
            concurrency: 4,
            relationship_types: vec![],
            direction: AStarDirection::Outgoing.as_str().to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AStarDirection {
    Outgoing,
    Incoming,
}

impl AStarDirection {
    fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "incoming" => AStarDirection::Incoming,
            _ => AStarDirection::Outgoing,
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            AStarDirection::Outgoing => "outgoing",
            AStarDirection::Incoming => "incoming",
        }
    }
    fn default_as_str() -> String {
        Self::Outgoing.as_str().to_string()
    }
}

impl AStarConfig {
    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.source_node < 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "source_node".to_string(),
                reason: "Must be a non-negative i64".to_string(),
            });
        }

        if self.target_node < 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "target_node".to_string(),
                reason: "Must be a non-negative i64".to_string(),
            });
        }

        if self.concurrency == 0 {
            return Err(ConfigError::MustBePositive {
                name: "concurrency".to_string(),
                value: 0.0,
            });
        }

        if self.latitude_property.is_empty() {
            return Err(ConfigError::RequiredParameter {
                name: "latitude_property".to_string(),
            });
        }

        if self.longitude_property.is_empty() {
            return Err(ConfigError::RequiredParameter {
                name: "longitude_property".to_string(),
            });
        }

        match self.direction.to_ascii_lowercase().as_str() {
            "outgoing" | "incoming" => {}
            other => {
                return Err(ConfigError::InvalidParameter {
                    parameter: "direction".to_string(),
                    reason: format!("Must be 'outgoing' or 'incoming' (got '{other}')"),
                });
            }
        }

        Ok(())
    }
}

impl crate::config::ValidatedConfig for AStarConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        AStarConfig::validate(self)
    }
}

/// A* algorithm result
///
/// Translation of: `org.neo4j.gds.paths.dijkstra.PathFindingResult`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AStarResult {
    /// Path from source to target
    pub path: Option<Vec<NodeId>>,
    /// Total cost of the path
    pub total_cost: f64,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Number of nodes explored
    pub nodes_explored: usize,
}

impl AStarResult {
    /// Create a new A* result
    pub fn new(
        path: Option<Vec<NodeId>>,
        total_cost: f64,
        execution_time_ms: u64,
        nodes_explored: usize,
    ) -> Self {
        Self {
            path,
            total_cost,
            execution_time_ms,
            nodes_explored,
        }
    }

    /// Check if a path was found
    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    /// Get path length (number of nodes)
    pub fn path_length(&self) -> usize {
        self.path.as_ref().map_or(0, |p| p.len())
    }
}

/// Statistics about A* computation
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Summary of a mutate operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AStarMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AStarWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for A*: summary + updated store
#[derive(Debug, Clone)]
pub struct AStarMutateResult {
    pub summary: AStarMutationSummary,
    pub updated_store: std::sync::Arc<crate::types::prelude::DefaultGraphStore>,
}

fn checked_u64(value: NodeId) -> u64 {
    u64::try_from(value).unwrap_or(0)
}

fn spec_path_to_core(path: &[NodeId], source: NodeId, target: NodeId, cost: f64) -> PathResult {
    let path_ids = path
        .iter()
        .copied()
        .filter(|node_id| *node_id >= 0)
        .map(checked_u64)
        .collect();

    PathResult {
        source: checked_u64(source),
        target: checked_u64(target),
        path: path_ids,
        cost,
    }
}

/// A* result builder (pathfinding-family adapter).
pub struct AStarResultBuilder {
    result: AStarResult,
    execution_time: Duration,
    source_node: NodeId,
    target_node: NodeId,
    additional: HashMap<String, String>,
}

impl AStarResultBuilder {
    pub fn new(
        result: AStarResult,
        execution_time: Duration,
        source_node: NodeId,
        target_node: NodeId,
    ) -> Self {
        Self {
            result,
            execution_time,
            source_node,
            target_node,
            additional: HashMap::new(),
        }
    }

    pub fn result(
        result: AStarResult,
        execution_time: Duration,
        source_node: NodeId,
        target_node: NodeId,
    ) -> Result<PathFindingResult, AlgorithmError> {
        Self::new(result, execution_time, source_node, target_node).build_pathfinding_result()
    }

    pub fn result_with_additional(
        result: AStarResult,
        execution_time: Duration,
        source_node: NodeId,
        target_node: NodeId,
        additional: HashMap<String, String>,
    ) -> Result<PathFindingResult, AlgorithmError> {
        let mut builder = Self::new(result, execution_time, source_node, target_node);
        builder.additional = additional;
        builder.build_pathfinding_result()
    }

    pub fn build_pathfinding_result(mut self) -> Result<PathFindingResult, AlgorithmError> {
        let mut paths = Vec::new();
        if let Some(path) = &self.result.path {
            paths.push(spec_path_to_core(
                path,
                self.source_node,
                self.target_node,
                self.result.total_cost,
            ));
        }

        let targets_found = if self.result.path.is_some() {
            1u64
        } else {
            0u64
        };
        let all_targets_reached = targets_found == 1;
        let nodes_visited = self.result.nodes_explored as u64;

        self.additional
            .entry("nodes_visited".to_string())
            .or_insert_with(|| nodes_visited.to_string());
        self.additional
            .entry("targets_found".to_string())
            .or_insert_with(|| targets_found.to_string());
        self.additional
            .entry("all_targets_reached".to_string())
            .or_insert_with(|| all_targets_reached.to_string());

        let metadata = ExecutionMetadata {
            execution_time: self.execution_time,
            iterations: None,
            converged: Some(all_targets_reached),
            additional: self.additional,
        };

        PathFindingResultBuilder::new()
            .with_paths(paths)
            .with_metadata(metadata)
            .build()
            .map_err(|e| AlgorithmError::Execution(e.to_string()))
    }
}

// Generate the algorithm specification using focused macro
define_algorithm_spec! {
    name: "astar",
    output_type: AStarResult,
    projection_hint: Dense,
    modes: [Stream, WriteNodeProperty],

    execute: |_self, graph_store, config, _context| {
        use super::AStarStorageRuntime;
        use super::AStarComputationRuntime;
        use crate::core::utils::progress::Tasks;

        let start_time = std::time::Instant::now();

        // Parse config
        let parsed_config: AStarConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(e.to_string()))?;

        // Validate config
        parsed_config.validate()
            .map_err(|e| AlgorithmError::Execution(e.to_string()))?;

        // Build filtered/oriented graph view via overloads
        let rel_types: HashSet<RelationshipType> = if !parsed_config.relationship_types.is_empty() {
            RelationshipType::list_of(parsed_config.relationship_types.clone()).into_iter().collect()
        } else { HashSet::new() };
        let orientation = match AStarDirection::from_str(&parsed_config.direction) {
            AStarDirection::Outgoing => Orientation::Natural,
            AStarDirection::Incoming => Orientation::Reverse,
        };
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, orientation)
            .map_err(|e| AlgorithmError::Execution(e.to_string()))?;
        let lat_values = graph.node_properties(&parsed_config.latitude_property);
        let lon_values = graph.node_properties(&parsed_config.longitude_property);

        // Create storage runtime for accessing graph data
        let mut storage = match (lat_values, lon_values) {
            (Some(lat), Some(lon)) => AStarStorageRuntime::new_with_values(
                parsed_config.source_node,
                parsed_config.target_node,
                parsed_config.latitude_property.clone(),
                parsed_config.longitude_property.clone(),
                lat,
                lon,
            ),
            _ => AStarStorageRuntime::new(
                parsed_config.source_node,
                parsed_config.target_node,
                parsed_config.latitude_property.clone(),
                parsed_config.longitude_property.clone(),
            ),
        };

        // Create computation runtime for A* algorithm
        let mut computation = AStarComputationRuntime::new();

        // Progress tracking: A* volume is best-effort (relationship count);
        // work units are counted inside the driver loop in storage.
        let volume = graph.relationship_count();
        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("astar".to_string(), volume),
            parsed_config.concurrency,
        );

        // Execute A* algorithm
        let direction = AStarDirection::from_str(&parsed_config.direction);
        let result = storage
            .compute_astar_path(
                &mut computation,
                Some(graph.as_ref()),
                direction as u8,
                &mut progress_tracker,
            )
            .map_err(AlgorithmError::Execution)?;

        let execution_time = start_time.elapsed().as_millis() as u64;

        Ok(AStarResult::new(
            result.path,
            result.total_cost,
            execution_time,
            result.nodes_explored,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::algorithm::AlgorithmSpec; // bring trait methods into scope
    use serde_json::json; // macro for tests

    #[test]
    fn test_astar_config_default() {
        let config = AStarConfig::default();
        assert_eq!(config.source_node, 0);
        assert_eq!(config.target_node, 1);
        assert_eq!(config.latitude_property, "latitude");
        assert_eq!(config.longitude_property, "longitude");
        assert_eq!(config.concurrency, 4);
    }

    #[test]
    fn test_astar_config_validation() {
        let mut config = AStarConfig::default();

        // Valid config
        assert!(config.validate().is_ok());

        // Invalid concurrency
        config.concurrency = 0;
        assert!(config.validate().is_err());

        // Invalid latitude property
        config.concurrency = 4;
        config.latitude_property = String::new();
        assert!(config.validate().is_err());

        // Invalid longitude property
        config.latitude_property = "lat".to_string();
        config.longitude_property = String::new();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_astar_result() {
        let path = Some(vec![0, 1, 2]);
        let result = AStarResult::new(path.clone(), 10.5, 100, 5);

        assert!(result.has_path());
        assert_eq!(result.path_length(), 3);
        assert_eq!(result.total_cost, 10.5);
        assert_eq!(result.execution_time_ms, 100);
        assert_eq!(result.nodes_explored, 5);

        let no_path_result = AStarResult::new(None, f64::INFINITY, 50, 3);
        assert!(!no_path_result.has_path());
        assert_eq!(no_path_result.path_length(), 0);
    }

    #[test]
    fn test_astar_algorithm_spec_contract() {
        let spec = ASTARAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.graph_name(), "test_graph");
        assert_eq!(spec.name(), "astar");
    }

    #[test]
    fn test_astar_execution_modes() {
        let spec = ASTARAlgorithmSpec::new("test_graph".to_string());

        // Test that the spec can be created
        assert_eq!(spec.name(), "astar");
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_astar_config_validation_integration() {
        let config_input = r#"{
            "source_node": 0,
            "target_node": 1,
            "latitude_property": "lat",
            "longitude_property": "lon",
            "concurrency": 4
        }"#;

        let config: AStarConfig = serde_json::from_str(config_input).unwrap();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_astar_focused_macro_integration() {
        use crate::projection::eval::algorithm::ExecutionContext;

        let spec = ASTARAlgorithmSpec::new("test_graph".to_string());
        let _config = AStarConfig::default();

        // Test that the macro-generated spec works
        assert_eq!(spec.name(), "astar");
        assert_eq!(spec.graph_name(), "test_graph");

        // Test config validation through spec
        let validation_config = spec.validation_config(&ExecutionContext::new("test"));
        assert!(validation_config.validate_before_load(&json!({})).is_ok());
    }
}
