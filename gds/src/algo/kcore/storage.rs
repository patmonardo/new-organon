//! K-Core storage runtime
//!
//! Controller: owns the undirected graph view, drives progress, and delegates
//! stateful work to the computation runtime.

use super::spec::KCoreConfig;
use super::{KCoreComputationResult, KCoreComputationRuntime};
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
pub struct KCoreStorageRuntime {
    graph: Arc<dyn Graph>,
}

impl KCoreStorageRuntime {
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

    /// Controller entrypoint: obtains the undirected view, wires progress, and calls computation.
    pub fn compute_kcore(
        &self,
        computation: &mut KCoreComputationRuntime,
        _config: &KCoreConfig,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<KCoreComputationResult, AlgorithmError> {
        let node_count = self.graph.node_count() as usize;
        if node_count == 0 {
            return Ok(KCoreComputationResult {
                core_values: Vec::new(),
                degeneracy: 0,
            });
        }

        termination_flag.assert_running();
        progress_tracker.begin_subtask_with_volume(node_count);

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

        let result = computation.compute(node_count, neighbors);

        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        Ok(result)
    }
}
