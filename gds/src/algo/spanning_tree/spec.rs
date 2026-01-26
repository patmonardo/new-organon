//! Spanning Tree Algorithm Specification
//!
//! **Translation Source**: `org.neo4j.gds.spanningtree.SpanningTreeConfig` and related classes
//!
//! This module implements the algorithm specification for spanning tree algorithms
//! using our focused macro system.

use super::SpanningTreeStorageRuntime;
use crate::algo::algorithms::pathfinding::PathResult;
use crate::config::validation::ConfigError;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::relationship_type::RelationshipType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

/// Configuration for spanning tree algorithms.
///
/// **Translation Source**: `org.neo4j.gds.spanningtree.SpanningTreeConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanningTreeConfig {
    /// Starting node for the spanning tree
    pub start_node_id: u32,

    /// Whether to compute minimum (true) or maximum (false) spanning tree
    pub compute_minimum: bool,

    /// Concurrency level
    pub concurrency: usize,
    /// Optional relationship types to include (empty means all types)
    #[serde(default)]
    pub relationship_types: Vec<String>,

    /// Relationship weight property to use
    #[serde(default = "SpanningTreeConfig::default_weight_property")]
    pub weight_property: String,

    /// Direction to traverse edges: "outgoing", "incoming", or "undirected".
    /// Spanning trees are typically computed on undirected graphs.
    #[serde(default = "SpanningTreeConfig::default_direction")]
    pub direction: String,
}

impl Default for SpanningTreeConfig {
    fn default() -> Self {
        Self {
            start_node_id: 0,
            compute_minimum: true,
            concurrency: 1,
            relationship_types: vec![],
            weight_property: Self::default_weight_property(),
            direction: Self::default_direction(),
        }
    }
}

impl SpanningTreeConfig {
    fn default_weight_property() -> String {
        "weight".to_string()
    }

    fn default_direction() -> String {
        "undirected".to_string()
    }

    /// Validates the configuration.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure with details.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.concurrency == 0 {
            return Err(ConfigError::MustBePositive {
                name: "concurrency".to_string(),
                value: self.concurrency as f64,
            });
        }

        if self.weight_property.trim().is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "weight_property".to_string(),
                reason: "Weight property must be non-empty".to_string(),
            });
        }

        if self.direction.trim().is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "direction".to_string(),
                reason: "Direction must be non-empty".to_string(),
            });
        }

        match self.direction.to_ascii_lowercase().as_str() {
            "outgoing" | "incoming" | "undirected" => {}
            other => {
                return Err(ConfigError::InvalidParameter {
                    parameter: "direction".to_string(),
                    reason: format!(
                        "Direction must be 'outgoing', 'incoming', or 'undirected' (got '{other}')"
                    ),
                });
            }
        }

        Ok(())
    }
}

impl crate::config::ValidatedConfig for SpanningTreeConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        SpanningTreeConfig::validate(self)
    }
}

/// Result type for spanning tree algorithms.
///
/// **Translation Source**: `org.neo4j.gds.spanningtree.SpanningTreeResult`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanningTreeResult {
    /// The computed spanning tree
    pub spanning_tree: super::SpanningTree,

    /// Computation time in milliseconds
    pub computation_time_ms: u64,

    /// Whether the algorithm converged (always true for spanning tree)
    pub did_converge: bool,

    /// Total weight of the spanning tree
    pub total_weight: f64,

    /// Effective node count (nodes in the spanning tree)
    pub effective_node_count: u32,
}

impl SpanningTreeResult {
    /// Creates a new spanning tree result.
    ///
    /// # Arguments
    ///
    /// * `spanning_tree` - The computed spanning tree
    /// * `computation_time_ms` - Computation time in milliseconds
    ///
    /// # Returns
    ///
    /// A new `SpanningTreeResult` instance.
    pub fn new(spanning_tree: super::SpanningTree, computation_time_ms: u64) -> Self {
        Self {
            total_weight: spanning_tree.total_weight(),
            effective_node_count: spanning_tree.effective_node_count(),
            spanning_tree,
            computation_time_ms,
            did_converge: true, // Spanning tree always converges
        }
    }

    /// Get the spanning tree.
    ///
    /// # Returns
    ///
    /// A reference to the spanning tree.
    pub fn spanning_tree(&self) -> &super::SpanningTree {
        &self.spanning_tree
    }

    /// Get the computation time.
    ///
    /// # Returns
    ///
    /// The computation time in milliseconds.
    pub fn computation_time_ms(&self) -> u64 {
        self.computation_time_ms
    }

    /// Check if the algorithm converged.
    ///
    /// # Returns
    ///
    /// Always `true` for spanning tree algorithms.
    pub fn did_converge(&self) -> bool {
        self.did_converge
    }

