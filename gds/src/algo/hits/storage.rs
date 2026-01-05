//! HITS storage runtime
//!
//! Storage owns top-level control:
//! - graph projection
//! - selecting Pregel runtime config/messenger
//! - (eventually) termination and progress bridging

use crate::core::utils::progress::ProgressTracker;
use crate::projection::eval::procedure::AlgorithmError;
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::Graph;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

use super::computation;

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

impl crate::config::Config for HitsPregelRuntimeConfig {}

impl crate::config::ConcurrencyConfig for HitsPregelRuntimeConfig {
    fn concurrency(&self) -> usize {
        self.concurrency
    }
}

impl crate::config::IterationsConfig for HitsPregelRuntimeConfig {
    fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    fn tolerance(&self) -> Option<f64> {
        None
    }
}

impl crate::config::PregelRuntimeConfig for HitsPregelRuntimeConfig {
    fn is_asynchronous(&self) -> bool {
        false
    }

    fn partitioning(&self) -> crate::core::utils::partition::Partitioning {
        crate::core::utils::partition::Partitioning::Range
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
        max_iterations: usize,
        tolerance: f64,
        concurrency: usize,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> HitsRunResult {
        computation::run_hits(
            Arc::clone(&self.graph),
            max_iterations,
            tolerance,
            concurrency,
            progress_tracker,
        )
    }
}
