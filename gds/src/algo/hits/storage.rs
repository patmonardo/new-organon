//! HITS storage runtime
//!
//! Storage owns top-level control:
//! - graph projection
//! - selecting Pregel runtime config/messenger
//! - (eventually) termination and progress bridging

use crate::algo::hits::HitsComputationRuntime;
use crate::config::{ConcurrencyConfig, Config, IterationsConfig, PregelRuntimeConfig};
use crate::core::utils::partition::Partitioning;
use crate::core::utils::progress::ProgressTracker;
use crate::pregel::{Pregel, SyncQueueMessenger};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::Graph;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HitsRunResult {
    pub hub_scores: Vec<f64>,
    pub authority_scores: Vec<f64>,
    pub iterations_ran: usize,
    pub did_converge: bool,
}

/// A tiny Pregel runtime config used by HITS.
///
/// We keep this separate from user-facing `HitsConfig` so storage can translate
/// algorithm iterations into Pregel supersteps.
#[derive(Debug, Clone)]
pub struct HitsPregelRuntimeConfig {
    pub concurrency: usize,
    pub max_iterations: usize,
}

impl Config for HitsPregelRuntimeConfig {}

impl ConcurrencyConfig for HitsPregelRuntimeConfig {
    fn concurrency(&self) -> usize {
        self.concurrency
    }
}

impl IterationsConfig for HitsPregelRuntimeConfig {
    fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    fn tolerance(&self) -> Option<f64> {
        None
    }
}

impl PregelRuntimeConfig for HitsPregelRuntimeConfig {
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

pub struct HitsStorageRuntime<'a, G: GraphStore> {
    graph_store: &'a G,
    graph: Arc<dyn Graph>,
}

impl<'a, G: GraphStore> HitsStorageRuntime<'a, G> {
    pub fn with_default_projection(graph_store: &'a G) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = graph_store.relationship_types();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        Ok(Self { graph_store, graph })
    }

    pub fn graph_store(&self) -> &'a G {
        self.graph_store
    }

    pub fn graph(&self) -> Arc<dyn Graph> {
        Arc::clone(&self.graph)
    }

    pub fn run(
        &self,
        computation: &HitsComputationRuntime,
        max_iterations: usize,
        concurrency: usize,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> HitsRunResult {
        let supersteps = 1usize.saturating_add(max_iterations.saturating_mul(4));

        let config = HitsPregelRuntimeConfig {
            concurrency: concurrency.max(1),
            max_iterations: supersteps,
        };

        let schema = computation.schema();
        let init_fn = computation.init_fn();
        let compute_fn = computation.compute_fn();
        let master_compute_fn = computation.master_compute_fn();

        let messenger = Arc::new(SyncQueueMessenger::new(self.graph.node_count()));

        let result = Pregel::new(
            Arc::clone(&self.graph),
            config,
            schema,
            init_fn,
            compute_fn,
            messenger,
            None,
        )
        .with_master_compute_fn(master_compute_fn)
        .run();

        progress_tracker.log_progress(max_iterations);

        computation.finalize(&result, self.graph.node_count())
    }
}
