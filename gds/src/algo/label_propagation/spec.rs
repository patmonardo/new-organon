//! Label Propagation algorithm specification (executor integration)

use crate::core::utils::progress::{ProgressTracker, TaskProgressTracker, Tasks};
use crate::define_algorithm_spec;
use crate::projection::eval::procedure::*;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelPropResult {
    pub labels: Vec<u64>,
    pub did_converge: bool,
    pub ran_iterations: u64,
    pub node_count: usize,
    pub execution_time: Duration,
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

        if parsed.concurrency == 0 {
            return Err(AlgorithmError::Execution("concurrency must be > 0".into()));
        }
        if parsed.max_iterations == 0 {
            return Err(AlgorithmError::Execution("Must iterate at least 1 time".into()));
        }

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
