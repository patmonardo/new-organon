//! K1-Coloring algorithm specification

use crate::algo::k1coloring::storage::K1ColoringStorageRuntime;
use crate::algo::k1coloring::K1ColoringComputationRuntime;
use crate::core::utils::progress::{ProgressTracker, Tasks};
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
    crate::core::utils::partition::DEFAULT_BATCH_SIZE
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
    pub fn validate(&self) -> Result<(), crate::config::validation::ConfigError> {
        if self.concurrency == 0 {
            return Err(crate::config::validation::ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be positive".to_string(),
            });
        }
        if self.max_iterations == 0 {
            return Err(crate::config::validation::ConfigError::InvalidParameter {
                parameter: "maxIterations".to_string(),
                reason: "Must iterate at least 1 time".to_string(),
            });
        }
        if self.min_batch_size == 0 {
            return Err(crate::config::validation::ConfigError::InvalidParameter {
                parameter: "minBatchSize".to_string(),
                reason: "minBatchSize must be positive".to_string(),
            });
        }
        Ok(())
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
        let graph = storage.graph();
        let node_count = graph.node_count();

        let mut progress = crate::core::utils::progress::TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("k1coloring".to_string(), parsed.max_iterations as usize),
            parsed.concurrency,
        );
        progress.begin_subtask_with_volume(parsed.max_iterations as usize);

        let fallback = graph.default_property_value();
        let neighbors = |node_idx: usize| -> Vec<usize> {
            graph
                .stream_relationships(node_idx as i64, fallback)
                .map(|cursor| cursor.target_id())
                .filter(|t| *t >= 0)
                .map(|t| t as usize)
                .collect()
        };

        let mut runtime = K1ColoringComputationRuntime::new(node_count as usize, parsed.max_iterations)
            .concurrency(parsed.concurrency);

        let run = runtime.compute(node_count as usize, neighbors);

        progress.log_progress(parsed.max_iterations as usize);
        progress.end_subtask();

        Ok(K1ColoringResult {
            colors: run.colors,
            ran_iterations: run.ran_iterations,
            did_converge: run.did_converge,
        })
    }
}
