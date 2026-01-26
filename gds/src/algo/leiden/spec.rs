use crate::config::validation::ConfigError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

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

impl LeidenConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if !(0.0..=100.0).contains(&self.gamma) {
            return Err(ConfigError::InvalidParameter {
                parameter: "gamma".to_string(),
                reason: "gamma must be between 0 and 100".to_string(),
            });
        }
        if !(0.0..=1.0).contains(&self.theta) {
            return Err(ConfigError::InvalidParameter {
                parameter: "theta".to_string(),
                reason: "theta must be between 0 and 1".to_string(),
            });
        }
        if !(0.0..=1.0).contains(&self.tolerance) {
            return Err(ConfigError::InvalidParameter {
                parameter: "tolerance".to_string(),
                reason: "tolerance must be between 0 and 1".to_string(),
            });
        }
        if self.max_iterations == 0 || self.max_iterations > 1000 {
            return Err(ConfigError::InvalidParameter {
                parameter: "maxIterations".to_string(),
                reason: "max_iterations must be between 1 and 1000".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for LeidenConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        LeidenConfig::validate(self)
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

    /// Number of nodes processed.
    pub node_count: usize,

    /// Execution time for the computation.
    pub execution_time: Duration,
}

/// Aggregated Leiden stats.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LeidenStats {
    pub community_count: u64,
    pub modularity: f64,
    pub levels: usize,
    pub converged: bool,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeidenMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeidenWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for Leiden: summary + updated store.
#[derive(Debug, Clone)]
pub struct LeidenMutateResult {
    pub summary: LeidenMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// Leiden result builder (facade adapter).
pub struct LeidenResultBuilder {
    result: LeidenResult,
}

impl LeidenResultBuilder {
    pub fn new(result: LeidenResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> LeidenStats {
        LeidenStats {
            community_count: self.result.community_count,
            modularity: self.result.modularity,
            levels: self.result.levels,
            converged: self.result.converged,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}
