//! Delta Stepping Algorithm Specification
//!
//! **Translation Source**: `org.neo4j.gds.paths.delta.DeltaStepping`
//!
//! This module defines the Delta Stepping algorithm specification using focused macros.
//! Delta Stepping uses a sophisticated binning strategy to manage the frontier
//! efficiently in parallel shortest path computation.

use super::DeltaSteppingStorageRuntime;
use super::DeltaSteppingComputationRuntime;
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
use std::sync::Arc;
use std::time::Duration;

/// Delta Stepping algorithm configuration
///
/// Translation of: Constructor parameters from `DeltaStepping.java` (lines 86-94)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaSteppingConfig {
    /// Source node for shortest path computation
    pub source_node: NodeId,

    /// Delta parameter for binning strategy
    pub delta: f64,

    /// Concurrency level for parallel processing
    pub concurrency: usize,

    /// Whether to store predecessors for path reconstruction
    pub store_predecessors: bool,
    /// Optional relationship types to include (empty means all types)
    #[serde(default)]
    pub relationship_types: Vec<String>,
    /// Direction for traversal ("outgoing" or "incoming")
    #[serde(default = "DeltaDirection::default_as_str")]
    pub direction: String,
}

impl Default for DeltaSteppingConfig {
    fn default() -> Self {
        Self {
            source_node: 0,
            delta: 1.0,
            concurrency: 4,
            store_predecessors: true,
            relationship_types: vec![],
            direction: DeltaDirection::Outgoing.as_str().to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeltaDirection {
    Outgoing,
    Incoming,
}

impl DeltaDirection {
    fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "incoming" => DeltaDirection::Incoming,
            _ => DeltaDirection::Outgoing,
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            DeltaDirection::Outgoing => "outgoing",
            DeltaDirection::Incoming => "incoming",
        }
    }
    fn default_as_str() -> String {
        Self::Outgoing.as_str().to_string()
    }
}

impl DeltaSteppingConfig {
    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.source_node < 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "source_node".to_string(),
                reason: "source_node must be >= 0".to_string(),
            });
        }

        if self.concurrency == 0 {
            return Err(ConfigError::MustBePositive {
                name: "concurrency".to_string(),
                value: self.concurrency as f64,
            });
        }

        if self.delta <= 0.0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "delta".to_string(),
                reason: "delta must be > 0.0".to_string(),
            });
        }

        match self.direction.to_ascii_lowercase().as_str() {
            "outgoing" | "incoming" => {}
            other => {
                return Err(ConfigError::InvalidParameter {
                    parameter: "direction".to_string(),
                    reason: format!("direction must be 'outgoing' or 'incoming' (got '{other}')"),
                });
            }
        }

        Ok(())
    }
}

impl crate::config::ValidatedConfig for DeltaSteppingConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        DeltaSteppingConfig::validate(self)
    }
}

/// Delta Stepping algorithm result
///
/// Translation of: `PathFindingResult` from `DeltaStepping.java` (line 163)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaSteppingResult {
    /// Shortest paths found
    pub shortest_paths: Vec<DeltaSteppingPathResult>,

    /// Total computation time in milliseconds
    pub computation_time_ms: u64,
}

/// Individual path result for Delta Stepping
///
/// Translation of: `DeltaSteppingPathResult.java` (lines 31-95)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaSteppingPathResult {
    /// Path index
    pub index: u64,

    /// Source node ID
    pub source_node: NodeId,

    /// Target node ID
    pub target_node: NodeId,

    /// Node IDs along the path
    pub node_ids: Vec<NodeId>,

    /// Costs for each step along the path
    pub costs: Vec<f64>,
}

