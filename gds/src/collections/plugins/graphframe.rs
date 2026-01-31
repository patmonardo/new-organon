//! GraphFrame plugin helpers for Polars-backed exports.
//!
//! This module provides a minimal GraphFrame-style export that turns a
//! [`DefaultGraphStore`] into Polars DataFrames (nodes + edges). The primary
//! goal is a lightweight, 32-bit Polars-compatible path that treats the
//! GraphStore as the GraphFrame layer.

use crate::collections::catalog::schema::CollectionsSchema;
use crate::collections::catalog::{
    CollectionsCatalogDisk, CollectionsCatalogDiskEntry, CollectionsIoFormat, CollectionsIoPolicy,
};
use crate::collections::dataframe::PolarsDataFrameCollection;
use crate::config::CollectionsBackend;
use crate::types::graph_store::{DefaultGraphStore, GraphStore};
use crate::types::ValueType;
use polars::prelude::{NamedFrom, PlSmallStr, Series};
use serde_json::Value as JsonValue;
use std::collections::HashSet;
use std::path::PathBuf;

/// Output tables for a GraphFrame-style export.
#[derive(Debug, Clone)]
pub struct GraphFrameTables {
    pub nodes: PolarsDataFrameCollection,
    pub edges: PolarsDataFrameCollection,
}

/// On-disk catalog entries for a GraphFrame export.
#[derive(Debug, Clone)]
pub struct GraphFrameCatalogWriteResult {
    pub nodes: CollectionsCatalogDiskEntry,
    pub edges: CollectionsCatalogDiskEntry,
    pub graph: CollectionsCatalogDiskEntry,
}

/// Errors that can occur when exporting a GraphStore into GraphFrame tables.
#[derive(Debug, thiserror::Error)]
pub enum GraphFramePluginError {
    #[error("node id out of 32-bit range: {0}")]
    NodeIdOutOfRange(i64),

    #[error("graph export error: {0}")]
    Graph(String),

    #[error("polars error: {0}")]
    Polars(String),
}

impl From<polars::error::PolarsError> for GraphFramePluginError {
    fn from(err: polars::error::PolarsError) -> Self {
        GraphFramePluginError::Polars(err.to_string())
    }
}

/// Minimal plugin contract for producing GraphFrame tables.
pub trait GraphFramePlugin {
    fn name(&self) -> &'static str;

    fn write_graph_store(
        &self,
        store: &DefaultGraphStore,
    ) -> Result<GraphFrameTables, GraphFramePluginError>;
}

/// Polars-backed GraphFrame plugin that emits 32-bit node/edge identifiers.
#[derive(Debug, Clone)]
pub struct GraphFramePolars32Plugin {
    pub include_labels: bool,
    pub include_relationship_type: bool,
    pub include_weight: bool,
}

impl Default for GraphFramePolars32Plugin {
    fn default() -> Self {
        Self {
            include_labels: true,
            include_relationship_type: true,
            include_weight: true,
        }
    }
}

