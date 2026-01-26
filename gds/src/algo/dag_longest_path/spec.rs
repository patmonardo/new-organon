//! DagLongestPath Specification
//!
//! **Translation Source**: `org.neo4j.gds.dag.longestPath.DagLongestPathBaseConfig`

use crate::algo::algorithms::pathfinding::PathResult;
use crate::types::graph::NodeId;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DagLongestPathConfig {
    // No specific configuration needed beyond base algorithm settings
}

/// Result row for longest path streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathRow {
    /// Index of this path
    pub index: u64,
    /// Source node of the path
    pub source_node: NodeId,
    /// Target node of the path
    pub target_node: NodeId,
    /// Total cost of the path
    pub total_cost: f64,
    /// Sequence of node IDs in the path
    pub node_ids: Vec<NodeId>,
    /// Costs at each step in the path
    pub costs: Vec<f64>,
}

/// Result row for longest path streaming (alias for public facade usage)
pub type DagLongestPathRow = PathRow;

/// Result of dag longest path computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagLongestPathResult {
    /// Collection of paths
    pub paths: Vec<PathRow>,
}

/// Statistics for dag longest path computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagLongestPathStats {
    pub path_count: usize,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagLongestPathMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagLongestPathWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for dag longest path: summary + updated store
#[derive(Debug, Clone)]
pub struct DagLongestPathMutateResult {
    pub summary: DagLongestPathMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

fn checked_u64(value: NodeId) -> u64 {
    u64::try_from(value).unwrap_or(0)
}

fn result_paths(result: &DagLongestPathResult) -> Vec<PathResult> {
    result
        .paths
        .iter()
        .map(|row| PathResult {
            source: checked_u64(row.source_node),
            target: checked_u64(row.target_node),
            path: row
                .node_ids
                .iter()
                .copied()
                .filter(|node_id| *node_id >= 0)
                .map(checked_u64)
                .collect(),
            cost: row.total_cost,
        })
        .collect()
}

/// Dag longest path result builder (facade adapter).
pub struct DagLongestPathResultBuilder {
    result: DagLongestPathResult,
    execution_time: Duration,
}

impl DagLongestPathResultBuilder {
    pub fn new(result: DagLongestPathResult, execution_time: Duration) -> Self {
        Self {
            result,
            execution_time,
        }
    }

    pub fn rows(&self) -> Vec<DagLongestPathRow> {
        self.result.paths.clone()
    }

    pub fn stats(&self) -> DagLongestPathStats {
        DagLongestPathStats {
            path_count: self.result.paths.len(),
            execution_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn paths(&self) -> Vec<PathResult> {
        result_paths(&self.result)
    }

    pub fn mutation_summary(
        &self,
        property_name: &str,
        nodes_updated: u64,
    ) -> DagLongestPathMutationSummary {
        DagLongestPathMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn write_summary(
        &self,
        property_name: &str,
        nodes_written: u64,
    ) -> DagLongestPathWriteSummary {
        DagLongestPathWriteSummary {
            nodes_written,
            property_name: property_name.to_string(),
            execution_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.execution_time.as_millis() as u64
    }
}

pub struct DagLongestPathAlgorithmSpec {
    graph_name: String,
}

impl DagLongestPathAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
