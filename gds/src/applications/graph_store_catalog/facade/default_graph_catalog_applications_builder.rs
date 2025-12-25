use super::*;
use crate::applications::services::logging::Log;
use crate::types::catalog::{GraphCatalog, InMemoryGraphCatalog};
use std::sync::Arc;

use crate::applications::graph_store_catalog::applications::*;
use crate::applications::graph_store_catalog::loaders::GraphStoreCatalogService;
use crate::applications::graph_store_catalog::services::progress_tracker_factory::{
    TaskRegistryFactory, UserLogRegistryFactory,
};

/// Builder for DefaultGraphCatalogApplications.
///
/// Mirrors Java DefaultGraphCatalogApplicationsBuilder class.
/// Implements the builder pattern for constructing the facade with all dependencies.
pub struct DefaultGraphCatalogApplicationsBuilder {
    pub log: Log,
    pub graph_store_catalog_service: Arc<dyn GraphStoreCatalogService>,
    pub graph_memory_usage_application: GraphMemoryUsageApplication,
    pub drop_graph_application: DropGraphApplication,
    pub drop_node_properties_application: DropNodePropertiesApplication,
    pub drop_relationships_application: DropRelationshipsApplication,
    pub stream_node_properties_application: StreamNodePropertiesApplication,
    pub stream_relationship_properties_application: StreamRelationshipPropertiesApplication,
    pub stream_relationships_application: StreamRelationshipsApplication,
    pub write_node_properties_application: WriteNodePropertiesApplication,
    pub write_node_label_application: WriteNodeLabelApplication,
    pub write_relationship_properties_application: WriteRelationshipPropertiesApplication,
    pub write_relationships_application: WriteRelationshipsApplication,
    pub export_to_csv_application: ExportToCsvApplication,
    pub export_to_database_application: ExportToDatabaseApplication,
    pub native_project_application: NativeProjectApplication,
    pub generic_project_application: GenericProjectApplication,
    pub generate_graph_application: GenerateGraphApplication,
    pub graph_sampling_application: GraphSamplingApplication,
    pub task_registry_factory: TaskRegistryFactory,
    pub user_log_registry_factory: UserLogRegistryFactory,
}

impl DefaultGraphCatalogApplicationsBuilder {
    /// Creates a new builder with default values.
    pub fn new(log: Log) -> Self {
        // Default substrate: in-memory catalog
        let catalog: Arc<dyn GraphCatalog> = Arc::new(InMemoryGraphCatalog::new());
        let graph_store_catalog_service: Arc<dyn GraphStoreCatalogService> =
            Arc::new(CatalogBackedGraphStoreCatalogService::new(catalog.clone()));

        Self {
            log: log.clone(),
            graph_store_catalog_service: graph_store_catalog_service.clone(),
            graph_memory_usage_application: GraphMemoryUsageApplication::new(
                graph_store_catalog_service.clone(),
            ),
            drop_graph_application: DropGraphApplication::new(graph_store_catalog_service.clone()),
            drop_node_properties_application: DropNodePropertiesApplication::new(log.clone()),
            drop_relationships_application: DropRelationshipsApplication::new(log.clone()),
            stream_node_properties_application: StreamNodePropertiesApplication,
            stream_relationship_properties_application: StreamRelationshipPropertiesApplication,
            stream_relationships_application: StreamRelationshipsApplication,
            write_node_properties_application: WriteNodePropertiesApplication,
            write_node_label_application: WriteNodeLabelApplication,
            write_relationship_properties_application: WriteRelationshipPropertiesApplication,
            write_relationships_application: WriteRelationshipsApplication,
            export_to_csv_application: ExportToCsvApplication,
            export_to_database_application: ExportToDatabaseApplication,
            native_project_application: NativeProjectApplication,
            generic_project_application: GenericProjectApplication,
            generate_graph_application: GenerateGraphApplication,
            graph_sampling_application: GraphSamplingApplication,
            task_registry_factory: TaskRegistryFactory::new(),
            user_log_registry_factory: UserLogRegistryFactory::new(),
        }
    }

    /// Sets the graph store catalog service.
    pub fn with_graph_store_catalog_service(
        mut self,
        service: Arc<dyn GraphStoreCatalogService>,
    ) -> Self {
        self.graph_store_catalog_service = service.clone();
        self.graph_memory_usage_application = GraphMemoryUsageApplication::new(service.clone());
        self.drop_graph_application = DropGraphApplication::new(service.clone());
        self
    }

    /// Sets the task registry factory.
    pub fn with_task_registry_factory(mut self, factory: TaskRegistryFactory) -> Self {
        self.task_registry_factory = factory;
        self
    }

    /// Sets the user log registry factory.
    pub fn with_user_log_registry_factory(mut self, factory: UserLogRegistryFactory) -> Self {
        self.user_log_registry_factory = factory;
        self
    }

    /// Builds the DefaultGraphCatalogApplications.
    pub fn build(self) -> DefaultGraphCatalogApplications {
        DefaultGraphCatalogApplications::new(self)
    }
}

/// Minimal GraphStoreCatalogService backed by a `types::catalog::GraphCatalog`.
#[derive(Clone)]
pub struct CatalogBackedGraphStoreCatalogService {
    catalog: Arc<dyn GraphCatalog>,
}

impl CatalogBackedGraphStoreCatalogService {
    pub fn new(catalog: Arc<dyn GraphCatalog>) -> Self {
        Self { catalog }
    }
}

impl GraphStoreCatalogService for CatalogBackedGraphStoreCatalogService {
    fn graph_catalog(
        &self,
        _user: &dyn crate::core::User,
        _database_id: &crate::types::graph_store::DatabaseId,
    ) -> Arc<dyn GraphCatalog> {
        self.catalog.clone()
    }
}
