//! GraphStoreService (Java parity, simplified).
//!
//! Java reference: `GraphStoreService.addNodeProperties(...)`.
//!
//! This is a control-plane helper: it writes node properties back into the in-memory graph store.
//! The translation for "filtered graphs" is intentionally not implemented here; it requires
//! deeper graph view infrastructure.

use std::collections::HashSet;
use std::sync::Arc;

use crate::applications::services::logging::Log;
use crate::projection::NodeLabel;
use crate::types::graph_store::{GraphStore, GraphStoreResult};
use crate::types::properties::node::NodePropertyValues;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodePropertiesWritten(pub usize);

#[derive(Clone)]
pub struct NodeProperty {
    pub key: String,
    pub values: Arc<dyn NodePropertyValues>,
}

impl NodeProperty {
    pub fn new(key: impl Into<String>, values: Arc<dyn NodePropertyValues>) -> Self {
        Self {
            key: key.into(),
            values,
        }
    }
}

pub struct GraphStoreService {
    pub log: Log,
}

impl GraphStoreService {
    pub fn new(log: Log) -> Self {
        Self { log }
    }

    pub fn add_node_properties<S>(
        &self,
        graph_store: &mut S,
        labels_to_update: HashSet<NodeLabel>,
        node_properties: &[NodeProperty],
    ) -> GraphStoreResult<NodePropertiesWritten>
    where
        S: GraphStore,
    {
        self.log.info("Updating in-memory graph store");

        for node_property in node_properties {
            graph_store.add_node_property(
                labels_to_update.clone(),
                node_property.key.clone(),
                Arc::clone(&node_property.values) as Arc<dyn NodePropertyValues>,
            )?;
        }

        Ok(NodePropertiesWritten(
            node_properties.len() * graph_store.node_count(),
        ))
    }
}
