use super::GATComputationRuntime;
use super::config::GATConfig;
use super::types::GATResult;
use crate::types::graph::Graph;

pub struct GATStorageRuntime;

impl Default for GATStorageRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl GATStorageRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn compute(&self, graph: &dyn Graph, config: &GATConfig) -> GATResult {
        GATComputationRuntime::run(graph, config)
    }
}
