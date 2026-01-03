//! HITS Specification
//!
//! This module implements the `AlgorithmSpec` trait for HITS.

use crate::define_algorithm_spec;
use crate::core::utils::progress::{ProgressTracker, Tasks};
use crate::projection::eval::procedure::*;
use std::time::Duration;
use std::time::Instant;

use crate::projection::{Orientation, RelationshipType};
use std::collections::HashSet;

use super::computation::run_hits;
use super::storage::HitsStorageRuntime;

// ============================================================================
// Configuration
// ============================================================================

/// HITS Configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HitsConfig {
    /// Convergence tolerance
    pub tolerance: f64,
    /// Maximum number of iterations
    pub max_iterations: usize,
}

impl Default for HitsConfig {
    fn default() -> Self {
        Self {
            tolerance: 1e-6,
            max_iterations: 100,
        }
    }
}

// ============================================================================
// Result
// ============================================================================

/// HITS Result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HitsResult {
    /// Hub scores for each node
    pub hub_scores: Vec<f64>,
    /// Authority scores for each node
    pub authority_scores: Vec<f64>,
    /// Number of iterations run
    pub iterations: usize,
    /// Whether the algorithm converged
    pub did_converge: bool,
    /// Execution time
    pub execution_time: Duration,
}

// ============================================================================
// Algorithm Spec (Using macro for boilerplate)
// ============================================================================

define_algorithm_spec! {
    name: "hits",
    output_type: HitsResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |self, graph_store, config, context| {
        // Parse configuration
        let parsed_config: HitsConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {}", e)))?;

        context.log(
            LogLevel::Info,
            &format!(
                "Computing HITS on graph with {} nodes",
                graph_store.node_count()
            ),
        );

        let start = Instant::now();

        let mut progress_tracker = ProgressTracker::new(Tasks::leaf(
            "hits",
            parsed_config.max_iterations,
        ));
        progress_tracker.begin_subtask(parsed_config.max_iterations);

        // Create storage runtime
        let storage = HitsStorageRuntime::new(graph_store)?;

        // Build an unfiltered natural-orientation view.
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::Execution(format!("Failed to get graph: {}", e)))?;

        let result = run_hits(graph, parsed_config.max_iterations, parsed_config.tolerance);
        progress_tracker.log_progress(parsed_config.max_iterations);
        progress_tracker.end_subtask();

        context.log(
            LogLevel::Info,
            &format!(
                "HITS computed: {} nodes, iterations: {}",
                storage.node_count(),
                result.iterations_ran
            ),
        );

        Ok(HitsResult {
            hub_scores: result.hub_scores,
            authority_scores: result.authority_scores,
            iterations: result.iterations_ran,
            did_converge: result.did_converge,
            execution_time: start.elapsed(),
        })
    }
}
