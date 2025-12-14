/// Base trait for loading graph stores.
///
/// Mirrors Java GraphStoreLoader interface.
/// Base trait with 4 methods for graph loading operations.
pub trait GraphStoreLoader {
    /// Returns the graph project configuration.
    fn graph_project_config(&self) -> Box<dyn GraphProjectConfig>;

    /// Returns the loaded graph store.
    fn graph_store(&self) -> crate::types::graph_store::DefaultGraphStore;

    /// Returns the result store for the operation.
    fn result_store(&self) -> Box<dyn ResultStore>;

    /// Returns the graph dimensions.
    fn graph_dimensions(&self) -> Box<dyn crate::core::GraphDimensions>;
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

/// Placeholder for GraphStoreCatalogService trait.
/// In real implementation, this would be the actual catalog service.
pub trait GraphStoreCatalogService {
    fn get_graph_store(
        &self,
        user: &dyn crate::core::User,
        database_id: &crate::types::graph_store::DatabaseId,
        graph_name: &str,
    ) -> crate::types::graph_store::DefaultGraphStore;
}
