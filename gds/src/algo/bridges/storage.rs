//! Bridges Storage Runtime
//!
//! Bridges are defined on undirected graphs; this storage layer always builds an undirected
//! graph view from the GraphStore and exposes a neighbor callback.

use crate::projection::eval::procedure::AlgorithmError;
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::id_map::NodeId;
use crate::types::graph::Graph;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

pub struct BridgesStorageRuntime<'a, G: GraphStore> {
    graph_store: &'a G,
    graph: Arc<dyn Graph>,
}

impl<'a, G: GraphStore> BridgesStorageRuntime<'a, G> {
    pub fn new(graph_store: &'a G) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        Ok(Self { graph_store, graph })
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn graph_store(&self) -> &'a G {
        self.graph_store
    }

    pub fn graph(&self) -> &Arc<dyn Graph> {
        &self.graph
    }

    pub fn neighbors(&self, node_idx: usize) -> Vec<usize> {
        let node_id = match NodeId::try_from(node_idx as i64) {
            Ok(id) => id,
            Err(_) => return Vec::new(),
        };

        let fallback = self.graph.default_property_value();
        self.graph
            .stream_relationships(node_id, fallback)
            .map(|cursor| cursor.target_id())
            .filter(|target| *target >= 0)
            .map(|target| target as usize)
            .collect()
    }
}
