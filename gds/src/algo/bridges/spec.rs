//! Bridges Algorithm Specification
//!
//! Java parity reference: `org.neo4j.gds.bridges.Bridges`.

use crate::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::core::utils::progress::{ProgressTracker, TaskProgressTracker, Tasks};
use crate::define_algorithm_spec;
use crate::projection::eval::procedure::*;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::computation::{Bridge, BridgesComputationRuntime};
use super::storage::BridgesStorageRuntime;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BridgesConfig {
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

fn default_concurrency() -> usize {
    4
}

impl Default for BridgesConfig {
    fn default() -> Self {
        Self {
            concurrency: default_concurrency(),
        }
    }
}

impl BridgesConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be positive".to_string(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BridgesResult {
    pub bridges: Vec<Bridge>,
    pub node_count: usize,
    pub execution_time: Duration,
}

define_algorithm_spec! {
    name: "bridges",
    output_type: BridgesResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |self, graph_store, config, context| {
        let parsed_config: BridgesConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {}", e)))?;
        parsed_config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {}", e)))?;

        let start = Instant::now();

        let storage = BridgesStorageRuntime::new(graph_store)?;
        let node_count = storage.node_count();

        context.log(
            LogLevel::Info,
            &format!(
                "Computing bridges (concurrency={}) on graph with {} nodes",
                parsed_config.concurrency,
                node_count
            ),
        );

        let tracker = Arc::new(Mutex::new(TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("bridges".to_string(), node_count),
            parsed_config.concurrency,
        )));
        tracker.lock().unwrap().begin_subtask_with_volume(node_count);

        let on_node_scanned = {
            let tracker = Arc::clone(&tracker);
            Arc::new(move || {
                tracker.lock().unwrap().log_progress(1);
            })
        };

        let termination = TerminationFlag::running_true();

        let mut computation = BridgesComputationRuntime::new_with_stack_capacity(
            storage.node_count(),
            storage.relationship_count(),
        );

        let bridges = storage
            .compute_bridges(&mut computation, &termination, on_node_scanned)
            .map_err(|e| AlgorithmError::Execution(format!("Bridges terminated: {e}")))?
            .bridges;

        tracker.lock().unwrap().end_subtask();

        Ok(BridgesResult {
            bridges,
            node_count,
            execution_time: start.elapsed(),
        })
    }
}
