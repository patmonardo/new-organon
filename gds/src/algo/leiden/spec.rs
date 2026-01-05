use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeidenConfig {
    /// Resolution parameter. Higher values lead to more (smaller) communities.
    #[serde(default = "default_gamma")]
    pub gamma: f64,

    /// Randomness parameter used in the refinement phase.
    ///
    /// Kept for API parity with the Java implementation.
    #[serde(default = "default_theta")]
    pub theta: f64,

    /// Convergence tolerance on modularity improvement.
    #[serde(default = "default_tolerance")]
    pub tolerance: f64,

    /// Maximum number of Leiden levels.
    #[serde(default = "default_max_iterations")]
    pub max_iterations: usize,

    /// RNG seed for reproducibility.
    #[serde(default = "default_random_seed")]
    pub random_seed: u64,

    /// Optional starting communities for each node.
    ///
    /// If present, must be empty (treated as None) or match `node_count`.
    #[serde(default)]
    pub seed_communities: Option<Vec<u64>>,
}

fn default_gamma() -> f64 {
    1.0
}

fn default_theta() -> f64 {
    0.01
}

fn default_tolerance() -> f64 {
    0.0001
}

fn default_max_iterations() -> usize {
    10
}

fn default_random_seed() -> u64 {
    42
}

impl Default for LeidenConfig {
    fn default() -> Self {
        Self {
            gamma: default_gamma(),
            theta: default_theta(),
            tolerance: default_tolerance(),
            max_iterations: default_max_iterations(),
            random_seed: default_random_seed(),
            seed_communities: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeidenResult {
    /// Community assignment per original node.
    pub communities: Vec<u64>,

    /// Number of distinct communities.
    pub community_count: u64,

    /// Final modularity value.
    pub modularity: f64,

    /// Number of Leiden levels executed.
    pub levels: usize,

    /// Whether convergence was reached within `tolerance`.
    pub converged: bool,
}
