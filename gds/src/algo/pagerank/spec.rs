//! PageRank algorithm specification (executor integration)

use crate::config::base_types::AlgoBaseConfig;
use crate::config::PageRankConfig;
use crate::define_algorithm_spec;
use crate::projection::eval::procedure::*;
use crate::projection::Orientation;
use crate::core::utils::progress::{ProgressTracker, Tasks, TaskProgressTracker};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

use super::computation::PageRankComputationRuntime;
use super::storage::PageRankStorageRuntime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRankConfigInput {
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,

    #[serde(default = "default_max_iterations", rename = "maxIterations")]
    pub max_iterations: usize,

    #[serde(default = "default_tolerance")]
    pub tolerance: f64,

    #[serde(default = "default_damping_factor", rename = "dampingFactor")]
    pub damping_factor: f64,

    #[serde(default, rename = "sourceNodes")]
    pub source_nodes: Option<Vec<u64>>,
}

fn default_concurrency() -> usize {
    4
}

fn default_max_iterations() -> usize {
    20
}

fn default_tolerance() -> f64 {
    1e-7
}

fn default_damping_factor() -> f64 {
    0.85
}

impl Default for PageRankConfigInput {
    fn default() -> Self {
        Self {
            concurrency: default_concurrency(),
            max_iterations: default_max_iterations(),
            tolerance: default_tolerance(),
            damping_factor: default_damping_factor(),
            source_nodes: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRankResult {
    pub scores: Vec<f64>,
    pub ran_iterations: usize,
    pub did_converge: bool,
    pub node_count: usize,
    pub execution_time: Duration,
}

define_algorithm_spec! {
    name: "pagerank",
    output_type: PageRankResult,
    projection_hint: VertexCentric,
    modes: [Stream, Stats],

    execute: |_self, graph_store, config, _context| {
        let parsed: PageRankConfigInput = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;

        if parsed.concurrency == 0 {
            return Err(AlgorithmError::Execution("concurrency must be > 0".into()));
        }

        let start = Instant::now();

        let storage = PageRankStorageRuntime::with_orientation(
            graph_store,
            Orientation::Natural,
        )?;

        let pr_config = PageRankConfig::builder()
            .base(AlgoBaseConfig {
                concurrency: parsed.concurrency,
                ..AlgoBaseConfig::default()
            })
            .max_iterations(parsed.max_iterations)
            .damping_factor(parsed.damping_factor)
            .tolerance(parsed.tolerance)
            .build()
            .map_err(|e| AlgorithmError::Execution(format!("PageRankConfig invalid: {e}")))?;

        let sources = parsed.source_nodes.map(|v| v.into_iter().collect());

        let computation = PageRankComputationRuntime::new(
            pr_config.max_iterations,
            pr_config.damping_factor,
            pr_config.tolerance,
            sources,
        );

        // Lightweight progress hook (executor can override/ignore).
        let mut progress = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("pagerank".to_string(), parsed.max_iterations),
            parsed.concurrency,
        );
        progress.begin_subtask_with_volume(parsed.max_iterations);

        let run = storage.run(&computation, parsed.concurrency, &mut progress);

        progress.log_progress(parsed.max_iterations);
        progress.end_subtask();

        Ok(PageRankResult {
            scores: run.scores,
            ran_iterations: run.ran_iterations,
            did_converge: run.did_converge,
            node_count: graph_store.node_count(),
            execution_time: start.elapsed(),
        })
    }
}
