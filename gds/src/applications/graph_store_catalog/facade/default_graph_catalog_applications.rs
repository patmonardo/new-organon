use super::*;
use crate::applications::services::logging::Log;
use crate::core::User;
use crate::types::graph_store::{DatabaseId, DeletionResult};
use crate::applications::graph_store_catalog::services::GraphListingService;
use std::sync::Arc;

use crate::applications::graph_store_catalog::applications::{
    DropGraphApplication, DropNodePropertiesApplication, DropRelationshipsApplication,
    ExportToCsvApplication, ExportToDatabaseApplication, GenerateGraphApplication,
    GenericProjectApplication, GraphMemoryUsageApplication, GraphSamplingApplication,
    ListGraphApplication, NativeProjectApplication, StreamNodePropertiesApplication,
    StreamRelationshipPropertiesApplication, StreamRelationshipsApplication,
    WriteNodeLabelApplication, WriteNodePropertiesApplication,
    WriteRelationshipPropertiesApplication, WriteRelationshipsApplication,
};
use crate::applications::graph_store_catalog::loaders::GraphStoreCatalogService;
use crate::applications::graph_store_catalog::results::GraphMemoryUsage;
use crate::applications::graph_store_catalog::services::progress_tracker_factory::{
    TaskRegistryFactory, UserLogRegistryFactory,
};

/// Default implementation of GraphCatalogApplications.
///
/// Mirrors Java DefaultGraphCatalogApplications class.
/// This is the concrete implementation that orchestrates all the applications.
#[allow(dead_code)]
pub struct DefaultGraphCatalogApplications {
    log: Log,
    graph_store_catalog_service: Arc<dyn GraphStoreCatalogService>,
    graph_listing_service: GraphListingService,
    graph_memory_usage_application: GraphMemoryUsageApplication,
    list_graph_application: ListGraphApplication,
    drop_graph_application: DropGraphApplication,
    drop_node_properties_application: DropNodePropertiesApplication,
    drop_relationships_application: DropRelationshipsApplication,
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
    task_registry_factory: TaskRegistryFactory,
    user_log_registry_factory: UserLogRegistryFactory,
}

impl DefaultGraphCatalogApplications {
    /// Creates a new DefaultGraphCatalogApplications using the builder pattern.
    pub fn new(builder: DefaultGraphCatalogApplicationsBuilder) -> Self {
        let graph_listing_service =
            GraphListingService::new(builder.graph_store_catalog_service.clone());
        Self {
            log: builder.log,
            graph_store_catalog_service: builder.graph_store_catalog_service.clone(),
            graph_listing_service: graph_listing_service.clone(),
            graph_memory_usage_application: builder.graph_memory_usage_application,
            list_graph_application: ListGraphApplication::new(graph_listing_service),
            drop_graph_application: builder.drop_graph_application,
            drop_node_properties_application: builder.drop_node_properties_application,
            drop_relationships_application: builder.drop_relationships_application,
            stream_node_properties_application: builder.stream_node_properties_application,
            stream_relationship_properties_application: builder
                .stream_relationship_properties_application,
            stream_relationships_application: builder.stream_relationships_application,
            write_node_properties_application: builder.write_node_properties_application,
            write_node_label_application: builder.write_node_label_application,
            write_relationship_properties_application: builder
                .write_relationship_properties_application,
            write_relationships_application: builder.write_relationships_application,
            export_to_csv_application: builder.export_to_csv_application,
            export_to_database_application: builder.export_to_database_application,
            native_project_application: builder.native_project_application,
            generic_project_application: builder.generic_project_application,
            generate_graph_application: builder.generate_graph_application,
            graph_sampling_application: builder.graph_sampling_application,
            task_registry_factory: builder.task_registry_factory,
            user_log_registry_factory: builder.user_log_registry_factory,
        }
    }
}

impl GraphCatalogApplications for DefaultGraphCatalogApplications {
    fn graph_exists(&self, user: &dyn User, database_id: &DatabaseId, graph_name: &str) -> bool {
        let catalog = self.graph_store_catalog_service.graph_catalog(user, database_id);
        catalog.get(graph_name).is_some()
    }

