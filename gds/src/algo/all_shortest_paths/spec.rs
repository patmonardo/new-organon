//! All Shortest Paths Algorithm Specification
//!
//! This module implements the All Shortest Paths algorithm using focused macros.
//! All Shortest Paths computes shortest paths between all pairs of nodes in a graph.
//!
//! **Algorithm**: Multi-source parallel shortest path computation
//! **Complexity**: O(V²) for unweighted, O(V² log V) for weighted
//! **Use Case**: Network analysis, centrality measures, graph connectivity

use crate::config::validation::ConfigError;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::LogLevel;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::relationship_type::RelationshipType;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use super::AllShortestPathsComputationRuntime;
use super::{AlgorithmType, AllShortestPathsStorageRuntime, ShortestPathResult};

// ============================================================================
// Configuration
// ============================================================================

/// All Shortest Paths Configuration
///
/// Specifies how to compute all shortest paths.
/// **Translation Source**: Java GDS AllShortestPathsParameters + user config
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllShortestPathsConfig {
    /// Algorithm type (weighted vs unweighted)
    pub algorithm_type: AlgorithmType,
    /// Number of parallel workers
    pub concurrency: usize,
    /// Whether to stream results (avoid O(V²) memory)
    pub stream_results: bool,
    /// Maximum number of results to return (for memory control)
    pub max_results: Option<usize>,

    /// Optional relationship types to include (empty means all types)
    #[serde(default)]
    pub relationship_types: Vec<String>,

    /// Direction to traverse edges: "outgoing", "incoming", or "undirected"
    #[serde(default = "AllShortestPathsConfig::default_direction")]
    pub direction: String,

    /// Relationship weight property to use (only relevant for weighted runs)
    #[serde(default = "AllShortestPathsConfig::default_weight_property")]
    pub weight_property: String,
}

impl Default for AllShortestPathsConfig {
    fn default() -> Self {
        Self {
            algorithm_type: AlgorithmType::Unweighted,
            concurrency: num_cpus::get(),
            stream_results: true,
            max_results: None,
            relationship_types: vec![],
            direction: Self::default_direction(),
            weight_property: Self::default_weight_property(),
        }
    }
}

impl AllShortestPathsConfig {
    fn default_direction() -> String {
        "outgoing".to_string()
    }

    fn default_weight_property() -> String {
        "weight".to_string()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate concurrency
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "Concurrency must be greater than 0".to_string(),
            });
        }

        // Validate max_results
        if let Some(max) = self.max_results {
            if max == 0 {
                return Err(ConfigError::InvalidParameter {
                    parameter: "max_results".to_string(),
                    reason: "Max results must be greater than 0".to_string(),
                });
            }
        }

        if self.direction.trim().is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "direction".to_string(),
                reason: "Direction must be non-empty".to_string(),
            });
        }

        if self.weight_property.trim().is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "weight_property".to_string(),
                reason: "Weight property must be non-empty".to_string(),
            });
        }

        Ok(())
    }
}

impl crate::config::ValidatedConfig for AllShortestPathsConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        AllShortestPathsConfig::validate(self)
    }
}

// ============================================================================
// Result Type
// ============================================================================

/// All Shortest Paths computation result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllShortestPathsResult {
    /// All shortest path results
    pub results: Vec<ShortestPathResult>,
    /// Number of nodes processed
    pub node_count: usize,
    /// Maximum distance found
    pub max_distance: f64,
    /// Minimum distance found
    pub min_distance: f64,
    /// Number of infinite distances (unreachable pairs)
    pub infinite_distances: usize,
    /// Execution time
    pub execution_time: Duration,
}

/// Aggregated statistics for an all-shortest-paths run.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllShortestPathsStats {
    pub node_count: u64,
    pub result_count: u64,
    pub max_distance: f64,
    pub min_distance: f64,
    pub infinite_distances: u64,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllShortestPathsMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllShortestPathsWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for AllShortestPaths: summary + updated store.
