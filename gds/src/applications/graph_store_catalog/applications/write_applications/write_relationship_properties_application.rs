use crate::applications::graph_store_catalog::results::WriteResult;
use crate::applications::services::logging::Log;
use crate::types::graph_store::DefaultGraphStore;
use crate::types::graph_store::GraphStore as _;

/// WriteRelationshipPropertiesApplication
///
/// Java parity: writes relationship properties back to the host database.
/// Rust pass-1: compute *what would be written* from the current GraphStore.
#[derive(Clone, Debug)]
pub struct WriteRelationshipPropertiesApplication {
    #[allow(dead_code)]
    log: Log,
}

impl WriteRelationshipPropertiesApplication {
    pub fn new(log: Log) -> Self {
        Self { log }
    }

    pub fn compute(
        &self,
        graph_store: &DefaultGraphStore,
        relationship_properties: &[String],
    ) -> Result<WriteResult, String> {
        if relationship_properties.is_empty() {
            return Err("relationship_properties must not be empty".to_string());
        }

        for k in relationship_properties.iter() {
            let t = k.trim();
            if t.is_empty() {
                return Err(
                    "relationship_properties must not contain empty property names".to_string(),
                );
            }

            // Relationship properties are namespaced per relationship type; accept when present
            // for at least one type.
            let mut found = false;
            for rel_type in graph_store.relationship_types() {
                if graph_store.has_relationship_property(&rel_type, t) {
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(format!("Unknown relationship property '{t}'"));
            }
        }

        let relationships_written = graph_store.relationship_count() as u64;
        let properties_written = relationships_written * relationship_properties.len() as u64;

        Ok(WriteResult::new(
            0,
            relationships_written,
            properties_written,
        ))
    }
}

impl Default for WriteRelationshipPropertiesApplication {
    fn default() -> Self {
        Self::new(Log::default())
    }
}
