//! SCC Storage Runtime
//!
//! Builds graph views / adjacency lists from a `GraphStore` and delegates to
//! the SCC computation runtime.

use super::computation::SccComputationResult;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::GraphStore;
use std::time::Instant;

/// SCC storage runtime for accessing graph data
///
/// Translation of: `org.neo4j.gds.scc.Scc` (lines 36-65)
pub struct SccStorageRuntime {
    #[allow(dead_code)]
    concurrency: usize,
}

impl SccStorageRuntime {
    /// Create new SCC storage runtime
    ///
    /// Translation of: `Scc(Graph graph, ProgressTracker progressTracker, TerminationFlag terminationFlag)`
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }

    /// Compute strongly connected components
    ///
    /// Translation of: `Scc.compute()` (lines 70-78)
    pub fn compute_scc<G: GraphStore>(
        &self,
        computation: &mut super::computation::SccComputationRuntime,
        graph_store: &G,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<SccComputationResult, String> {
        let start_time = Instant::now();

        progress_tracker.begin_subtask_with_volume(graph_store.node_count());

        let result = (|| {
            // Obtain a directed graph view (Natural orientation, all relationship types)
            let rel_types: std::collections::HashSet<RelationshipType> =
                std::collections::HashSet::new();
            let graph_view = graph_store
                .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
                .map_err(|e| format!("Failed to obtain graph view: {}", e))?;

            let node_count = graph_view.node_count();
            if node_count == 0 {
                return Ok(SccComputationResult::new(Vec::new(), 0, 0));
            }

            // Build adjacency lists.
            let fallback = graph_view.default_property_value();
            let mut outgoing: Vec<Vec<usize>> = vec![Vec::new(); node_count];

            const LOG_BATCH: usize = 256;
            let mut pending = 0usize;

            for (node, outgoing_list) in outgoing.iter_mut().enumerate().take(node_count) {
                if !termination_flag.running() {
                    return Err("Algorithm terminated by user".to_string());
                }

                // NodeId is i64; node indices are contiguous starting at 0.
                let node_id: i64 = node as i64;
                *outgoing_list = graph_view
                    .stream_relationships(node_id, fallback)
                    .map(|cursor| cursor.target_id())
                    .filter(|target| *target >= 0)
                    .map(|target| target as usize)
                    .filter(|target| *target < node_count)
                    .collect();

                pending += 1;
                if pending >= LOG_BATCH {
                    progress_tracker.log_progress(pending);
                    pending = 0;
                }
            }

            if pending > 0 {
                progress_tracker.log_progress(pending);
            }

            // Reverse adjacency.
            let mut incoming: Vec<Vec<usize>> = vec![Vec::new(); node_count];
            for (source, neighbors) in outgoing.iter().enumerate() {
                for &target in neighbors {
                    if target < node_count {
                        incoming[target].push(source);
                    }
                }
            }

            let (components, component_count) =
                computation.compute(&outgoing, &incoming, termination_flag)?;

            let computation_time = start_time.elapsed().as_millis() as u64;
            Ok(SccComputationResult::new(
                components,
                component_count,
                computation_time,
            ))
        })();

        match result {
            Ok(value) => {
                progress_tracker.end_subtask();
                Ok(value)
            }
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                Err(e)
            }
        }
    }
}