impl GraphFramePolars32Plugin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_options(
        include_labels: bool,
        include_relationship_type: bool,
        include_weight: bool,
    ) -> Self {
        Self {
            include_labels,
            include_relationship_type,
            include_weight,
        }
    }

    fn id_to_i32(value: i64) -> Result<i32, GraphFramePluginError> {
        if value < i32::MIN as i64 || value > i32::MAX as i64 {
            return Err(GraphFramePluginError::NodeIdOutOfRange(value));
        }
        Ok(value as i32)
    }

    fn build_nodes(
        &self,
        store: &DefaultGraphStore,
    ) -> Result<PolarsDataFrameCollection, GraphFramePluginError> {
        let id_map = store.nodes();
        let mut node_ids: Vec<i32> = Vec::with_capacity(id_map.node_count());
        let mut labels: Vec<String> = if self.include_labels {
            Vec::with_capacity(id_map.node_count())
        } else {
            Vec::new()
        };

        for node_id in id_map.iter() {
            node_ids.push(Self::id_to_i32(node_id)?);
            if self.include_labels {
                let mut label_names: Vec<String> = id_map
                    .node_labels(node_id)
                    .into_iter()
                    .map(|label| label.name().to_string())
                    .collect();
                label_names.sort();
                labels.push(label_names.join("|"));
            }
        }

        let mut columns = vec![Series::new(PlSmallStr::from_static("id"), node_ids)];
        if self.include_labels {
            columns.push(Series::new(PlSmallStr::from_static("labels"), labels));
        }

        Ok(PolarsDataFrameCollection::from_series(columns)?)
    }

    fn build_edges(
        &self,
        store: &DefaultGraphStore,
    ) -> Result<PolarsDataFrameCollection, GraphFramePluginError> {
        let mut sources: Vec<i32> = Vec::with_capacity(store.relationship_count());
        let mut targets: Vec<i32> = Vec::with_capacity(store.relationship_count());
        let mut types: Vec<String> = if self.include_relationship_type {
            Vec::with_capacity(store.relationship_count())
        } else {
            Vec::new()
        };
        let mut weights: Vec<f64> = if self.include_weight {
            Vec::with_capacity(store.relationship_count())
        } else {
            Vec::new()
        };

        if self.include_relationship_type {
            let relationship_types = store.relationship_types();
            for rel_type in relationship_types {
                let mut filter = HashSet::new();
                filter.insert(rel_type.clone());
                let graph = store
                    .get_graph_with_types(&filter)
                    .map_err(|err| GraphFramePluginError::Graph(err.to_string()))?;
                let fallback = graph.default_property_value();
                for source in graph.iter() {
                    for cursor in graph.stream_relationships(source, fallback) {
                        sources.push(Self::id_to_i32(cursor.source_id())?);
                        targets.push(Self::id_to_i32(cursor.target_id())?);
                        types.push(rel_type.name().to_string());
                        if self.include_weight {
                            weights.push(cursor.property());
                        }
                    }
                }
            }
        } else {
            let graph = store.get_graph();
            let fallback = graph.default_property_value();
            for source in graph.iter() {
                for cursor in graph.stream_relationships(source, fallback) {
                    sources.push(Self::id_to_i32(cursor.source_id())?);
                    targets.push(Self::id_to_i32(cursor.target_id())?);
                    if self.include_weight {
                        weights.push(cursor.property());
                    }
                }
            }
        }

        let mut columns = vec![
            Series::new(PlSmallStr::from_static("src"), sources),
            Series::new(PlSmallStr::from_static("dst"), targets),
        ];
        if self.include_relationship_type {
            columns.push(Series::new(PlSmallStr::from_static("type"), types));
        }
        if self.include_weight {
            columns.push(Series::new(PlSmallStr::from_static("weight"), weights));
        }

        Ok(PolarsDataFrameCollection::from_series(columns)?)
    }

    fn build_graph_table(
        &self,
        store: &DefaultGraphStore,
        graph_name: &str,
    ) -> Result<PolarsDataFrameCollection, GraphFramePluginError> {
        let schema_json: JsonValue = store.schema().to_map();
        let schema_json = serde_json::to_string(&schema_json)
            .map_err(|err| GraphFramePluginError::Graph(err.to_string()))?;

        let node_count = store.node_count() as i64;
        let relationship_count = store.relationship_count() as i64;
        let direction = format!("{:?}", store.schema().direction());
        let created_at = store.creation_time().to_rfc3339();
        let modified_at = store.modification_time().to_rfc3339();

        let columns = vec![
            Series::new(
                PlSmallStr::from_static("graph_name"),
                vec![graph_name.to_string()],
            ),
            Series::new(PlSmallStr::from_static("node_count"), vec![node_count]),
            Series::new(
                PlSmallStr::from_static("relationship_count"),
                vec![relationship_count],
            ),
            Series::new(PlSmallStr::from_static("direction"), vec![direction]),
            Series::new(PlSmallStr::from_static("schema_json"), vec![schema_json]),
            Series::new(PlSmallStr::from_static("created_at"), vec![created_at]),
            Series::new(PlSmallStr::from_static("modified_at"), vec![modified_at]),
        ];

        Ok(PolarsDataFrameCollection::from_series(columns)?)
    }
}

impl GraphFramePlugin for GraphFramePolars32Plugin {
    fn name(&self) -> &'static str {
        "graphframe-polars32"
    }

    fn write_graph_store(
        &self,
        store: &DefaultGraphStore,
    ) -> Result<GraphFrameTables, GraphFramePluginError> {
        let nodes = self.build_nodes(store)?;
        let edges = self.build_edges(store)?;
        Ok(GraphFrameTables { nodes, edges })
    }
}

