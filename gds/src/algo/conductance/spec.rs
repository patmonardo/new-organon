use crate::config::validation::ConfigError;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// Configuration for conductance.
///
/// Java parity reference: `ConductanceParameters` / `ConductanceConfigTransformer`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConductanceConfig {
    /// Concurrency used for relationship counting.
    pub concurrency: usize,

    /// Minimum batch size for degree partitioning.
    pub min_batch_size: usize,

    /// When `true`, relationship weights are taken from the projected graph.
    /// When `false`, every relationship contributes weight `1.0`.
    pub has_relationship_weight_property: bool,

    /// Node property key storing community IDs (non-negative long values).
    pub community_property: String,
}

impl Default for ConductanceConfig {
    fn default() -> Self {
        Self {
            concurrency: 4,
            min_batch_size: 10_000,
            has_relationship_weight_property: false,
            community_property: String::new(),
        }
    }
}

impl ConductanceConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.community_property.is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "communityProperty".to_string(),
                reason: "community_property cannot be empty".to_string(),
            });
        }
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be positive".to_string(),
            });
        }
        if self.min_batch_size == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "minBatchSize".to_string(),
                reason: "min_batch_size must be positive".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for ConductanceConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        ConductanceConfig::validate(self)
    }
}

/// Result of conductance computation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConductanceResult {
    /// Per-community conductance values.
    pub community_conductances: HashMap<u64, f64>,

    /// Global average conductance over communities with counts.
    pub global_average_conductance: f64,

    /// Number of communities evaluated.
    pub community_count: usize,

    /// Number of nodes processed.
    pub node_count: usize,

    /// Execution time for the computation.
    pub execution_time: Duration,
}

/// Statistics for conductance computation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConductanceStats {
    pub community_count: usize,
    pub average_conductance: f64,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConductanceMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConductanceWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for conductance: summary + updated store.
#[derive(Debug, Clone)]
pub struct ConductanceMutateResult {
    pub summary: ConductanceMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// Conductance result builder (facade adapter).
pub struct ConductanceResultBuilder {
    result: ConductanceResult,
}

impl ConductanceResultBuilder {
    pub fn new(result: ConductanceResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> ConductanceStats {
        ConductanceStats {
            community_count: self.result.community_count,
            average_conductance: self.result.global_average_conductance,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}
