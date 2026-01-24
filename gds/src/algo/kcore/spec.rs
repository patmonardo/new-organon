//! K-Core Decomposition algorithm specification (executor integration)

use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::{TaskProgressTracker, Tasks};
use crate::define_algorithm_spec;
use crate::projection::eval::procedure::*;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

use super::KCoreComputationRuntime;
use super::storage::KCoreStorageRuntime;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KCoreResult {
    pub core_values: Vec<i32>,
    pub degeneracy: i32,
    pub node_count: usize,
    pub execution_time: Duration,
}

define_algorithm_spec! {
    name: "kcore",
    output_type: KCoreResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |_self, graph_store, config, _context| {
        let parsed: KCoreConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;

        if parsed.concurrency == 0 {
            return Err(AlgorithmError::Execution("concurrency must be > 0".into()));
        }

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
