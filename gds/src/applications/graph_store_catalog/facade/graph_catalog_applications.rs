use crate::applications::graph_store_catalog::configs::MutateLabelConfig;
use crate::applications::graph_store_catalog::results::{
    GraphFilterResult, GraphGenerationStats, GraphMemoryUsage, GraphStoreExportResult,
    GraphStreamGraphPropertiesResult, GraphStreamNodePropertiesResult,
    GraphStreamRelationshipPropertiesResult, MemoryEstimateResult, MutateLabelResult,
    RandomWalkSamplingResult, TopologyResult,
};
use crate::core::User;
use crate::types::graph_store::DatabaseId;
use crate::types::graph_store::DeletionResult;
use serde_json::Value;
use std::collections::HashMap;

/// Main trait interface for GraphStore catalog operations.
///
/// Mirrors Java GraphCatalogApplications interface.
/// This is the primary facade that GDSL will consume - the TS Application Facade Consumer interface!
pub trait GraphCatalogApplications {
    /// Checks whether a named graph exists (in the catalog scope for this user/database).
    fn graph_exists(&self, user: &dyn User, database_id: &DatabaseId, graph_name: &str) -> bool;

    /// Lists all graphs in the catalog.
    fn list_graphs(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: Option<&str>,
        include_degree_distribution: bool,
    ) -> Vec<GraphStoreCatalogEntry>;

    /// Lists all graphs in the catalog (JSON response).
    fn list_graphs_json(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: Option<&str>,
        include_degree_distribution: bool,
    ) -> Value;

    /// Gets memory usage for a specific graph.
    fn graph_memory_usage(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
    ) -> GraphMemoryUsage;

    /// Drops a graph from the catalog.
    fn drop_graph(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        fail_if_missing: bool,
    ) -> Result<GraphStoreCatalogEntry, String>;

    /// Drops multiple graphs from the catalog.
    fn drop_graphs(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_names: &[String],
        fail_if_missing: bool,
    ) -> Result<Vec<GraphStoreCatalogEntry>, String>;

    /// Drops node properties from a graph.
    fn drop_node_properties(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        node_properties: &[String],
        fail_if_missing: bool,
    ) -> Result<u64, String>;

    /// Drops relationships from a graph.
    fn drop_relationships(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        relationship_type: &str,
    ) -> Result<DeletionResult, String>;

    /// Drops a graph property.
    fn drop_graph_property(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        graph_property: &str,
        fail_if_missing: bool,
    ) -> Result<u64, String>;

    /// Streams a graph property.
    fn stream_graph_property(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        graph_property: &str,
    ) -> Result<Vec<GraphStreamGraphPropertiesResult>, String>;

    /// Streams node properties from a graph.
    fn stream_node_properties(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        node_properties: &[String],
        node_labels: &[String],
        list_node_labels: bool,
    ) -> Result<Vec<GraphStreamNodePropertiesResult>, String>;

    /// Streams relationship properties from a graph.
    fn stream_relationship_properties(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        relationship_properties: &[String],
        relationship_types: &[String],
    ) -> Result<Vec<GraphStreamRelationshipPropertiesResult>, String>;

    /// Streams relationships from a graph.
    fn stream_relationships(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        relationship_types: &[String],
    ) -> Result<Vec<TopologyResult>, String>;

    /// Writes node properties to the database.
    fn write_node_properties(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        node_properties: &[String],
    ) -> Result<WriteResult, String>;

    /// Writes node labels to the database.
    fn write_node_labels(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        node_labels: &[String],
    ) -> Result<WriteResult, String>;

    /// Writes relationship properties to the database.
    fn write_relationship_properties(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        relationship_properties: &[String],
    ) -> Result<WriteResult, String>;

    /// Writes relationships to the database.
    fn write_relationships(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        relationship_type: &str,
    ) -> Result<WriteResult, String>;

    /// Exports graph to CSV.
    fn export_to_csv(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        export_path: &str,
    ) -> Result<ExportResult, String>;

    /// Estimates CSV export.
    fn export_to_csv_estimate(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
    ) -> Result<GraphStoreExportResult, String>;

    /// Exports graph to database.
    fn export_to_database(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        target_database: &str,
    ) -> Result<ExportResult, String>;

    /// Projects a graph using native projection.
    fn project_native(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        projection_config: &NativeProjectionConfig,
    ) -> Result<ProjectionResult, String>;

