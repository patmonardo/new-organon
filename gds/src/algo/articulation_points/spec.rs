//! Articulation Points Algorithm Specification

use crate::config::validation::ConfigError;
use crate::core::utils::progress::{ProgressTracker, TaskProgressTracker, Tasks};
use crate::define_algorithm_spec;
use crate::projection::eval::procedure::*;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::computation::ArticulationPointsComputationRuntime;
use super::storage::ArticulationPointsStorageRuntime;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArticulationPointsConfig {
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

fn default_concurrency() -> usize {
    4
}

impl Default for ArticulationPointsConfig {
    fn default() -> Self {
        Self {
            concurrency: default_concurrency(),
        }
    }
}

impl ArticulationPointsConfig {
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
pub struct ArticulationPointsResult {
    pub articulation_points: Vec<u64>,
    pub node_count: usize,
    pub execution_time: Duration,
}

define_algorithm_spec! {
    name: "articulation_points",
    output_type: ArticulationPointsResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |self, graph_store, config, context| {
        let parsed_config: ArticulationPointsConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {}", e)))?;
        parsed_config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {}", e)))?;

        let start = Instant::now();

        let storage = ArticulationPointsStorageRuntime::new(graph_store)?;
        let node_count = storage.node_count();
        let relationship_count = storage.relationship_count();

        context.log(
            LogLevel::Info,
            &format!(
                "Computing articulation points (concurrency={}) on graph with {} nodes",
                parsed_config.concurrency,
                node_count
            ),
        );

        // Java parity: a single leaf task sized by node_count.
        let tracker = Arc::new(Mutex::new(TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("ArticulationPoints".to_string(), node_count),
            parsed_config.concurrency,
        )));
        tracker.lock().unwrap().begin_subtask_with_volume(node_count);

        let neighbors = |n: usize| storage.neighbors(n);
        let mut runtime = ArticulationPointsComputationRuntime::new(node_count);
        let result = runtime.compute_with_relationship_count(node_count, relationship_count, neighbors);

        let mut points: Vec<u64> = Vec::new();
        let mut idx = result.articulation_points.next_set_bit(0);
        while let Some(i) = idx {
            points.push(i as u64);
            idx = result.articulation_points.next_set_bit(i + 1);
        }

        tracker.lock().unwrap().log_progress(node_count);
        tracker.lock().unwrap().end_subtask();

        Ok(ArticulationPointsResult {
            articulation_points: points,
            node_count,
            execution_time: start.elapsed(),
        })
    }
}
