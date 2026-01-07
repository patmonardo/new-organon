use crate::types::graph::id_map::NodeId;
use serde::{Deserialize, Serialize};

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
