//! **DFS Algorithm Specification**
//!
//! **Translation Source**: `org.neo4j.gds.traversal.DFS`
//!
//! This module defines the DFS algorithm specification, configuration, and result types.

use super::DfsStorageRuntime;
use super::DfsComputationRuntime;
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
use crate::projection::RelationshipType;
use crate::types::graph::NodeId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// DFS algorithm configuration
///
/// Translation of: `DFSConfig.java` (lines 32-75)
/// This defines the parameters for DFS traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfsConfig {
    /// Source node for DFS traversal
    pub source_node: NodeId,
    /// Target nodes to find (empty means find all reachable)
    pub target_nodes: Vec<NodeId>,
    /// Maximum depth to traverse (None means unlimited)
    pub max_depth: Option<u32>,
    /// Whether to track paths during traversal
    pub track_paths: bool,
    /// Concurrency level for parallel processing
    pub concurrency: usize,
}

impl Default for DfsConfig {
    fn default() -> Self {
        Self {
            source_node: 0,
            target_nodes: Vec::new(),
            max_depth: None,
            track_paths: false,
            concurrency: 1,
        }
    }
}

impl DfsConfig {
    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.source_node < 0 {
            return Err(ConfigError::FieldValidation {
                field: "source_node".to_string(),
                message: "must be >= 0".to_string(),
            });
        }

        if self.target_nodes.iter().any(|&node_id| node_id < 0) {
            return Err(ConfigError::FieldValidation {
                field: "target_nodes".to_string(),
                message: "all target nodes must be >= 0".to_string(),
            });
        }

        if self.concurrency == 0 {
            return Err(ConfigError::FieldValidation {
                field: "concurrency".to_string(),
                message: "must be > 0".to_string(),
            });
        }

        if let Some(depth) = self.max_depth {
            if depth == 0 {
                return Err(ConfigError::FieldValidation {
                    field: "max_depth".to_string(),
                    message: "must be > 0 or None".to_string(),
                });
            }
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for DfsConfig {
    fn validate(&self) -> Result<(), crate::config::validation::ConfigError> {
        self.validate().map_err(
            |e| crate::config::validation::ConfigError::InvalidParameter {
                parameter: "DfsConfig".to_string(),
                reason: e.to_string(),
            },
        )
    }
}

/// DFS algorithm result
///
/// Translation of: `DFSResult.java` (lines 76-120)
/// This contains the results of DFS traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfsResult {
    /// Visited nodes in traversal order
    pub visited_nodes: Vec<NodeId>,
    /// Computation time in milliseconds
    pub computation_time_ms: u64,
}

/// Individual path result from DFS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfsPathResult {
    /// Source node
    pub source_node: NodeId,
    /// Target node
    pub target_node: NodeId,
    /// Path as sequence of node IDs
    pub node_ids: Vec<NodeId>,
    /// Path length (number of edges)
    pub path_length: u32,
}

/// Statistics about DFS computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfsStats {
    /// Number of nodes visited during traversal
    pub nodes_visited: u64,
    /// Maximum depth reached during traversal
    pub max_depth_reached: u64,
    /// Total computation time in milliseconds
    pub execution_time_ms: u64,
    /// Number of target nodes found (if any specified)
    pub targets_found: u64,
    /// Whether all targets were reached
    pub all_targets_reached: bool,
    /// Number of backtracking operations performed
    pub backtrack_operations: u64,
    /// Average branch depth before backtracking
    pub avg_branch_depth: f64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfsMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfsWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for DFS: summary + updated store
#[derive(Debug, Clone)]
pub struct DfsMutateResult {
    pub summary: DfsMutationSummary,
    pub updated_store: std::sync::Arc<crate::types::prelude::DefaultGraphStore>,
}

fn checked_u64(value: NodeId) -> u64 {
    u64::try_from(value).unwrap_or(0)
}

fn spec_path_to_core(source: NodeId, target: NodeId, index: usize) -> PathResult {
    let target_u64 = checked_u64(target);
    PathResult {
        source: checked_u64(source),
        target: target_u64,
        path: vec![target_u64],
        cost: index as f64,
    }
}

/// DFS result builder (pathfinding-family adapter).
pub struct DfsResultBuilder {
    result: DfsResult,
    execution_time: Duration,
    source_node: NodeId,
    target_count: usize,
}

impl DfsResultBuilder {
    pub fn new(
        result: DfsResult,
        execution_time: Duration,
        source_node: NodeId,
        target_count: usize,
    ) -> Self {
        Self {
            result,
            execution_time,
            source_node,
            target_count,
        }
    }

