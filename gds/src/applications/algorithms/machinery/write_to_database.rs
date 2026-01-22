//! WriteToDatabase (Java parity scaffold).
//!
//! Java reference: `WriteToDatabase`.
//! The real Java implementation writes via Neo4j exporter builders.
//!
//! Rust status:
//! - We don't have Neo4j exporter APIs.
//! - We *can* still support the control-flow shape and perform best-effort writes
//!   to the in-memory `GraphStore` (catalog-backed) via `GraphStoreService`.

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use crate::applications::graph_store_catalog::loaders::ResultStore;
use crate::applications::services::logging::Log;
use crate::concurrency::Concurrency;
use crate::core::utils::progress::JobId;
use crate::procedures::GraphFacade;
use crate::projection::NodeLabel;
use crate::types::graph_store::{GraphStore, GraphStoreResult};
use crate::types::properties::node::NodePropertyValues;

use super::{
    GraphStoreNodePropertiesWritten, GraphStoreService, Label, NodeProperty,
    RequestScopedDependencies, WriteContext,
};

/// Minimal adapter trait to keep Java parameter-shape without importing procedure-only configs.
pub trait WriteConfigLike {
    fn write_concurrency(&self) -> Concurrency;

    fn resolve_result_store<'a>(
        &self,
        result_store: Option<&'a dyn ResultStore>,
    ) -> Option<&'a dyn ResultStore> {
        result_store
    }
}

/// Minimal adapter trait to keep Java parameter-shape.
pub trait WritePropertyConfigLike {
    fn write_property(&self) -> &str;
}

pub struct WriteToDatabase {
    pub log: Log,
    pub request_scoped_dependencies: RequestScopedDependencies,
    pub write_context: WriteContext,
    pub graph_store_service: GraphStoreService,
}

impl WriteToDatabase {
    pub fn new(
        log: Log,
        request_scoped_dependencies: RequestScopedDependencies,
        write_context: WriteContext,
    ) -> Self {
        let graph_store_service = GraphStoreService::new(log.clone());
        Self {
            log,
            request_scoped_dependencies,
            write_context,
            graph_store_service,
        }
    }

    /// Java parity entry point (single property).
    ///
    /// Note: `graph`, `result_store`, `label`, `job_id`, and concurrency exist to preserve
    /// the Java call shape. The in-memory store write path does not use exporters.
    #[allow(unused_variables)]
    pub fn perform_single_property<S: GraphStore>(
        &self,
        graph: &GraphFacade,
        graph_store: &mut S,
        result_store: Option<&dyn ResultStore>,
        write_configuration: &dyn WriteConfigLike,
        write_property_configuration: &dyn WritePropertyConfigLike,
        label: &dyn Label,
        job_id: &JobId,
        labels_to_update: HashSet<NodeLabel>,
        node_property_values: Arc<dyn NodePropertyValues>,
    ) -> GraphStoreResult<GraphStoreNodePropertiesWritten> {
        self.log.info(&format!(
            "WriteToDatabase.perform_single_property: label={} property={} job_id={}",
            label.as_string(),
            write_property_configuration.write_property(),
            job_id
        ));

        let node_property = NodeProperty::new(
            write_property_configuration.write_property(),
            node_property_values,
        );
        self.graph_store_service.add_node_properties(
            graph_store,
            labels_to_update,
            &[node_property],
        )
    }

    /// Java parity entry point (multiple properties).
    #[allow(unused_variables)]
    pub fn perform_property_map<S: GraphStore>(
        &self,
        graph: &GraphFacade,
        graph_store: &mut S,
        result_store: Option<&dyn ResultStore>,
        write_configuration: &dyn WriteConfigLike,
        label: &dyn Label,
        job_id: &JobId,
        labels_to_update: HashSet<NodeLabel>,
        node_property_values_map: HashMap<String, Arc<dyn NodePropertyValues>>,
    ) -> GraphStoreResult<GraphStoreNodePropertiesWritten> {
        self.log.info(&format!(
            "WriteToDatabase.perform_property_map: label={} properties={} job_id={}",
            label.as_string(),
            node_property_values_map.len(),
            job_id
        ));

        let node_properties: Vec<NodeProperty> = node_property_values_map
            .into_iter()
            .map(|(key, values)| NodeProperty::new(key, values))
            .collect();
        self.graph_store_service.add_node_properties(
            graph_store,
            labels_to_update,
            &node_properties,
        )
    }
}
