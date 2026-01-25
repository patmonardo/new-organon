//! SCC Algorithm Specification (executor integration)

use crate::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use serde::{Deserialize, Serialize};

use super::storage::SccStorageRuntime;
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
}

impl SccResult {
    pub fn new(components: Vec<u64>, component_count: usize, computation_time_ms: u64) -> Self {
        Self {
            components,
            component_count,
            computation_time_ms,
        }
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

        Ok(SccResult::new(
            result.components,
            result.component_count,
            result.computation_time_ms,
        ))
    }
}
