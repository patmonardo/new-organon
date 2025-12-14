use crate::applications::graph_store_catalog::services::progress_tracker_factory::{
    TaskRegistryFactory, UserLogRegistryFactory,
};
use crate::applications::services::logging::Log;
use crate::types::graph_store::{DeletionResult, GraphName, GraphStore};

/// Application for dropping relationships from graphs.
///
/// Placeholder implementation: validates wiring and returns a `DeletionResult`.
pub struct DropRelationshipsApplication {
    log: Log,
}

impl DropRelationshipsApplication {
    pub fn new(log: Log) -> Self {
        Self { log }
    }

    pub fn compute<G: GraphStore>(
        &self,
        _task_registry_factory: &TaskRegistryFactory,
        _user_log_registry_factory: &UserLogRegistryFactory,
        _graph_store: &G,
        _relationship_type: &str,
    ) -> DeletionResult {
        let _ = &self.log;
        DeletionResult::new(GraphName::new("fictitious"))
    }
}

