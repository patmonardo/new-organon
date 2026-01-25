//! Closeness Centrality Algorithm Specification
//!
//! Java parity reference:
//! - `org.neo4j.gds.closeness.ClosenessCentrality`
//! - `org.neo4j.gds.closeness.ClosenessCentralityAlgorithmFactory` (progress task layout)

use crate::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::core::utils::progress::{ProgressTracker, TaskProgressTracker, Tasks};
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::*;
use crate::projection::Orientation;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::storage::ClosenessCentralityStorageRuntime;
use super::ClosenessCentralityComputationRuntime;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClosenessCentralityConfig {
    #[serde(default)]
    pub wasserman_faust: bool,

    #[serde(default = "default_direction")]
    pub direction: String,

    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

fn default_direction() -> String {
    "both".to_string()
}

fn default_concurrency() -> usize {
    4
}

impl Default for ClosenessCentralityConfig {
    fn default() -> Self {
        Self {
            wasserman_faust: false,
            direction: default_direction(),
            concurrency: default_concurrency(),
        }
    }
}

impl ClosenessCentralityConfig {
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

impl crate::config::ValidatedConfig for ClosenessCentralityConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        ClosenessCentralityConfig::validate(self)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClosenessCentralityResult {
    pub centralities: Vec<f64>,
    pub node_count: usize,
    pub execution_time: Duration,
}

fn orientation(direction: &str) -> Orientation {
    match direction {
        "incoming" => Orientation::Reverse,
        "outgoing" => Orientation::Natural,
        _ => Orientation::Undirected,
    }
}

define_algorithm_spec! {
    name: "closeness",
    output_type: ClosenessCentralityResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |_self, graph_store, config, _context| {
        let parsed: ClosenessCentralityConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {}", e)))?;
        parsed
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {}", e)))?;

        let start = Instant::now();

        let storage = ClosenessCentralityStorageRuntime::new(graph_store, orientation(&parsed.direction))?;
        let node_count = storage.node_count();
        let computation = ClosenessCentralityComputationRuntime::new();

        // Storage owns the pipeline; we track it as one leaf task.
        // Java parity uses a 2-phase task tree; we collapse into a single leaf here.
        let total_volume = node_count
            .saturating_mul(node_count)
            .saturating_add(node_count);

        let tracker = Arc::new(Mutex::new(TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("closeness".to_string(), total_volume),
            parsed.concurrency,
        )));
        tracker.lock().unwrap().begin_subtask_with_volume(total_volume);

        let on_farness = {
            let tracker = Arc::clone(&tracker);
            Arc::new(move |sources_done: usize| {
                tracker.lock().unwrap().log_progress(sources_done);
            })
        };

        let on_closeness = {
            let tracker = Arc::clone(&tracker);
            Arc::new(move |nodes_done: usize| {
                tracker.lock().unwrap().log_progress(nodes_done);
            })
        };

        let termination = TerminationFlag::running_true();
        let centralities = storage
            .compute_parallel(
                &computation,
                parsed.wasserman_faust,
                parsed.concurrency,
                &termination,
                on_farness,
                on_closeness,
            )
            .map_err(|e| AlgorithmError::Execution(format!("Closeness terminated: {e}")))?;

        tracker.lock().unwrap().end_subtask();

        Ok(ClosenessCentralityResult {
            centralities,
            node_count,
            execution_time: start.elapsed(),
        })
    }
}
