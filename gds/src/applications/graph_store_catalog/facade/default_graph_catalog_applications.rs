use super::*;
use crate::applications::graph_store_catalog::services::GraphListingService;
use crate::applications::services::logging::Log;
use crate::core::User;
use crate::types::graph_store::{DatabaseId, DeletionResult};
use std::sync::Arc;

use crate::applications::graph_store_catalog::applications::{
    DropGraphApplication, DropGraphPropertyApplication, DropNodePropertiesApplication,
    DropRelationshipsApplication, ExportToCsvApplication, ExportToCsvEstimateApplication,
    ExportToDatabaseApplication, GenerateGraphApplication, GenericProjectApplication,
    GraphMemoryUsageApplication, GraphSamplingApplication, ListGraphApplication,
    NativeProjectApplication, NodeLabelMutatorApplication, StreamGraphPropertiesApplication,
    StreamNodePropertiesApplication, StreamRelationshipPropertiesApplication,
    StreamRelationshipsApplication, SubGraphProjectApplication,
    EstimateCommonNeighbourAwareRandomWalkApplication, WriteNodeLabelApplication,
    WriteNodePropertiesApplication, WriteRelationshipPropertiesApplication, WriteRelationshipsApplication,
};
use crate::applications::graph_store_catalog::loaders::GraphStoreCatalogService;
use crate::applications::graph_store_catalog::results::GraphMemoryUsage;
use crate::applications::graph_store_catalog::services::progress_tracker_factory::{
    TaskRegistryFactory, UserLogRegistryFactory,
};
use crate::types::graph_store::GraphStore as _;
use serde_json::Value;
use std::collections::HashMap;

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
    export_to_csv_estimate_application: ExportToCsvEstimateApplication,
    export_to_database_application: ExportToDatabaseApplication,
    drop_graph_property_application: DropGraphPropertyApplication,
    stream_graph_properties_application: StreamGraphPropertiesApplication,
    node_label_mutator_application: NodeLabelMutatorApplication,
    native_project_application: NativeProjectApplication,
    generic_project_application: GenericProjectApplication,
    generate_graph_application: GenerateGraphApplication,
    graph_sampling_application: GraphSamplingApplication,
    sub_graph_project_application: SubGraphProjectApplication,
    estimate_common_neighbour_aware_random_walk_application:
        EstimateCommonNeighbourAwareRandomWalkApplication,
    task_registry_factory: TaskRegistryFactory,
    user_log_registry_factory: UserLogRegistryFactory,
}

impl DefaultGraphCatalogApplications {
    /// Creates a new DefaultGraphCatalogApplications with default dependencies.
    /// This is a convenience method that creates all default services internally.
    pub fn with_defaults(log: Log) -> Self {
        DefaultGraphCatalogApplicationsBuilder::new(log).build()
    }

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
            export_to_csv_estimate_application: builder.export_to_csv_estimate_application,
            export_to_database_application: builder.export_to_database_application,
            drop_graph_property_application: builder.drop_graph_property_application,
            stream_graph_properties_application: builder.stream_graph_properties_application,
            node_label_mutator_application: builder.node_label_mutator_application,
            native_project_application: builder.native_project_application,
            generic_project_application: builder.generic_project_application,
            generate_graph_application: builder.generate_graph_application,
            graph_sampling_application: builder.graph_sampling_application,
            sub_graph_project_application: builder.sub_graph_project_application,
            estimate_common_neighbour_aware_random_walk_application:
                builder.estimate_common_neighbour_aware_random_walk_application,
            task_registry_factory: builder.task_registry_factory,
            user_log_registry_factory: builder.user_log_registry_factory,
        }
    }
}

impl GraphCatalogApplications for DefaultGraphCatalogApplications {
    fn graph_exists(&self, user: &dyn User, database_id: &DatabaseId, graph_name: &str) -> bool {
        let catalog = self
            .graph_store_catalog_service
            .graph_catalog(user, database_id);
        catalog.get(graph_name).is_some()
    }

