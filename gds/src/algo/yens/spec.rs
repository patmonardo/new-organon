//! **Yen's Algorithm Specification**
//!
//! **Translation Source**: `org.neo4j.gds.paths.yens.Yens`
//!
//! This module defines the Yen's algorithm specification, configuration, and result types.

use super::YensStorageRuntime;
use super::YensComputationRuntime;
use crate::algo::algorithms::pathfinding::{
    PathFindingResult, PathFindingResultBuilder, PathResult,
};
use crate::algo::algorithms::{ExecutionMetadata, ResultBuilder};
use crate::config::validation::ConfigError;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::relationship_type::RelationshipType;
use crate::types::graph::NodeId;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::Duration;

/// Yen's algorithm configuration
///
/// Translation of: `ShortestPathYensBaseConfig.java`
/// This defines the parameters for Yen's K-shortest paths algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YensConfig {
    /// Source node for path finding
    pub source_node: NodeId,
    /// Target node for path finding
    pub target_node: NodeId,
    /// Number of shortest paths to find (K)
    pub k: usize,
    /// Property name for relationship weights
    #[serde(default = "YensConfig::default_weight_property")]
    pub weight_property: String,
    /// Optional relationship types to include (empty means all types)
    #[serde(default)]
    pub relationship_types: Vec<String>,
    /// Direction for traversal ("outgoing" or "incoming")
    #[serde(default = "YensConfig::default_direction")]
    pub direction: String,
    /// Whether to track relationships
    pub track_relationships: bool,
    /// Concurrency level for parallel processing
    pub concurrency: usize,
}

impl Default for YensConfig {
    fn default() -> Self {
        Self {
            source_node: 0,
            target_node: 1,
            k: 3,
            weight_property: Self::default_weight_property(),
            relationship_types: vec![],
            direction: Self::default_direction(),
            track_relationships: false,
            concurrency: 1,
        }
    }
}

impl YensConfig {
    fn default_weight_property() -> String {
        "weight".to_string()
    }

    fn default_direction() -> String {
        "outgoing".to_string()
    }

    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.source_node < 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "sourceNode".to_string(),
                reason: "must be >= 0".to_string(),
            });
        }
        if self.target_node < 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "targetNode".to_string(),
                reason: "must be >= 0".to_string(),
            });
        }
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "must be > 0".to_string(),
            });
        }
        if self.k == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "k".to_string(),
                reason: "must be > 0".to_string(),
            });
        }
        if self.source_node == self.target_node {
            return Err(ConfigError::InvalidParameter {
                parameter: "sourceNode".to_string(),
                reason: "source and target nodes must be different".to_string(),
            });
        }

        match self.direction.to_ascii_lowercase().as_str() {
            "outgoing" | "incoming" => {}
            other => {
                return Err(ConfigError::InvalidParameter {
                    parameter: "direction".to_string(),
                    reason: format!("must be 'outgoing' or 'incoming' (got '{other}')"),
                });
            }
        }

        if self.weight_property.trim().is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "weightProperty".to_string(),
                reason: "must be non-empty".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for YensConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        YensConfig::validate(self)
    }
}

/// Yen's algorithm result
///
/// Translation of: `PathFindingResult.java`
/// This contains the K shortest paths found by Yen's algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YensResult {
    /// List of K shortest paths
    pub paths: Vec<YensPathResult>,
    /// Number of paths found
    pub path_count: usize,
    /// Computation time in milliseconds
    pub computation_time_ms: u64,
}

/// Individual path result from Yen's algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YensPathResult {
    /// Index of this path (1st, 2nd, 3rd shortest, etc.)
    pub index: u32,
    /// Source node
    pub source_node: NodeId,
    /// Target node
    pub target_node: NodeId,
    /// Path as sequence of node IDs
    pub node_ids: Vec<NodeId>,
    /// Path as sequence of relationship IDs
    pub relationship_ids: Vec<NodeId>,
    /// Costs accumulated along the path
    pub costs: Vec<f64>,
    /// Total cost of the path
    pub total_cost: f64,
}

/// Statistics about Yen's execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YensStats {
    pub paths_found: u64,
    pub computation_time_ms: u64,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YensMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YensWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for Yen's K-shortest paths: summary + updated store
#[derive(Debug, Clone)]
pub struct YensMutateResult {
    pub summary: YensMutationSummary,
    pub updated_store: std::sync::Arc<crate::types::prelude::DefaultGraphStore>,
}

fn checked_u64(value: NodeId) -> u64 {
    u64::try_from(value).unwrap_or(0)
}

fn spec_path_to_core(path: &YensPathResult) -> PathResult {
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
        cost: path.total_cost,
    }
}

/// Yen's result builder (pathfinding-family adapter).
pub struct YensResultBuilder {
    result: YensResult,
    execution_time: Duration,
    k: usize,
    track_relationships: bool,
}

impl YensResultBuilder {
    pub fn new(
        result: YensResult,
        execution_time: Duration,
        k: usize,
        track_relationships: bool,
    ) -> Self {
        Self {
            result,
            execution_time,
            k,
            track_relationships,
        }
    }

    pub fn result(
        result: YensResult,
        execution_time: Duration,
        k: usize,
        track_relationships: bool,
    ) -> Result<PathFindingResult, AlgorithmError> {
        Self::new(result, execution_time, k, track_relationships).build_pathfinding_result()
    }