    /// Estimates memory usage for a native projection.
    ///
    /// Java parity: `estimateNativeProject`.
    /// Rust pass-1: deterministic estimate based on config / existing store shape.
    fn estimate_project_native(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        projection_config: &NativeProjectionConfig,
    ) -> Result<MemoryEstimateResult, String>;

    /// Projects a graph using generic projection.
    fn project_generic(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        projection_config: &GenericProjectionConfig,
    ) -> Result<ProjectionResult, String>;

    /// Projects a graph from Cypher.
    ///
    /// Java parity: `cypherProject`.
    /// Rust pass-1: deterministic/stub projection (no DB integration yet).
    fn project_cypher(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        node_query: &str,
        relationship_query: &str,
        configuration: &Value,
    ) -> Result<ProjectionResult, String>;

    /// Estimates memory usage for a Cypher projection.
    ///
    /// Java parity: `estimateCypherProject`.
    fn estimate_project_cypher(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        node_query: &str,
        relationship_query: &str,
        configuration: &Value,
    ) -> Result<MemoryEstimateResult, String>;

    /// Generates a synthetic graph.
    fn generate_graph(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        generation_config: &GraphGenerationConfig,
    ) -> Result<GenerationResult, String>;

    /// Generates a synthetic graph and returns Java-parity stats.
    ///
    /// Java parity: `generateGraph` returning `GraphGenerationStats`.
    fn generate_graph_stats(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        node_count: u64,
        average_degree: u64,
        configuration: &Value,
    ) -> Result<GraphGenerationStats, String>;

    /// Samples a graph using random walk.
    fn sample_graph(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        sampling_config: &SamplingConfig,
    ) -> Result<SamplingResult, String>;

    /// Subgraph projection.
    ///
    /// Java parity: `subGraphProject`.
    fn sub_graph_project(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        origin_graph_name: &str,
        node_filter: &str,
        relationship_filter: &str,
        configuration: &Value,
    ) -> Result<GraphFilterResult, String>;

    /// Random-walk with restarts sampling.
    ///
    /// Java parity: `sampleRandomWalkWithRestarts`.
    fn sample_random_walk_with_restarts(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        origin_graph_name: &str,
        configuration: &Value,
    ) -> Result<RandomWalkSamplingResult, String>;

    /// Common-neighbour-aware random walk sampling.
    ///
    /// Java parity: `sampleCommonNeighbourAwareRandomWalk`.
    fn sample_common_neighbour_aware_random_walk(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        origin_graph_name: &str,
        configuration: &Value,
    ) -> Result<RandomWalkSamplingResult, String>;

    /// Common-neighbour-aware random walk memory estimate.
    ///
    /// Java parity: `estimateCommonNeighbourAwareRandomWalk`.
    fn estimate_common_neighbour_aware_random_walk(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        configuration: &Value,
    ) -> Result<MemoryEstimateResult, String>;

    /// Mutates (adds) a node label.
    fn mutate_label(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        graph_name: &str,
        node_label: &str,
        config: &MutateLabelConfig,
    ) -> Result<MutateLabelResult, String>;
}

/// Placeholder result types for the facade operations

// Node property streaming uses `GraphStreamNodePropertiesResult` (Java parity)
// from `results::stream_results`.

#[derive(Clone, Debug)]
pub struct RelationshipPropertyResult {
    source_id: u64,
    target_id: u64,
    relationship_type: String,
    property_name: String,
    property_value: serde_json::Value,
}

impl RelationshipPropertyResult {
    pub fn new(
        source_id: u64,
        target_id: u64,
        relationship_type: String,
        property_name: String,
        property_value: serde_json::Value,
    ) -> Self {
        Self {
            source_id,
            target_id,
            relationship_type,
            property_name,
            property_value,
        }
    }

    pub fn source_id(&self) -> u64 {
        self.source_id
    }
    pub fn target_id(&self) -> u64 {
        self.target_id
    }
    pub fn relationship_type(&self) -> &str {
        &self.relationship_type
    }
    pub fn property_name(&self) -> &str {
        &self.property_name
    }
    pub fn property_value(&self) -> &serde_json::Value {
        &self.property_value
    }
}

// Relationship streaming uses `TopologyResult` (Java parity) from `results::other_results`.

#[derive(Clone, Debug)]
pub struct WriteResult {
    nodes_written: u64,
    relationships_written: u64,
    properties_written: u64,
}

