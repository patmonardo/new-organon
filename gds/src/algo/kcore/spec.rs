//! K-Core Decomposition algorithm specification (executor integration)

use crate::define_algorithm_spec;
use crate::projection::eval::procedure::*;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::core::utils::progress::{ProgressTracker, Tasks, TaskProgressTracker};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::time::{Duration, Instant};

use super::computation::KCoreComputationRuntime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KCoreConfig {
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

fn default_concurrency() -> usize {
    4
}

impl Default for KCoreConfig {
    fn default() -> Self {
        Self {
            concurrency: default_concurrency(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KCoreResult {
    pub core_values: Vec<i32>,
    pub degeneracy: i32,
    pub node_count: usize,
    pub execution_time: Duration,
}

define_algorithm_spec! {
    name: "kcore",
    output_type: KCoreResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |_self, graph_store, config, _context| {
        let parsed: KCoreConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;

        if parsed.concurrency == 0 {
            return Err(AlgorithmError::Execution("concurrency must be > 0".into()));
        }

        let start = Instant::now();

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let node_count = graph_view.node_count() as usize;

        let mut progress = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("KCoreDecomposition".to_string(), node_count),
            parsed.concurrency,
        );
        progress.begin_subtask_with_volume(node_count);

        let fallback = graph_view.default_property_value();
        let neighbors = |node_idx: usize| -> Vec<usize> {
            graph_view
                .stream_relationships(node_idx as i64, fallback)
                .map(|cursor| cursor.target_id())
                .filter(|t| *t >= 0)
                .map(|t| t as usize)
                .collect()
        };

        let mut runtime = KCoreComputationRuntime::new().concurrency(parsed.concurrency);
        let result = runtime.compute(node_count, neighbors);

        progress.log_progress(node_count);
        progress.end_subtask();

        Ok(KCoreResult {
            core_values: result.core_values,
            degeneracy: result.degeneracy,
            node_count,
            execution_time: start.elapsed(),
        })
    }
}
