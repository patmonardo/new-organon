//! K1-Coloring storage runtime
//!
//! The algorithm operates over an undirected graph view; storage is a thin adapter
//! that provides neighbor iteration helpers.

use super::spec::{K1ColoringConfig, K1ColoringResult};
use super::{K1ColoringComputationRuntime, K1IterationProgress};
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

use crate::types::graph::Graph;

#[derive(Clone)]
pub struct K1ColoringStorageRuntime {
    graph: Arc<dyn Graph>,
}

impl K1ColoringStorageRuntime {
    pub fn new<G: GraphStore>(graph_store: &G) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;
        Ok(Self { graph })
    }

    pub fn graph(&self) -> Arc<dyn Graph> {
        Arc::clone(&self.graph)
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count() as usize
    }

    /// Controller entrypoint: owns graph access, progress/termination, and delegates state to computation.
    pub fn compute_k1coloring(
        &self,
        computation: &mut K1ColoringComputationRuntime,
        _config: &K1ColoringConfig,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<K1ColoringResult, AlgorithmError> {
        let node_count = self.graph.node_count() as usize;
        if node_count == 0 {
            return Ok(K1ColoringResult {
                colors: Vec::new(),
                ran_iterations: 0,
                did_converge: true,
                node_count: 0,
                execution_time: std::time::Duration::default(),
            });
        }

        progress_tracker.begin_subtask_with_volume(_config.max_iterations as usize);

        let fallback = self.graph.default_property_value();
        let graph = Arc::clone(&self.graph);
        let neighbors = move |node_idx: usize| -> Vec<usize> {
            graph
                .stream_relationships(node_idx as i64, fallback)
                .map(|cursor| cursor.target_id())
                .filter(|t| *t >= 0)
                .map(|t| t as usize)
                .collect()
        };

        let run = computation.compute(
            node_count,
            neighbors,
            termination_flag,
            |_step: K1IterationProgress| {
                // Track one unit per iteration (post-validation).
                progress_tracker.log_progress(1);
            },
        );

        progress_tracker.end_subtask();

        Ok(K1ColoringResult {
            colors: run.colors,
            ran_iterations: run.ran_iterations,
            did_converge: run.did_converge,
            node_count,
            execution_time: std::time::Duration::default(),
        })
    }
}
