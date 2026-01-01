use serde::{Deserialize, Serialize};

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
