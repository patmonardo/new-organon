//! K1-Coloring algorithm specification

use crate::algo::k1coloring::storage::K1ColoringStorageRuntime;
use crate::algo::k1coloring::K1ColoringComputationRuntime;
use crate::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::core::utils::partition::DEFAULT_BATCH_SIZE;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::eval::procedure::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct K1ColoringConfig {
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,

    #[serde(default = "default_max_iterations", rename = "maxIterations")]
    pub max_iterations: u64,

    #[serde(default = "default_min_batch_size", rename = "minBatchSize")]
    pub min_batch_size: usize,
}

fn default_concurrency() -> usize {
    4
}

fn default_max_iterations() -> u64 {
    10
}

fn default_min_batch_size() -> usize {
    DEFAULT_BATCH_SIZE
}

impl Default for K1ColoringConfig {
    fn default() -> Self {
        Self {
            concurrency: default_concurrency(),
            max_iterations: default_max_iterations(),
            min_batch_size: default_min_batch_size(),
        }
    }
}

impl K1ColoringConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be positive".to_string(),
            });
        }
        if self.max_iterations == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "maxIterations".to_string(),
                reason: "Must iterate at least 1 time".to_string(),
            });
        }
        if self.min_batch_size == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "minBatchSize".to_string(),
                reason: "minBatchSize must be positive".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for K1ColoringConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        K1ColoringConfig::validate(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct K1ColoringResult {
    pub colors: Vec<u64>,
    pub ran_iterations: u64,
    pub did_converge: bool,
}

define_algorithm_spec! {
    name: "k1coloring",
    output_type: K1ColoringResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |_self, graph_store, config, _context| {
        let parsed: K1ColoringConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        parsed
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        let _start = Instant::now();

        let storage = K1ColoringStorageRuntime::new(graph_store)?;
        let node_count = storage.node_count();

        let task = Tasks::leaf_with_volume("k1coloring".to_string(), parsed.max_iterations as usize);

        let mut progress = TaskProgressTracker::with_concurrency(task, parsed.concurrency);
        let termination_flag = TerminationFlag::default();

        let mut runtime = K1ColoringComputationRuntime::new(node_count as usize, parsed.max_iterations)
            .concurrency(parsed.concurrency);

        storage.compute_k1coloring(
            &mut runtime,
            &parsed,
            &mut progress,
            &termination_flag,
        )
    }
}
