//! Articulation Points Algorithm Specification

use crate::config::validation::ConfigError;
use crate::core::utils::progress::{ProgressTracker, TaskProgressTracker, Tasks};
use crate::core::LogLevel;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::ArticulationPointsComputationRuntime;
use super::ArticulationPointsStorageRuntime;

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

impl crate::config::ValidatedConfig for ArticulationPointsConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        ArticulationPointsConfig::validate(self)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArticulationPointsResult {
    pub articulation_points: Vec<u64>,
    pub node_count: usize,
    pub execution_time: Duration,
}

/// Result row for articulation points stream.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct ArticulationPointRow {
    pub node_id: u64,
}

/// Statistics for articulation points computation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArticulationPointsStats {
    pub articulation_point_count: u64,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArticulationPointsMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArticulationPointsWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for articulation points: summary + updated store
#[derive(Debug, Clone)]
pub struct ArticulationPointsMutateResult {
    pub summary: ArticulationPointsMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

fn result_rows(result: &ArticulationPointsResult) -> Vec<ArticulationPointRow> {
    result
        .articulation_points
        .iter()
        .copied()
        .map(|node_id| ArticulationPointRow { node_id })
        .collect()
}

/// Articulation points result builder (facade adapter).
pub struct ArticulationPointsResultBuilder {
    result: ArticulationPointsResult,
}

impl ArticulationPointsResultBuilder {
    pub fn new(result: ArticulationPointsResult) -> Self {
        Self { result }
    }

    pub fn rows(&self) -> Vec<ArticulationPointRow> {
        result_rows(&self.result)
    }

    pub fn stats(&self) -> ArticulationPointsStats {
        ArticulationPointsStats {
            articulation_point_count: self.result.articulation_points.len() as u64,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
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
