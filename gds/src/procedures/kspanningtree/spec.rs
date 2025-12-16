//! KSpanningTree Specification
//!
//! **Translation Source**: `org.neo4j.gds.kspanningtree.KSpanningTreeBaseConfig`

use serde::{Deserialize, Serialize};

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
