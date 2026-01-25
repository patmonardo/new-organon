//! Harmonic Centrality Algorithm Specification
//!
//! Java parity reference: `org.neo4j.gds.harmonic.HarmonicCentrality`.
//!
//! Semantics:
//! - For each node $v$, compute $\sum_{u \ne v} 1 / d(v,u)$ where unreachable pairs contribute 0.
//! - Uses ANP MSBFS batching; accumulates into the reached node per BFS depth.
//! - Normalizes by `(nodeCount - 1)`.

use crate::collections::backends::vec::VecDouble;
use crate::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::core::utils::progress::{ProgressTracker, TaskProgressTracker, Tasks};
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::*;
use crate::projection::NodeLabel;
use crate::projection::Orientation;
use crate::types::properties::node::{DefaultDoubleNodePropertyValues, NodePropertyValues};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::storage::HarmonicStorageRuntime;
use super::HarmonicComputationRuntime;

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
    modes: [Stream, Stats, MutateNodeProperty],

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

        let tracker = Arc::new(Mutex::new(TaskProgressTracker::with_concurrency(
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
                "harmonic returned {} scores for {} nodes",
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