    fn list_graphs(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: Option<&str>,
        include_degree_distribution: bool,
    ) -> Vec<GraphStoreCatalogEntry> {
        self.graph_listing_service
            .list_graphs(user, database_id, graph_name, include_degree_distribution)
    }

    fn list_graphs_json(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: Option<&str>,
        include_degree_distribution: bool,
    ) -> serde_json::Value {
        self.list_graph_application.compute(graph_name, include_degree_distribution, user, database_id)
    }

    fn graph_memory_usage(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
    ) -> GraphMemoryUsage {
        self.graph_memory_usage_application
            .compute(user, database_id, graph_name)
    }

    fn drop_graph(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        fail_if_missing: bool,
    ) -> Result<GraphStoreCatalogEntry, String> {
        let results = self.drop_graph_application.compute(
            &[graph_name.to_string()],
            fail_if_missing,
            database_id,
            user,
            None,
        )?;
        results
            .into_iter()
            .next()
            .ok_or_else(|| "No graph was dropped".to_string())
    }

    fn drop_graphs(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_names: &[String],
        fail_if_missing: bool,
    ) -> Result<Vec<GraphStoreCatalogEntry>, String> {
        self.drop_graph_application
            .compute(graph_names, fail_if_missing, database_id, user, None)
    }

    fn drop_node_properties(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        node_properties: &[String],
        _fail_if_missing: bool,
    ) -> Result<u64, String> {
        // Get graph store from catalog
        let graph_store = self
            .graph_store_catalog_service
            .get_graph_store(user, database_id, graph_name)?;

        // Use the drop application
        let (modified_store, count) = self.drop_node_properties_application.compute(
            &self.task_registry_factory,
            &self.user_log_registry_factory,
            node_properties,
            graph_store.as_ref(),
        )?;

        // Put the modified store back in the catalog
        self.graph_store_catalog_service
            .graph_catalog(user, database_id)
            .set(graph_name, Arc::new(modified_store));

        Ok(count)
    }

    fn drop_relationships(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        relationship_type: &str,
    ) -> Result<DeletionResult, String> {
        // Get graph store from catalog
        let graph_store = self
            .graph_store_catalog_service
            .get_graph_store(user, database_id, graph_name)?;

        // Use the drop application
        let (modified_store, deletion_result) = self.drop_relationships_application.compute(
            &self.task_registry_factory,
            &self.user_log_registry_factory,
            graph_store.as_ref(),
            relationship_type,
        )?;

        // Put the modified store back in the catalog
        self.graph_store_catalog_service
            .graph_catalog(user, database_id)
            .set(graph_name, Arc::new(modified_store));

        Ok(deletion_result)
    }

    fn stream_node_properties(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        node_properties: &[String],
        node_labels: &[String],
        list_node_labels: bool,
    ) -> Result<Vec<crate::applications::graph_store_catalog::results::GraphStreamNodePropertiesResult>, String> {
        let graph_store = self
            .graph_store_catalog_service
            .get_graph_store(user, database_id, graph_name)?;

        // Java parity: ElementProjection.PROJECT_ALL is "*" and means "all labels".
        // We treat "*" as wildcard by passing an empty filter down to the application.
        let want_labels: Vec<crate::projection::NodeLabel> = if node_labels.iter().any(|s| s.trim() == "*") {
            Vec::new()
        } else {
            node_labels
                .iter()
                .filter_map(|s| {
                    let t = s.trim();
                    if t.is_empty() {
                        None
                    } else {
                        Some(crate::projection::NodeLabel::of(t))
                    }
                })
                .collect()
        };

        self.stream_node_properties_application.compute(
            graph_store.as_ref(),
            node_properties,
            &want_labels,
            list_node_labels,
        )
    }