impl WriteResult {
    pub fn new(nodes_written: u64, relationships_written: u64, properties_written: u64) -> Self {
        Self {
            nodes_written,
            relationships_written,
            properties_written,
        }
    }

    pub fn nodes_written(&self) -> u64 {
        self.nodes_written
    }
    pub fn relationships_written(&self) -> u64 {
        self.relationships_written
    }
    pub fn properties_written(&self) -> u64 {
        self.properties_written
    }
}

#[derive(Clone, Debug)]
pub struct ExportResult {
    nodes_exported: u64,
    relationships_exported: u64,
    file_path: Option<String>,
}

impl ExportResult {
    pub fn new(
        nodes_exported: u64,
        relationships_exported: u64,
        file_path: Option<String>,
    ) -> Self {
        Self {
            nodes_exported,
            relationships_exported,
            file_path,
        }
    }

    pub fn nodes_exported(&self) -> u64 {
        self.nodes_exported
    }
    pub fn relationships_exported(&self) -> u64 {
        self.relationships_exported
    }
    pub fn file_path(&self) -> Option<&str> {
        self.file_path.as_deref()
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionResult {
    graph_name: String,
    nodes_projected: u64,
    relationships_projected: u64,
    projection_time_ms: u64,
}

impl ProjectionResult {
    pub fn new(
        graph_name: String,
        nodes_projected: u64,
        relationships_projected: u64,
        projection_time_ms: u64,
    ) -> Self {
        Self {
            graph_name,
            nodes_projected,
            relationships_projected,
            projection_time_ms,
        }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
    pub fn nodes_projected(&self) -> u64 {
        self.nodes_projected
    }
    pub fn relationships_projected(&self) -> u64 {
        self.relationships_projected
    }
    pub fn projection_time_ms(&self) -> u64 {
        self.projection_time_ms
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GenerationResult {
    graph_name: String,
    nodes_generated: u64,
    relationships_generated: u64,
    generation_time_ms: u64,
}

impl GenerationResult {
    pub fn new(
        graph_name: String,
        nodes_generated: u64,
        relationships_generated: u64,
        generation_time_ms: u64,
    ) -> Self {
        Self {
            graph_name,
            nodes_generated,
            relationships_generated,
            generation_time_ms,
        }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
    pub fn nodes_generated(&self) -> u64 {
        self.nodes_generated
    }
    pub fn relationships_generated(&self) -> u64 {
        self.relationships_generated
    }
    pub fn generation_time_ms(&self) -> u64 {
        self.generation_time_ms
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SamplingResult {
    sampled_graph_name: String,
    original_nodes: u64,
    sampled_nodes: u64,
    original_relationships: u64,
    sampled_relationships: u64,
}

impl SamplingResult {
    pub fn new(
        sampled_graph_name: String,
        original_nodes: u64,
        sampled_nodes: u64,
        original_relationships: u64,
        sampled_relationships: u64,
    ) -> Self {
        Self {
            sampled_graph_name,
            original_nodes,
            sampled_nodes,
            original_relationships,
            sampled_relationships,
        }
    }

    pub fn sampled_graph_name(&self) -> &str {
        &self.sampled_graph_name
    }
    pub fn original_nodes(&self) -> u64 {
        self.original_nodes
    }
    pub fn sampled_nodes(&self) -> u64 {
        self.sampled_nodes
    }
    pub fn original_relationships(&self) -> u64 {
        self.original_relationships
    }
    pub fn sampled_relationships(&self) -> u64 {
        self.sampled_relationships
    }
}

// -----------------------------------------------------------------------------
// Projection config types (minimal Java-parity shape)
// -----------------------------------------------------------------------------
//
// Java parity note:
// - Java uses `GraphProjectConfig` subtypes (e.g. GraphProjectFromStoreConfig / FromCypherConfig)
//   that carry a rich set of fields (projections, filters, concurrency, jobId, etc.).
// - In Rust we keep pass-1 configs intentionally small and stable for TS-JSON transport:
//   they identify the output graph name, and optionally a source graph to clone from
//   (catalog-backed projection), with room for future expansion.
//
// This matches the "Projection/Factory is the real interface" approach: we can
// later swap the implementation to a real native factory (Arrow/Polars/etc.) without
// changing the TS boundary.

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeProjectionConfig {
    /// Name of the projected graph to store in the catalog.
    pub graph_name: String,
    /// Optional source graph in the catalog; when present we project by cloning/filtering it.
    pub source_graph_name: Option<String>,
    /// Optional node label filter (Java parity: ElementProjection.PROJECT_ALL == "*").
    /// - empty => all node labels
    /// - contains "*" => all node labels
    ///
    /// Pass-1 note: we validate these labels against the source store schema, but do not
    /// physically drop nodes yet (that requires a proper filtered IdMap / projection build).
    pub node_labels: Vec<String>,
    /// Optional node property filter (Java parity: "*" means PROJECT_ALL).
    /// - empty => keep all node properties
    /// - contains "*" => keep all node properties
    pub node_properties: Vec<String>,
    /// Optional relationship type filter (Java parity: ElementProjection.PROJECT_ALL == "*").
    /// - empty => all relationship types
    /// - contains "*" => all relationship types
    pub relationship_types: Vec<String>,
    /// Optional relationship property filter (Java parity: "*" means PROJECT_ALL).
    /// - empty => keep all relationship properties
    /// - contains "*" => keep all relationship properties
    pub relationship_properties: Vec<String>,
    /// Per-relationship-type property selector map.
    ///
    /// Java parity: this corresponds to “relationship projections” choosing a property key.
    /// If present, Projection/Factory will keep these keys and algorithms can use them
    /// to select relationship weights without additional knobs.
    pub relationship_property_selectors: HashMap<String, String>,
    /// Optional “default weight property” to use when a per-type selector is not specified.
    pub weight_property: Option<String>,
    /// If true, allow generating a small fictitious graph when no source is provided.
    pub fictitious_loading: bool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericProjectionConfig {
    pub graph_name: String,
    pub source_graph_name: Option<String>,
    pub node_labels: Vec<String>,
    pub node_properties: Vec<String>,
    pub relationship_types: Vec<String>,
    pub relationship_properties: Vec<String>,
    pub relationship_property_selectors: HashMap<String, String>,
    pub weight_property: Option<String>,
    pub fictitious_loading: bool,
}

// -----------------------------------------------------------------------------
// Generate / Sample config types (pass-1 Java-parity shape)
// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphGenerationConfig {
    /// Name for the generated graph (stored in catalog). When omitted, defaults to "generated_graph".
    pub graph_name: Option<String>,
    /// Number of nodes to generate.
    pub node_count: Option<usize>,
    /// Node labels to assign.
    pub node_labels: Vec<String>,
    /// Relationship types to generate.
    pub relationships: Vec<GraphGenerationRelationshipConfig>,
    /// Directed graph when true.
    pub directed: Option<bool>,
    /// Build inverse indices for relationship types.
    pub inverse_indexed: Option<bool>,
    /// RNG seed for deterministic generation.
    pub seed: Option<u64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphGenerationRelationshipConfig {
    pub relationship_type: String,
    /// Probability in [0.0, 1.0] of creating an edge between node pairs.
    pub probability: f64,
}

impl Default for GraphGenerationRelationshipConfig {
    fn default() -> Self {
        Self {
            relationship_type: "REL".to_string(),
            probability: 0.1,
        }
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SamplingConfig {
    /// Name for the sampled graph (stored in catalog). When omitted, defaults to "{origin}_sampled".
    pub sampled_graph_name: Option<String>,
    /// Fixed sample size in nodes.
    pub sample_node_count: Option<usize>,
    /// Ratio of nodes to sample in (0.0, 1.0].
    pub sample_ratio: Option<f64>,
    /// RNG seed for deterministic selection.
    pub seed: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct GraphStoreCatalogEntry {
    graph_name: String,
    node_count: u64,
    relationship_count: u64,
    degree_distribution: Option<HashMap<u32, u64>>,
}

impl GraphStoreCatalogEntry {
    pub fn new(
        graph_name: String,
        node_count: u64,
        relationship_count: u64,
        degree_distribution: Option<HashMap<u32, u64>>,
    ) -> Self {
        Self {
            graph_name,
            node_count,
            relationship_count,
            degree_distribution,
        }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn node_count(&self) -> u64 {
        self.node_count
    }

    pub fn relationship_count(&self) -> u64 {
        self.relationship_count
    }

    pub fn degree_distribution(&self) -> Option<&HashMap<u32, u64>> {
        self.degree_distribution.as_ref()
    }
}
