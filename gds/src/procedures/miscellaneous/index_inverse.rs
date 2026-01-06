//! IndexInverse procedure facade.
//!
//! Thin controller that builds inverse relationship indices via the algorithm
//! storage runtime and returns a new graph store.

use crate::algo::index_inverse::{
	IndexInverseComputationRuntime, IndexInverseConfig, IndexInverseStorageRuntime,
};
use crate::procedures::traits::Result;
use crate::projection::eval::procedure::AlgorithmError;
use crate::types::graph_store::GraphName;
use crate::types::prelude::DefaultGraphStore;
use std::sync::Arc;

pub struct IndexInverseFacade {
	graph_store: Arc<DefaultGraphStore>,
	mutate_graph_name: String,
	concurrency: usize,
}

impl IndexInverseFacade {
	pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
		Self {
			graph_store,
			mutate_graph_name: "index_inverse".to_string(),
			concurrency: 4,
		}
	}

	pub fn mutate_graph_name(mut self, name: impl Into<String>) -> Self {
		self.mutate_graph_name = name.into();
		self
	}

	pub fn concurrency(mut self, concurrency: usize) -> Self {
		self.concurrency = concurrency;
		self
	}

	pub fn to_store(&self, graph_name: &str) -> Result<DefaultGraphStore> {
		let mut computation = IndexInverseComputationRuntime::new();
		let storage = IndexInverseStorageRuntime::new(self.concurrency);

		let config = IndexInverseConfig {
			relationship_types: vec!["*".to_string()],
			concurrency: self.concurrency,
			mutate_graph_name: graph_name.to_string(),
		};

		let result = storage
			.compute(self.graph_store.as_ref(), &config, &mut computation)
			.map_err(|e| AlgorithmError::Execution(e))?;

		Ok(result.graph_store)
	}

	pub fn graph_name(&self) -> GraphName {
		GraphName::new(&self.mutate_graph_name)
	}
}

