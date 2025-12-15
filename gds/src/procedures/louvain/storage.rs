use crate::types::graph::Graph;

use super::computation::LouvainComputationRuntime;
use super::spec::LouvainResult;

pub struct LouvainStorageRuntime {
    #[allow(dead_code)]
    concurrency: usize,
}

impl LouvainStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }

    pub fn compute_louvain(
        &self,
        computation: &mut LouvainComputationRuntime,
        graph: &dyn Graph,
    ) -> LouvainResult {
        let node_count = graph.node_count();
        let fallback = graph.default_property_value();

        let get_neighbors = |node: usize| -> Vec<usize> {
            let id = node as u64;
            let mut out: Vec<usize> = graph
                .stream_relationships(id as i64, fallback)
                .map(|c| c.target_id() as usize)
                .collect();
            let mut inc: Vec<usize> = graph
                .stream_inverse_relationships(id as i64, fallback)
                .map(|c| c.source_id() as usize)
                .collect();
            out.append(&mut inc);
            out
        };

        computation.compute(node_count, get_neighbors)
    }
}
