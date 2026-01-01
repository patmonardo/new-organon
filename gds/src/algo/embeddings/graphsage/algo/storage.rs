//! GraphSAGE Storage Runtime

use super::computation::GraphSageComputationRuntime;
use super::spec::{GraphSageConfig, GraphSageResult};
use crate::graph::Graph;

pub struct GraphSageStorageRuntime;

impl Default for GraphSageStorageRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphSageStorageRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn compute(&self, graph: &dyn Graph, config: &GraphSageConfig) -> GraphSageResult {
        GraphSageComputationRuntime::run(graph, config)
    }
}
