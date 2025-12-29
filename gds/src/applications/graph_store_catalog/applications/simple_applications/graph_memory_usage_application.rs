use super::super::super::loaders::GraphStoreCatalogService;
use crate::applications::graph_store_catalog::results::GraphMemoryUsage;
use crate::core::User;
use crate::types::catalog::GraphCatalog;
use crate::types::graph_store::DatabaseId;
use std::collections::HashMap;
use std::sync::Arc;

/// Application for computing graph memory usage.
///
/// Mirrors Java GraphMemoryUsageApplication class.
/// Single compute method that returns memory usage information.
pub struct GraphMemoryUsageApplication {
    graph_store_catalog_service: Arc<dyn GraphStoreCatalogService>,
}

impl GraphMemoryUsageApplication {
    /// Creates a new GraphMemoryUsageApplication.
    pub fn new(graph_store_catalog_service: Arc<dyn GraphStoreCatalogService>) -> Self {
        Self {
            graph_store_catalog_service,
        }
    }

    /// Computes the memory usage for a graph.
    ///
    /// In Java, this calls graphStoreCatalogService.sizeOf(user, databaseId, graphName).
    /// Returns GraphMemoryUsage with memory statistics.
    pub fn compute(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
    ) -> GraphMemoryUsage {
        // For now we route through the catalog substrate.
        let catalog = self
            .graph_store_catalog_service
            .graph_catalog(user, database_id);
        match GraphCatalog::size_of(catalog.as_ref(), graph_name) {
            Ok(mu) => GraphMemoryUsage::new(
                graph_name.to_string(),
                format!("{} bytes", mu.bytes),
                mu.bytes,
                HashMap::new(),
                mu.nodes,
                mu.relationships,
            ),
            Err(_) => GraphMemoryUsage::new(
                graph_name.to_string(),
                "0 bytes".to_string(),
                0,
                HashMap::new(),
                0,
                0,
            ),
        }
    }
}
