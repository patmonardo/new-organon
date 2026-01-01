use crate::types::graph::graph::Graph;

use super::computation::NodeSimilarityComputationResult;

pub struct NodeSimilarityStorageRuntime {
    concurrency: usize,
}

impl NodeSimilarityStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }

    pub fn compute(
        &self,
        computation: &mut super::computation::NodeSimilarityComputationRuntime,
        graph: &dyn Graph,
        config: &super::spec::NodeSimilarityConfig,
    ) -> Vec<NodeSimilarityComputationResult> {
        // This delegates to the computation runtime which holds the logic.
        // In some GDS algos, storage handles memory estimation and allocation.
        // For similarity, we are collecting results.

        computation.compute(graph, config, self.concurrency)
    }
}