/// Convenience helper for the default Polars 32-bit plugin.
pub fn write_graph_store_polars32(
    store: &DefaultGraphStore,
) -> Result<GraphFrameTables, GraphFramePluginError> {
    GraphFramePolars32Plugin::default().write_graph_store(store)
}

/// Write GraphFrame tables into a disk catalog using Polars-backed tables.
pub fn write_graph_store_polars32_to_catalog(
    store: &DefaultGraphStore,
    catalog_root: impl Into<PathBuf>,
    graph_name: &str,
    format: CollectionsIoFormat,
) -> Result<GraphFrameCatalogWriteResult, GraphFramePluginError> {
    if graph_name.trim().is_empty() {
        return Err(GraphFramePluginError::Graph(
            "graph_name must not be empty".to_string(),
        ));
    }

    let plugin = GraphFramePolars32Plugin::default();
    let tables = plugin.write_graph_store(store)?;
    let graph_table = plugin.build_graph_table(store, graph_name)?;

    let mut catalog = CollectionsCatalogDisk::load(catalog_root)
        .map_err(|err| GraphFramePluginError::Graph(err.to_string()))?;

    let nodes_name = format!("{graph_name}.nodes");
    let edges_name = format!("{graph_name}.edges");
    let graph_entry_name = format!("{graph_name}.graph");

    let io_policy = CollectionsIoPolicy {
        format,
        ..Default::default()
    };

    let nodes_schema = Some(CollectionsSchema::from_polars(tables.nodes.dataframe()));
    let edges_schema = Some(CollectionsSchema::from_polars(tables.edges.dataframe()));
    let graph_schema = Some(CollectionsSchema::from_polars(graph_table.dataframe()));

    let nodes_entry = CollectionsCatalogDiskEntry {
        name: nodes_name.clone(),
        value_type: ValueType::Unknown,
        schema: nodes_schema,
        backend: CollectionsBackend::Arrow,
        extensions: Vec::new(),
        io_policy: io_policy.clone(),
        data_path: catalog.default_data_path(&nodes_name, format),
    };

    let edges_entry = CollectionsCatalogDiskEntry {
        name: edges_name.clone(),
        value_type: ValueType::Unknown,
        schema: edges_schema,
        backend: CollectionsBackend::Arrow,
        extensions: Vec::new(),
        io_policy: io_policy.clone(),
        data_path: catalog.default_data_path(&edges_name, format),
    };

    let graph_entry = CollectionsCatalogDiskEntry {
        name: graph_entry_name.clone(),
        value_type: ValueType::Unknown,
        schema: graph_schema,
        backend: CollectionsBackend::Arrow,
        extensions: Vec::new(),
        io_policy: io_policy.clone(),
        data_path: catalog.default_data_path(&graph_entry_name, format),
    };

    let _ = catalog.remove(&nodes_name);
    let _ = catalog.remove(&edges_name);
    let _ = catalog.remove(&graph_entry_name);

    catalog
        .register(nodes_entry.clone())
        .map_err(|err| GraphFramePluginError::Graph(err.to_string()))?;
    catalog
        .register(edges_entry.clone())
        .map_err(|err| GraphFramePluginError::Graph(err.to_string()))?;
    catalog
        .register(graph_entry.clone())
        .map_err(|err| GraphFramePluginError::Graph(err.to_string()))?;

    catalog
        .write_table(&nodes_entry, &tables.nodes)
        .map_err(|err| GraphFramePluginError::Graph(err.to_string()))?;
    catalog
        .write_table(&edges_entry, &tables.edges)
        .map_err(|err| GraphFramePluginError::Graph(err.to_string()))?;
    catalog
        .write_table(&graph_entry, &graph_table)
        .map_err(|err| GraphFramePluginError::Graph(err.to_string()))?;

    catalog
        .save()
        .map_err(|err| GraphFramePluginError::Graph(err.to_string()))?;

    Ok(GraphFrameCatalogWriteResult {
        nodes: nodes_entry,
        edges: edges_entry,
        graph: graph_entry,
    })
}
