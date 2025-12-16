//! DagLongestPath Specification
//!
//! **Translation Source**: `org.neo4j.gds.dag.longestPath.DagLongestPathBaseConfig`

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct DagLongestPathConfig {
    // No specific configuration needed beyond base algorithm settings
}


/// Result row for longest path streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathRow {
    /// Index of this path
    pub index: u64,
    /// Source node of the path
    pub source_node: u64,
    /// Target node of the path
    pub target_node: u64,
    /// Total cost of the path
    pub total_cost: f64,
    /// Sequence of node IDs in the path
    pub node_ids: Vec<u64>,
    /// Costs at each step in the path
    pub costs: Vec<f64>,
}

/// Result of dag longest path computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagLongestPathResult {
    /// Collection of paths
    pub paths: Vec<PathRow>,
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
