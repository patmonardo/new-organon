use crate::core::User;
use crate::applications::graph_store_catalog::facade::GraphStoreCatalogEntry;

/// Service for listing graphs in the catalog.
///
/// Mirrors Java GraphListingService class.
/// Simple accessor service for retrieving graph catalog entries.
pub struct GraphListingService {
    // In Java, this would hold a GraphStoreCatalogService
}

impl GraphListingService {
    /// Creates a new GraphListingService.
    pub fn new() -> Self {
        Self {}
    }

    /// Lists all graphs for a user.
    /// In Java, this calls graphStoreCatalogService.getAllGraphStores() or similar.
    pub fn list_graphs(&self, _user: &dyn User) -> Vec<GraphStoreCatalogEntry> {
        // Placeholder implementation - in real implementation would query catalog
        vec![]
    }

    /// Lists graphs for a specific user.
    /// In Java, this filters by user permissions.
    pub fn list_for_user(&self, user: &dyn User) -> Vec<GraphStoreCatalogEntry> {
        // Placeholder implementation - in real implementation would filter by user
        self.list_graphs(user)
    }
}

impl Default for GraphListingService {
    fn default() -> Self {
        Self::new()
    }
}

