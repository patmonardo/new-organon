//! SCC Algorithm Specification (executor integration)

use crate::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

use super::SccStorageRuntime;
use super::SccComputationRuntime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SccConfig {
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

fn default_concurrency() -> usize {
    4
}

impl Default for SccConfig {
    fn default() -> Self {
        Self {
            concurrency: default_concurrency(),
        }
    }
}

impl SccConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be > 0".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for SccConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        SccConfig::validate(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SccResult {
    pub components: Vec<u64>,
    pub component_count: usize,
    pub computation_time_ms: u64,
    pub node_count: usize,
    pub execution_time: Duration,
}

impl SccResult {
    pub fn new(components: Vec<u64>, component_count: usize, computation_time_ms: u64) -> Self {
        Self {
            components,
            component_count,
            computation_time_ms,
            node_count: 0,
            execution_time: Duration::default(),
        }
    }
}

/// Aggregated SCC stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SccStats {
    pub component_count: usize,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SccMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SccWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for SCC: summary + updated store
#[derive(Debug, Clone)]
pub struct SccMutateResult {
    pub summary: SccMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// SCC result builder (facade adapter).
pub struct SccResultBuilder {
    result: SccResult,
}

impl SccResultBuilder {
    pub fn new(result: SccResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> SccStats {
        SccStats {
            component_count: self.result.component_count,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}

define_algorithm_spec! {
    name: "scc",
    output_type: SccResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |_self, graph_store, config_input, _context| {
        let parsed_config: SccConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to parse config: {}", e)))?;

        parsed_config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        let storage = SccStorageRuntime::new(parsed_config.concurrency);
        let mut computation = SccComputationRuntime::new();
        let start = std::time::Instant::now();

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("scc".to_string(), graph_store.node_count()),
            parsed_config.concurrency,
        );
        let termination_flag = TerminationFlag::default();

        let result = storage
            .compute_scc(
                &mut computation,
                graph_store,
                &mut progress_tracker,
                &termination_flag,
            )
            .map_err(AlgorithmError::Execution)?;

        let mut out = SccResult::new(
            result.components,
            result.component_count,
            result.computation_time_ms,
        );
        out.node_count = graph_store.node_count();
        out.execution_time = start.elapsed();
        Ok(out)
    }
}
