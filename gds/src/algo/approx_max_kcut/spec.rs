//! ApproxMaxKCut algorithm specification.

use crate::config::validation::ConfigError;
use std::sync::Arc;
use std::time::Duration;

/// Configuration for approx max k-cut computation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApproxMaxKCutConfig {
    #[serde(default = "default_k")]
    pub k: u8,
    #[serde(default = "default_iterations")]
    pub iterations: usize,
    #[serde(default = "default_random_seed")]
    pub random_seed: u64,
    #[serde(default = "default_minimize")]
    pub minimize: bool,
    #[serde(default = "default_has_relationship_weight_property")]
    pub has_relationship_weight_property: bool,
    #[serde(default = "default_min_community_sizes")]
    pub min_community_sizes: Vec<usize>,
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

fn default_k() -> u8 {
    2
}

fn default_iterations() -> usize {
    8
}

fn default_random_seed() -> u64 {
    0
}

fn default_minimize() -> bool {
    false
}

fn default_has_relationship_weight_property() -> bool {
    false
}

fn default_min_community_sizes() -> Vec<usize> {
    vec![0, 0]
}

fn default_concurrency() -> usize {
    4
}

impl Default for ApproxMaxKCutConfig {
    fn default() -> Self {
        Self {
            k: default_k(),
            iterations: default_iterations(),
            random_seed: default_random_seed(),
            minimize: default_minimize(),
            has_relationship_weight_property: default_has_relationship_weight_property(),
            min_community_sizes: default_min_community_sizes(),
            concurrency: default_concurrency(),
        }
    }
}

impl ApproxMaxKCutConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.k < 2 || self.k > 127 {
            return Err(ConfigError::InvalidParameter {
                parameter: "k".to_string(),
                reason: "k must be between 2 and 127".to_string(),
            });
        }
        if self.iterations == 0 || self.iterations > 1000 {
            return Err(ConfigError::InvalidParameter {
                parameter: "iterations".to_string(),
                reason: "iterations must be between 1 and 1000".to_string(),
            });
        }
        if self.concurrency == 0 || self.concurrency > 1024 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be between 1 and 1024".to_string(),
            });
        }
        if self.min_community_sizes.len() != self.k as usize {
            return Err(ConfigError::InvalidParameter {
                parameter: "minCommunitySizes".to_string(),
                reason: format!(
                    "min_community_sizes length ({}) must equal k ({})",
                    self.min_community_sizes.len(),
                    self.k
                ),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for ApproxMaxKCutConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        ApproxMaxKCutConfig::validate(self)
    }
}

/// Result for approx max k-cut computation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApproxMaxKCutResult {
    pub communities: Vec<u8>,
    pub cut_cost: f64,
    pub k: u8,
    pub node_count: usize,
    pub execution_time: Duration,
}

/// Statistics for approx max k-cut computation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApproxMaxKCutStats {
    pub cut_cost: f64,
    pub k: u8,
    pub node_count: usize,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApproxMaxKCutMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApproxMaxKCutWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for ApproxMaxKCut: summary + updated store
#[derive(Debug, Clone)]
pub struct ApproxMaxKCutMutateResult {
    pub summary: ApproxMaxKCutMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// ApproxMaxKCut result builder (facade adapter).
pub struct ApproxMaxKCutResultBuilder {
    result: ApproxMaxKCutResult,
}

impl ApproxMaxKCutResultBuilder {
    pub fn new(result: ApproxMaxKCutResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> ApproxMaxKCutStats {
        ApproxMaxKCutStats {
            cut_cost: self.result.cut_cost,
            k: self.result.k,
            node_count: self.result.node_count,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}
