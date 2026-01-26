use super::spec::{ConductanceConfig, ConductanceResult};
use super::ConductanceComputationRuntime;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::GraphStore;
use crate::types::properties::node::NodePropertyValues;
use std::collections::HashSet;
use std::sync::Arc;

pub struct ConductanceStorageRuntime {}

impl ConductanceStorageRuntime {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compute_conductance(
        &self,
        computation: &mut ConductanceComputationRuntime,
        graph_store: &impl GraphStore,
        config: &ConductanceConfig,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<ConductanceResult, String> {
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| format!("failed to build graph view: {e}"))?;

        let node_count = graph.node_count();
        if node_count == 0 {
            return Ok(ConductanceResult {
                community_conductances: std::collections::HashMap::new(),
                global_average_conductance: 0.0,
                community_count: 0,
                node_count: 0,
                execution_time: std::time::Duration::default(),
            });
        }

        if config.community_property.is_empty() {
            return Err("community_property cannot be empty".to_string());
        }

        let community_props: Arc<dyn NodePropertyValues> = graph
            .node_properties(&config.community_property)
            .ok_or_else(|| {
                format!(
                    "Community property '{}' not found",
                    config.community_property
                )
            })?;

        // Root task ("Conductance") and its Java-parity subtasks are driven here.
        progress_tracker.begin_subtask_with_description("Conductance");

        // 1) Count relationships
        progress_tracker
            .begin_subtask_with_description_and_volume("count relationships", node_count);
        let locals = computation.count_relationships(
            Arc::clone(&graph),
            Arc::clone(&community_props),
            config,
            termination_flag,
        )?;
        // We don't log per-partition here (tracker isn't thread-shared); mark phase complete.
        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask_with_description("count relationships");

        // 2) Accumulate counts
        progress_tracker.begin_subtask_with_description("accumulate counts");
        let counts = computation.accumulate_counts(locals);
        progress_tracker.end_subtask_with_description("accumulate counts");

        // 3) Compute conductances
        progress_tracker.begin_subtask_with_description("perform conductance computations");
        let mut result = computation.compute_conductances(counts);
        result.node_count = node_count;
        result.community_count = result.community_conductances.len();
        progress_tracker.end_subtask_with_description("perform conductance computations");

        progress_tracker.end_subtask_with_description("Conductance");
        Ok(result)
    }
}

impl Default for ConductanceStorageRuntime {
    fn default() -> Self {
        Self::new()
    }
}
