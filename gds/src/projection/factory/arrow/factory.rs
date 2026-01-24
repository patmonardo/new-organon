// Arrow Native Factory
//
// Translation from: NativeFactory.java (191 lines)
// Design: Arrow-native GraphStore construction
//
// Key differences from Java:
// - Neo4j Transaction API → Arrow RecordBatch API
// - Neo4j cursors → Arrow batch iterators
// - Database I/O → In-memory columnar data

use super::config::{ArrowProjectionConfig, ArrowProjectionError};
use super::reference::{EdgeTableReference, NodeTableReference};
use crate::projection::factory::arrow::reference::ArrowReference;
use crate::projection::factory::GraphStoreFactory;
use crate::types::graph::PartialIdMap;
use crate::types::graph::RelationshipTopology;
use crate::types::graph::SimpleIdMap;
use crate::types::graph_store::DefaultGraphStore;
use crate::types::schema::GraphSchema;
use std::sync::Arc;

/// Arrow-native factory for creating GraphStores.
///
/// This is the **entry point for all external data** into rust-gds.
/// Arrow IS the native format (like Neo4j is native for Java GDS).
///
/// # Design Philosophy
///
/// - **Zero-copy where possible**: Arrow arrays → PropertyValues directly
/// - **Parallel by default**: Chunked, concurrent import
/// - **Schema-driven**: Arrow schema → GraphStore schema
/// - **Memory-efficient**: Streaming, bounded buffers
/// - **NOT IO**: Assumes Arrow tables already in memory
///
/// # Usage
///
/// ```ignore
/// use gds::projection::factory::arrow::{ArrowNativeFactory, ArrowProjectionConfig};
/// use arrow::record_batch::RecordBatch;
///
/// // Assume you have Arrow tables loaded (from Parquet, CSV, etc.)
/// let node_table: RecordBatch = ...;
/// let edge_table: RecordBatch = ...;
///
/// // Create factory
/// let factory = ArrowNativeFactory::new(node_table, edge_table);
///
/// // Configure import
/// let config = ArrowProjectionConfig::builder()
///     .concurrency(8)
///     .log_progress(true)
///     .build()?;
///
/// // Build GraphStore
/// let graph_store = factory.build_graph_store(&config)?;
/// ```
///
/// # Translation Notes
///
/// Java GDS `NativeFactory`:
/// - Extends CSRGraphStoreFactory<GraphProjectFromStoreConfig>
/// - Uses Neo4j Transaction API for database access
/// - ScanningNodesImporter + ScanningRelationshipsImporter
///
/// rust-gds `ArrowNativeFactory`:
/// - Implements GraphStoreFactory trait
/// - Uses Arrow RecordBatch API for in-memory data
/// - Will use NodeBatchImporter + EdgeBatchImporter (future phases)
#[derive(Debug, Clone)]
pub struct ArrowNativeFactory {
    // Phase 1: Basic structure (no implementation yet)
    // Future phases will add:
    // - TableReference for node/edge tables
    // - BatchScanner for parallel iteration
    // - ImportTask for concurrent execution
    // - PropertyMapper for Arrow → GDS types
    /// Marker to indicate this is a placeholder
    /// Will be replaced in Phase 2+ with actual table references
    _placeholder: (),
    /// Preferred collections backend (Arrow default; Huge/Vec allowed for testing)
    collections_backend: crate::config::CollectionsBackend,
    /// Optional in-memory node table reference
    node_table: Option<Arc<NodeTableReference>>,
    /// Optional in-memory edge table reference
    edge_table: Option<Arc<EdgeTableReference>>,
}

impl ArrowNativeFactory {
    /// Create a new Arrow-native factory.
    ///
    /// # Phase 1 Status
    ///
    /// This is a skeleton implementation. Future phases will accept:
    /// - Arrow RecordBatch or Table references
    /// - Multiple node/edge tables
    /// - Schema metadata
    ///
    /// For now, just creates an empty factory for testing.
    pub fn new() -> Self {
        Self {
            _placeholder: (),
            collections_backend: crate::config::CollectionsBackend::Arrow,
            node_table: None,
            edge_table: None,
        }
    }

    /// Create a new factory with an explicit collections backend preference.
    pub fn with_collections_backend(backend: crate::config::CollectionsBackend) -> Self {
        Self {
            _placeholder: (),
            collections_backend: backend,
            node_table: None,
            edge_table: None,
        }
    }

