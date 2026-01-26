use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::NodeId;
use crate::types::graph::Graph;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

use super::spec::{ModularityConfig, ModularityResult};
use super::ModularityComputationRuntime;

#[derive(Clone)]
pub struct ModularityStorageRuntime {
    graph: Arc<dyn Graph>,
}

impl ModularityStorageRuntime {
    pub fn new<G: GraphStore>(graph_store: &G) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;
        Ok(Self { graph })
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn compute_modularity(
        &self,
        computation: &ModularityComputationRuntime,
        config: &ModularityConfig,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<ModularityResult, AlgorithmError> {
        let node_count = self.graph.node_count();
        if node_count == 0 {
            return Ok(computation.compute(0, |_| None, |_| Vec::new()));
        }

        progress_tracker.begin_subtask_with_volume(node_count);

        let community_props = self
            .graph
            .node_properties(&config.community_property)
            .ok_or_else(|| {
                AlgorithmError::Execution(format!(
                    "Community property '{}' not found",
                    config.community_property
                ))
            })?;

        let weight_fallback = self.graph.default_property_value();

        let communities: Vec<Option<u64>> = (0..node_count)
            .map(|node_idx| {
                termination_flag.assert_running();
                let node_id = i64::try_from(node_idx).map_err(|_| {
                    AlgorithmError::Execution(format!(
                        "node_id must fit into i64 (got {})",
                        node_idx
                    ))
                })?;

                match community_props.long_value(node_id as u64) {
                    Ok(community) if community >= 0 => Ok(Some(community as u64)),
                    _ => Ok(None),
                }
            })
            .collect::<Result<_, AlgorithmError>>()?;

        let get_community =
            |node_idx: usize| -> Option<u64> { communities.get(node_idx).copied().flatten() };

        let get_neighbors = |node_idx: usize| -> Vec<(usize, f64)> {
            termination_flag.assert_running();
            let Ok(node_id) = i64::try_from(node_idx) else {
                return Vec::new();
            };
            self.graph
                .stream_relationships(node_id as NodeId, weight_fallback)
                .map(|cursor| (cursor.target_id() as usize, cursor.property()))
                .collect()
        };

        let result = computation.compute(node_count, get_community, get_neighbors);

        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        Ok(result)
    }
}