    pub fn build_pathfinding_result(self) -> Result<PathFindingResult, AlgorithmError> {
        let paths: Vec<PathResult> = self.result.paths.iter().map(spec_path_to_core).collect();

        let additional = HashMap::from([
            (
                "computation_time_ms".to_string(),
                self.result.computation_time_ms.to_string(),
            ),
            ("k".to_string(), self.k.to_string()),
            (
                "track_relationships".to_string(),
                self.track_relationships.to_string(),
            ),
        ]);

        let metadata = ExecutionMetadata {
            execution_time: self.execution_time,
            iterations: None,
            converged: None,
            additional,
        };

        PathFindingResultBuilder::new()
            .with_paths(paths)
            .with_metadata(metadata)
            .build()
            .map_err(|e| AlgorithmError::Execution(e.to_string()))
    }
}

// Generate the algorithm specification using focused macros
define_algorithm_spec! {
    name: "yens",
    output_type: YensResult,
    projection_hint: Dense,
    modes: [Stream, WriteNodeProperty],
    execute: |_self, graph_store, config_input, _context| {
        // Parse and validate configuration
        let parsed_config: YensConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to parse config: {}", e)))?;

        parsed_config.validate()
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Config validation failed: {}", e)))?;

        // Create storage and computation runtimes
        let storage = YensStorageRuntime::new(
            parsed_config.source_node,
            parsed_config.target_node,
            parsed_config.k,
            parsed_config.track_relationships,
            parsed_config.concurrency,
        );

        let mut computation = YensComputationRuntime::new(
            parsed_config.source_node,
            parsed_config.target_node,
            parsed_config.k,
            parsed_config.track_relationships,
            parsed_config.concurrency,
        );

        let rel_types: HashSet<RelationshipType> = if parsed_config.relationship_types.is_empty() {
            graph_store.relationship_types()
        } else {
            RelationshipType::list_of(parsed_config.relationship_types.clone())
                .into_iter()
                .collect()
        };

        let (orientation, direction_byte) = match parsed_config.direction.to_ascii_lowercase().as_str() {
            "incoming" => (Orientation::Reverse, 1u8),
            _ => (Orientation::Natural, 0u8),
        };

        let selectors: HashMap<RelationshipType, String> = rel_types
            .iter()
            .map(|t| (t.clone(), parsed_config.weight_property.clone()))
            .collect();

        let graph = graph_store
            .get_graph_with_types_selectors_and_orientation(&rel_types, &selectors, orientation)
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to obtain graph view: {}", e)))?;

        // Execute Yen's algorithm
        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("yens".to_string(), parsed_config.k),
            parsed_config.concurrency,
        );

        let result = storage.compute_yens(
            &mut computation,
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
    use crate::projection::eval::algorithm::{AlgorithmSpec, ExecutionContext};
    use serde_json::json;

    #[test]
    fn test_yens_result() {
        let result = YensResult {
            paths: vec![YensPathResult {
                index: 0,
                source_node: 0,
                target_node: 3,
                node_ids: vec![0, 1, 3],
                relationship_ids: vec![10, 13],
                costs: vec![0.0, 1.0, 2.0],
                total_cost: 2.0,
            }],
            path_count: 1,
            computation_time_ms: 5,
        };

        assert_eq!(result.paths.len(), 1);
        assert_eq!(result.path_count, 1);
        assert_eq!(result.computation_time_ms, 5);
    }

    #[test]
    fn test_yens_path_result() {
        let path = YensPathResult {
            index: 0,
            source_node: 0,
            target_node: 3,
            node_ids: vec![0, 1, 2, 3],
            relationship_ids: vec![10, 11, 12],
            costs: vec![0.0, 1.0, 2.0, 3.0],
            total_cost: 3.0,
        };

        assert_eq!(path.index, 0);
        assert_eq!(path.source_node, 0);
        assert_eq!(path.target_node, 3);
        assert_eq!(path.node_ids.len(), 4);
        assert_eq!(path.total_cost, 3.0);
    }

    #[test]
    fn test_yens_config_default() {
        let config = YensConfig::default();
        assert_eq!(config.source_node, 0);
        assert_eq!(config.target_node, 1);
        assert_eq!(config.k, 3);
        assert_eq!(config.weight_property, "weight");
        assert!(config.relationship_types.is_empty());
        assert_eq!(config.direction, "outgoing");
        assert!(!config.track_relationships);
        assert_eq!(config.concurrency, 1);
    }

    #[test]
    fn test_yens_config_validation() {
        let mut config = YensConfig::default();
        assert!(config.validate().is_ok());

        config.concurrency = 0;
        assert!(config.validate().is_err());

        config.concurrency = 1;
        config.k = 0;
        assert!(config.validate().is_err());

        config.k = 3;
        config.target_node = config.source_node;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_yens_algorithm_spec_contract() {
        let spec = YENSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.name(), "yens");
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_yens_execution_modes() {
        let spec = YENSAlgorithmSpec::new("test_graph".to_string());
        // Test that the algorithm can be created
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_yens_config_validation_integration() {
        let spec = YENSAlgorithmSpec::new("test_graph".to_string());
        let valid_config = json!({
            "source_node": 0,
            "target_node": 3,
            "k": 5,
            "track_relationships": true,
            "concurrency": 4
        });

        let validation_config = spec.validation_config(&ExecutionContext::new("test_user"));
        // Note: graph-aware validation is deferred; config-level validation covers basics.
        assert!(validation_config
            .validate_before_load(&valid_config)
            .is_ok());
    }

    #[test]
    fn test_yens_focused_macro_integration() {
        let spec = YENSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.name(), "yens");
        assert_eq!(spec.graph_name(), "test_graph");

        // Test that the algorithm can be created
        assert_eq!(spec.graph_name(), "test_graph");
    }
}
