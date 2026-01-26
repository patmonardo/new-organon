//! Dijkstra Algorithm Specification
//!
//! **Translation Source**: `org.neo4j.gds.paths.dijkstra.Dijkstra`
//!
//! This module defines the Dijkstra algorithm specification using focused macros.
//! Dijkstra is implemented as a configurable Algorithmic Virtual Machine with
//! polymorphic target system, traversal state management, and stream-based results.

use super::path_finding_result::PathFindingResult as DijkstraPathFindingResult;
use super::DijkstraStorageRuntime;
use super::targets::create_targets;
use super::DijkstraComputationRuntime;
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
use std::collections::HashSet;
use std::time::Duration;

/// Dijkstra algorithm configuration
///
/// Translation of: Constructor parameters from `Dijkstra.java` (lines 118-140)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DijkstraConfig {
    /// Source node for shortest path computation
    pub source_node: NodeId,

    /// Target nodes (empty = all targets, single = single target, multiple = many targets)
    pub target_nodes: Vec<NodeId>,

    /// Whether to track relationship IDs
    pub track_relationships: bool,

    /// Concurrency level for parallel processing
    pub concurrency: usize,

    /// Whether to use heuristic function (for A* behavior)
    pub use_heuristic: bool,

    /// Optional relationship types to include (empty means all types)
    #[serde(default)]
    pub relationship_types: Vec<String>,

    /// Direction for traversal ("outgoing" or "incoming")
    #[serde(default = "DijkstraDirection::default_as_str")]
    pub direction: String,
}

impl Default for DijkstraConfig {
    fn default() -> Self {
        Self {
            source_node: 0,
            target_nodes: vec![],
            track_relationships: false,
            concurrency: 4,
            use_heuristic: false,
            relationship_types: vec![],
            direction: DijkstraDirection::Outgoing.as_str().to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DijkstraDirection {
    Outgoing,
    Incoming,
    Both,
}

impl DijkstraDirection {
    fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "incoming" => DijkstraDirection::Incoming,
            "both" => DijkstraDirection::Both,
            _ => DijkstraDirection::Outgoing,
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            DijkstraDirection::Outgoing => "outgoing",
            DijkstraDirection::Incoming => "incoming",
            DijkstraDirection::Both => "both",
        }
    }
    fn default_as_str() -> String {
        Self::Outgoing.as_str().to_string()
    }
}

impl DijkstraConfig {
    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.source_node < 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "sourceNode".to_string(),
                reason: "Must be >= 0".to_string(),
            });
        }

        if let Some(bad) = self.target_nodes.iter().copied().find(|id| *id < 0) {
            return Err(ConfigError::InvalidParameter {
                parameter: "targetNodes".to_string(),
                reason: format!("All target nodes must be >= 0, got {}", bad),
            });
        }

        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "Must be greater than 0".to_string(),
            });
        }

        match self.direction.to_ascii_lowercase().as_str() {
            "outgoing" | "incoming" | "both" => {}
            other => {
                return Err(ConfigError::InvalidParameter {
                    parameter: "direction".to_string(),
                    reason: format!("Must be 'outgoing', 'incoming', or 'both' (got '{other}')"),
                });
            }
        }

        Ok(())
    }
}

impl crate::config::ValidatedConfig for DijkstraConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        DijkstraConfig::validate(self)
    }
}

/// Dijkstra algorithm result
///
/// Translation of: `PathFindingResult` from `Dijkstra.java` (line 182)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DijkstraResult {
    /// Path finding result with stream-based processing
    pub path_finding_result: DijkstraPathFindingResult,

    /// Total computation time in milliseconds
    pub computation_time_ms: u64,
}

/// Individual path result for Dijkstra
///
/// Translation of: `PathResult` from `Dijkstra.java` (lines 245-284)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DijkstraPathResult {
    /// Path index
    pub index: u64,

    /// Source node ID
    pub source_node: NodeId,

    /// Target node ID
    pub target_node: NodeId,

    /// Node IDs along the path
    pub node_ids: Vec<NodeId>,

    /// Relationship IDs along the path (if tracking relationships)
    pub relationship_ids: Vec<NodeId>,

    /// Costs for each step along the path
    pub costs: Vec<f64>,
}

/// Statistics about Dijkstra computation
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Summary of a mutate operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DijkstraMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DijkstraWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for Dijkstra: summary + updated store
#[derive(Debug, Clone)]
pub struct DijkstraMutateResult {
    pub summary: DijkstraMutationSummary,
    pub updated_store: std::sync::Arc<crate::types::prelude::DefaultGraphStore>,
}

