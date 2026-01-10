use super::super::super::loaders::GraphStoreCatalogService;
use crate::core::User;
use crate::types::catalog::{Dropped, GraphCatalog};
use crate::types::graph_store::DatabaseId;
use std::sync::Arc;

/// Application for dropping graphs from the catalog.
///
/// Mirrors Java DropGraphApplication class.
/// Contains graph dropping logic with validation and error handling.
pub struct DropGraphApplication {
    graph_store_catalog_service: Arc<dyn GraphStoreCatalogService>,
}

impl DropGraphApplication {
    /// Creates a new DropGraphApplication.
    pub fn new(graph_store_catalog_service: Arc<dyn GraphStoreCatalogService>) -> Self {
        Self {
            graph_store_catalog_service,
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
        database_id: &DatabaseId,
        operator: &dyn User,
        username_override: Option<&str>,
    ) -> Result<Vec<Dropped>, String> {
        let _username_override = username_override;

        let catalog = self
            .graph_store_catalog_service
            .graph_catalog(operator, database_id);
        let refs: Vec<&str> = graph_names.iter().map(|s| s.as_str()).collect();

        let dropped = GraphCatalog::drop(catalog.as_ref(), &refs, should_fail_if_missing)
            .map_err(|e| e.to_string())?;

        Ok(dropped)
    }
}
