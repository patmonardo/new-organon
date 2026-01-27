//! K1-Coloring algorithm specification

use crate::algo::k1coloring::K1ColoringStorageRuntime;
use crate::algo::k1coloring::K1ColoringComputationRuntime;
use crate::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::core::utils::partition::DEFAULT_BATCH_SIZE;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};

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
    pub node_count: usize,
    pub execution_time: Duration,
}

/// Aggregated K1-Coloring stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct K1ColoringStats {
    pub did_converge: bool,
    pub ran_iterations: u64,
    pub color_count: usize,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct K1ColoringMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct K1ColoringWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for K1Coloring: summary + updated store.
#[derive(Debug, Clone)]
pub struct K1ColoringMutateResult {
    pub summary: K1ColoringMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// K1Coloring result builder (facade adapter).
pub struct K1ColoringResultBuilder {
    result: K1ColoringResult,
}

impl K1ColoringResultBuilder {
    pub fn new(result: K1ColoringResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> K1ColoringStats {
        let color_count = self
            .result
            .colors
            .iter()
            .copied()
            .collect::<std::collections::HashSet<u64>>()
            .len();

        K1ColoringStats {
            did_converge: self.result.did_converge,
            ran_iterations: self.result.ran_iterations,
            color_count,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
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

        let start = Instant::now();

        let storage = K1ColoringStorageRuntime::new(graph_store)?;
        let node_count = storage.node_count();

        let task = Tasks::leaf_with_volume("k1coloring".to_string(), parsed.max_iterations as usize);

        let mut progress = TaskProgressTracker::with_concurrency(task, parsed.concurrency);
        let termination_flag = TerminationFlag::default();

        let mut runtime = K1ColoringComputationRuntime::new(node_count as usize, parsed.max_iterations)
            .concurrency(parsed.concurrency);

        let mut result = storage.compute_k1coloring(
            &mut runtime,
            &parsed,
            &mut progress,
            &termination_flag,
        )?;

        result.node_count = storage.node_count();
        result.execution_time = start.elapsed();

        Ok(result)
    }
}