    /// Create a factory from in-memory Arrow table references.
    pub fn from_tables(
        node_table: Arc<NodeTableReference>,
        edge_table: Arc<EdgeTableReference>,
    ) -> Self {
        Self {
            _placeholder: (),
            collections_backend: crate::config::CollectionsBackend::Arrow,
            node_table: Some(node_table),
            edge_table: Some(edge_table),
        }
    }
}

impl Default for ArrowNativeFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphStoreFactory for ArrowNativeFactory {
    type Config = ArrowProjectionConfig;
    type Error = ArrowProjectionError;

    /// Build a GraphStore from Arrow tables.
    ///
    /// # Phase 1 Status
    ///
    /// Skeleton only! Returns a placeholder error.
    /// Future phases will implement:
    ///
    /// 1. Validate configuration
    /// 2. Infer schema from Arrow tables
    /// 3. Create node/edge importers
    /// 4. Execute parallel import
    /// 5. Construct GraphStore
    ///
    /// # Translation from Java
    ///
    /// ```java
    /// // Java GDS NativeFactory.build()
    /// public CSRGraphStore build() {
    ///     validate(dimensions, storeConfig);
    ///     var concurrency = graphProjectConfig.readConcurrency();
    ///     try {
    ///         progressTracker.beginSubTask();
    ///         Nodes nodes = loadNodes(concurrency);
    ///         RelationshipImportResult relationships = loadRelationships(nodes.idMap(), concurrency);
    ///         CSRGraphStore graphStore = createGraphStore(nodes, relationships);
    ///         logLoadingSummary(graphStore);
    ///         return graphStore;
    ///     } finally {
    ///         progressTracker.endSubTask();
    ///     }
    /// }
    /// ```
    fn build_graph_store(&self, config: &Self::Config) -> Result<DefaultGraphStore, Self::Error> {
        // Phase 1: Validate config at least
        config.validate()?;

        // Placeholder: acknowledge the configured backend to avoid dead code warnings
        let _backend = config.collections_backend;
        let _factory_backend = self.collections_backend;
        if let (Some(node_table), Some(edge_table)) = (&self.node_table, &self.edge_table) {
            return self.build_from_tables(node_table, edge_table, config);
        }

        // Note(gds,2025-01-31): Add support for loading tables from config-driven references.
        Err(ArrowProjectionError::InvalidConfig(
            "ArrowNativeFactory needs in-memory node/edge tables (use from_tables())".to_string(),
        ))
    }

    /// Estimate memory usage during loading.
    ///
    /// # Phase 1 Status
    ///
    /// Placeholder! Uses simple counts when tables exist.
    /// Future phases will calculate:
    /// - Buffer sizes for parallel import
    /// - Temporary structures (ID maps, property buffers)
    /// - GraphStore final size estimate
    fn estimate_memory(&self, config: &Self::Config) -> Result<(usize, usize), Self::Error> {
        if let (Some(node_table), Some(edge_table)) = (&self.node_table, &self.edge_table) {
            let nodes = node_table.row_count();
            let edges = edge_table.row_count();
            // Crude estimate: counts as proxies for now
            return Ok((nodes, edges));
        }

        // Note(gds,2025-01-31): Use Arrow metadata to estimate buffers even without in-memory tables.
        let _ = config;
        Ok((0, 0))
    }

    /// Get node count from Arrow table.
    ///
    /// # Phase 1 Status
    ///
    /// Placeholder! Returns 0.
    /// Future phases will read Arrow table metadata.
    fn node_count(&self, _config: &Self::Config) -> Result<usize, Self::Error> {
        if let Some(node_table) = &self.node_table {
            return Ok(node_table.row_count());
        }

        // Note(gds,2025-01-31): Support node counts from Arrow metadata when tables are not in memory.
        Err(ArrowProjectionError::InvalidConfig(
            "node table missing; provide via ArrowNativeFactory::from_tables()".to_string(),
        ))
    }

    /// Get edge count from Arrow table.
    ///
    /// # Phase 1 Status
    ///
    /// Placeholder! Returns 0.
    /// Future phases will read Arrow table metadata.
    fn edge_count(&self, _config: &Self::Config) -> Result<usize, Self::Error> {
        if let Some(edge_table) = &self.edge_table {
            return Ok(edge_table.row_count());
        }

        // Note(gds,2025-01-31): Support edge counts from Arrow metadata when tables are not in memory.
        Err(ArrowProjectionError::InvalidConfig(
            "edge table missing; provide via ArrowNativeFactory::from_tables()".to_string(),
        ))
    }
}