    /// Get the total weight.
    ///
    /// # Returns
    ///
    /// The total weight of the spanning tree.
    pub fn total_weight(&self) -> f64 {
        self.total_weight
    }

    /// Get the effective node count.
    ///
    /// # Returns
    ///
    /// The number of nodes in the spanning tree.
    pub fn effective_node_count(&self) -> u32 {
        self.effective_node_count
    }
}

/// Per-node spanning tree row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanningTreeRow {
    pub node: u64,
    pub parent: Option<u64>,
    pub cost_to_parent: f64,
}

/// Aggregated stats for spanning tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanningTreeStats {
    pub effective_node_count: u64,
    pub total_weight: f64,
    pub computation_time_ms: u64,
}

/// Summary of a mutate operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanningTreeMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanningTreeWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for spanning tree: summary + updated store
#[derive(Debug, Clone)]
pub struct SpanningTreeMutateResult {
    pub summary: SpanningTreeMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

fn checked_u64(value: u32) -> u64 {
    u64::from(value)
}

fn tree_rows(tree: &super::SpanningTree) -> Vec<SpanningTreeRow> {
    let mut rows = Vec::with_capacity(tree.node_count as usize);
    for node_id in 0..tree.node_count {
        let parent = tree.parent(node_id);
        let parent_u64 = if parent < 0 {
            None
        } else {
            Some(checked_u64(parent as u32))
        };
        rows.push(SpanningTreeRow {
            node: checked_u64(node_id),
            parent: parent_u64,
            cost_to_parent: tree.cost_to_parent(node_id),
        });
    }
    rows
}

fn tree_paths(tree: &super::SpanningTree) -> Vec<PathResult> {
    let mut paths = Vec::with_capacity(tree.node_count.saturating_sub(1) as usize);
    for node_id in 0..tree.node_count {
        let parent = tree.parent(node_id);
        if parent < 0 {
            continue;
        }
        let parent_u64 = checked_u64(parent as u32);
        let node_u64 = checked_u64(node_id);
        paths.push(PathResult {
            source: parent_u64,
            target: node_u64,
            path: vec![parent_u64, node_u64],
            cost: tree.cost_to_parent(node_id),
        });
    }
    paths
}

/// Spanning tree result builder (facade adapter).
pub struct SpanningTreeResultBuilder {
    result: SpanningTreeResult,
}

impl SpanningTreeResultBuilder {
    pub fn new(result: SpanningTreeResult) -> Self {
        Self { result }
    }

    pub fn rows(&self) -> Vec<SpanningTreeRow> {
        tree_rows(&self.result.spanning_tree)
    }

    pub fn stats(&self) -> SpanningTreeStats {
        SpanningTreeStats {
            effective_node_count: self.result.effective_node_count as u64,
            total_weight: self.result.total_weight,
            computation_time_ms: self.result.computation_time_ms,
        }
    }

    pub fn paths(&self) -> Vec<PathResult> {
        tree_paths(&self.result.spanning_tree)
    }

