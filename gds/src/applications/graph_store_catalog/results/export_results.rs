// Export result types - direct translation from Java

/// Base result for graph store export operations.
/// Mirrors Java GraphStoreExportResult abstract class.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GraphStoreExportResult {
    pub graph_name: String,
    pub node_count: u64,
    pub relationship_count: u64,
    pub relationship_type_count: u64,
    pub node_property_count: u64,
    pub relationship_property_count: u64,
    pub write_millis: u64,
}

impl GraphStoreExportResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        graph_name: String,
        node_count: u64,
        relationship_count: u64,
        relationship_type_count: u64,
        node_property_count: u64,
        relationship_property_count: u64,
        write_millis: u64,
    ) -> Self {
        Self {
            graph_name,
            node_count,
            relationship_count,
            relationship_type_count,
            node_property_count,
            relationship_property_count,
            write_millis,
        }
    }
}

/// Lightweight export summary used by CSV/database export stubs.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportResult {
    pub nodes_exported: u64,
    pub relationships_exported: u64,
    pub export_path: Option<String>,
}

impl ExportResult {
    pub fn new(
        nodes_exported: u64,
        relationships_exported: u64,
        export_path: Option<String>,
    ) -> Self {
        Self {
            nodes_exported,
            relationships_exported,
            export_path,
        }
    }
}

/// Result for database export operations.
/// Mirrors Java DatabaseExportResult class.
#[derive(Clone, Debug)]
pub struct DatabaseExportResult {
    pub base: GraphStoreExportResult,
    pub db_name: String,
}

impl DatabaseExportResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        graph_name: String,
        db_name: String,
        node_count: u64,
        relationship_count: u64,
        relationship_type_count: u64,
        node_property_count: u64,
        relationship_property_count: u64,
        write_millis: u64,
    ) -> Self {
        Self {
            base: GraphStoreExportResult::new(
                graph_name,
                node_count,
                relationship_count,
                relationship_type_count,
                node_property_count,
                relationship_property_count,
                write_millis,
            ),
            db_name,
        }
    }
}

/// Result for file export operations.
/// Mirrors Java FileExportResult class.
#[derive(Clone, Debug)]
pub struct FileExportResult {
    pub base: GraphStoreExportResult,
    pub export_name: String,
}

impl FileExportResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        graph_name: String,
        export_name: String,
        node_count: u64,
        relationship_count: u64,
        relationship_type_count: u64,
        node_property_count: u64,
        relationship_property_count: u64,
        write_millis: u64,
    ) -> Self {
        Self {
            base: GraphStoreExportResult::new(
                graph_name,
                node_count,
                relationship_count,
                relationship_type_count,
                node_property_count,
                relationship_property_count,
                write_millis,
            ),
            export_name,
        }
    }
}
