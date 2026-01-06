//! CollapsePath procedure facade.
//!
//! Uses the graph-store helper to collapse degree-2 chains. This mirrors the
//! Java miscellaneous facade without extra optimizations.

use crate::procedures::traits::Result;
use crate::projection::eval::procedure::AlgorithmError;
use crate::types::graph_store::GraphName;
use crate::types::prelude::DefaultGraphStore;
use std::sync::Arc;

pub struct CollapsePathFacade {
	graph_store: Arc<DefaultGraphStore>,
	max_hops: Option<usize>,
}

impl CollapsePathFacade {
	pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
		Self {
			graph_store,
			max_hops: None,
		}
	}

	/// Optional hop cap when collapsing chains.
	pub fn max_hops(mut self, max_hops: usize) -> Self {
		self.max_hops = Some(max_hops);
		self
	}

	/// Produce a new graph store with collapsed paths.
	pub fn to_store(&self, graph_name: &str) -> Result<DefaultGraphStore> {
		self.graph_store
			.collapse_paths_degree2(GraphName::new(graph_name), self.max_hops)
			.map_err(|e| AlgorithmError::Execution(e.to_string()))
	}
}

