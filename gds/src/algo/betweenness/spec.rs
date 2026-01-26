//! Betweenness Centrality specification

use crate::algo::betweenness::BetweennessCentralityComputationRuntime;
use crate::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::core::utils::progress::ProgressTracker;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use super::BetweennessCentralityStorageRuntime;

/// Configuration for betweenness centrality.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BetweennessCentralityConfig {
    /// Traversal direction: "outgoing", "incoming", or "both".
    #[serde(default = "default_direction")]
    pub direction: String,

    /// Requested parallelism.
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,

    /// Optional relationship weight property.
    #[serde(default)]
    pub relationship_weight_property: Option<String>,

    /// Sampling strategy: "all" or "random_degree".
    #[serde(default = "default_sampling_strategy")]
    pub sampling_strategy: String,

    /// Optional number of sources to process (<= node_count).
    #[serde(default)]
    pub sampling_size: Option<usize>,

    /// RNG seed for sampling.
    #[serde(default = "default_random_seed")]
    pub random_seed: u64,
}

fn default_direction() -> String {
    "both".to_string()
}

fn default_concurrency() -> usize {
    4
}

fn default_sampling_strategy() -> String {
    "all".to_string()
}

fn default_random_seed() -> u64 {
    42
}

impl Default for BetweennessCentralityConfig {
    fn default() -> Self {
        Self {
            direction: default_direction(),
            concurrency: default_concurrency(),
            relationship_weight_property: None,
            sampling_strategy: default_sampling_strategy(),
            sampling_size: None,
            random_seed: default_random_seed(),
        }
    }
}

impl crate::config::ValidatedConfig for BetweennessCentralityConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        crate::config::validate_positive(self.concurrency as f64, "concurrency")?;
        if let Some(size) = self.sampling_size {
            crate::config::validate_positive(size as f64, "samplingSize")?;
        }
        Ok(())
    }
}

/// Executor-facing result type.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BetweennessCentralityResult {
    pub centralities: Vec<f64>,
    pub node_count: usize,
    pub execution_time: Duration,
}

/// Statistics about betweenness centrality in the graph.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BetweennessCentralityStats {
    /// Minimum betweenness score
    pub min: f64,
    /// Maximum betweenness score
    pub max: f64,
    /// Average betweenness
    pub mean: f64,
    /// Standard deviation
    pub stddev: f64,
    /// Median (50th percentile)
    pub p50: f64,
    /// 90th percentile
    pub p90: f64,
    /// 99th percentile
    pub p99: f64,
    /// Number of "bridge" nodes (high betweenness > mean + stddev)
    pub bridge_nodes: u64,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BetweennessCentralityMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BetweennessCentralityWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for betweenness centrality: summary + updated store
#[derive(Debug, Clone)]
pub struct BetweennessCentralityMutateResult {
    pub summary: BetweennessCentralityMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// Betweenness result builder (facade adapter).
pub struct BetweennessCentralityResultBuilder {
    result: BetweennessCentralityResult,
}

impl BetweennessCentralityResultBuilder {
    pub fn new(result: BetweennessCentralityResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> BetweennessCentralityStats {
        let scores = &self.result.centralities;
        if scores.is_empty() {
            return BetweennessCentralityStats {
                min: 0.0,
                max: 0.0,
                mean: 0.0,
                stddev: 0.0,
                p50: 0.0,
                p90: 0.0,
                p99: 0.0,
                bridge_nodes: 0,
                execution_time_ms: self.result.execution_time.as_millis() as u64,
            };
        }

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

        let threshold = mean + stddev;
        let bridge_nodes = scores.iter().filter(|v| **v > threshold).count() as u64;

        BetweennessCentralityStats {
            min,
            max,
            mean,
            stddev,
            p50: percentile(50.0),
            p90: percentile(90.0),
            p99: percentile(99.0),
            bridge_nodes,
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
    name: "betweenness",
    output_type: BetweennessCentralityResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |_self, graph_store, config, _context| {
        let parsed: BetweennessCentralityConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {}", e)))?;

        if parsed.concurrency == 0 {
            return Err(AlgorithmError::Execution("concurrency must be positive".into()));
        }
        if let Some(sz) = parsed.sampling_size {
            if sz == 0 {
                return Err(AlgorithmError::Execution("sampling_size must be positive".into()));
            }
        }

        let start = Instant::now();

        let orientation = orientation(&parsed.direction);
        let storage = BetweennessCentralityStorageRuntime::new(
            graph_store,
            orientation,
            parsed.relationship_weight_property.as_deref(),
        )?;

        let node_count = storage.node_count();
        let sources = storage.select_sources(
            &parsed.sampling_strategy,
            parsed.sampling_size,
            parsed.random_seed,
        );
        let divisor = if orientation == Orientation::Undirected { 2.0 } else { 1.0 };

        let mut computation = BetweennessCentralityComputationRuntime::new(node_count);

        let tracker = Arc::new(Mutex::new(TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("betweenness".to_string(), sources.len()),
            parsed.concurrency,
        )));
        tracker.lock().unwrap().begin_subtask_with_volume(sources.len());

        let termination = TerminationFlag::default();
        let on_done = {
            let tracker = Arc::clone(&tracker);
            Arc::new(move || {
                tracker.lock().unwrap().log_progress(1);
            })
        };

        let result = storage
            .compute_betweenness(
                &mut computation,
                &sources,
                divisor,
                parsed.concurrency,
                &termination,
                on_done,
            )
            .map_err(|e| AlgorithmError::Execution(format!("terminated: {e}")))?;

        tracker.lock().unwrap().end_subtask();

        Ok(BetweennessCentralityResult {
            centralities: result.centralities,
            node_count,
            execution_time: start.elapsed(),
        })
    }
}

// Re-exported and aliased from `betweenness/mod.rs`.
