use crate::config::validation::ConfigError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModularityConfig {
    #[serde(default, rename = "communityProperty")]
    pub community_property: String,
}

impl Default for ModularityConfig {
    fn default() -> Self {
        Self {
            community_property: String::new(),
        }
    }
}

impl ModularityConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.community_property.trim().is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "communityProperty".to_string(),
                reason: "community_property cannot be empty".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for ModularityConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        ModularityConfig::validate(self)
    }
}

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
    pub execution_time: Duration,
}

/// Statistics for modularity computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModularityStats {
    pub total_modularity: f64,
    pub community_count: usize,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModularityMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModularityWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for modularity: summary + updated store.
#[derive(Debug, Clone)]
pub struct ModularityMutateResult {
    pub summary: ModularityMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// Modularity result builder (facade adapter).
pub struct ModularityResultBuilder {
    result: ModularityResult,
}

impl ModularityResultBuilder {
    pub fn new(result: ModularityResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> ModularityStats {
        ModularityStats {
            total_modularity: self.result.total_modularity,
            community_count: self.result.community_count,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}
