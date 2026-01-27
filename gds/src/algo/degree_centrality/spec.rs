//! Degree Centrality algorithm specification
//!
//! Java parity reference: `org.neo4j.gds.degree.DegreeCentrality`.

use crate::algo::degree_centrality::DegreeCentralityComputationRuntime;
use crate::algo::degree_centrality::{DegreeCentralityStorageRuntime, Orientation};
use crate::collections::backends::vec::VecDouble;
use crate::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::core::utils::progress::{ProgressTracker, TaskProgressTracker, Tasks};
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::NodeLabel;
use crate::types::properties::node::{DefaultDoubleNodePropertyValues, NodePropertyValues};
use std::collections::HashSet;
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
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be positive".to_string(),
            });
        }
        if self.orientation.trim().is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "orientation".to_string(),
                reason: "orientation must be non-empty".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for DegreeCentralityConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        DegreeCentralityConfig::validate(self)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DegreeCentralityResult {
    pub centralities: Vec<f64>,
    pub node_count: usize,
    pub execution_time: Duration,
}

/// Statistics about degree distribution in the graph
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DegreeCentralityStats {
    /// Minimum degree found
    pub min: f64,
    /// Maximum degree found
    pub max: f64,
    /// Average degree
    pub mean: f64,
    /// Standard deviation
    pub stddev: f64,
    /// Median degree (50th percentile)
    pub p50: f64,
    /// 90th percentile degree
    pub p90: f64,
    /// 99th percentile degree
    pub p99: f64,
    /// Number of nodes with degree 0
    pub isolated_nodes: u64,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DegreeCentralityMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DegreeCentralityWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for degree centrality: summary + updated store
#[derive(Debug, Clone)]
pub struct DegreeCentralityMutateResult {
    pub summary: DegreeCentralityMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// Degree centrality result builder (facade adapter).
pub struct DegreeCentralityResultBuilder {
    result: DegreeCentralityResult,
}

impl DegreeCentralityResultBuilder {
    pub fn new(result: DegreeCentralityResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> DegreeCentralityStats {
        let scores = &self.result.centralities;
        if scores.is_empty() {
            return DegreeCentralityStats {
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

        DegreeCentralityStats {
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
    modes: [Stream, Stats, MutateNodeProperty],

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

        let tracker = Arc::new(Mutex::new(TaskProgressTracker::with_concurrency(
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
    },

    mutate_node_property: |_self, graph_store, config, result| {
        let mutate_property = config
            .get("mutateProperty")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AlgorithmError::Execution("Missing mutateProperty".to_string()))?;

        let labels: HashSet<NodeLabel> = config
            .get("nodeLabels")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| NodeLabel::of(s.to_string())))
                    .collect()
            })
            .unwrap_or_else(|| graph_store.node_labels());

        let node_count = graph_store.node_count();
        if result.centralities.len() != node_count {
            return Err(AlgorithmError::Execution(format!(
                "degree_centrality returned {} scores for {} nodes",
                result.centralities.len(),
                node_count
            )));
        }

        let backend = VecDouble::from(result.centralities.clone());
        let values = DefaultDoubleNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        graph_store
            .add_node_property(labels, mutate_property.to_string(), values)
            .map_err(|e| AlgorithmError::Execution(e.to_string()))?;

        Ok(node_count)
    }
}
