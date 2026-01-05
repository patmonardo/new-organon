//! Harmonic Centrality Storage Runtime
//!
//! This is the GraphStore-facing layer for Harmonic centrality.
//! It provides an oriented graph view and a neighbor callback compatible with
//! the MSBFS aggregated neighbor processing runtime.

use crate::projection::eval::procedure::AlgorithmError;
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::id_map::NodeId;
use crate::types::graph::Graph;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

pub struct HarmonicStorageRuntime<'a, G: GraphStore> {
    graph_store: &'a G,
    graph: Arc<dyn Graph>,
}

impl<'a, G: GraphStore> HarmonicStorageRuntime<'a, G> {
    pub fn with_orientation(
        graph_store: &'a G,
        orientation: Orientation,
    ) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, orientation)
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
