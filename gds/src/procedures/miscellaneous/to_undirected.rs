//! ToUndirected procedure facade.
//!
//! Simple wrapper over `DefaultGraphStore::to_undirected`, matching the Java
//! miscellaneous facade surface.

use crate::procedures::traits::Result;
use crate::projection::eval::procedure::AlgorithmError;
use crate::types::graph_store::GraphName;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
pub struct ToUndirectedStats {
	pub node_count: u64,
	pub relationship_count: u64,
}

pub struct ToUndirectedFacade {
	graph_store: Arc<DefaultGraphStore>,
}

impl ToUndirectedFacade {
	pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
		Self { graph_store }
	}

	pub fn to_store(&self, graph_name: &str) -> Result<DefaultGraphStore> {
		self.graph_store
			.to_undirected(GraphName::new(graph_name))
			.map_err(|e| AlgorithmError::Execution(e.to_string()))
	}

	pub fn stats(&self) -> Result<ToUndirectedStats> {
		let store = self.to_store("to_undirected_temp")?;
		Ok(ToUndirectedStats {
			node_count: GraphStore::node_count(&store) as u64,
			relationship_count: GraphStore::relationship_count(&store) as u64,
		})
	}
}

