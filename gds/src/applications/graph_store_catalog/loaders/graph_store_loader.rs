/// Base trait for loading graph stores.
///
/// Mirrors Java GraphStoreLoader interface.
/// Base trait with 4 methods for graph loading operations.
use crate::core::{GraphDimensions, User};
use crate::types::catalog::GraphCatalog;
use crate::types::graph_store::{DatabaseId, DefaultGraphStore};
pub trait GraphStoreLoader {
    /// Returns the graph project configuration.
    fn graph_project_config(&self) -> Box<dyn GraphProjectConfig>;

    /// Returns the loaded graph store.
    fn graph_store(&self) -> DefaultGraphStore;

    /// Returns the result store for the operation.
    fn result_store(&self) -> Box<dyn ResultStore>;

    /// Returns the graph dimensions.
    fn graph_dimensions(&self) -> Box<dyn GraphDimensions>;
}

/// Placeholder for GraphProjectConfig trait.
/// In real implementation, this would be the actual GraphProjectConfig type.
pub trait GraphProjectConfig {
    fn graph_name(&self) -> &str;
    fn username(&self) -> &str;
}

/// Placeholder for ResultStore trait.
/// In real implementation, this would be the actual ResultStore type.
pub trait ResultStore {
    fn is_empty(&self) -> bool;
}

/// GraphStoreCatalogService
///
/// Minimal loader/service boundary for the GraphStore Catalog applications layer.
/// This is the *application-facing* abstraction over the underlying `types::catalog::GraphCatalog`.
///
/// Notes:
/// - This is intentionally small today: it just exposes the catalog substrate and a helper
///   for resolving named `GraphStore`s.
/// - As we port more of the Java GDS subsystem, this will grow (permissions, per-db catalogs, etc.).
pub trait GraphStoreCatalogService: Send + Sync {
    /// Returns the catalog substrate for a given user/database.
    ///
    /// For now this is typically a single in-memory catalog, but the signature keeps the door open
    /// for per-user/per-db scoping later.
    fn graph_catalog(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
    ) -> std::sync::Arc<dyn GraphCatalog>;

    /// Resolve a named graph store from the catalog.
    fn get_graph_store(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
    ) -> Result<std::sync::Arc<DefaultGraphStore>, String> {
        self.graph_catalog(user, database_id)
            .get(graph_name)
            .ok_or_else(|| format!("Graph not found: {graph_name}"))
    }
}
