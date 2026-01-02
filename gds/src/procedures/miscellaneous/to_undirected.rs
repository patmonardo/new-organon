//! ToUndirected procedure facade.
//!
//! In Neo4j GDS, `toUndirected` is a utility that produces an undirected view/copy of a graph.
//! In this Rust port, the procedure facade is responsible for procedure semantics and
//! delegates the store transformation to `DefaultGraphStore`.

use crate::procedures::traits::Result;
use crate::types::graph_store::{DefaultGraphStore, GraphName};
use crate::types::prelude::GraphStore;
use std::sync::Arc;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ToUndirectedStats {
    pub node_count: u64,
    pub relationship_count: u64,
}

#[derive(Clone)]
pub struct ToUndirectedFacade {
    graph_store: Arc<DefaultGraphStore>,
}

impl ToUndirectedFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self { graph_store }
    }

    /// Creates a new undirected graph store.
    ///
    /// Relationship properties are currently not carried over.
    pub fn to_store(self, new_graph_name: impl AsRef<str>) -> Result<DefaultGraphStore> {
        let graph_name = GraphName::new(new_graph_name.as_ref());
        self.graph_store
            .to_undirected(graph_name)
            .map_err(|e| crate::projection::eval::procedure::AlgorithmError::Execution(e.to_string()))
    }

    /// Stats for an undirected projection.
    pub fn stats(self) -> Result<ToUndirectedStats> {
        let store = self.to_store("toUndirected")?;
        Ok(ToUndirectedStats {
            node_count: GraphStore::node_count(&store) as u64,
            relationship_count: GraphStore::relationship_count(&store) as u64,
        })
    }
}