impl crate::projection::factory::GraphStoreFactoryTyped for ArrowNativeFactory {
    type Config = ArrowProjectionConfig;
    type Error = ArrowProjectionError;
    type Store = crate::types::graph_store::DefaultGraphStore;

    fn build_graph_store(&self, config: &Self::Config) -> Result<Self::Store, Self::Error> {
        // Delegate to existing GraphStoreFactory implementation
        <ArrowNativeFactory as crate::projection::factory::GraphStoreFactory>::build_graph_store(
            self, config,
        )
    }

    fn estimate_memory(&self, config: &Self::Config) -> Result<(usize, usize), Self::Error> {
        <ArrowNativeFactory as crate::projection::factory::GraphStoreFactory>::estimate_memory(
            self, config,
        )
    }

    fn node_count(&self, config: &Self::Config) -> Result<usize, Self::Error> {
        <ArrowNativeFactory as crate::projection::factory::GraphStoreFactory>::node_count(
            self, config,
        )
    }

    fn edge_count(&self, config: &Self::Config) -> Result<usize, Self::Error> {
        <ArrowNativeFactory as crate::projection::factory::GraphStoreFactory>::edge_count(
            self, config,
        )
    }
}

impl ArrowNativeFactory {
    fn build_from_tables(
        &self,
        node_table: &Arc<NodeTableReference>,
        edge_table: &Arc<EdgeTableReference>,
        _config: &ArrowProjectionConfig,
    ) -> Result<DefaultGraphStore, ArrowProjectionError> {
        // Build id map from node ids (assume dense i64 ids)
        let node_ids: Vec<i64> = node_table.id_column().values().to_vec();
        let id_map = SimpleIdMap::from_original_ids(node_ids.clone());

        // Build topology (outgoing adjacency) from source/target columns
        let source = edge_table.source_column();
        let target = edge_table.target_column();
        let node_count = node_ids.len();
        let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        for (&s, &t) in source.values().iter().zip(target.values().iter()) {
            let mapped_source = id_map.to_mapped_node_id(s).ok_or_else(|| {
                ArrowProjectionError::Import(format!("source id {s} missing in nodes"))
            })?;
            let mapped_target = id_map.to_mapped_node_id(t).ok_or_else(|| {
                ArrowProjectionError::Import(format!("target id {t} missing in nodes"))
            })?;
            outgoing[mapped_source as usize].push(mapped_target);
        }
        let topology = RelationshipTopology::new(outgoing, None);

        // Schema: basic node/relationship schemas
        let schema = GraphSchema::empty();

        // Backend selection for properties
        // Assemble DefaultGraphStore
        let graph_name = crate::types::graph_store::GraphName::new("arrow_graph");
        let database_info = crate::types::graph_store::DatabaseInfo::new(
            crate::types::graph_store::DatabaseId::new("arrow"),
            crate::types::graph_store::DatabaseLocation::remote(
                "local",
                0,
                None::<String>,
                None::<String>,
            ),
        );
        let capabilities = crate::types::graph_store::Capabilities::default();
        let relationship_topologies = {
            let mut m = std::collections::HashMap::new();
            m.insert(
                crate::projection::relationship_type::RelationshipType::of("REL"),
                topology,
            );
            m
        };

        let mut store = DefaultGraphStore::new(
            crate::config::GraphStoreConfig::default(),
            graph_name,
            database_info,
            schema,
            capabilities,
            id_map,
            relationship_topologies,
        );

        // Node properties: id column (required)
        store
            .add_node_property_i64("id".to_string(), node_ids.clone())
            .map_err(|e| ArrowProjectionError::Import(e.to_string()))?;

        // Additional node properties: infer numeric columns
        for idx in node_table.property_column_indices() {
            let field = &node_table.schema().fields[idx];
            match field.data_type() {
                arrow2::datatypes::DataType::Int64 => {
                    if let Some(arr) = node_table
                        .chunk()
                        .arrays()
                        .get(idx)
                        .and_then(|a| a.as_any().downcast_ref::<arrow2::array::Int64Array>())
                    {
                        let vals = arr.values().to_vec();
                        store
                            .add_node_property_i64(field.name.clone(), vals)
                            .map_err(|e| ArrowProjectionError::Import(e.to_string()))?;
                    }
                }
                arrow2::datatypes::DataType::Float64 => {
                    if let Some(arr) = node_table
                        .chunk()
                        .arrays()
                        .get(idx)
                        .and_then(|a| a.as_any().downcast_ref::<arrow2::array::Float64Array>())
                    {
                        let vals = arr.values().to_vec();
                        store
                            .add_node_property_f64(field.name.clone(), vals)
                            .map_err(|e| ArrowProjectionError::Import(e.to_string()))?;
                    }
                }
                _ => {
                    // Skip unsupported types for now
                }
            }
        }

        store
            .add_graph_property_i64("node_count".to_string(), vec![node_count as i64])
            .map_err(|e| ArrowProjectionError::Import(e.to_string()))?;

        Ok(store)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_creation() {
        let factory = ArrowNativeFactory::new();
        // Just ensuring it compiles and creates
        assert_eq!(factory._placeholder, ());
    }

    #[test]
    fn test_factory_default() {
        let factory = ArrowNativeFactory::default();
        assert_eq!(factory._placeholder, ());
    }

    #[test]
    fn test_build_graph_store_validates_config() {
        let factory = ArrowNativeFactory::new();

        // Valid config should pass validation (then fail on unimplemented)
        let config = ArrowProjectionConfig::default();
        let result = factory.build_graph_store(&config);
        assert!(result.is_err());
        match result {
            Err(ArrowProjectionError::InvalidConfig(msg)) => {
                assert!(msg.contains("needs in-memory node/edge tables"));
            }
            _ => panic!("Expected InvalidConfig error"),
        }
    }

    #[test]
    fn test_build_graph_store_rejects_invalid_config() {
        let factory = ArrowNativeFactory::new();

        // Invalid config should fail validation
        let config = ArrowProjectionConfig {
            node_table_name: "".to_string(), // Invalid!
            ..Default::default()
        };

        let result = factory.build_graph_store(&config);
        assert!(result.is_err());
        match result {
            Err(ArrowProjectionError::InvalidConfig(_)) => {
                // Expected!
            }
            _ => panic!("Expected InvalidConfig error"),
        }
    }

    #[test]
    fn test_estimate_memory_placeholder() {
        let factory = ArrowNativeFactory::new();
        let config = ArrowProjectionConfig::default();

        let result = factory.estimate_memory(&config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (0, 0));
    }

    #[test]
    fn test_node_and_edge_count_require_tables() {
        let factory = ArrowNativeFactory::new();
        let config = ArrowProjectionConfig::default();

        assert!(factory.node_count(&config).is_err());
        assert!(factory.edge_count(&config).is_err());
    }

    #[test]
    fn test_node_and_edge_count_from_tables() {
        use crate::projection::factory::arrow::test_utils::{sample_edge_table, sample_node_table};

        let nodes = Arc::new(sample_node_table());
        let edges = Arc::new(sample_edge_table());
        let factory = ArrowNativeFactory::from_tables(nodes, edges);
        let config = ArrowProjectionConfig::default();

        assert_eq!(factory.node_count(&config).unwrap(), 3);
        assert_eq!(factory.edge_count(&config).unwrap(), 2);
    }

    #[test]
    fn test_estimate_memory_from_tables() {
        use crate::projection::factory::arrow::test_utils::{sample_edge_table, sample_node_table};

        let nodes = Arc::new(sample_node_table());
        let edges = Arc::new(sample_edge_table());
        let factory = ArrowNativeFactory::from_tables(nodes, edges);
        let config = ArrowProjectionConfig::default();

        assert_eq!(factory.estimate_memory(&config).unwrap(), (3, 2));
    }

    #[test]
    fn build_from_in_memory_tables() {
        use crate::projection::factory::arrow::test_utils::{sample_edge_table, sample_node_table};
        use crate::types::graph_store::GraphStore;

        let nodes = Arc::new(sample_node_table());
        let edges = Arc::new(sample_edge_table());

        let mut config = ArrowProjectionConfig::default();
        config.collections_backend = crate::config::CollectionsBackend::Arrow;

        let factory = ArrowNativeFactory::from_tables(nodes, edges);
        let store = factory
            .build_graph_store(&config)
            .expect("build graph store");

        assert_eq!(store.node_count(), 3);
        assert_eq!(store.relationship_count(), 2);
        assert!(store.has_node_property("id"));
    }
}
