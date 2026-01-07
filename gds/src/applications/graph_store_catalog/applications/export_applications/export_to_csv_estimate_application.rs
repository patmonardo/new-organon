use crate::applications::graph_store_catalog::results::GraphStoreExportResult;
use crate::types::graph_store::GraphStore;

/// Estimates CSV export statistics without writing files.
/// Mirrors Java `exportToCsvEstimate`.
pub struct ExportToCsvEstimateApplication;

impl ExportToCsvEstimateApplication {
    pub fn new() -> Self {
        Self
    }

    pub fn compute(
        &self,
        graph_store: &impl GraphStore,
        graph_name: &str,
    ) -> GraphStoreExportResult {
        let node_count = graph_store.node_count() as u64;
        let relationship_count = graph_store.relationship_count() as u64;
        let relationship_type_count = graph_store.relationship_types().len() as u64;
        let node_property_count = graph_store.node_property_keys().len() as u64;
        let relationship_property_count = graph_store.relationship_property_keys().len() as u64;

        GraphStoreExportResult::new(
            graph_name.to_string(),
            node_count,
            relationship_count,
            relationship_type_count,
            node_property_count,
            relationship_property_count,
            0,
        )
    }
}

impl Default for ExportToCsvEstimateApplication {
    fn default() -> Self {
        Self::new()
    }
}
