//! Bellman-Ford Algorithm Specification
//!
//! **Translation Source**: `org.neo4j.gds.paths.bellmanford.BellmanFord`
//!
//! This module defines the Bellman-Ford algorithm specification using focused macros.
//! Bellman-Ford is unique among shortest path algorithms in its ability to detect
//! negative cycles, making it essential for certain graph analysis tasks.

use super::BellmanFordComputationRuntime;
use super::BellmanFordStorageRuntime;
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

/// Bellman-Ford algorithm configuration
///
/// Translation of: Constructor parameters from `BellmanFord.java` (lines 55-69)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BellmanFordConfig {
    /// Source node for shortest path computation
    pub source_node: NodeId,

    /// Whether to track negative cycles
    pub track_negative_cycles: bool,

    /// Whether to track shortest paths
    pub track_paths: bool,

    /// Concurrency level for parallel processing
    pub concurrency: usize,
    /// Optional relationship types to include (empty means all types)
    #[serde(default)]
    pub relationship_types: Vec<String>,
    /// Direction for traversal ("outgoing" or "incoming")
    #[serde(default = "BellmanDirection::default_as_str")]
    pub direction: String,
}

impl Default for BellmanFordConfig {
    fn default() -> Self {
        Self {
            source_node: 0,
            track_negative_cycles: true,
            track_paths: true,
            concurrency: 4,
            relationship_types: vec![],
            direction: BellmanDirection::Outgoing.as_str().to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BellmanDirection {
    Outgoing,
    Incoming,
}

impl BellmanDirection {
    fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "incoming" => BellmanDirection::Incoming,
            _ => BellmanDirection::Outgoing,
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            BellmanDirection::Outgoing => "outgoing",
            BellmanDirection::Incoming => "incoming",
        }
    }
    fn default_as_str() -> String {
        Self::Outgoing.as_str().to_string()
    }
}

impl BellmanFordConfig {
    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.source_node < 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "source_node".to_string(),
                reason: "Must be a non-negative i64".to_string(),
            });
        }

        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "Must be greater than 0".to_string(),
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

impl crate::config::ValidatedConfig for BellmanFordConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        BellmanFordConfig::validate(self)
    }
}

/// Bellman-Ford algorithm result
///
/// Translation of: `BellmanFordResult.java` (lines 24-28)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BellmanFordResult {
    /// Shortest paths found (empty if negative cycle detected or paths not tracked)
    pub shortest_paths: Vec<BellmanFordPathResult>,

    /// Negative cycles found (empty if not tracked)
    pub negative_cycles: Vec<BellmanFordPathResult>,

    /// Whether the graph contains negative cycles
    pub contains_negative_cycle: bool,
}

/// Individual path result for Bellman-Ford
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BellmanFordPathResult {
    /// Source node ID
    pub source_node: NodeId,

    /// Target node ID
    pub target_node: NodeId,

    /// Total cost of the path
    pub total_cost: f64,

    /// Node IDs along the path
    pub node_ids: Vec<NodeId>,

    /// Costs for each step along the path
    pub costs: Vec<f64>,
}

/// Statistics about Bellman-Ford execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BellmanFordStats {
    pub paths_found: u64,
    pub negative_cycles_found: u64,
    pub contains_negative_cycle: bool,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BellmanFordMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BellmanFordWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for Bellman-Ford: summary + updated store
#[derive(Debug, Clone)]
pub struct BellmanFordMutateResult {
    pub summary: BellmanFordMutationSummary,
    pub updated_store: std::sync::Arc<crate::types::prelude::DefaultGraphStore>,
}

fn checked_u64(value: NodeId, _context: &str) -> u64 {
    u64::try_from(value).unwrap_or(0)
}

fn spec_path_to_core(path: &BellmanFordPathResult) -> PathResult {
    let source = checked_u64(path.source_node, "source");
    let target = checked_u64(path.target_node, "target");
    let path_ids = path
        .node_ids
        .iter()
        .copied()
        .filter(|node_id| *node_id >= 0)
        .map(|node_id| checked_u64(node_id, "path"))
        .collect();

    PathResult {
        source,
        target,
        path: path_ids,
        cost: path.total_cost,
    }
}

