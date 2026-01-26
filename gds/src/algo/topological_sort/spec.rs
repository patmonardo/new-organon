//! TopologicalSort Specification
//!
//! **Translation Source**: `org.neo4j.gds.dag.topologicalsort.TopologicalSortBaseConfig`

use crate::algo::algorithms::pathfinding::PathResult;
use crate::config::validation::ConfigError;
use crate::types::graph::NodeId;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologicalSortConfig {
    /// Whether to compute maximum distance from source nodes
    #[serde(default = "default_compute_max_distance")]
    pub compute_max_distance_from_source: bool,
    /// Concurrency level
    pub concurrency: usize,
}

fn default_compute_max_distance() -> bool {
    false
}

impl Default for TopologicalSortConfig {
    fn default() -> Self {
        Self {
            compute_max_distance_from_source: default_compute_max_distance(),
            concurrency: 4,
        }
    }
}

impl TopologicalSortConfig {
    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.concurrency == 0 {
            return Err(ConfigError::MustBePositive {
                name: "concurrency".to_string(),
                value: self.concurrency as f64,
            });
        }

        Ok(())
    }
}

impl crate::config::ValidatedConfig for TopologicalSortConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        TopologicalSortConfig::validate(self)
    }
}

/// Result of topological sort computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologicalSortResult {
    /// Sorted nodes in topological order
    pub sorted_nodes: Vec<NodeId>,
    /// Optional maximum distance from source for each node
    pub max_source_distances: Option<Vec<f64>>,
}

/// Result row for topological sort stream mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologicalSortRow {
    pub node_id: NodeId,
    pub max_distance: Option<f64>,
}

/// Statistics for topological sort computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologicalSortStats {
    pub node_count: usize,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologicalSortMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologicalSortWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for topological sort: summary + updated store
#[derive(Debug, Clone)]
pub struct TopologicalSortMutateResult {
    pub summary: TopologicalSortMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

fn checked_u64(value: NodeId) -> u64 {
    u64::try_from(value).unwrap_or(0)
}

fn result_paths(result: &TopologicalSortResult) -> Vec<PathResult> {
    let mut paths: Vec<PathResult> = Vec::new();
    let mut prev: Option<u64> = None;

    for (idx, node_id) in result.sorted_nodes.iter().enumerate() {
        let node_u64 = checked_u64(*node_id);
        if let Some(prev_id) = prev {
            let cost = result
                .max_source_distances
                .as_ref()
                .and_then(|d| d.get(*node_id as usize).copied())
                .unwrap_or(idx as f64);
            paths.push(PathResult {
                source: prev_id,
                target: node_u64,
                path: vec![prev_id, node_u64],
                cost,
            });
        }
        prev = Some(node_u64);
    }

    paths
}

/// Topological sort result builder (facade adapter).
pub struct TopologicalSortResultBuilder {
    result: TopologicalSortResult,
    execution_time: Duration,
}

impl TopologicalSortResultBuilder {
    pub fn new(result: TopologicalSortResult, execution_time: Duration) -> Self {
        Self {
            result,
            execution_time,
        }
    }

    pub fn rows(&self) -> Vec<TopologicalSortRow> {
        self.result
            .sorted_nodes
            .iter()
            .copied()
            .map(|node_id| TopologicalSortRow {
                node_id,
                max_distance: self
                    .result
                    .max_source_distances
                    .as_ref()
                    .and_then(|d| d.get(node_id as usize).copied()),
            })
            .collect()
    }

    pub fn stats(&self) -> TopologicalSortStats {
        TopologicalSortStats {
            node_count: self.result.sorted_nodes.len(),
            execution_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn paths(&self) -> Vec<PathResult> {
        result_paths(&self.result)
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.execution_time.as_millis() as u64
    }
}

pub struct TopologicalSortAlgorithmSpec {
    graph_name: String,
}

impl TopologicalSortAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
