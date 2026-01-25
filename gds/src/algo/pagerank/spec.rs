//! PageRank algorithm specification (executor integration)

use crate::collections::backends::vec::VecDouble;
use crate::config::base_types::AlgoBaseConfig;
use crate::config::validation::{validate_positive, validate_range};
use crate::core::utils::partition::Partitioning;
use crate::core::utils::progress::{ProgressTracker, TaskProgressTracker, Tasks};
use crate::define_algorithm_spec;
use crate::define_config;
use crate::projection::eval::algorithm::*;
use crate::projection::NodeLabel;
use crate::projection::Orientation;
use crate::types::properties::node::{DefaultDoubleNodePropertyValues, NodePropertyValues};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::storage::PageRankStorageRuntime;
use super::PageRankComputationRuntime;

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

// PageRank runtime configuration (canonicalized into the algorithm spec)
define_config!(
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct PageRankConfig {
        validate = |cfg: &PageRankConfig| {
            validate_positive(cfg.base.concurrency as f64, "concurrency")?;
            validate_positive(cfg.max_iterations as f64, "maxIterations")?;
            validate_range(cfg.damping_factor, 0.0, 1.0, "dampingFactor")?;
            validate_positive(cfg.tolerance, "tolerance")?;
            Ok(())
        },
        base: AlgoBaseConfig = AlgoBaseConfig::default(),
        max_iterations: usize = 20,
        tolerance: f64 = 1e-7,
        damping_factor: f64 = 0.85,
        // keep node ids as u64 here to match runtime usage
        source_nodes: Option<Vec<u64>> = None,
    }
);

impl crate::config::ConcurrencyConfig for PageRankConfig {
    fn concurrency(&self) -> usize {
        self.base.concurrency
    }
}

impl crate::config::IterationsConfig for PageRankConfig {
    fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    fn tolerance(&self) -> Option<f64> {
        Some(self.tolerance)
    }
}

impl crate::config::pregel_config::PregelRuntimeConfig for PageRankConfig {
    fn is_asynchronous(&self) -> bool {
        false
    }

    fn partitioning(&self) -> Partitioning {
        Partitioning::Range
    }

    fn track_sender(&self) -> bool {
        false
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
    modes: [Stream, Stats, MutateNodeProperty],

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
        if result.scores.len() != node_count {
            return Err(AlgorithmError::Execution(format!(
                "pagerank returned {} scores for {} nodes",
                result.scores.len(),
                node_count
            )));
        }

        let backend = VecDouble::from(result.scores.clone());
        let values = DefaultDoubleNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        graph_store
            .add_node_property(labels, mutate_property.to_string(), values)
            .map_err(|e| AlgorithmError::Execution(e.to_string()))?;

        Ok(node_count)
    }
}