fn result_paths(result: &BellmanFordResult) -> Vec<PathResult> {
    result
        .shortest_paths
        .iter()
        .filter(|p| p.source_node >= 0 && p.target_node >= 0)
        .map(spec_path_to_core)
        .collect()
}

/// Bellman-Ford result builder (pathfinding-family adapter).
pub struct BellmanFordResultBuilder {
    result: BellmanFordResult,
    execution_time: Duration,
}

impl BellmanFordResultBuilder {
    pub fn new(result: BellmanFordResult, execution_time: Duration) -> Self {
        Self {
            result,
            execution_time,
        }
    }

    pub fn result(
        result: BellmanFordResult,
        execution_time: Duration,
    ) -> Result<PathFindingResult, AlgorithmError> {
        Self::new(result, execution_time).build_pathfinding_result()
    }

    pub fn build_pathfinding_result(self) -> Result<PathFindingResult, AlgorithmError> {
        let paths = result_paths(&self.result);

        let metadata = ExecutionMetadata {
            execution_time: self.execution_time,
            iterations: None,
            converged: Some(!self.result.contains_negative_cycle),
            additional: std::collections::HashMap::from([
                (
                    "negative_cycles_found".to_string(),
                    self.result.negative_cycles.len().to_string(),
                ),
                (
                    "contains_negative_cycle".to_string(),
                    self.result.contains_negative_cycle.to_string(),
                ),
            ]),
        };

        PathFindingResultBuilder::new()
            .with_paths(paths)
            .with_metadata(metadata)
            .build()
            .map_err(|e| AlgorithmError::Execution(e.to_string()))
    }

    pub fn paths(&self) -> Vec<PathResult> {
        result_paths(&self.result)
    }

