//! Degree Centrality algorithm specification
//!
//! Java parity reference: `org.neo4j.gds.degree.DegreeCentrality`.

use crate::algo::degree_centrality::storage::{DegreeCentralityStorageRuntime, Orientation};
use crate::algo::degree_centrality::DegreeCentralityComputationRuntime;
use crate::collections::backends::vec::VecDouble;
use crate::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::core::utils::progress::{ProgressTracker, TaskProgressTracker, Tasks};
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::*;
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