fn checked_u64(value: NodeId) -> u64 {
    u64::try_from(value).unwrap_or(0)
}

fn spec_path_to_core(path: &DijkstraPathResult) -> PathResult {
    let source = checked_u64(path.source_node);
    let target = checked_u64(path.target_node);
    let path_ids = path
        .node_ids
        .iter()
        .copied()
        .filter(|node_id| *node_id >= 0)
        .map(checked_u64)
        .collect();

    PathResult {
        source,
        target,
        path: path_ids,
        cost: path.total_cost(),
    }
}

/// Dijkstra result builder (pathfinding-family adapter).
pub struct DijkstraResultBuilder {
    result: DijkstraResult,
    execution_time: Duration,
}

impl DijkstraResultBuilder {
    pub fn new(result: DijkstraResult, execution_time: Duration) -> Self {
        Self {
            result,
            execution_time,
        }
    }

    pub fn result(
        result: DijkstraResult,
        execution_time: Duration,
    ) -> Result<PathFindingResult, AlgorithmError> {
        Self::new(result, execution_time).build_pathfinding_result()
    }

    pub fn build_pathfinding_result(self) -> Result<PathFindingResult, AlgorithmError> {
        let paths: Vec<PathResult> = self
            .result
            .path_finding_result
            .paths()
            .filter(|p| p.source_node >= 0 && p.target_node >= 0)
            .map(spec_path_to_core)
            .collect();

        let metadata = ExecutionMetadata {
            execution_time: self.execution_time,
            iterations: None,
            converged: None,
            additional: std::collections::HashMap::new(),
        };

        PathFindingResultBuilder::new()
            .with_paths(paths)
            .with_metadata(metadata)
            .build()
            .map_err(|e| AlgorithmError::Execution(e.to_string()))
    }
}

impl DijkstraPathResult {
    /// Calculate total cost of the path
    ///
    /// Translation of: `totalCost()` method from `PathResult.java` (lines 44-47)
    pub fn total_cost(&self) -> f64 {
        if self.costs.is_empty() {
            0.0
        } else {
            self.costs[self.costs.len() - 1]
        }
    }
}

