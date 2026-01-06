//! Degree Centrality algorithm specification
//!
//! Java parity reference: `org.neo4j.gds.degree.DegreeCentrality`.

use crate::algo::degree_centrality::computation::DegreeCentralityComputationRuntime;
use crate::algo::degree_centrality::storage::{DegreeCentralityStorageRuntime, Orientation};
use crate::core::utils::progress::{ProgressTracker, Tasks};
use crate::define_algorithm_spec;
use crate::projection::eval::procedure::*;
use crate::concurrency::TerminationFlag;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DegreeCentralityConfig {
    #[serde(default)]
    pub normalize: bool,

    /// One of: "natural", "reverse", "undirected".
    #[serde(default = "default_orientation")]
    pub orientation: String,

    #[serde(default)]
    pub weighted: bool,

    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

fn default_orientation() -> String {
    "natural".to_string()
}

fn default_concurrency() -> usize {
    4
}

impl Default for DegreeCentralityConfig {
    fn default() -> Self {
        Self {
            normalize: false,
            orientation: default_orientation(),
            weighted: false,
            concurrency: default_concurrency(),
        }
    }
}

impl DegreeCentralityConfig {
    pub fn validate(&self) -> Result<(), crate::config::validation::ConfigError> {
        if self.concurrency == 0 {
            return Err(crate::config::validation::ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be positive".to_string(),
            });
        }
        if self.orientation.trim().is_empty() {
            return Err(crate::config::validation::ConfigError::InvalidParameter {
                parameter: "orientation".to_string(),
                reason: "orientation must be non-empty".to_string(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DegreeCentralityResult {
    pub centralities: Vec<f64>,
    pub node_count: usize,
    pub execution_time: Duration,
}

fn parse_orientation(value: &str) -> Result<Orientation, AlgorithmError> {
    match value.to_lowercase().as_str() {
        "natural" | "outgoing" => Ok(Orientation::Natural),
        "reverse" | "incoming" => Ok(Orientation::Reverse),
        "undirected" | "both" => Ok(Orientation::Undirected),
        other => Err(AlgorithmError::Execution(format!(
            "Invalid orientation '{other}'. Use 'natural', 'reverse', or 'undirected'"
        ))),
    }
}

define_algorithm_spec! {
    name: "degree_centrality",
    output_type: DegreeCentralityResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |_self, graph_store, config, _context| {
        let parsed: DegreeCentralityConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        parsed
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        let start = Instant::now();
        let orientation = parse_orientation(&parsed.orientation)?;

        let storage = DegreeCentralityStorageRuntime::with_settings(
            graph_store,
            orientation,
            parsed.weighted,
        )?;

        let node_count = storage.node_count();

        let tracker = Arc::new(Mutex::new(crate::core::utils::progress::TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("degree_centrality".to_string(), node_count),
            parsed.concurrency,
        )));
        tracker.lock().unwrap().begin_subtask_with_volume(node_count);

        let on_nodes_done = {
            let tracker = Arc::clone(&tracker);
            Arc::new(move |n: usize| {
                tracker.lock().unwrap().log_progress(n);
            })
        };

        let termination = TerminationFlag::default();
        let computation = DegreeCentralityComputationRuntime::new();

        let mut centralities = storage
            .compute_parallel(&computation, parsed.concurrency, &termination, on_nodes_done)
            .map_err(|e| AlgorithmError::Execution(format!("Degree centrality terminated: {e}")))?;

        if parsed.normalize {
            computation.normalize_scores(&mut centralities);
        }

        tracker.lock().unwrap().end_subtask();

        Ok(DegreeCentralityResult {
            centralities,
            node_count,
            execution_time: start.elapsed(),
        })
    }
}
