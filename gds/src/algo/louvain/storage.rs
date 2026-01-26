use crate::algo::modularity_optimization::ModularityOptimizationInput;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::Graph;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

use super::spec::LouvainResult;
use super::LouvainComputationRuntime;

#[derive(Clone)]
pub struct LouvainStorageRuntime {
    graph: Arc<dyn Graph>,
    #[allow(dead_code)]
    concurrency: usize,
}

impl LouvainStorageRuntime {
    pub fn new<G: GraphStore>(graph_store: &G, concurrency: usize) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;
        Ok(Self { graph, concurrency })
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count() as usize
    }

    pub fn compute_louvain(
        &self,
        computation: &mut LouvainComputationRuntime,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<LouvainResult, AlgorithmError> {
        let node_count = self.graph.node_count();
        if node_count == 0 {
            return Ok(LouvainResult {
                data: Vec::new(),
                node_count: 0,
                execution_time: std::time::Duration::default(),
            });
        }

        // For Louvain, treat unweighted relationships as weight=1.0 (matches other procedures).
        let weight_fallback = 1.0;

        progress_tracker.begin_subtask_with_volume(node_count);

        let mut adj: Vec<Vec<(usize, f64)>> = vec![Vec::new(); node_count];
        for node_id in 0..node_count {
            termination_flag.assert_running();
            let stream = self
                .graph
                .stream_relationships_weighted(node_id as i64, weight_fallback);
            for cursor in stream {
                let t = cursor.target_id();
                if t >= 0 {
                    adj[node_id].push((t as usize, cursor.weight()));
                }
            }
            progress_tracker.log_progress(1);
        }

        let input = ModularityOptimizationInput::new(node_count, adj);
        let result = computation.compute(&input);

        progress_tracker.log_progress(1);
        progress_tracker.end_subtask();

        Ok(result)
    }
}
