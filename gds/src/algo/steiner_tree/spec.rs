use crate::algo::algorithms::pathfinding::PathResult;
use crate::types::graph::NodeId;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

/// Configuration for Steiner Tree algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteinerTreeConfig {
    /// Source node from which to start the tree
    pub source_node: NodeId,

    /// Terminal nodes that must be included in the tree
    pub target_nodes: Vec<NodeId>,

    /// Optional relationship weight property
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_weight_property: Option<String>,

    /// Delta parameter for delta-stepping optimization (default: 1.0)
    /// Smaller values: more accurate but slower
    /// Larger values: faster but may miss optimizations
    #[serde(default = "default_delta")]
    pub delta: f64,

    /// Whether to apply rerouting optimization (default: true)
    ///
    /// Note: pruning of non-terminal leaves is always applied; this flag only
    /// controls optional rerouting/post-optimizations.
    #[serde(default = "default_apply_rerouting")]
    pub apply_rerouting: bool,
}

fn default_delta() -> f64 {
    1.0
}

fn default_apply_rerouting() -> bool {
    true
}

/// Result of Steiner Tree computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteinerTreeResult {
    /// Parent node for each node in the tree (-1 for root, -2 for pruned)
    pub parent_array: Vec<i64>,

    /// Cost of edge to parent for each node
    pub relationship_to_parent_cost: Vec<f64>,

    /// Total cost of the Steiner tree
    pub total_cost: f64,

    /// Number of nodes included in the tree
    pub effective_node_count: u64,

    /// Number of terminal nodes reached
    pub effective_target_nodes_count: u64,
}

/// Constants for parent array encoding
pub const ROOT_NODE: i64 = -1;
pub const PRUNED: i64 = -2;

/// Result row for Steiner tree stream mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteinerTreeRow {
    pub node: u64,
    pub parent: Option<u64>,
    pub cost_to_parent: f64,
}

/// Statistics for Steiner tree computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteinerTreeStats {
    pub effective_node_count: u64,
    pub effective_target_nodes_count: u64,
    pub total_cost: f64,
    pub computation_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteinerTreeMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteinerTreeWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for Steiner tree: summary + updated store
#[derive(Debug, Clone)]
pub struct SteinerTreeMutateResult {
    pub summary: SteinerTreeMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

fn result_rows(result: &SteinerTreeResult) -> Vec<SteinerTreeRow> {
    result
        .parent_array
        .iter()
        .enumerate()
        .filter_map(|(node_id, &parent)| {
            if parent >= 0 {
                Some(SteinerTreeRow {
                    node: node_id as u64,
                    parent: Some(parent as u64),
                    cost_to_parent: result
                        .relationship_to_parent_cost
                        .get(node_id)
                        .copied()
                        .unwrap_or(0.0),
                })
            } else if parent == ROOT_NODE {
                Some(SteinerTreeRow {
                    node: node_id as u64,
                    parent: None,
                    cost_to_parent: 0.0,
                })
            } else {
                None
            }
        })
        .collect()
}

fn result_paths(result: &SteinerTreeResult) -> Vec<PathResult> {
    result
        .parent_array
        .iter()
        .enumerate()
        .filter_map(|(node_id, &parent)| {
            if parent >= 0 {
                let parent_u64 = parent as u64;
                let node_u64 = node_id as u64;
                Some(PathResult {
                    source: parent_u64,
                    target: node_u64,
                    path: vec![parent_u64, node_u64],
                    cost: result
                        .relationship_to_parent_cost
                        .get(node_id)
                        .copied()
                        .unwrap_or(0.0),
                })
            } else {
                None
            }
        })
        .collect()
}

/// Steiner tree result builder (facade adapter).
pub struct SteinerTreeResultBuilder {
    result: SteinerTreeResult,
    execution_time: Duration,
}

impl SteinerTreeResultBuilder {
    pub fn new(result: SteinerTreeResult, execution_time: Duration) -> Self {
        Self {
            result,
            execution_time,
        }
    }

    pub fn rows(&self) -> Vec<SteinerTreeRow> {
        result_rows(&self.result)
    }

    pub fn stats(&self) -> SteinerTreeStats {
        SteinerTreeStats {
            effective_node_count: self.result.effective_node_count,
            effective_target_nodes_count: self.result.effective_target_nodes_count,
            total_cost: self.result.total_cost,
            computation_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn paths(&self) -> Vec<PathResult> {
        result_paths(&self.result)
    }

    pub fn mutation_summary(
        &self,
        property_name: &str,
        nodes_updated: u64,
    ) -> SteinerTreeMutationSummary {
        SteinerTreeMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn write_summary(
        &self,
        property_name: &str,
        nodes_written: u64,
    ) -> SteinerTreeWriteSummary {
        SteinerTreeWriteSummary {
            nodes_written,
            property_name: property_name.to_string(),
            execution_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.execution_time.as_millis() as u64
    }
}
