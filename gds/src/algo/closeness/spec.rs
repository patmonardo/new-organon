//! Closeness Centrality Algorithm Specification
//!
//! Java parity reference:
//! - `org.neo4j.gds.closeness.ClosenessCentrality`
//! - `org.neo4j.gds.closeness.ClosenessCentralityAlgorithmFactory` (progress task layout)

use crate::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::core::utils::progress::{ProgressTracker, TaskProgressTracker, Tasks};
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::ClosenessCentralityStorageRuntime;
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

/// Statistics about closeness centrality.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClosenessCentralityStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub stddev: f64,
    pub p50: f64,
    pub p90: f64,
    pub p99: f64,
    pub isolated_nodes: u64,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClosenessCentralityMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClosenessCentralityWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for closeness centrality: summary + updated store
#[derive(Debug, Clone)]
pub struct ClosenessCentralityMutateResult {
    pub summary: ClosenessCentralityMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// Closeness result builder (facade adapter).
pub struct ClosenessCentralityResultBuilder {
    result: ClosenessCentralityResult,
}

impl ClosenessCentralityResultBuilder {
    pub fn new(result: ClosenessCentralityResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> ClosenessCentralityStats {
        let scores = &self.result.centralities;
        if scores.is_empty() {
            return ClosenessCentralityStats {
                min: 0.0,
                max: 0.0,
                mean: 0.0,
                stddev: 0.0,
                p50: 0.0,
                p90: 0.0,
                p99: 0.0,
                isolated_nodes: 0,
                execution_time_ms: self.result.execution_time.as_millis() as u64,
            };
        }

        let isolated_nodes = scores.iter().filter(|v| **v == 0.0).count() as u64;
        let mut sorted = scores.clone();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let min = *sorted.first().unwrap_or(&0.0);
        let max = *sorted.last().unwrap_or(&0.0);
        let mean = scores.iter().sum::<f64>() / scores.len() as f64;
        let var = scores
            .iter()
            .map(|x| {
                let d = x - mean;
                d * d
            })
            .sum::<f64>()
            / scores.len() as f64;
        let stddev = var.sqrt();

        let percentile = |p: f64| -> f64 {
            let idx =
                ((p.clamp(0.0, 100.0) / 100.0) * (sorted.len() as f64 - 1.0)).round() as usize;
            sorted[idx]
        };

        ClosenessCentralityStats {
            min,
            max,
            mean,
            stddev,
            p50: percentile(50.0),
            p90: percentile(90.0),
            p99: percentile(99.0),
            isolated_nodes,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
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
