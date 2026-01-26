//! Label Propagation algorithm specification (executor integration)

use crate::config::validation::ConfigError;
use crate::core::utils::progress::{ProgressTracker, TaskProgressTracker, Tasks};
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::LabelPropComputationRuntime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelPropConfig {
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,

    #[serde(default = "default_max_iterations", rename = "maxIterations")]
    pub max_iterations: u64,

    #[serde(default, rename = "nodeWeightProperty")]
    pub node_weight_property: Option<String>,

    #[serde(default, rename = "seedProperty")]
    pub seed_property: Option<String>,
}

fn default_concurrency() -> usize {
    4
}

fn default_max_iterations() -> u64 {
    10
}

impl Default for LabelPropConfig {
    fn default() -> Self {
        Self {
            concurrency: default_concurrency(),
            max_iterations: default_max_iterations(),
            node_weight_property: None,
            seed_property: None,
        }
    }
}

impl LabelPropConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be > 0".to_string(),
            });
        }
        if self.max_iterations == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "maxIterations".to_string(),
                reason: "maxIterations must be > 0".to_string(),
            });
        }
        if let Some(prop) = &self.node_weight_property {
            if prop.trim().is_empty() {
                return Err(ConfigError::InvalidParameter {
                    parameter: "nodeWeightProperty".to_string(),
                    reason: "node_weight_property cannot be empty".to_string(),
                });
            }
        }
        if let Some(prop) = &self.seed_property {
            if prop.trim().is_empty() {
                return Err(ConfigError::InvalidParameter {
                    parameter: "seedProperty".to_string(),
                    reason: "seed_property cannot be empty".to_string(),
                });
            }
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for LabelPropConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        LabelPropConfig::validate(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelPropResult {
    pub labels: Vec<u64>,
    pub did_converge: bool,
    pub ran_iterations: u64,
    pub node_count: usize,
    pub execution_time: Duration,
}

/// Aggregated label propagation stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelPropStats {
    pub did_converge: bool,
    pub ran_iterations: u64,
    pub community_count: usize,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelPropMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelPropWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for label propagation: summary + updated store.
#[derive(Debug, Clone)]
pub struct LabelPropMutateResult {
    pub summary: LabelPropMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// Label propagation result builder (facade adapter).
pub struct LabelPropResultBuilder {
    result: LabelPropResult,
}

impl LabelPropResultBuilder {
    pub fn new(result: LabelPropResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> LabelPropStats {
        let community_count = self
            .result
            .labels
            .iter()
            .copied()
            .collect::<std::collections::HashSet<u64>>()
            .len();

        LabelPropStats {
            did_converge: self.result.did_converge,
            ran_iterations: self.result.ran_iterations,
            community_count,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}

pub type LabelPropAlgorithmSpec = LABEL_PROPAGATIONAlgorithmSpec;

define_algorithm_spec! {
    name: "label_propagation",
    output_type: LabelPropResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |_self, graph_store, config, _context| {
        let parsed: LabelPropConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;

        parsed
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        let start = Instant::now();

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let node_count = graph_view.node_count() as usize;

        let mut progress = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("label_propagation".to_string(), parsed.max_iterations as usize),
            parsed.concurrency,
        );
        progress.begin_subtask_with_volume(parsed.max_iterations as usize);

        // Default: weight 1.0 for all nodes
        let mut weights = vec![1.0f64; node_count];
        if let Some(key) = &parsed.node_weight_property {
            if graph_view.available_node_properties().contains(key) {
                if let Some(pv) = graph_view.node_properties(key) {
                    for i in 0..node_count {
                        weights[i] = pv.double_value(i as u64).unwrap_or(1.0);
                    }
                }
            }
        }

        // Default initial labels: identity; executor spec does not implement full Java seed shifting.
        // The procedure facade layer provides Java-exact initialization.

        let fallback = graph_view.default_property_value();
        let neighbors = |node_idx: usize| -> Vec<(usize, f64)> {
            graph_view
                .stream_relationships_weighted(node_idx as i64, fallback)
                .map(|cursor| (cursor.target_id(), cursor.weight()))
                .filter(|(t, _)| *t >= 0)
                .map(|(t, w)| (t as usize, w))
                .collect()
        };

        let mut runtime = LabelPropComputationRuntime::new(node_count, parsed.max_iterations)
            .concurrency(parsed.concurrency)
            .with_weights(weights);

        let run = runtime.compute(node_count as u64, neighbors);

        progress.log_progress(parsed.max_iterations as usize);
        progress.end_subtask();

        Ok(LabelPropResult {
            labels: run.labels,
            did_converge: run.did_converge,
            ran_iterations: run.ran_iterations,
            node_count,
            execution_time: start.elapsed(),
        })
    }
}
