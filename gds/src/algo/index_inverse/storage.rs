//! IndexInverse storage runtime.
//!
//! Translation source: `org.neo4j.gds.indexInverse.InverseRelationships`.
//! Builds a graph store with inverse relationship indices populated.

use super::computation::IndexInverseComputationRuntime;
use super::spec::{IndexInverseConfig, IndexInverseResult};
use crate::types::graph_store::{GraphName, GraphStore};
use crate::types::prelude::DefaultGraphStore;

pub struct IndexInverseStorageRuntime {
    concurrency: usize,
}

impl IndexInverseStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }

    /// Build inverse indices for the configured relationship types.
    ///
    /// Note: current implementation applies to all relationship types in the
    /// store. Relationship-type filtering can be layered later if needed.
    pub fn compute(
        &self,
        graph_store: &DefaultGraphStore,
        config: &IndexInverseConfig,
        _computation: &mut IndexInverseComputationRuntime,
    ) -> Result<IndexInverseResult, String> {
        let graph_name = GraphName::new(&config.mutate_graph_name);

        let new_store = graph_store
            .with_inverse_indices(graph_name)
            .map_err(|e| e.to_string())?;

        let node_count = new_store.node_count() as u64;
        let relationship_count = new_store.relationship_count() as u64;

        Ok(IndexInverseResult {
            graph_name: config.mutate_graph_name.clone(),
            node_count,
            relationship_count,
            graph_store: new_store,
        })
    }

    pub fn concurrency(&self) -> usize {
        self.concurrency
    }
}
