use crate::applications::graph_store_catalog::results::WriteResult;
use crate::applications::services::logging::Log;
use crate::projection::RelationshipType;
use crate::types::graph_store::DefaultGraphStore;
use crate::types::graph_store::GraphStore as _;

/// WriteRelationshipsApplication
///
/// Java parity: exports relationships back to the host database.
/// Rust pass-1: compute *what would be written* from the current GraphStore.
#[derive(Clone, Debug)]
pub struct WriteRelationshipsApplication {
    #[allow(dead_code)]
    log: Log,
}

impl WriteRelationshipsApplication {
    pub fn new(log: Log) -> Self {
        Self { log }
    }

    pub fn compute(
        &self,
        graph_store: &DefaultGraphStore,
        relationship_type: &str,
    ) -> Result<WriteResult, String> {
        let t = relationship_type.trim();
        if t.is_empty() {
            return Err("relationship_type must not be empty".to_string());
        }

        let rel_type = RelationshipType::of(t);
        if !graph_store.has_relationship_type(&rel_type) {
            return Err(format!("Unknown relationship type '{t}'"));
        }

        let relationships_written = graph_store.relationship_count_for_type(&rel_type) as u64;
        Ok(WriteResult::new(0, relationships_written, 0))
    }
}

impl Default for WriteRelationshipsApplication {
    fn default() -> Self {
        Self::new(Log::default())
    }
}
