use crate::procedures::prize_collecting_steiner_tree::spec::{PRUNED, PCSTreeResult};

/// Storage for Prize-Collecting Steiner Tree computation
#[derive(Debug)]
pub struct PCSTreeStorage {
    /// Parent of each node in tree
    pub parent: Vec<i64>,

    /// Cost of edge to parent
    pub parent_cost: Vec<f64>,

    /// Total edge cost
    pub total_edge_cost: f64,

    /// Total prize collected
    pub total_prize: f64,

    /// Number of nodes in tree
    pub effective_node_count: u64,
}

impl PCSTreeStorage {
    pub fn new(node_count: usize) -> Self {
        Self {
            parent: vec![PRUNED; node_count],
            parent_cost: vec![0.0; node_count],
            total_edge_cost: 0.0,
            total_prize: 0.0,
            effective_node_count: 0,
        }
    }

    /// Convert storage to result
    pub fn into_result(self) -> PCSTreeResult {
        let net_value = self.total_prize - self.total_edge_cost;

        PCSTreeResult {
            parent_array: self.parent,
            relationship_to_parent_cost: self.parent_cost,
            total_edge_cost: self.total_edge_cost,
            total_prize: self.total_prize,
            net_value,
            effective_node_count: self.effective_node_count,
        }
    }
}
