use crate::applications::graph_store_catalog::services::progress_tracker_factory::{
    TaskRegistryFactory, UserLogRegistryFactory,
};
use crate::applications::services::logging::Log;
use crate::types::graph_store::GraphStore;

/// Application for dropping node properties from graphs.
///
/// Placeholder implementation: validates wiring and returns a count.
pub struct DropNodePropertiesApplication {
    log: Log,
}

impl DropNodePropertiesApplication {
    pub fn new(log: Log) -> Self {
        Self { log }
    }

    pub fn compute<G: GraphStore>(
        &self,
        _task_registry_factory: &TaskRegistryFactory,
        _user_log_registry_factory: &UserLogRegistryFactory,
        node_properties: &[String],
        _graph_store: &G,
    ) -> u64 {
        let _ = &self.log;
        node_properties.len() as u64
    }
}
