use super::computation::GATComputationRuntime;
use super::config::GATConfig;
use super::types::GATResult;
use crate::graph::Graph;

pub struct GATStorageRuntime;

impl GATStorageRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn compute(&self, graph: &dyn Graph, config: &GATConfig) -> GATResult {
        GATComputationRuntime::run(graph, config)
    }
}