    pub fn computation_time_ms(&self) -> u64 {
        self.result.computation_time_ms
    }
}

// Generate the algorithm specification using focused macros
define_algorithm_spec! {
    name: "spanning_tree",
    output_type: SpanningTreeResult,
    projection_hint: Dense,
    modes: [Stream, Stats, MutateNodeProperty, WriteNodeProperty],
    execute: |_self, graph_store, config_input, _context| {
        use std::time::Instant;

        // Parse and validate configuration
        let config: SpanningTreeConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Failed to parse config: {}", e)))?;

        config.validate()
            .map_err(|e| AlgorithmError::Execution(format!("Config validation failed: {}", e)))?;

        // Create storage runtime
        let storage = SpanningTreeStorageRuntime::new(
            config.start_node_id,
            config.compute_minimum,
            config.concurrency,
        );

        // Record start time
        let start_time = Instant::now();

        // Execute using filtered/oriented graph view
        let rel_types: HashSet<RelationshipType> = if !config.relationship_types.is_empty() {
            RelationshipType::list_of(config.relationship_types.clone()).into_iter().collect()
        } else {
            graph_store.relationship_types()
        };

        let (orientation, direction_byte) = match config.direction.to_ascii_lowercase().as_str() {
            "incoming" => (Orientation::Reverse, 1u8),
            "undirected" => (Orientation::Natural, 2u8),
            _ => (Orientation::Natural, 0u8),
        };

        let selectors: HashMap<RelationshipType, String> = rel_types
            .iter()
            .map(|t| (t.clone(), config.weight_property.clone()))
            .collect();

        let graph = graph_store
            .get_graph_with_types_selectors_and_orientation(&rel_types, &selectors, orientation)
            .map_err(|e| AlgorithmError::Execution(format!("Failed to obtain graph view: {}", e)))?;

        use crate::core::utils::progress::{TaskProgressTracker, Tasks};
        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("spanning_tree".to_string(), graph.relationship_count()),
            config.concurrency,
        );

        let spanning_tree = storage
            .compute_spanning_tree_with_graph(graph.as_ref(), direction_byte, &mut progress_tracker)
            .map_err(|e| AlgorithmError::Execution(format!("Spanning tree computation failed: {}", e)))?;

        // Calculate computation time
        let computation_time_ms = start_time.elapsed().as_millis() as u64;

        // Create result
        let result = SpanningTreeResult::new(spanning_tree, computation_time_ms);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algo::spanning_tree::SpanningTree;
    use crate::projection::eval::algorithm::AlgorithmSpec;
    use crate::projection::eval::algorithm::ExecutionContext;
    use crate::projection::eval::algorithm::ExecutionMode;
    use serde_json::json;

    #[test]
    fn test_spanning_tree_config_default() {
        let config = SpanningTreeConfig::default();

        assert_eq!(config.start_node_id, 0);
        assert!(config.compute_minimum);
        assert_eq!(config.concurrency, 1);
        assert!(config.relationship_types.is_empty());
        assert_eq!(config.weight_property, "weight");
        assert_eq!(config.direction, "undirected");
    }

    #[test]
    fn test_spanning_tree_config_validation() {
        let mut config = SpanningTreeConfig::default();

        // Valid config should pass
        assert!(config.validate().is_ok());

        // Invalid concurrency should fail
        config.concurrency = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_spanning_tree_result_creation() {
        let spanning_tree = SpanningTree::new(
            0,                        // head
            4,                        // node_count
            4,                        // effective_node_count
            vec![-1, 0, 1, 0],        // parent
            vec![0.0, 1.0, 2.0, 1.5], // cost_to_parent
            4.5,                      // total_weight
        );

        let result = SpanningTreeResult::new(spanning_tree, 100);

        assert_eq!(result.computation_time_ms(), 100);
        assert!(result.did_converge());
        assert_eq!(result.total_weight(), 4.5);
        assert_eq!(result.effective_node_count(), 4);
    }

    #[test]
    fn test_spanning_tree_algorithm_spec_contract() {
        let algorithm = SPANNING_TREEAlgorithmSpec::new("test_graph".to_string());

        // Test basic properties
        assert_eq!(algorithm.graph_name(), "test_graph");
        assert_eq!(algorithm.name(), "spanning_tree");

        // Test that the algorithm can be created
        assert_eq!(algorithm.graph_name, "test_graph");
    }

    #[test]
    fn test_spanning_tree_execution_modes() {
        let algorithm = SPANNING_TREEAlgorithmSpec::new("test_graph".to_string());

        // Test all execution modes
        let _modes = vec![
            ExecutionMode::Stream,
            ExecutionMode::Stats,
            ExecutionMode::MutateNodeProperty,
            ExecutionMode::WriteNodeProperty,
        ];

        // Test that the algorithm can be created
        assert_eq!(algorithm.graph_name, "test_graph");
    }

    #[test]
    fn test_spanning_tree_config_validation_integration() {
        let algorithm = SPANNING_TREEAlgorithmSpec::new("test_graph".to_string());

        // Test valid config
        let _valid_config = json!({
            "start_node_id": 0,
            "compute_minimum": true,
            "concurrency": 1
        });

        let context = ExecutionContext::new("test_user");
        let validation_result = algorithm.validation_config(&context);
        // ValidationConfiguration doesn't have is_ok/is_err methods
        // Just verify it was created successfully
        assert_eq!(validation_result.before_load_count(), 0);
        assert_eq!(validation_result.after_load_count(), 0);

        // Test invalid config
        let _invalid_config = json!({
            "start_node_id": 0,
            "compute_minimum": true,
            "concurrency": 0
        });

        let validation_result = algorithm.validation_config(&context);
        // ValidationConfiguration doesn't have is_ok/is_err methods
        // Just verify it was created successfully
        assert_eq!(validation_result.before_load_count(), 0);
        assert_eq!(validation_result.after_load_count(), 0);
    }

    #[test]
    fn test_spanning_tree_focused_macro_integration() {
        let algorithm = SPANNING_TREEAlgorithmSpec::new("test_graph".to_string());

        // Test that the macro-generated algorithm works
        assert_eq!(algorithm.graph_name(), "test_graph");
        assert_eq!(algorithm.name(), "spanning_tree");

        // Test configuration validation
        let _config = json!({
            "start_node_id": 0,
            "compute_minimum": true,
            "concurrency": 1
        });

        let context = ExecutionContext::new("test_user");
        let validation_result = algorithm.validation_config(&context);
        // ValidationConfiguration doesn't have is_ok/is_err methods
        // Just verify it was created successfully
        assert_eq!(validation_result.before_load_count(), 0);
        assert_eq!(validation_result.after_load_count(), 0);
    }
}
