use crate::applications::graph_store_catalog::services::progress_tracker_factory::{
    TaskRegistryFactory, UserLogRegistryFactory,
};
use crate::applications::services::logging::Log;
use crate::types::graph_store::{DefaultGraphStore, GraphStore};

/// Application for dropping node properties from graphs.
///
/// Mirrors Java DropNodePropertiesApplication class.
/// Removes specified node properties from the graph store.
pub struct DropNodePropertiesApplication {
    log: Log,
}

impl DropNodePropertiesApplication {
    pub fn new(log: Log) -> Self {
        Self { log }
    }

    pub fn compute(
        &self,
        _task_registry_factory: &TaskRegistryFactory,
        _user_log_registry_factory: &UserLogRegistryFactory,
        node_properties: &[String],
        graph_store: &DefaultGraphStore,
    ) -> Result<(DefaultGraphStore, u64), String> {
        let mut modified_store = graph_store.clone();
        let mut removed_count = 0u64;

        for property_key in node_properties {
            match modified_store.remove_node_property(property_key) {
                Ok(()) => {
                    removed_count += 1;
                    // Log successful removal
                    self.log
                        .info(&format!("Removed node property: {}", property_key));
                }
                Err(e) => {
                    // Log the error but continue with other properties
                    self.log.warn(&format!(
                        "Failed to remove node property '{}': {}",
                        property_key, e
                    ));
                }
            }
        }

        Ok((modified_store, removed_count))
    }
}
