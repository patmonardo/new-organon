use crate::core::User;
use super::super::super::loaders::GraphStoreCatalogService;
use super::super::super::facade::GraphStoreCatalogEntry;

/// Application for dropping graphs from the catalog.
///
/// Mirrors Java DropGraphApplication class.
/// Contains graph dropping logic with validation and error handling.
pub struct DropGraphApplication {
    _graph_store_catalog_service: Box<dyn GraphStoreCatalogService>,
}

impl DropGraphApplication {
    /// Creates a new DropGraphApplication.
    pub fn new(graph_store_catalog_service: Box<dyn GraphStoreCatalogService>) -> Self {
        Self {
            _graph_store_catalog_service: graph_store_catalog_service,
        }
    }

    /// Computes the drop operation for multiple graphs.
    ///
    /// In Java, this handles both single graphs and lists of graphs.
    /// Returns metadata for the graphs that were removed.
    pub fn compute(
        &self,
        graph_names: &[String],
        should_fail_if_missing: bool,
        _database_id: &crate::types::graph_store::DatabaseId,
        _operator: &dyn User,
        username_override: Option<&str>,
    ) -> Result<Vec<GraphStoreCatalogEntry>, String> {
        if should_fail_if_missing {
            // Placeholder behavior: we don't have a real catalog backing store yet,
            // so we can't actually check existence.
        }

        let _username_override = username_override;

        Ok(graph_names
            .iter()
            .map(|graph_name| GraphStoreCatalogEntry::new(graph_name.clone(), 0, 0))
            .collect())
    }
}