    fn stream_relationship_properties(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        relationship_properties: &[String],
        relationship_types: &[String],
    ) -> Result<
        Vec<crate::applications::graph_store_catalog::results::GraphStreamRelationshipPropertiesResult>,
        String,
    > {
        let graph_store = self
            .graph_store_catalog_service
            .get_graph_store(user, database_id, graph_name)?;

        // Java parity: ElementProjection.PROJECT_ALL is "*" and means "all relationship types".
        // We treat "*" as wildcard by passing an empty filter down to the application.
        let want_types: Vec<crate::projection::RelationshipType> = if relationship_types.iter().any(|s| s.trim() == "*") {
            Vec::new()
        } else {
            relationship_types
                .iter()
                .filter_map(|s| {
                    let t = s.trim();
                    if t.is_empty() {
                        None
                    } else {
                        Some(crate::projection::RelationshipType::of(t))
                    }
                })
                .collect()
        };

        // If `relationship_types` is empty, the application defaults to "all types in store".
        self.stream_relationship_properties_application
            .compute(graph_store.as_ref(), relationship_properties, &want_types)
    }

    fn stream_relationships(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        relationship_types: &[String],
    ) -> Result<Vec<crate::applications::graph_store_catalog::results::TopologyResult>, String> {
        let graph_store = self
            .graph_store_catalog_service
            .get_graph_store(user, database_id, graph_name)?;

        // Java parity: ElementProjection.PROJECT_ALL is "*" and means "all relationship types".
        let want_types: Vec<crate::projection::RelationshipType> = if relationship_types.iter().any(|s| s.trim() == "*") {
            Vec::new()
        } else {
            relationship_types
                .iter()
                .filter_map(|s| {
                    let t = s.trim();
                    if t.is_empty() {
                        None
                    } else {
                        Some(crate::projection::RelationshipType::of(t))
                    }
                })
                .collect()
        };

        self.stream_relationships_application
            .compute(graph_store.as_ref(), &want_types)
    }

    fn write_node_properties(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        node_properties: &[String],
    ) -> Result<WriteResult, String> {
        // Placeholder implementation
        Ok(WriteResult::new(100, 0, node_properties.len() as u64))
    }

    fn write_node_labels(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        _node_labels: &[String],
    ) -> Result<WriteResult, String> {
        // Placeholder implementation
        Ok(WriteResult::new(100, 0, 0))
    }

    fn write_relationship_properties(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        relationship_properties: &[String],
    ) -> Result<WriteResult, String> {
        // Placeholder implementation
        Ok(WriteResult::new(
            0,
            100,
            relationship_properties.len() as u64,
        ))
    }

    fn write_relationships(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        _relationship_type: &str,
    ) -> Result<WriteResult, String> {
        // Placeholder implementation
        Ok(WriteResult::new(0, 100, 0))
    }

    fn export_to_csv(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        export_path: &str,
    ) -> Result<ExportResult, String> {
        // Placeholder implementation
        Ok(ExportResult::new(1000, 5000, Some(export_path.to_string())))
    }

    fn export_to_database(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        _target_database: &str,
    ) -> Result<ExportResult, String> {
        // Placeholder implementation
        Ok(ExportResult::new(1000, 5000, None))
    }

    fn project_native(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        projection_config: &NativeProjectionConfig,
    ) -> Result<ProjectionResult, String> {
        self.native_project_application.project(
            self.graph_store_catalog_service.clone(),
            user,
            database_id,
            projection_config,
        )
    }

    fn project_generic(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        projection_config: &GenericProjectionConfig,
    ) -> Result<ProjectionResult, String> {
        self.generic_project_application.project(
            self.graph_store_catalog_service.clone(),
            user,
            database_id,
            projection_config,
        )
    }

    fn generate_graph(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _generation_config: &GraphGenerationConfig,
    ) -> Result<GenerationResult, String> {
        // Placeholder implementation
        Ok(GenerationResult::new(
            "generated_graph".to_string(),
            2000,
            10000,
            200,
        ))
    }

    fn sample_graph(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        _sampling_config: &SamplingConfig,
    ) -> Result<SamplingResult, String> {
        // Placeholder implementation
        Ok(SamplingResult::new(
            "sampled_graph".to_string(),
            1000,
            500,
            5000,
            2500,
        ))
    }
}