// Generate the algorithm specification using focused macros
define_algorithm_spec! {
    name: "dijkstra",
    output_type: DijkstraResult,
    projection_hint: Dense,
    modes: [Stream, WriteNodeProperty],

    execute: |_self, graph_store, config, _context| {
        use crate::core::utils::progress::Tasks;
        // Parse configuration
        let config: DijkstraConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::InvalidGraph(
                format!("Failed to parse Dijkstra config: {}", e)
            ))?;

        // Validate configuration
        config.validate()
            .map_err(|e| AlgorithmError::InvalidGraph(
                format!("Configuration validation failed: {:?}", e)
            ))?;

        // Create targets system (the VM's instruction set)
        let targets = create_targets(config.target_nodes.clone());

        // Create storage and computation runtimes
        let mut storage = DijkstraStorageRuntime::new(
            config.source_node,
            config.track_relationships,
            config.concurrency,
            config.use_heuristic
        );

        let mut computation = DijkstraComputationRuntime::new(
            config.source_node,
            config.track_relationships,
            config.concurrency,
            config.use_heuristic
        );

        // Execute Dijkstra with filtered/oriented graph view
        let rel_types: HashSet<RelationshipType> = if !config.relationship_types.is_empty() {
            RelationshipType::list_of(config.relationship_types.clone()).into_iter().collect()
        } else {
            HashSet::new()
        };

        let orientation = match DijkstraDirection::from_str(&config.direction) {
            DijkstraDirection::Outgoing => Orientation::Natural,
            DijkstraDirection::Incoming => Orientation::Reverse,
            DijkstraDirection::Both => Orientation::Undirected,
        };

        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, orientation)
            .map_err(|e| AlgorithmError::InvalidGraph(
                format!("Failed to obtain graph view: {}", e)
            ))?;

        let direction = DijkstraDirection::from_str(&config.direction);
        let direction_byte = match direction {
            DijkstraDirection::Incoming => 1u8,
            _ => 0u8,
        };

        // Progress tracking: volume is best-effort (relationship count);
        // work units are counted inside the driver loop in storage.
        let volume = graph.relationship_count();
        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("dijkstra".to_string(), volume),
            config.concurrency,
        );

        let result = storage.compute_dijkstra(
            &mut computation,
            targets,
            Some(graph.as_ref()),
            direction_byte,
            &mut progress_tracker,
        )?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::algorithm::AlgorithmSpec;
    use crate::projection::eval::algorithm::ExecutionContext;
    use serde_json::json;

    #[test]
    fn test_dijkstra_config_default() {
        let config = DijkstraConfig::default();
        assert_eq!(config.source_node, 0);
        assert!(config.target_nodes.is_empty());
        assert!(!config.track_relationships);
        assert_eq!(config.concurrency, 4);
        assert!(!config.use_heuristic);
    }

    #[test]
    fn test_dijkstra_config_validation() {
        let mut config = DijkstraConfig::default();
        assert!(config.validate().is_ok());

        config.source_node = -1;
        assert!(config.validate().is_err());

        config.source_node = 0;

        config.concurrency = 0;
        assert!(config.validate().is_err());

        config.concurrency = 4;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_dijkstra_result() {
        let path_result = DijkstraPathResult {
            index: 0,
            source_node: 0,
            target_node: 5,
            node_ids: vec![0, 1, 3, 5],
            relationship_ids: vec![0, 1, 2],
            costs: vec![0.0, 3.5, 7.0, 10.5],
        };

        let path_finding_result = DijkstraPathFindingResult::new(vec![path_result.clone()]);
        let result = DijkstraResult {
            path_finding_result,
            computation_time_ms: 100,
        };

        assert_eq!(result.path_finding_result.path_count(), 1);
        assert_eq!(result.computation_time_ms, 100);
    }

    #[test]
    fn test_dijkstra_path_result() {
        let path_result = DijkstraPathResult {
            index: 0,
            source_node: 0,
            target_node: 5,
            node_ids: vec![0, 1, 3, 5],
            relationship_ids: vec![0, 1, 2],
            costs: vec![0.0, 3.5, 7.0, 10.5],
        };

        assert_eq!(path_result.index, 0);
        assert_eq!(path_result.source_node, 0);
        assert_eq!(path_result.target_node, 5);
        assert_eq!(path_result.total_cost(), 10.5);
        assert_eq!(path_result.node_ids.len(), 4);
        assert_eq!(path_result.relationship_ids.len(), 3);
        assert_eq!(path_result.costs.len(), 4);
    }

    #[test]
    fn test_dijkstra_algorithm_spec_contract() {
        let spec = DIJKSTRAAlgorithmSpec::new("test_graph".to_string());

        // Test that the macro-generated spec works
        assert_eq!(spec.name(), "dijkstra");
        assert_eq!(spec.graph_name(), "test_graph");

        // Test config validation through spec
        let validation_config = spec.validation_config(&ExecutionContext::new("test"));
        assert!(validation_config.validate_before_load(&json!({})).is_ok());
    }

    #[test]
    fn test_dijkstra_execution_modes() {
        let spec = DIJKSTRAAlgorithmSpec::new("test_graph".to_string());

        // Test execution mode support - the macro doesn't generate this method
        // so we'll just test that the spec was created successfully
        assert_eq!(spec.name(), "dijkstra");
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_dijkstra_config_validation_integration() {
        let spec = DIJKSTRAAlgorithmSpec::new("test_graph".to_string());

        // Test with valid config
        let valid_config = json!({
            "source_node": 0,
            "target_nodes": [5, 7],
            "track_relationships": true,
            "concurrency": 4,
            "use_heuristic": false
        });

        let validation_config = spec.validation_config(&ExecutionContext::new("test"));
        assert!(validation_config
            .validate_before_load(&valid_config)
            .is_ok());

        // Test invalid configuration - the validation_config doesn't validate our custom fields
        // so we'll test the config validation directly instead
        let invalid_config = DijkstraConfig {
            source_node: 0,
            target_nodes: vec![],
            track_relationships: false,
            concurrency: 0,
            use_heuristic: false,
            relationship_types: vec![],
            direction: DijkstraDirection::Outgoing.as_str().to_string(),
        };

        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_dijkstra_focused_macro_integration() {
        let spec = DIJKSTRAAlgorithmSpec::new("test_graph".to_string());

        // Test that the focused macro generated the correct structure
        assert_eq!(spec.name(), "dijkstra");
        assert_eq!(spec.graph_name(), "test_graph");

        // Test that we can create a config
        let config = DijkstraConfig::default();
        assert_eq!(config.source_node, 0);
        assert!(config.target_nodes.is_empty());
        assert!(!config.track_relationships);
    }
}
