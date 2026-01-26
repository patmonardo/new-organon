//! CELF (Cost-Effective Lazy Forward) Algorithm Specification
//!
//! Influence Maximization under the Independent Cascade model.
//! Finds k seed nodes that maximize expected spread via Monte Carlo simulation.

use crate::config::validation::ConfigError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

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

impl CELFConfig {
    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.seed_set_size == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "seed_set_size".to_string(),
                reason: "seed_set_size must be positive".to_string(),
            });
        }
        if self.monte_carlo_simulations == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "monte_carlo_simulations".to_string(),
                reason: "monte_carlo_simulations must be positive".to_string(),
            });
        }
        if !(0.0..=1.0).contains(&self.propagation_probability) {
            return Err(ConfigError::InvalidParameter {
                parameter: "propagation_probability".to_string(),
                reason: "propagation_probability must be in [0, 1]".to_string(),
            });
        }
        if self.batch_size == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "batch_size".to_string(),
                reason: "batch_size must be positive".to_string(),
            });
        }
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be positive".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for CELFConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        CELFConfig::validate(self)
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

/// Result row for CELF stream mode
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CELFRow {
    pub node_id: u64,
    pub spread: f64,
}

/// Statistics for CELF computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CELFStats {
    pub seed_count: usize,
    pub total_spread: f64,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CELFMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CELFWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for CELF: summary + updated store
#[derive(Debug, Clone)]
pub struct CELFMutateResult {
    pub summary: CELFMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

fn result_rows(result: &CELFResult) -> Vec<CELFRow> {
    result
        .seed_set_nodes
        .iter()
        .map(|(node_id, spread)| CELFRow {
            node_id: *node_id,
            spread: *spread,
        })
        .collect()
}

/// CELF result builder (facade adapter).
pub struct CELFResultBuilder {
    result: CELFResult,
    execution_time: Duration,
}

impl CELFResultBuilder {
    pub fn new(result: CELFResult, execution_time: Duration) -> Self {
        Self {
            result,
            execution_time,
        }
    }

    pub fn rows(&self) -> Vec<CELFRow> {
        let mut rows = result_rows(&self.result);
        rows.sort_by(|a, b| {
            b.spread
                .partial_cmp(&a.spread)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.node_id.cmp(&b.node_id))
        });
        rows
    }

    pub fn stats(&self) -> CELFStats {
        CELFStats {
            seed_count: self.result.seed_set_nodes.len(),
            total_spread: self.result.total_spread(),
            execution_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.execution_time.as_millis() as u64
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
