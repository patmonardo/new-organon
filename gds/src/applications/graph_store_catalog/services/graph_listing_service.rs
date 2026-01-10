use crate::applications::graph_store_catalog::loaders::GraphStoreCatalogService;
use crate::core::User;
use crate::types::catalog::{GraphCatalog, ListEntry};
use crate::types::graph_store::DatabaseId;
use std::sync::Arc;

/// Service for listing graphs in the catalog.
///
/// Mirrors Java GraphListingService class.
/// Simple accessor service for retrieving graph catalog entries.
#[derive(Clone)]
pub struct GraphListingService {
    graph_store_catalog_service: Arc<dyn GraphStoreCatalogService>,
}

impl GraphListingService {
    /// Creates a new GraphListingService.
    pub fn new(graph_store_catalog_service: Arc<dyn GraphStoreCatalogService>) -> Self {
        Self {
            graph_store_catalog_service,
        }
    }

    /// Lists all graphs for a user.
    /// In Java, this calls graphStoreCatalogService.getAllGraphStores() or similar.
    pub fn list_graphs(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: Option<&str>,
        include_degree_distribution: bool,
    ) -> Vec<ListEntry> {
        let catalog = self
            .graph_store_catalog_service
            .graph_catalog(user, database_id);
        GraphCatalog::list(catalog.as_ref(), graph_name, include_degree_distribution)
    }

    /// Lists graphs for a specific user.
    /// In Java, this filters by user permissions.
    pub fn list_for_user(&self, user: &dyn User, database_id: &DatabaseId) -> Vec<ListEntry> {
        // Placeholder policy: no additional filtering yet.
        self.list_graphs(user, database_id, None, false)
    }
}

impl Default for GraphListingService {
    fn default() -> Self {
        panic!("GraphListingService::default() is not supported; construct with a GraphStoreCatalogService");
    }
}
