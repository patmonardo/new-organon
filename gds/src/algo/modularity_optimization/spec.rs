use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModularityOptimizationConfig {
    pub max_iterations: usize,
    pub tolerance: f64,
    /// Modularity resolution parameter. $\gamma = 1$ is classic modularity.
    pub gamma: f64,
}

impl Default for ModularityOptimizationConfig {
    fn default() -> Self {
        Self {
            max_iterations: 20,
            tolerance: 1e-6,
            gamma: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModularityOptimizationResult {
    pub communities: Vec<u64>,
    pub modularity: f64,
    pub ran_iterations: usize,
    pub did_converge: bool,
}