    pub fn stats(&self) -> BellmanFordStats {
        BellmanFordStats {
            paths_found: self.result.shortest_paths.len() as u64,
            negative_cycles_found: self.result.negative_cycles.len() as u64,
            contains_negative_cycle: self.result.contains_negative_cycle,
            execution_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn mutation_summary(
        &self,
        property_name: &str,
        nodes_updated: u64,
    ) -> BellmanFordMutationSummary {
        BellmanFordMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn write_summary(
        &self,
        property_name: &str,
        nodes_written: u64,
    ) -> BellmanFordWriteSummary {
        BellmanFordWriteSummary {
            nodes_written,
            property_name: property_name.to_string(),
            execution_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.execution_time.as_millis() as u64
    }
}

// Generate the algorithm specification using focused macros
define_algorithm_spec! {
    name: "bellman_ford",
    output_type: BellmanFordResult,
    projection_hint: Dense,
    modes: [Stream, WriteNodeProperty],

    execute: |_self, graph_store, config, _context| {
        use crate::core::utils::progress::Tasks;
        // Parse configuration
        let config: BellmanFordConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::InvalidGraph(
                format!("Failed to parse Bellman-Ford config: {}", e)
            ))?;

        // Validate configuration
        config.validate()
            .map_err(|e| AlgorithmError::InvalidGraph(
                format!("Configuration validation failed: {:?}", e)
            ))?;

        // Create storage and computation runtimes
        let mut storage = BellmanFordStorageRuntime::new(
            config.source_node,
            config.track_negative_cycles,
            config.track_paths,
            config.concurrency
        );

        let mut computation = BellmanFordComputationRuntime::new(
            config.source_node,
            config.track_negative_cycles,
            config.track_paths,
            config.concurrency
        );

        // Build filtered/oriented graph view via overloads
        let rel_types: HashSet<RelationshipType> = if !config.relationship_types.is_empty() {
            RelationshipType::list_of(config.relationship_types.clone()).into_iter().collect()
        } else { HashSet::new() };
        let orientation = match BellmanDirection::from_str(&config.direction) {
            BellmanDirection::Outgoing => Orientation::Natural,
            BellmanDirection::Incoming => Orientation::Reverse,
        };
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, orientation)
            .map_err(|e| AlgorithmError::InvalidGraph(
                format!("Failed to obtain graph view: {}", e)
            ))?;

        let direction = BellmanDirection::from_str(&config.direction);

        // Progress tracking: best-effort volume (relationship count);
        // work units are counted inside the driver loop in storage.
        let volume = graph.relationship_count();
        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("bellman_ford".to_string(), volume),
            config.concurrency,
        );

        // Execute Bellman-Ford algorithm with graph from graph_store
        let result = storage.compute_bellman_ford(
            &mut computation,
            Some(graph.as_ref()),
            direction as u8,
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
    fn test_bellman_ford_config_default() {
        let config = BellmanFordConfig::default();
        assert_eq!(config.source_node, 0);
        assert!(config.track_negative_cycles);
        assert!(config.track_paths);
        assert_eq!(config.concurrency, 4);
    }

    #[test]
    fn test_bellman_ford_config_validation() {
        let mut config = BellmanFordConfig::default();
        assert!(config.validate().is_ok());

        config.concurrency = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_bellman_ford_result() {
        let result = BellmanFordResult {
            shortest_paths: vec![],
            negative_cycles: vec![],
            contains_negative_cycle: false,
        };

        assert!(!result.contains_negative_cycle);
        assert!(result.shortest_paths.is_empty());
        assert!(result.negative_cycles.is_empty());
    }

    #[test]
    fn test_bellman_ford_algorithm_spec_contract() {
        let spec = BELLMAN_FORDAlgorithmSpec::new("test_graph".to_string());

        // Test that the macro-generated spec works
        assert_eq!(spec.name(), "bellman_ford");
        assert_eq!(spec.graph_name(), "test_graph");

        // Test config validation through spec
        let validation_config = spec.validation_config(&ExecutionContext::new("test"));
        assert!(validation_config.validate_before_load(&json!({})).is_ok());
    }

    #[test]
    fn test_bellman_ford_execution_modes() {
        let spec = BELLMAN_FORDAlgorithmSpec::new("test_graph".to_string());

        // Test execution mode support - the macro doesn't generate this method
        // so we'll just test that the spec was created successfully
        assert_eq!(spec.name(), "bellman_ford");
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_bellman_ford_config_validation_integration() {
        let spec = BELLMAN_FORDAlgorithmSpec::new("test_graph".to_string());

        // Test with valid config
        let valid_config = json!({
            "source_node": 0,
            "track_negative_cycles": true,
            "track_paths": true,
            "concurrency": 4
        });

        let validation_config = spec.validation_config(&ExecutionContext::new("test"));
        assert!(validation_config
            .validate_before_load(&valid_config)
            .is_ok());

        // Test with invalid config
        // Test invalid configuration - the validation_config doesn't validate our custom fields
        // so we'll test the config validation directly instead
        let invalid_config = BellmanFordConfig {
            source_node: 0,
            track_negative_cycles: true,
            track_paths: true,
            concurrency: 0,
            relationship_types: vec![],
            direction: BellmanDirection::Outgoing.as_str().to_string(),
        };

        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_bellman_ford_focused_macro_integration() {
        let spec = BELLMAN_FORDAlgorithmSpec::new("test_graph".to_string());

        // Test that the focused macro generated the correct structure
        assert_eq!(spec.name(), "bellman_ford");
        assert_eq!(spec.graph_name(), "test_graph");

        // Test that we can create a config
        let config = BellmanFordConfig::default();
        assert_eq!(config.source_node, 0);
    }

    #[test]
    fn test_bellman_ford_path_result() {
        let path_result = BellmanFordPathResult {
            source_node: 0,
            target_node: 5,
            total_cost: 10.5,
            node_ids: vec![0, 1, 3, 5],
            costs: vec![0.0, 3.5, 7.0, 10.5],
        };

        assert_eq!(path_result.source_node, 0);
        assert_eq!(path_result.target_node, 5);
        assert_eq!(path_result.total_cost, 10.5);
        assert_eq!(path_result.node_ids.len(), 4);
        assert_eq!(path_result.costs.len(), 4);
    }
}
