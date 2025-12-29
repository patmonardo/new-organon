use crate::applications::graph_store_catalog::services::progress_tracker_factory::{
    TaskRegistryFactory, UserLogRegistryFactory,
};
use crate::applications::services::logging::Log;
use crate::projection::RelationshipType;
use crate::types::graph_store::{DefaultGraphStore, DeletionResult, GraphStore};

/// Application for dropping relationships from graphs.
///
/// Mirrors Java DropRelationshipsApplication class.
/// Removes relationships of specified type from the graph store.
pub struct DropRelationshipsApplication {
    log: Log,
}

impl DropRelationshipsApplication {
    pub fn new(log: Log) -> Self {
        Self { log }
    }

    pub fn compute(
        &self,
        _task_registry_factory: &TaskRegistryFactory,
        _user_log_registry_factory: &UserLogRegistryFactory,
        graph_store: &DefaultGraphStore,
        relationship_type: &str,
    ) -> Result<(DefaultGraphStore, DeletionResult), String> {
        let mut modified_store = graph_store.clone();
        let rel_type = RelationshipType::of(relationship_type);

        match modified_store.delete_relationships(&rel_type) {
            Ok(deletion_result) => {
                self.log.info(&format!(
                    "Deleted relationships of type '{}': {} relationships removed",
                    relationship_type,
                    deletion_result.deleted_relationship_count().unwrap_or(0)
                ));
                Ok((modified_store, deletion_result))
            }
            Err(e) => Err(format!(
                "Failed to delete relationships of type '{}': {}",
                relationship_type, e
            )),
        }
    }
}
