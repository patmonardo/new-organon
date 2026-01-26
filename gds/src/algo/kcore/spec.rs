//! K-Core Decomposition algorithm specification (executor integration)

use crate::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::core::utils::progress::{TaskProgressTracker, Tasks};
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::KCoreComputationRuntime;
use super::KCoreStorageRuntime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KCoreConfig {
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

fn default_concurrency() -> usize {
    4
}

impl Default for KCoreConfig {
    fn default() -> Self {
        Self {
            concurrency: default_concurrency(),
        }
    }
}

impl KCoreConfig {
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

impl crate::config::ValidatedConfig for KCoreConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        KCoreConfig::validate(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KCoreResult {
    pub core_values: Vec<i32>,
    pub degeneracy: i32,
    pub node_count: usize,
    pub execution_time: Duration,
}

/// Aggregated k-core stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KCoreStats {
    pub degeneracy: i32,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KCoreMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KCoreWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for KCore: summary + updated store.
#[derive(Debug, Clone)]
pub struct KCoreMutateResult {
    pub summary: KCoreMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// KCore result builder (facade adapter).
pub struct KCoreResultBuilder {
    result: KCoreResult,
}

impl KCoreResultBuilder {
    pub fn new(result: KCoreResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> KCoreStats {
        KCoreStats {
            degeneracy: self.result.degeneracy,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}

define_algorithm_spec! {
    name: "kcore",
    output_type: KCoreResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |_self, graph_store, config, _context| {
        let parsed: KCoreConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;

        parsed
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        let start = Instant::now();

        let storage = KCoreStorageRuntime::new(graph_store)?;
        let node_count = storage.node_count();

        let mut progress = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("KCoreDecomposition".to_string(), node_count),
            parsed.concurrency,
        );
        let termination_flag = TerminationFlag::default();

        let mut runtime = KCoreComputationRuntime::new().concurrency(parsed.concurrency);
        let result = storage.compute_kcore(
            &mut runtime,
            &parsed,
            &mut progress,
            &termination_flag,
        )?;

        Ok(KCoreResult {
            core_values: result.core_values,
            degeneracy: result.degeneracy,
            node_count,
            execution_time: start.elapsed(),
        })
    }
}
