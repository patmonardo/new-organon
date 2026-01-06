//! TopologicalSort Specification
//!
//! **Translation Source**: `org.neo4j.gds.dag.topologicalsort.TopologicalSortBaseConfig`

use serde::{Deserialize, Serialize};
use crate::types::graph::id_map::NodeId;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TopologicalSortConfig {
    /// Whether to compute maximum distance from source nodes
    #[serde(default = "default_compute_max_distance")]
    pub compute_max_distance_from_source: bool,
}

fn default_compute_max_distance() -> bool {
    false
}

/// Result of topological sort computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologicalSortResult {
    /// Sorted nodes in topological order
    pub sorted_nodes: Vec<NodeId>,
    /// Optional maximum distance from source for each node
    pub max_source_distances: Option<Vec<f64>>,
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
