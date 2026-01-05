//! SCC Algorithm Specification (executor integration)

use crate::core::utils::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::eval::procedure::AlgorithmError;
use serde::{Deserialize, Serialize};

use super::computation::SccComputationRuntime;
use super::storage::SccStorageRuntime;

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

        if parsed_config.concurrency == 0 {
            return Err(AlgorithmError::Execution("concurrency must be > 0".into()));
        }

        let storage = SccStorageRuntime::new(parsed_config.concurrency);
        let mut computation = SccComputationRuntime::new();

        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("scc".to_string(), graph_store.node_count()),
            parsed_config.concurrency,
        );
        let termination_flag = crate::concurrency::TerminationFlag::default();

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
