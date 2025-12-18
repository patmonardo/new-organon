use crate::procedures::steiner_tree::spec::{SteinerTreeResult, PRUNED};

/// Storage for Steiner Tree computation
#[derive(Debug)]
pub struct SteinerTreeStorage {
    /// Parent of each node in the tree
    pub parent: Vec<i64>,

    /// Cost of the edge to parent
    pub parent_cost: Vec<f64>,

    /// Total cost of the tree
    pub total_cost: f64,

    /// Number of nodes in the tree (excluding pruned)
    pub effective_node_count: u64,

    /// Number of terminals reached
    pub effective_target_nodes_count: u64,
}

impl SteinerTreeStorage {
    pub fn new(node_count: usize) -> Self {
        Self {
            parent: vec![PRUNED; node_count],
            parent_cost: vec![0.0; node_count],
            total_cost: 0.0,
            effective_node_count: 0,
            effective_target_nodes_count: 0,
        }
    }

    /// Convert storage to result
    pub fn into_result(self) -> SteinerTreeResult {
        SteinerTreeResult {
            parent_array: self.parent,
            relationship_to_parent_cost: self.parent_cost,
            total_cost: self.total_cost,
            effective_node_count: self.effective_node_count,
            effective_target_nodes_count: self.effective_target_nodes_count,
        }
    }
}
