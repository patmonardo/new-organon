//! ToUndirected storage runtime.
//!
//! Translation source: `org.neo4j.gds.undirected.ToUndirectedAlgorithmFactory`.
//!
//! Builds a graph view for the requested relationship type and invokes the
//! computation runtime to produce an undirected projection.

use super::computation::ToUndirectedComputationRuntime;
use super::spec::{ToUndirectedConfig, ToUndirectedResult};
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph_store::GraphStore;
use std::collections::HashSet;

pub struct ToUndirectedStorageRuntime {
    concurrency: usize,
}

impl ToUndirectedStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }

    pub fn compute(
        &self,
        graph_store: &impl GraphStore,
        config: &ToUndirectedConfig,
    ) -> Result<ToUndirectedResult, String> {
        if config.relationship_type.is_empty() {
            return Err("relationship_type must be provided".to_string());
        }

        let mut rels = HashSet::new();
        rels.insert(RelationshipType::of(&config.relationship_type));

        let graph = graph_store
            .get_graph_with_types_and_orientation(&rels, Orientation::Natural)
            .map_err(|e| format!("failed to build graph for relationship type '{}': {e}", config.relationship_type))?;

        let computation = ToUndirectedComputationRuntime::new();
        Ok(computation.compute(graph.as_ref(), &config.mutate_relationship_type))
    }

    pub fn concurrency(&self) -> usize {
        self.concurrency
    }
}
