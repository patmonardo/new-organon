//! CELF (Cost-Effective Lazy Forward) Algorithm Specification
//!
//! Influence Maximization under the Independent Cascade model.
//! Finds k seed nodes that maximize expected spread via Monte Carlo simulation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for CELF algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CELFConfig {
    /// Number of seed nodes to select
    pub seed_set_size: usize,
    /// Number of Monte Carlo simulations per evaluation
    pub monte_carlo_simulations: usize,
    /// Edge propagation probability for Independent Cascade model
    pub propagation_probability: f64,
    /// Batch size for lazy forward evaluation (trade-off between accuracy and speed)
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    /// Random seed for reproducibility
    #[serde(default)]
    pub random_seed: u64,
    /// Concurrency level
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

fn default_batch_size() -> usize {
    10
}

fn default_concurrency() -> usize {
    4
}

impl Default for CELFConfig {
    fn default() -> Self {
        Self {
            seed_set_size: 10,
            monte_carlo_simulations: 100,
            propagation_probability: 0.1,
            batch_size: 10,
            random_seed: 42,
            concurrency: 4,
        }
    }
}

/// Result of CELF computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CELFResult {
    /// Map of seed node IDs to their spread values
    pub seed_set_nodes: HashMap<u64, f64>,
}

impl CELFResult {
    /// Compute total spread across all seed nodes
    pub fn total_spread(&self) -> f64 {
        self.seed_set_nodes.values().sum()
    }

    /// Number of seed nodes selected
    pub fn seed_count(&self) -> usize {
        self.seed_set_nodes.len()
    }
}

pub struct CELFAlgorithmSpec {
    graph_name: String,
}

impl CELFAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
