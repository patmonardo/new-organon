use crate::applications::graph_store_catalog::results::ExportResult;
use crate::types::graph_store::DefaultGraphStore;

/// ExportToDatabaseApplication
///
/// Java parity: exports the graph back into a Neo4j database.
/// Rust: we don't have a DB integration layer yet, so this returns a clear error.
#[derive(Clone, Debug, Default)]
pub struct ExportToDatabaseApplication;

impl ExportToDatabaseApplication {
    pub fn compute(
        &self,
        _graph_store: &DefaultGraphStore,
        _target_database: &str,
    ) -> Result<ExportResult, String> {
        Err(
            "export_to_database is not supported yet (no database integration in Rust pass-1)"
                .to_string(),
        )
    }
}
