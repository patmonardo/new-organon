//! Harmonic Centrality Algorithm Specification
//!
//! Java parity reference: `org.neo4j.gds.harmonic.HarmonicCentrality`.
//!
//! Semantics:
//! - For each node $v$, compute $\sum_{u \ne v} 1 / d(v,u)$ where unreachable pairs contribute 0.
//! - Uses ANP MSBFS batching; accumulates into the reached node per BFS depth.
//! - Normalizes by `(nodeCount - 1)`.

use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::{ProgressTracker, Tasks};
use crate::define_algorithm_spec;
use crate::projection::eval::procedure::*;
use crate::projection::Orientation;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::computation::HarmonicComputationRuntime;
use super::storage::HarmonicStorageRuntime;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HarmonicDirection {
    Incoming,
    Outgoing,
    Both,
}

impl Default for HarmonicDirection {
    fn default() -> Self {
        Self::Both
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HarmonicConfig {
    #[serde(default)]
    pub direction: HarmonicDirection,
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

fn default_concurrency() -> usize {
    4
}

impl Default for HarmonicConfig {
    fn default() -> Self {
        Self {
            direction: HarmonicDirection::default(),
            concurrency: default_concurrency(),
        }
    }
}

impl HarmonicConfig {
    pub fn validate(&self) -> Result<(), crate::config::validation::ConfigError> {
        if self.concurrency == 0 {
            return Err(crate::config::validation::ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be positive".to_string(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HarmonicResult {
    pub centralities: Vec<f64>,
    pub node_count: usize,
    pub execution_time: Duration,
}

fn orientation_from_direction(direction: HarmonicDirection) -> Orientation {
    match direction {
        HarmonicDirection::Incoming => Orientation::Reverse,
        HarmonicDirection::Outgoing => Orientation::Natural,
        HarmonicDirection::Both => Orientation::Undirected,
    }
}

define_algorithm_spec! {
    name: "harmonic",
    output_type: HarmonicResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |self, graph_store, config, context| {
        let parsed_config: HarmonicConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {}", e)))?;
        parsed_config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {}", e)))?;

        let start = Instant::now();

        let concurrency = parsed_config.concurrency;
        let orientation = orientation_from_direction(parsed_config.direction);
        let node_count = graph_store.node_count();

        context.log(
            LogLevel::Info,
            &format!(
                "Computing harmonic centrality (direction={:?}, concurrency={}) on graph with {} nodes",
                parsed_config.direction,
                concurrency,
                node_count
            ),
        );

        let storage = HarmonicStorageRuntime::with_orientation(graph_store, orientation)?;
        let computation = HarmonicComputationRuntime::new(storage.node_count());

        let tracker = Arc::new(Mutex::new(crate::core::utils::progress::TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("harmonic".to_string(), node_count),
            concurrency,
        )));
        tracker.lock().unwrap().begin_subtask_with_volume(node_count);

        let on_sources_done = {
            let tracker = Arc::clone(&tracker);
            Arc::new(move |n: usize| {
                tracker.lock().unwrap().log_progress(n);
            })
        };

        let termination = TerminationFlag::default();
        let centralities = storage
            .compute_parallel(&computation, concurrency, &termination, on_sources_done)
            .map_err(|e| AlgorithmError::Execution(format!("Harmonic terminated: {}", e)))?;

        tracker.lock().unwrap().end_subtask();

        Ok(HarmonicResult {
            centralities,
            node_count: storage.node_count(),
            execution_time: start.elapsed(),
        })
    }
}
