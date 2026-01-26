//! KSpanningTree Specification
//!
//! **Translation Source**: `org.neo4j.gds.kspanningtree.KSpanningTreeBaseConfig`

use crate::algo::algorithms::pathfinding::PathResult;
use crate::config::validation::ConfigError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpanningTreeConfig {
    /// Source node to start spanning tree from
    pub source_node: u64,
    /// Number of spanning trees to create (k)
    pub k: u64,
    /// Objective: "min" for minimum spanning tree, "max" for maximum
    pub objective: String,
    /// Optional relationship weight property
    #[serde(default = "default_weight_property")]
    pub weight_property: Option<String>,
}

fn default_weight_property() -> Option<String> {
    None
}

impl Default for KSpanningTreeConfig {
    fn default() -> Self {
        Self {
            source_node: 0,
            k: 1,
            objective: "min".to_string(),
            weight_property: None,
        }
    }
}

impl KSpanningTreeConfig {
    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.k == 0 {
            return Err(ConfigError::MustBePositive {
                name: "k".to_string(),
                value: self.k as f64,
            });
        }

        if self.objective != "min" && self.objective != "max" {
            return Err(ConfigError::InvalidParameter {
                parameter: "objective".to_string(),
                reason: format!(
                    "objective must be 'min' or 'max' (got '{}')",
                    self.objective
                ),
            });
        }

        if let Some(weight_property) = &self.weight_property {
            if weight_property.trim().is_empty() {
                return Err(ConfigError::InvalidParameter {
                    parameter: "weight_property".to_string(),
                    reason: "weight_property must be non-empty".to_string(),
                });
            }
        }

        Ok(())
    }
}

impl crate::config::ValidatedConfig for KSpanningTreeConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        KSpanningTreeConfig::validate(self)
    }
}

/// Result of k-spanning tree computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpanningTreeResult {
    /// Parent node for each node (-1 if no parent or not in tree)
    pub parent: Vec<i64>,
    /// Cost to reach parent (-1.0 if not in tree)
    pub cost_to_parent: Vec<f64>,
    /// Total weight of the spanning tree
    pub total_cost: f64,
    /// Root node of the tree
    pub root: u64,
    /// Effective node count (how many nodes are in the tree)
    pub node_count: usize,
}

/// Result row for k-spanning tree stream mode
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KSpanningTreeRow {
    pub node_id: u64,
    pub parent_id: i64,
    pub cost: f64,
}

/// Statistics for k-spanning tree computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpanningTreeStats {
    pub node_count: usize,
    pub total_cost: f64,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpanningTreeMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpanningTreeWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for k-spanning tree: summary + updated store
#[derive(Debug, Clone)]
pub struct KSpanningTreeMutateResult {
    pub summary: KSpanningTreeMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

fn result_rows(result: &KSpanningTreeResult) -> Vec<KSpanningTreeRow> {
    result
        .parent
        .iter()
        .enumerate()
        .filter_map(|(node_id, &parent_id)| {
            if parent_id != -1 || node_id == result.root as usize {
                Some(KSpanningTreeRow {
                    node_id: node_id as u64,
                    parent_id,
                    cost: result.cost_to_parent.get(node_id).copied().unwrap_or(0.0),
                })
            } else {
                None
            }
        })
        .collect()
}

fn result_paths(result: &KSpanningTreeResult) -> Vec<PathResult> {
    let mut paths: Vec<PathResult> = Vec::new();

    for (node_id, &parent_id) in result.parent.iter().enumerate() {
        if parent_id >= 0 {
            let parent_u64 = parent_id as u64;
            let node_u64 = node_id as u64;
            paths.push(PathResult {
                source: parent_u64,
                target: node_u64,
                path: vec![parent_u64, node_u64],
                cost: result.cost_to_parent.get(node_id).copied().unwrap_or(0.0),
            });
        }
    }

    paths
}

/// K-spanning tree result builder (facade adapter).
pub struct KSpanningTreeResultBuilder {
    result: KSpanningTreeResult,
    execution_time: Duration,
}

impl KSpanningTreeResultBuilder {
    pub fn new(result: KSpanningTreeResult, execution_time: Duration) -> Self {
        Self {
            result,
            execution_time,
        }
    }

    pub fn rows(&self) -> Vec<KSpanningTreeRow> {
        result_rows(&self.result)
    }

    pub fn stats(&self) -> KSpanningTreeStats {
        let node_count = self
            .result
            .parent
            .iter()
            .enumerate()
            .filter(|(idx, &p)| p != -1 || *idx == self.result.root as usize)
            .count();

        KSpanningTreeStats {
            node_count,
            total_cost: self.result.total_cost,
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
    ) -> KSpanningTreeMutationSummary {
        KSpanningTreeMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn write_summary(
        &self,
        property_name: &str,
        nodes_written: u64,
    ) -> KSpanningTreeWriteSummary {
        KSpanningTreeWriteSummary {
            nodes_written,
            property_name: property_name.to_string(),
            execution_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.execution_time.as_millis() as u64
    }
}

pub struct KSpanningTreeAlgorithmSpec {
    graph_name: String,
}

impl KSpanningTreeAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