    pub fn result(
        result: DfsResult,
        execution_time: Duration,
        source_node: NodeId,
        target_count: usize,
    ) -> Result<PathFindingResult, AlgorithmError> {
        Self::new(result, execution_time, source_node, target_count).build_pathfinding_result()
    }

    pub fn build_pathfinding_result(self) -> Result<PathFindingResult, AlgorithmError> {
        let paths: Vec<PathResult> = self
            .result
            .visited_nodes
            .iter()
            .copied()
            .enumerate()
            .map(|(index, node_id)| spec_path_to_core(self.source_node, node_id, index))
            .collect();

        let mut additional = HashMap::new();
        additional.insert(
            "computation_time_ms".to_string(),
            self.result.computation_time_ms.to_string(),
        );
        additional.insert("targets".to_string(), self.target_count.to_string());

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
    name: "dfs",
    output_type: DfsResult,
    projection_hint: Dense,
    modes: [Stream, WriteNodeProperty],
    execute: |_self, graph_store, config_input, _context| {
        // Parse and validate configuration
        let parsed_config: DfsConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to parse config: {}", e)))?;

        parsed_config.validate()
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Config validation failed: {}", e)))?;

        // Create storage and computation runtimes
        let storage = DfsStorageRuntime::new(
            parsed_config.source_node,
            parsed_config.target_nodes.clone(),
            parsed_config.max_depth,
            parsed_config.track_paths,
            parsed_config.concurrency,
        );

        // Execute DFS algorithm with a filtered/oriented view (defaults: all types, NATURAL)
        let rel_types: std::collections::HashSet<RelationshipType> = std::collections::HashSet::new();
        let graph_view = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to obtain graph view: {}", e)))?;

        let node_count = graph_view.node_count() as usize;

        let mut computation = DfsComputationRuntime::new(
            parsed_config.source_node,
            parsed_config.track_paths,
            parsed_config.concurrency,
            node_count,
        );

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf("DFS".to_string()),
            parsed_config.concurrency,
        );

        let result =
            storage.compute_dfs(&mut computation, Some(graph_view.as_ref()), &mut progress_tracker)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::algorithm::{AlgorithmSpec, ExecutionContext};
    use serde_json::json;

    #[test]
    fn test_dfs_result() {
        let result = DfsResult {
            visited_nodes: vec![0, 1, 2],
            computation_time_ms: 5,
        };

        assert_eq!(result.visited_nodes.len(), 3);
        assert_eq!(result.computation_time_ms, 5);
    }

    #[test]
    fn test_dfs_path_result() {
        let path = DfsPathResult {
            source_node: 0,
            target_node: 3,
            node_ids: vec![0, 1, 2, 3],
            path_length: 3,
        };

        assert_eq!(path.source_node, 0);
        assert_eq!(path.target_node, 3);
        assert_eq!(path.node_ids.len(), 4);
        assert_eq!(path.path_length, 3);
    }

    #[test]
    fn test_dfs_config_default() {
        let config = DfsConfig::default();
        assert_eq!(config.source_node, 0);
        assert!(config.target_nodes.is_empty());
        assert!(config.max_depth.is_none());
        assert!(!config.track_paths);
        assert_eq!(config.concurrency, 1);
    }

    #[test]
    fn test_dfs_config_validation() {
        let mut config = DfsConfig::default();
        assert!(config.validate().is_ok());

        config.concurrency = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_dfs_algorithm_spec_contract() {
        let spec = DFSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.name(), "dfs");
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_dfs_execution_modes() {
        let spec = DFSAlgorithmSpec::new("test_graph".to_string());
        // Test that the algorithm can be created
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_dfs_config_validation_integration() {
        let spec = DFSAlgorithmSpec::new("test_graph".to_string());
        let valid_config = json!({
            "source_node": 0,
            "target_nodes": [1, 2],
            "max_depth": 5,
            "track_paths": true,
            "concurrency": 4
        });

        let validation_config = spec.validation_config(&ExecutionContext::new("test_user"));
        // Note: graph-aware validation is deferred; config-level validation covers basics.
        assert!(validation_config
            .validate_before_load(&valid_config)
            .is_ok());
    }

    #[test]
    fn test_dfs_focused_macro_integration() {
        let spec = DFSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.name(), "dfs");
        assert_eq!(spec.graph_name(), "test_graph");

        // Test that the algorithm can be created
        assert_eq!(spec.graph_name(), "test_graph");
    }
}
