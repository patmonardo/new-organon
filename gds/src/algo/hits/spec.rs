//! HITS algorithm specification

use crate::algo::hits::computation::HitsComputationRuntime;
use crate::algo::hits::storage::HitsStorageRuntime;
use crate::core::utils::progress::{ProgressTracker, Tasks};
use crate::define_algorithm_spec;
use crate::projection::eval::procedure::*;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HitsConfig {
    #[serde(default = "default_max_iterations")]
    pub max_iterations: usize,

    #[serde(default = "default_tolerance")]
    pub tolerance: f64,

    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

fn default_max_iterations() -> usize {
    20
}

fn default_tolerance() -> f64 {
    1e-4
}

fn default_concurrency() -> usize {
    4
}

impl Default for HitsConfig {
    fn default() -> Self {
        Self {
            max_iterations: default_max_iterations(),
            tolerance: default_tolerance(),
            concurrency: default_concurrency(),
        }
    }
}

impl HitsConfig {
    pub fn validate(&self) -> Result<(), crate::config::validation::ConfigError> {
        if self.concurrency == 0 {
            return Err(crate::config::validation::ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be positive".to_string(),
            });
        }
        if self.max_iterations == 0 {
            return Err(crate::config::validation::ConfigError::InvalidParameter {
                parameter: "maxIterations".to_string(),
                reason: "maxIterations must be positive".to_string(),
            });
        }
        if self.tolerance <= 0.0 {
            return Err(crate::config::validation::ConfigError::InvalidParameter {
                parameter: "tolerance".to_string(),
                reason: "tolerance must be positive".to_string(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HitsResult {
    pub hub_scores: Vec<f64>,
    pub authority_scores: Vec<f64>,
    pub iterations: usize,
    pub converged: bool,
    pub execution_time: Duration,
}

define_algorithm_spec! {
    name: "hits",
    output_type: HitsResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |_self, graph_store, config, _context| {
        let parsed: HitsConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        parsed
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        let start = Instant::now();

        let storage = HitsStorageRuntime::with_default_projection(graph_store)?;
        let computation = HitsComputationRuntime::new(parsed.tolerance);

        let mut tracker = crate::core::utils::progress::TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("hits".to_string(), parsed.max_iterations),
            parsed.concurrency,
        );

        tracker.begin_subtask_with_volume(parsed.max_iterations);

        let run = storage.run(&computation, parsed.max_iterations, parsed.concurrency, &mut tracker);

        tracker.end_subtask();

        Ok(HitsResult {
            hub_scores: run.hub_scores,
            authority_scores: run.authority_scores,
            iterations: run.iterations_ran,
            converged: run.did_converge,
            execution_time: start.elapsed(),
        })
    }
}
