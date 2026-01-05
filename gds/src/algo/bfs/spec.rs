//! **BFS Algorithm Specification**
//!
//! **Translation Source**: `org.neo4j.gds.traversal.BFS`
//!
//! This module defines the BFS algorithm specification, configuration, and result types.

use super::computation::BfsComputationRuntime;
use super::storage::BfsStorageRuntime;
use crate::core::utils::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::codegen::config::validation::ConfigError;
use crate::projection::eval::procedure::AlgorithmError;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use serde::{Deserialize, Serialize};

/// BFS algorithm configuration
///
/// Translation of: `BFSConfig.java` (lines 32-75)
/// This defines the parameters for BFS traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BfsConfig {
    /// Source node for BFS traversal
    pub source_node: NodeId,
    /// Target nodes to find (empty means find all reachable)
    pub target_nodes: Vec<NodeId>,
    /// Maximum depth to traverse (None means unlimited)
    pub max_depth: Option<u32>,
    /// Whether to track paths during traversal
    pub track_paths: bool,
    /// Concurrency level for parallel processing
    pub concurrency: usize,
    /// Delta parameter for chunking (default 64)
    pub delta: usize,
}

impl Default for BfsConfig {
    fn default() -> Self {
        Self {
            source_node: 0,
            target_nodes: Vec::new(),
            max_depth: None,
            track_paths: false,
            concurrency: 1,
            delta: 64, // Default delta from Java BFS
        }
    }
}

impl BfsConfig {
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
        Ok(())
    }
}

/// BFS algorithm result
///
/// Translation of: `BFSResult.java` (lines 76-120)
/// This contains the results of BFS traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BfsResult {
    /// Visited nodes in traversal order
    pub visited_nodes: Vec<NodeId>,
    /// Computation time in milliseconds
    pub computation_time_ms: u64,
}

/// Individual path result from BFS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BfsPathResult {
    /// Source node
    pub source_node: NodeId,
    /// Target node
    pub target_node: NodeId,
    /// Path as sequence of node IDs
    pub node_ids: Vec<NodeId>,
    /// Path length (number of edges)
    pub path_length: u32,
}

// Generate the algorithm specification using focused macros
define_algorithm_spec! {
    name: "bfs",
    output_type: BfsResult,
    projection_hint: Dense,
    modes: [Stream, WriteNodeProperty],
    execute: |_self, graph_store, config_input, _context| {
        // Parse and validate configuration
        let parsed_config: BfsConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to parse config: {}", e)))?;

        parsed_config.validate()
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Config validation failed: {}", e)))?;

        // Create storage and computation runtimes
        let storage = BfsStorageRuntime::new(
            parsed_config.source_node,
            parsed_config.target_nodes.clone(),
            parsed_config.max_depth,
            parsed_config.track_paths,
        );

        // Execute BFS algorithm with a filtered/oriented view (defaults: all types, NATURAL)
        let rel_types: std::collections::HashSet<RelationshipType> = std::collections::HashSet::new();
        let graph_view = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to obtain graph view: {}", e)))?;

        let node_count = graph_view.node_count() as usize;

        let mut computation = BfsComputationRuntime::new(
            parsed_config.source_node,
            parsed_config.track_paths,
            parsed_config.concurrency,
            node_count,
        );

        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_concurrency(
            Tasks::leaf("BFS".to_string()),
            parsed_config.concurrency,
        );

        let result =
            storage.compute_bfs(&mut computation, Some(graph_view.as_ref()), &mut progress_tracker)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::procedure::AlgorithmSpec;

    #[test]
    fn test_bfs_result() {
        let result = BfsResult {
            visited_nodes: vec![(0, 0), (1, 1), (2, 1)],
            paths: vec![BfsPathResult {
                source_node: 0,
                target_node: 2,
                node_ids: vec![0, 1, 2],
                path_length: 2,
            }],
            nodes_visited: 3,
            computation_time_ms: 5,
        };

        assert_eq!(result.visited_nodes.len(), 3);
        assert_eq!(result.paths.len(), 1);
        assert_eq!(result.nodes_visited, 3);
    }

    #[test]
    fn test_bfs_path_result() {
        let path = BfsPathResult {
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
    fn test_bfs_config_default() {
        let config = BfsConfig::default();
        assert_eq!(config.source_node, 0);
        assert!(config.target_nodes.is_empty());
        assert!(config.max_depth.is_none());
        assert!(!config.track_paths);
        assert_eq!(config.concurrency, 1);
    }

    #[test]
    fn test_bfs_config_validation() {
        let mut config = BfsConfig::default();
        assert!(config.validate().is_ok());

        config.concurrency = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_bfs_algorithm_spec_contract() {
        let spec = BFSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.name(), "bfs");
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_bfs_execution_modes() {
        let spec = BFSAlgorithmSpec::new("test_graph".to_string());
        // Test that the algorithm can be created
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_bfs_config_validation_integration() {
        // Macro validation_config does not validate custom fields; use BfsConfig::validate()
        let config = BfsConfig {
            concurrency: 4,
            ..Default::default()
        };
        assert!(config.validate().is_ok());

        let invalid_config = BfsConfig {
            concurrency: 0,
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_bfs_focused_macro_integration() {
        let spec = BFSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.name(), "bfs");
        assert_eq!(spec.graph_name(), "test_graph");

        // Test that the algorithm can be created
        assert_eq!(spec.graph_name(), "test_graph");
    }
}