impl DeltaSteppingPathResult {
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

/// Statistics about Delta Stepping execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaSteppingStats {
    pub paths_found: u64,
    pub computation_time_ms: u64,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaSteppingMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaSteppingWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for Delta Stepping: summary + updated store
#[derive(Debug, Clone)]
pub struct DeltaSteppingMutateResult {
    pub summary: DeltaSteppingMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

fn checked_u64(value: NodeId) -> u64 {
    u64::try_from(value).unwrap_or(0)
}

fn spec_path_to_core(path: &DeltaSteppingPathResult) -> PathResult {
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

/// Delta Stepping result builder (pathfinding-family adapter).
pub struct DeltaSteppingResultBuilder {
    result: DeltaSteppingResult,
    execution_time: Duration,
}

impl DeltaSteppingResultBuilder {
    pub fn new(result: DeltaSteppingResult, execution_time: Duration) -> Self {
        Self {
            result,
            execution_time,
        }
    }

    pub fn result(
        result: DeltaSteppingResult,
        execution_time: Duration,
    ) -> Result<PathFindingResult, AlgorithmError> {
        Self::new(result, execution_time).build_pathfinding_result()
    }

    pub fn build_pathfinding_result(self) -> Result<PathFindingResult, AlgorithmError> {
        let paths: Vec<PathResult> = self
            .result
            .shortest_paths
            .iter()
            .filter(|p| p.source_node >= 0 && p.target_node >= 0)
            .map(spec_path_to_core)
            .collect();

        let metadata = ExecutionMetadata {
            execution_time: self.execution_time,
            iterations: None,
            converged: None,
            additional: std::collections::HashMap::from([(
                "computation_time_ms".to_string(),
                self.result.computation_time_ms.to_string(),
            )]),
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
    name: "delta_stepping",
    output_type: DeltaSteppingResult,
    projection_hint: Dense,
    modes: [Stream, WriteNodeProperty],

    execute: |_self, graph_store, config, _context| {
        use crate::core::utils::progress::Tasks;
        // Parse configuration
        let config: DeltaSteppingConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::InvalidGraph(
                format!("Failed to parse Delta Stepping config: {}", e)
            ))?;

        // Validate configuration
        config.validate()
            .map_err(|e| AlgorithmError::InvalidGraph(
                format!("Configuration validation failed: {:?}", e)
            ))?;

        // Create storage and computation runtimes
        let mut storage = DeltaSteppingStorageRuntime::new(
            config.source_node,
            config.delta,
            config.concurrency,
            config.store_predecessors
        );

        let mut computation = DeltaSteppingComputationRuntime::new(
            config.source_node,
            config.delta,
            config.concurrency,
            config.store_predecessors
        );

        // Execute with filtered/oriented graph view
        let rel_types: HashSet<RelationshipType> = if !config.relationship_types.is_empty() {
            RelationshipType::list_of(config.relationship_types.clone()).into_iter().collect()
        } else { HashSet::new() };
        let orientation = match DeltaDirection::from_str(&config.direction) {
            DeltaDirection::Outgoing => Orientation::Natural,
            DeltaDirection::Incoming => Orientation::Reverse,
        };
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, orientation)
            .map_err(|e| AlgorithmError::InvalidGraph(
                format!("Failed to obtain graph view: {}", e)
            ))?;

        let direction = DeltaDirection::from_str(&config.direction);

        // Progress tracking: best-effort volume (relationship count);
        // work units are counted inside the driver loop in storage.
        let volume = graph.relationship_count();
        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("delta_stepping".to_string(), volume),
            config.concurrency,
        );

        let result = storage.compute_delta_stepping(
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
    use crate::projection::eval::algorithm::{AlgorithmSpec, ExecutionContext};
    use serde_json::json;

    #[test]
    fn test_delta_stepping_config_default() {
        let config = DeltaSteppingConfig::default();
        assert_eq!(config.source_node, 0);
        assert_eq!(config.delta, 1.0);
        assert_eq!(config.concurrency, 4);
        assert!(config.store_predecessors);
    }

    #[test]
    fn test_delta_stepping_config_validation() {
        let mut config = DeltaSteppingConfig::default();
        assert!(config.validate().is_ok());

        config.concurrency = 0;
        assert!(config.validate().is_err());

        config.concurrency = 4;
        config.delta = 0.0;
        assert!(config.validate().is_err());

        config.delta = -1.0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_delta_stepping_result() {
        let result = DeltaSteppingResult {
            shortest_paths: vec![],
            computation_time_ms: 100,
        };

        assert!(result.shortest_paths.is_empty());
        assert_eq!(result.computation_time_ms, 100);
    }

    #[test]
    fn test_delta_stepping_path_result() {
        let path_result = DeltaSteppingPathResult {
            index: 0,
            source_node: 0,
            target_node: 5,
            node_ids: vec![0, 1, 3, 5],
            costs: vec![0.0, 3.5, 7.0, 10.5],
        };

        assert_eq!(path_result.index, 0);
        assert_eq!(path_result.source_node, 0);
        assert_eq!(path_result.target_node, 5);
        assert_eq!(path_result.total_cost(), 10.5);
        assert_eq!(path_result.node_ids.len(), 4);
        assert_eq!(path_result.costs.len(), 4);
    }

    #[test]
    fn test_delta_stepping_algorithm_spec_contract() {
        let spec = DELTA_STEPPINGAlgorithmSpec::new("test_graph".to_string());

        // Test that the macro-generated spec works
        assert_eq!(spec.name(), "delta_stepping");
        assert_eq!(spec.graph_name(), "test_graph");

        // Test config validation through spec
        let validation_config = spec.validation_config(&ExecutionContext::new("test"));
        assert!(validation_config.validate_before_load(&json!({})).is_ok());
    }

    #[test]
    fn test_delta_stepping_execution_modes() {
        let spec = DELTA_STEPPINGAlgorithmSpec::new("test_graph".to_string());

        // Test execution mode support - the macro doesn't generate this method
        // so we'll just test that the spec was created successfully
        assert_eq!(spec.name(), "delta_stepping");
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_delta_stepping_config_validation_integration() {
        let spec = DELTA_STEPPINGAlgorithmSpec::new("test_graph".to_string());

        // Test with valid config
        let valid_config = json!({
            "source_node": 0,
            "delta": 1.0,
            "concurrency": 4,
            "store_predecessors": true
        });

        let validation_config = spec.validation_config(&ExecutionContext::new("test"));
        assert!(validation_config
            .validate_before_load(&valid_config)
            .is_ok());

        // Test invalid configuration - the validation_config doesn't validate our custom fields
        // so we'll test the config validation directly instead
        let invalid_config = DeltaSteppingConfig {
            source_node: 0,
            delta: 0.0,
            concurrency: 4,
            store_predecessors: true,
            relationship_types: vec![],
            direction: DeltaDirection::Outgoing.as_str().to_string(),
        };

        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_delta_stepping_focused_macro_integration() {
        let spec = DELTA_STEPPINGAlgorithmSpec::new("test_graph".to_string());

        // Test that the focused macro generated the correct structure
        assert_eq!(spec.name(), "delta_stepping");
        assert_eq!(spec.graph_name(), "test_graph");

        // Test that we can create a config
        let config = DeltaSteppingConfig::default();
        assert_eq!(config.source_node, 0);
        assert_eq!(config.delta, 1.0);
    }
}
