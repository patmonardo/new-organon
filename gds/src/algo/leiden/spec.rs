use serde::{Deserialize, Serialize};

/// Configuration for the Leiden algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeidenConfig {
    /// Resolution parameter (default: 1.0)
    /// Higher values lead to more/smaller communities
    pub gamma: f64,

    /// Randomness parameter for refinement phase (default: 0.01)
    /// Range: [0.0, 1.0]
    /// 0.0 = deterministic, 1.0 = fully random
    pub theta: f64,

    /// Convergence tolerance (default: 0.0001)
    /// Algorithm stops when modularity improvement < tolerance
    pub tolerance: f64,

    /// Maximum number of iterations/levels (default: 10)
    pub max_iterations: usize,

    /// Optional seed for initial community assignments
    /// If None, each node starts in its own community
    pub seed_communities: Option<Vec<u64>>,

    /// Random seed for reproducibility (default: 42)
    pub random_seed: u64,
}

impl Default for LeidenConfig {
    fn default() -> Self {
        Self {
            gamma: 1.0,
            theta: 0.01,
            tolerance: 0.0001,
            max_iterations: 10,
            seed_communities: None,
            random_seed: 42,
        }
    }
}

/// Result of the Leiden algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeidenResult {
    /// Community assignment for each node
    /// communities[node_id] = community_id
    pub communities: Vec<u64>,

    /// Final modularity score
    pub modularity: f64,

    /// Number of levels/iterations executed
    pub levels: usize,

    /// Whether the algorithm converged (true) or hit max_iterations (false)
    pub converged: bool,

    /// Modularity scores at each level (for diagnostics)
    pub modularities: Vec<f64>,

    /// Number of communities found
    pub community_count: u64,
}

impl LeidenResult {
    pub fn new(
        communities: Vec<u64>,
        modularity: f64,
        levels: usize,
        converged: bool,
        modularities: Vec<f64>,
    ) -> Self {
        let community_count = communities.iter().copied().max().unwrap_or(0) + 1;

        Self {
            communities,
            modularity,
            levels,
            converged,
            modularities,
            community_count,
        }
    }
}
