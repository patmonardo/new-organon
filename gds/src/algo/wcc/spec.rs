//! WCC Algorithm Specification (executor integration)

use super::storage::WccStorageRuntime;
use super::WccComputationRuntime;
use crate::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::eval::procedure::AlgorithmError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WccConfig {
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,

    /// Optional threshold over relationship property values.
    /// Only relationships with `property > threshold` are considered.
    #[serde(default)]
    pub threshold: Option<f64>,
}

fn default_concurrency() -> usize {
    4
}

impl Default for WccConfig {
    fn default() -> Self {
        Self {
            concurrency: default_concurrency(),
            threshold: None,
        }
    }
}

impl WccConfig {
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

impl crate::config::ValidatedConfig for WccConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        WccConfig::validate(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WccResult {
    pub components: Vec<u64>,
    pub component_count: usize,
}

define_algorithm_spec! {
    name: "wcc",
    output_type: WccResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |_self, graph_store, config_input, _context| {
        let parsed_config: WccConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to parse config: {}", e)))?;

        parsed_config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        let storage = WccStorageRuntime::new(parsed_config.concurrency);
        let mut computation = WccComputationRuntime::new()
            .concurrency(parsed_config.concurrency)
            .threshold(parsed_config.threshold);

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("wcc".to_string(), graph_store.relationship_count()),
            parsed_config.concurrency,
        );
        let termination_flag = TerminationFlag::default();

        let result = storage
            .compute_wcc(
                &mut computation,
                graph_store,
                &mut progress_tracker,
                &termination_flag,
            )
            .map_err(AlgorithmError::Execution)?;

        Ok(WccResult {
            components: result.components,
            component_count: result.component_count,
        })
    }
}
