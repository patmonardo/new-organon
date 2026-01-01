//! ApproxMaxKCut specification: configuration and results

/// Configuration for approximate maximum k-cut
#[derive(Clone, Debug)]
pub struct ApproxMaxKCutConfig {
    /// Number of communities to partition into (2..127)
    pub k: u8,
    /// Number of GRASP iterations
    pub iterations: usize,
    /// Random seed for reproducibility
    pub random_seed: u64,
    /// Whether to minimize cut cost (false = maximize, which is typical)
    pub minimize: bool,
    /// Whether to use relationship weights (true) or treat all edges as weight 1.0 (false)
    pub has_relationship_weight_property: bool,
    /// Minimum number of nodes per community (length must equal k)
    pub min_community_sizes: Vec<usize>,
}

impl Default for ApproxMaxKCutConfig {
    fn default() -> Self {
        Self {
            k: 2,
            iterations: 8,
            random_seed: 0,
            minimize: false,
            has_relationship_weight_property: false,
            min_community_sizes: vec![0, 0],
        }
    }
}

/// Result of approximate maximum k-cut computation
#[derive(Clone, Debug)]
pub struct ApproxMaxKCutResult {
    /// Community assignment for each node (values 0..k-1)
    pub communities: Vec<u8>,
    /// Final cut cost (sum of weights of edges crossing communities)
    pub cut_cost: f64,
}
