//! SCC Storage Runtime

use super::{SccComputationResult, SccComputationRuntime};
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;
use std::time::Instant;

pub struct SccStorageRuntime {
    #[allow(dead_code)]
    concurrency: usize,
}

impl SccStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }

    /// Compute SCCs for the given graph store using a directed (natural) graph view.
    ///
    /// Returns the per-node component assignment and component count.
    pub fn compute_scc(
        &self,
        computation: &mut SccComputationRuntime,
        graph_store: &impl GraphStore,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<SccComputationResult, String> {
        let start = Instant::now();

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| format!("Failed to obtain graph view: {e}"))?;

        let node_count = graph_view.node_count();
        progress_tracker.begin_subtask_with_volume(node_count);

        let (components, component_count) = computation
            .compute(graph_view.as_ref(), progress_tracker, termination_flag)
            .map_err(|e| format!("SCC computation failed: {e}"))?;

        progress_tracker.end_subtask();

        Ok(SccComputationResult {
            components,
            component_count,
            computation_time_ms: start.elapsed().as_millis() as u64,
        })
    }
}
