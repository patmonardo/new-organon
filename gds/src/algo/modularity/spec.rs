use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CommunityModularity {
    pub community_id: u64,
    pub modularity: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModularityResult {
    pub node_count: usize,
    /// Sum of all observed relationship weights across node adjacency (i.e. $2m$ for undirected graphs).
    pub total_relationship_weight: f64,
    pub total_modularity: f64,
    pub community_count: usize,
    pub community_modularities: Vec<CommunityModularity>,
}
