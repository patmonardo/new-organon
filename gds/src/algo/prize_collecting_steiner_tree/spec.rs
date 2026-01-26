use crate::algo::algorithms::pathfinding::PathResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

/// Configuration for Prize-Collecting Steiner Tree algorithm
///
/// PCST balances edge costs against node prizes - nodes with higher prizes
/// are more valuable to include in the tree, but connecting them has edge costs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PCSTreeConfig {
    /// Prize value for each node (higher prize = more valuable to include)
    /// Nodes with prize 0.0 may be excluded from final tree
    pub prizes: Vec<f64>,

    /// Optional relationship weight property name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_weight_property: Option<String>,
}

/// Result of Prize-Collecting Steiner Tree computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PCSTreeResult {
    /// Parent node for each node in tree (-1 for root, -2 for pruned)
    pub parent_array: Vec<i64>,

    /// Cost of edge to parent for each node
    pub relationship_to_parent_cost: Vec<f64>,

    /// Total cost of edges in tree
    pub total_edge_cost: f64,

    /// Total prize collected from included nodes
    pub total_prize: f64,

    /// Net value (total_prize - total_edge_cost)
    pub net_value: f64,

    /// Number of nodes included in tree
    pub effective_node_count: u64,
}

/// Constants for parent array encoding
pub const ROOT_NODE: i64 = -1;
pub const PRUNED: i64 = -2;

/// Result row for Prize-Collecting Steiner Tree stream mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PCSTreeRow {
    pub node: u64,
    pub parent: Option<u64>,
    pub cost_to_parent: f64,
}

/// Statistics for Prize-Collecting Steiner Tree computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PCSTreeStats {
    pub node_count: usize,
    pub total_prize: f64,
    pub total_cost: f64,
    pub net_value: f64,
    pub computation_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PCSTreeMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PCSTreeWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for Prize-Collecting Steiner Tree: summary + updated store
#[derive(Debug, Clone)]
pub struct PCSTreeMutateResult {
    pub summary: PCSTreeMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

fn result_rows(result: &PCSTreeResult) -> Vec<PCSTreeRow> {
    result
        .parent_array
        .iter()
        .enumerate()
        .filter_map(|(node_id, &parent)| {
            if parent >= 0 {
                Some(PCSTreeRow {
                    node: node_id as u64,
                    parent: Some(parent as u64),
                    cost_to_parent: result
                        .relationship_to_parent_cost
                        .get(node_id)
                        .copied()
                        .unwrap_or(0.0),
                })
            } else if parent == ROOT_NODE {
                Some(PCSTreeRow {
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

fn result_paths(result: &PCSTreeResult) -> Vec<PathResult> {
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

/// Prize-Collecting Steiner Tree result builder (facade adapter).
pub struct PCSTreeResultBuilder {
    result: PCSTreeResult,
    execution_time: Duration,
}

impl PCSTreeResultBuilder {
    pub fn new(result: PCSTreeResult, execution_time: Duration) -> Self {
        Self {
            result,
            execution_time,
        }
    }

    pub fn rows(&self) -> Vec<PCSTreeRow> {
        result_rows(&self.result)
    }

    pub fn stats(&self) -> PCSTreeStats {
        PCSTreeStats {
            node_count: self.result.effective_node_count as usize,
            total_prize: self.result.total_prize,
            total_cost: self.result.total_edge_cost,
            net_value: self.result.net_value,
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
    ) -> PCSTreeMutationSummary {
        PCSTreeMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn write_summary(&self, property_name: &str, nodes_written: u64) -> PCSTreeWriteSummary {
        PCSTreeWriteSummary {
            nodes_written,
            property_name: property_name.to_string(),
            execution_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.execution_time.as_millis() as u64
    }
}
