use super::super::super::loaders::GraphStoreCatalogService;
use crate::applications::graph_store_catalog::results::GraphMemoryUsage;
use crate::core::User;
use crate::types::graph_store::{DatabaseId, GraphStore};
use std::collections::HashMap;

/// Application for computing graph memory usage.
///
/// Mirrors Java GraphMemoryUsageApplication class.
/// Single compute method that returns memory usage information.
pub struct GraphMemoryUsageApplication {
    graph_store_catalog_service: Box<dyn GraphStoreCatalogService>,
}

impl GraphMemoryUsageApplication {
    /// Creates a new GraphMemoryUsageApplication.
    pub fn new(graph_store_catalog_service: Box<dyn GraphStoreCatalogService>) -> Self {
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
        // In Java, this would call the catalog service to get actual memory usage
        let graph_store =
            self.graph_store_catalog_service
                .get_graph_store(user, database_id, graph_name);

        let estimated_bytes = self.estimate_memory_usage(&graph_store);
        GraphMemoryUsage::new(
            graph_name.to_string(),
            format!("{} bytes", estimated_bytes),
            estimated_bytes,
            HashMap::new(),
            graph_store.node_count() as u64,
            graph_store.relationship_count() as u64,
        )
    }

    /// Estimates memory usage based on graph dimensions.
    fn estimate_memory_usage<G: GraphStore>(&self, graph_store: &G) -> u64 {
        // Simple estimation: nodes * 100 bytes + relationships * 50 bytes
        (graph_store.node_count() as u64) * 100 + (graph_store.relationship_count() as u64) * 50
    }
}