    fn list_graphs(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: Option<&str>,
        include_degree_distribution: bool,
    ) -> Vec<GraphStoreCatalogEntry> {
        self.graph_listing_service.list_graphs(
            user,
            database_id,
            graph_name,
            include_degree_distribution,
        )
    }

    fn list_graphs_json(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: Option<&str>,
        include_degree_distribution: bool,
    ) -> serde_json::Value {
        self.list_graph_application.compute(
            graph_name,
            include_degree_distribution,
            user,
            database_id,
        )
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
        let graph_store =
            self.graph_store_catalog_service
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
        let graph_store =
            self.graph_store_catalog_service
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

    fn drop_graph_property(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        graph_property: &str,
        fail_if_missing: bool,
    ) -> Result<u64, String> {
        let graph_store =
            self.graph_store_catalog_service
                .get_graph_store(user, database_id, graph_name)?;

        let (modified_store, removed) = self.drop_graph_property_application.compute(
            graph_store.as_ref(),
            graph_property,
            fail_if_missing,
        )?;

        self.graph_store_catalog_service
            .graph_catalog(user, database_id)
            .set(graph_name, Arc::new(modified_store));

        Ok(removed)
    }

    fn stream_graph_property(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        graph_property: &str,
    ) -> Result<
        Vec<crate::applications::graph_store_catalog::results::GraphStreamGraphPropertiesResult>,
        String,
    > {
        let graph_store =
            self.graph_store_catalog_service
                .get_graph_store(user, database_id, graph_name)?;
        self.stream_graph_properties_application
            .compute(graph_store.as_ref(), graph_property)
    }

    fn stream_node_properties(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        node_properties: &[String],
        node_labels: &[String],
        list_node_labels: bool,
    ) -> Result<
        Vec<crate::applications::graph_store_catalog::results::GraphStreamNodePropertiesResult>,
        String,
    > {
        let graph_store =
            self.graph_store_catalog_service
                .get_graph_store(user, database_id, graph_name)?;

        // Java parity: ElementProjection.PROJECT_ALL is "*" and means "all labels".
        // We treat "*" as wildcard by passing an empty filter down to the application.
        let want_labels: Vec<crate::projection::NodeLabel> =
            if node_labels.iter().any(|s| s.trim() == "*") {
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
    >{
        let graph_store =
            self.graph_store_catalog_service
                .get_graph_store(user, database_id, graph_name)?;

        // Java parity: ElementProjection.PROJECT_ALL is "*" and means "all relationship types".
        // We treat "*" as wildcard by passing an empty filter down to the application.
        let want_types: Vec<crate::projection::RelationshipType> =
            if relationship_types.iter().any(|s| s.trim() == "*") {
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
        self.stream_relationship_properties_application.compute(
            graph_store.as_ref(),
            relationship_properties,
            &want_types,
        )
    }

    fn stream_relationships(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        relationship_types: &[String],
    ) -> Result<Vec<crate::applications::graph_store_catalog::results::TopologyResult>, String>
    {
        let graph_store =
            self.graph_store_catalog_service
                .get_graph_store(user, database_id, graph_name)?;

        // Java parity: ElementProjection.PROJECT_ALL is "*" and means "all relationship types".
        let want_types: Vec<crate::projection::RelationshipType> =
            if relationship_types.iter().any(|s| s.trim() == "*") {
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
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        node_properties: &[String],
    ) -> Result<WriteResult, String> {
        let graph_store =
            self.graph_store_catalog_service
                .get_graph_store(user, database_id, graph_name)?;

        // Pass-1: no DB exporter backend yet; compute what would be written.
        self.write_node_properties_application
            .compute(graph_store.as_ref(), node_properties, &[])
    }

    fn write_node_labels(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        node_labels: &[String],
    ) -> Result<WriteResult, String> {
        let graph_store =
            self.graph_store_catalog_service
                .get_graph_store(user, database_id, graph_name)?;
        self.write_node_label_application
            .compute(graph_store.as_ref(), node_labels)
    }

    fn write_relationship_properties(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        relationship_properties: &[String],
    ) -> Result<WriteResult, String> {
        let graph_store =
            self.graph_store_catalog_service
                .get_graph_store(user, database_id, graph_name)?;
        self.write_relationship_properties_application
            .compute(graph_store.as_ref(), relationship_properties)
    }

    fn write_relationships(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        relationship_type: &str,
    ) -> Result<WriteResult, String> {
        let graph_store =
            self.graph_store_catalog_service
                .get_graph_store(user, database_id, graph_name)?;
        self.write_relationships_application
            .compute(graph_store.as_ref(), relationship_type)
    }

    fn export_to_csv(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        export_path: &str,
    ) -> Result<ExportResult, String> {
        let graph_store =
            self.graph_store_catalog_service
                .get_graph_store(user, database_id, graph_name)?;
        self.export_to_csv_application
            .compute(graph_store.as_ref(), export_path)
    }

    fn export_to_csv_estimate(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
    ) -> Result<crate::applications::graph_store_catalog::results::GraphStoreExportResult, String>
    {
        let graph_store =
            self.graph_store_catalog_service
                .get_graph_store(user, database_id, graph_name)?;
        Ok(self
            .export_to_csv_estimate_application
            .compute(graph_store.as_ref(), graph_name))
    }

    fn export_to_database(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        target_database: &str,
    ) -> Result<ExportResult, String> {
        let graph_store =
            self.graph_store_catalog_service
                .get_graph_store(user, database_id, graph_name)?;
        self.export_to_database_application
            .compute(graph_store.as_ref(), target_database)
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

    fn estimate_project_native(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        projection_config: &NativeProjectionConfig,
    ) -> Result<crate::applications::graph_store_catalog::results::MemoryEstimateResult, String>
    {
        // Pass-1 estimate: derive from either source graph size or a small fictitious default.
        let (n, r) = if let Some(source) = projection_config.source_graph_name.as_deref() {
            let store = self
                .graph_store_catalog_service
                .get_graph_store(user, database_id, source)?;
            (store.node_count() as u64, store.relationship_count() as u64)
        } else if projection_config.fictitious_loading {
            (16u64, 0u64)
        } else {
            return Err("estimate_project_native requires sourceGraphName or fictitiousLoading".to_string());
        };

        // Simple model: base + per-node + per-relationship, scaled slightly by selected props.
        let base = 500_000u64;
        let per_node = 96u64;
        let per_rel = 64u64;
        let property_factor =
            (projection_config.node_properties.len() + projection_config.relationship_properties.len())
                as u64;

        let bytes = base + (n * per_node) + (r * per_rel) + (property_factor * 1_024);
        let mut details: HashMap<String, Value> = HashMap::new();
        details.insert("nodeCount".to_string(), serde_json::json!(n));
        details.insert("relationshipCount".to_string(), serde_json::json!(r));
        details.insert(
            "graphName".to_string(),
            serde_json::json!(projection_config.graph_name),
        );

        Ok(crate::applications::graph_store_catalog::results::MemoryEstimateResult::new(
            bytes, details,
        ))
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

    fn project_cypher(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        node_query: &str,
        relationship_query: &str,
        configuration: &Value,
    ) -> Result<ProjectionResult, String> {
        if node_query.trim().is_empty() {
            return Err("nodeQuery must be non-empty".to_string());
        }
        if relationship_query.trim().is_empty() {
            return Err("relationshipQuery must be non-empty".to_string());
        }

        // Pass-1: no DB integration, so we synthesize a small deterministic graph.
        let seed = configuration
            .get("seed")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let node_count = configuration
            .get("nodeCount")
            .and_then(|v| v.as_u64())
            .unwrap_or(16)
            .max(1);
        let average_degree = configuration
            .get("averageDegree")
            .and_then(|v| v.as_u64())
            .unwrap_or(1);
        let p = if node_count <= 1 {
            0.0
        } else {
            ((average_degree as f64) / ((node_count - 1) as f64)).clamp(0.0, 1.0)
        };

        let gen_cfg = GraphGenerationConfig {
            graph_name: Some(graph_name.to_string()),
            node_count: Some(node_count as usize),
            node_labels: vec!["Node".to_string()],
            relationships: vec![GraphGenerationRelationshipConfig {
                relationship_type: "REL".to_string(),
                probability: p,
            }],
            directed: Some(true),
            inverse_indexed: Some(true),
            seed: Some(seed),
        };

        let gen = self
            .generate_graph_application
            .compute(user, database_id, &gen_cfg)?;

        Ok(ProjectionResult::new(
            gen.graph_name().to_string(),
            gen.nodes_generated(),
            gen.relationships_generated(),
            gen.generation_time_ms(),
        ))
    }

    fn estimate_project_cypher(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        node_query: &str,
        relationship_query: &str,
        configuration: &Value,
    ) -> Result<crate::applications::graph_store_catalog::results::MemoryEstimateResult, String>
    {
        if node_query.trim().is_empty() {
            return Err("nodeQuery must be non-empty".to_string());
        }
        if relationship_query.trim().is_empty() {
            return Err("relationshipQuery must be non-empty".to_string());
        }

        let node_count = configuration
            .get("nodeCount")
            .and_then(|v| v.as_u64())
            .unwrap_or(16)
            .max(1);
        let average_degree = configuration
            .get("averageDegree")
            .and_then(|v| v.as_u64())
            .unwrap_or(1);

        let relationship_count = if node_count <= 1 {
            0u64
        } else {
            // directed-ish approximation
            node_count.saturating_mul(average_degree)
        };

        let bytes = 500_000u64 + (node_count * 128u64) + (relationship_count * 96u64);
        let mut details: HashMap<String, Value> = HashMap::new();
        details.insert("nodeCount".to_string(), serde_json::json!(node_count));
        details.insert(
            "relationshipCount".to_string(),
            serde_json::json!(relationship_count),
        );
        details.insert("nodeQuery".to_string(), serde_json::json!(node_query));
        details.insert(
            "relationshipQuery".to_string(),
            serde_json::json!(relationship_query),
        );
        details.insert("configuration".to_string(), configuration.clone());

        Ok(crate::applications::graph_store_catalog::results::MemoryEstimateResult::new(
            bytes, details,
        ))
    }

    fn generate_graph(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        generation_config: &GraphGenerationConfig,
    ) -> Result<GenerationResult, String> {
        self.generate_graph_application
            .compute(user, database_id, generation_config)
    }

    fn generate_graph_stats(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        node_count: u64,
        average_degree: u64,
        configuration: &Value,
    ) -> Result<crate::applications::graph_store_catalog::results::GraphGenerationStats, String>
    {
        if node_count == 0 {
            return Err("nodeCount must be > 0".to_string());
        }

        let relationship_seed = configuration
            .get("relationshipSeed")
            .and_then(|v| v.as_u64());
        let relationship_distribution = configuration
            .get("relationshipDistribution")
            .and_then(|v| v.as_str())
            .unwrap_or("UNSPECIFIED")
            .to_string();

        let relationship_property_map: HashMap<String, Value> = configuration
            .get("relationshipProperty")
            .and_then(|v| v.as_object())
            .map(|m| {
                m.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();

        let p = if node_count <= 1 {
            0.0
        } else {
            ((average_degree as f64) / ((node_count - 1) as f64)).clamp(0.0, 1.0)
        };

        let gen_cfg = GraphGenerationConfig {
            graph_name: Some(graph_name.to_string()),
            node_count: Some(node_count as usize),
            node_labels: vec!["Node".to_string()],
            relationships: vec![GraphGenerationRelationshipConfig {
                relationship_type: "REL".to_string(),
                probability: p,
            }],
            directed: Some(true),
            inverse_indexed: Some(true),
            seed: relationship_seed.or_else(|| configuration.get("seed").and_then(|v| v.as_u64())),
        };

        let gen = self
            .generate_graph_application
            .compute(user, database_id, &gen_cfg)?;

        let mut stats = crate::applications::graph_store_catalog::results::GraphGenerationStats::new(
            gen.graph_name().to_string(),
            average_degree as f64,
            relationship_distribution,
            relationship_property_map,
            relationship_seed,
        );
        stats.nodes = gen.nodes_generated();
        stats.relationships = gen.relationships_generated();
        stats.generate_millis = gen.generation_time_ms();
        Ok(stats)
    }

    fn sample_graph(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        sampling_config: &SamplingConfig,
    ) -> Result<SamplingResult, String> {
        self.graph_sampling_application
            .compute(user, database_id, graph_name, sampling_config)
    }

    fn sub_graph_project(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        origin_graph_name: &str,
        node_filter: &str,
        relationship_filter: &str,
        configuration: &Value,
    ) -> Result<crate::applications::graph_store_catalog::results::GraphFilterResult, String>
    {
        let sample_node_count = configuration
            .get("sampleNodeCount")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize);
        let sample_ratio = configuration
            .get("sampleRatio")
            .and_then(|v| v.as_f64());
        let seed = configuration.get("seed").and_then(|v| v.as_u64());

        self.sub_graph_project_application.compute(
            user,
            database_id,
            graph_name,
            origin_graph_name,
            node_filter,
            relationship_filter,
            sample_node_count,
            sample_ratio,
            seed,
        )
    }

    fn sample_random_walk_with_restarts(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        origin_graph_name: &str,
        configuration: &Value,
    ) -> Result<crate::applications::graph_store_catalog::results::RandomWalkSamplingResult, String>
    {
        let started = std::time::Instant::now();
        let sample_node_count = configuration
            .get("sampleNodeCount")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize);
        let sample_ratio = configuration
            .get("sampleRatio")
            .and_then(|v| v.as_f64());
        let seed = configuration.get("seed").and_then(|v| v.as_u64());

        let sampling_config = SamplingConfig {
            sampled_graph_name: Some(graph_name.to_string()),
            sample_node_count,
            sample_ratio,
            seed,
        };

        let s = self
            .graph_sampling_application
            .compute(user, database_id, origin_graph_name, &sampling_config)?;

        Ok(crate::applications::graph_store_catalog::results::RandomWalkSamplingResult::new(
            graph_name.to_string(),
            origin_graph_name.to_string(),
            s.sampled_nodes(),
            s.sampled_relationships(),
            s.sampled_nodes(),
            started.elapsed().as_millis() as u64,
        ))
    }

    fn sample_common_neighbour_aware_random_walk(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        origin_graph_name: &str,
        configuration: &Value,
    ) -> Result<crate::applications::graph_store_catalog::results::RandomWalkSamplingResult, String>
    {
        // Pass-1: same deterministic sampling substrate as RW with restarts.
        self.sample_random_walk_with_restarts(
            user,
            database_id,
            graph_name,
            origin_graph_name,
            configuration,
        )
    }

    fn estimate_common_neighbour_aware_random_walk(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        configuration: &Value,
    ) -> Result<crate::applications::graph_store_catalog::results::MemoryEstimateResult, String>
    {
        let graph_store = self
            .graph_store_catalog_service
            .get_graph_store(user, database_id, graph_name)?;
        Ok(self
            .estimate_common_neighbour_aware_random_walk_application
            .compute(graph_store.as_ref(), configuration))
    }

    fn mutate_label(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        node_label: &str,
        config: &crate::applications::graph_store_catalog::configs::MutateLabelConfig,
    ) -> Result<crate::applications::graph_store_catalog::results::MutateLabelResult, String> {
        let graph_store =
            self.graph_store_catalog_service
                .get_graph_store(user, database_id, graph_name)?;

        // Java parity: label mutation uses an Expression parsed from the config.
        // Pass-1: we validate/parse and report "would mutate".
        let expr = crate::applications::graph_store_catalog::applications::Expression::new(
            config.node_filter.clone(),
        );
        Ok(self.node_label_mutator_application.compute(
            graph_store.as_ref(),
            graph_name,
            node_label,
            config,
            &expr,
        ))
    }
}
