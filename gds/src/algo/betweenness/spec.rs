//! Betweenness Centrality specification

use crate::algo::betweenness::BetweennessCentralityComputationRuntime;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::eval::procedure::*;
use crate::projection::Orientation;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use super::storage::BetweennessCentralityStorageRuntime;

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

/// Executor-facing result type.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BetweennessCentralityResult {
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
