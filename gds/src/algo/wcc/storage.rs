//! WCC Storage Runtime

use super::{WccComputationResult, WccComputationRuntime};
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;

pub struct WccStorageRuntime {
    #[allow(dead_code)]
    concurrency: usize,
}

impl WccStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }

    /// Compute WCC given a graph store. Uses a directed (natural) view; WCC unions
    /// endpoints so results correspond to weak connectivity.
    pub fn compute_wcc(
        &self,
        computation: &mut WccComputationRuntime,
        graph_store: &impl GraphStore,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<WccComputationResult, String> {
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| format!("Failed to obtain graph view: {e}"))?;

        progress_tracker.begin_subtask_with_volume(graph_view.relationship_count());

        let result = computation
            .compute(graph_view.as_ref(), progress_tracker, termination_flag)
            .map_err(|e| format!("WCC computation failed: {e}"))?;

        progress_tracker.end_subtask();
        Ok(result)
    }
}