#[derive(Debug, Clone)]
pub struct AllShortestPathsMutateResult {
    pub summary: AllShortestPathsMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// AllShortestPaths result builder (facade adapter).
pub struct AllShortestPathsResultBuilder {
    result: AllShortestPathsResult,
}

impl AllShortestPathsResultBuilder {
    pub fn new(result: AllShortestPathsResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> AllShortestPathsStats {
        AllShortestPathsStats {
            node_count: self.result.node_count as u64,
            result_count: self.result.results.len() as u64,
            max_distance: self.result.max_distance,
            min_distance: self.result.min_distance,
            infinite_distances: self.result.infinite_distances as u64,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}

// ============================================================================
// Algorithm Specification (Generated Boilerplate + Manual Logic)
// ============================================================================

define_algorithm_spec! {
    name: "all_shortest_paths",
    output_type: AllShortestPathsResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |self, graph_store, config, context| {
        use crate::core::utils::progress::Tasks;
        // Extract configuration
        let parsed_config: AllShortestPathsConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {}", e)))?;

        parsed_config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Config validation failed: {e}")))?;

        let algorithm_type = parsed_config.algorithm_type;
        let concurrency = parsed_config.concurrency;
        let stream_results = parsed_config.stream_results;

        let rel_types: HashSet<RelationshipType> = if parsed_config.relationship_types.is_empty() {
            graph_store.relationship_types()
        } else {
            RelationshipType::list_of(parsed_config.relationship_types.clone())
                .into_iter()
                .collect()
        };

        let (orientation, direction_byte) = match parsed_config.direction.to_ascii_lowercase().as_str() {
            "incoming" => (Orientation::Reverse, 1u8),
            "undirected" => (Orientation::Natural, 2u8),
            _ => (Orientation::Natural, 0u8),
        };

        let selectors: HashMap<RelationshipType, String> = rel_types
            .iter()
            .map(|t| (t.clone(), parsed_config.weight_property.clone()))
            .collect();

            let graph = graph_store
            .get_graph_with_types_selectors_and_orientation(&rel_types, &selectors, orientation)
            .map_err(|e| AlgorithmError::Execution(format!("Failed to obtain graph view: {}", e)))?;

            let node_count = graph.node_count();

        context.log(
            LogLevel::Info,
            &format!(
                "Computing all shortest paths (algorithm={:?}, concurrency={}, streaming={}) on graph with {} nodes",
                algorithm_type,
                concurrency,
                stream_results,
                    node_count
            ),
        );

        // Create storage runtime (Gross pole - knows Graph)
        let storage = AllShortestPathsStorageRuntime::with_settings(
            graph.as_ref(),
            algorithm_type,
            concurrency,
        );

        // Create computation runtime (Subtle pole - knows shortest path results)
        let mut computation = AllShortestPathsComputationRuntime::new();

        let start = std::time::Instant::now();

        if stream_results {
            // Streaming mode: Process results as they come
            let mut progress_tracker = TaskProgressTracker::with_concurrency(
                Tasks::leaf_with_volume("all_shortest_paths".to_string(), node_count),
                concurrency,
            );

            let receiver = storage
                .compute_all_shortest_paths_streaming(
                    &mut computation,
                    direction_byte,
                    &mut progress_tracker,
                )?;

            // Collect results from stream
            for _result in receiver.iter() {
                // Check max_results limit
                if let Some(max) = parsed_config.max_results {
                    if computation.total_results() >= max {
                        break;
                    }
                }
            }
        } else {
            // Non-streaming mode: Process all nodes sequentially
            let node_count = storage.node_count();
            for source_node in 0..node_count as i64 {
                // **FUNCTOR IN ACTION**:
                // Project from Storage (Gross/GraphStore)
                // to Computation (Subtle/shortest path results)
                let source_results = storage.compute_shortest_paths(source_node, direction_byte)?;
                computation.add_shortest_paths(source_results);

                // Check max_results limit
                if let Some(max) = parsed_config.max_results {
                    if computation.total_results() >= max {
                        break;
                    }
                }
            }
        }

        context.log(
            LogLevel::Info,
            &format!(
                "All shortest paths computed: {} nodes, {} results, max_distance={}, min_distance={}, infinite_distances={}",
                computation.node_count(),
                computation.total_results(),
                computation.max_distance(),
                computation.min_distance(),
                computation.infinite_distances()
            ),
        );

        Ok(AllShortestPathsResult {
            results: computation.get_results().clone(),
            node_count: computation.node_count(),
            max_distance: computation.max_distance(),
            min_distance: computation.min_distance(),
            infinite_distances: computation.infinite_distances(),
            execution_time: start.elapsed(),
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::algorithm::{AlgorithmSpec, ProjectionHint};
    use crate::types::graph::NodeId;

    #[test]
    fn test_all_shortest_paths_algorithm_name() {
        let spec = ALL_SHORTEST_PATHSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.name(), "all_shortest_paths");
    }

    #[test]
    fn test_all_shortest_paths_graph_name() {
        let spec = ALL_SHORTEST_PATHSAlgorithmSpec::new("my_graph".to_string());
        assert_eq!(spec.graph_name(), "my_graph");
    }

    #[test]
    fn test_all_shortest_paths_projection_hint() {
        let spec = ALL_SHORTEST_PATHSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.projection_hint(), ProjectionHint::Dense);
    }

    #[test]
    fn test_all_shortest_paths_config_default() {
        let config = AllShortestPathsConfig::default();
        assert_eq!(config.algorithm_type, AlgorithmType::Unweighted);
        assert!(config.concurrency > 0);
        assert!(config.stream_results);
        assert!(config.max_results.is_none());
        assert!(config.relationship_types.is_empty());
        assert_eq!(config.direction, "outgoing");
        assert_eq!(config.weight_property, "weight");
    }

    #[test]
    fn test_all_shortest_paths_config_validation() {
        let config = AllShortestPathsConfig::default();
        assert!(config.validate().is_ok());

        // Test invalid config
        let invalid_config = AllShortestPathsConfig {
            concurrency: 0,
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_all_shortest_paths_computation_runtime() {
        let mut runtime = AllShortestPathsComputationRuntime::new();

        // Add some test results
        runtime.add_result(ShortestPathResult {
            source: 0 as NodeId,
            target: 1 as NodeId,
            distance: 2.0,
        });
        runtime.add_result(ShortestPathResult {
            source: 0 as NodeId,
            target: 2 as NodeId,
            distance: 4.0,
        });
        runtime.add_result(ShortestPathResult {
            source: 1 as NodeId,
            target: 2 as NodeId,
            distance: f64::INFINITY,
        });

        assert_eq!(runtime.total_results(), 3);
        assert_eq!(runtime.max_distance(), 4.0);
        assert_eq!(runtime.min_distance(), 2.0);
        assert_eq!(runtime.infinite_distances(), 1);
        assert_eq!(runtime.finite_distances(), 2);
        assert!(!runtime.is_connected());
    }
}
