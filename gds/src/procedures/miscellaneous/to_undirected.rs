//! ToUndirected procedure facade.
//!
//! Procedure facade that delegates to the undirected algorithm runtimes.

use crate::algo::algorithms::Result;
use crate::algo::undirected::{
    ToUndirectedComputationRuntime, ToUndirectedConfig, ToUndirectedStats,
    ToUndirectedStorageRuntime,
};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::DefaultGraphStore;
use std::sync::Arc;

pub struct ToUndirectedFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: ToUndirectedConfig,
}

impl ToUndirectedFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: ToUndirectedConfig::default(),
        }
    }

    pub fn relationship_type(mut self, rel_type: impl Into<String>) -> Self {
        self.config.relationship_type = rel_type.into();
        self
    }

    pub fn mutate_relationship_type(mut self, rel_type: impl Into<String>) -> Self {
        self.config.mutate_relationship_type = rel_type.into();
        self
    }

    pub fn mutate_graph_name(mut self, graph_name: impl Into<String>) -> Self {
        self.config.mutate_graph_name = graph_name.into();
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    fn validate_config(&self, config: &ToUndirectedConfig) -> Result<()> {
        if config.relationship_type.is_empty() {
            return Err(AlgorithmError::Execution(
                "relationshipType must be provided".to_string(),
            ));
        }
        if config.mutate_relationship_type.is_empty() {
            return Err(AlgorithmError::Execution(
                "mutateRelationshipType must be provided".to_string(),
            ));
        }
        if config.mutate_graph_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "mutateGraphName must be provided".to_string(),
            ));
        }
        Ok(())
    }

    /// Produce a new graph store with an undirected relationship type added.
    pub fn to_store(&self, graph_name: &str) -> Result<DefaultGraphStore> {
        let mut computation = ToUndirectedComputationRuntime::new();
        let storage = ToUndirectedStorageRuntime::new(self.config.concurrency);
        let mut config = self.config.clone();
        config.mutate_graph_name = graph_name.to_string();
        self.validate_config(&config)?;
        storage
            .compute(&self.graph_store, &config, &mut computation)
            .map(|r| r.graph_store)
            .map_err(AlgorithmError::Execution)
    }

    pub fn stats(&self) -> Result<ToUndirectedStats> {
        let mut computation = ToUndirectedComputationRuntime::new();
        let storage = ToUndirectedStorageRuntime::new(self.config.concurrency);
        self.validate_config(&self.config)?;
        let result = storage
            .compute(&self.graph_store, &self.config, &mut computation)
            .map_err(AlgorithmError::Execution)?;
        Ok(ToUndirectedStats {
            graph_name: result.graph_name,
            mutate_relationship_type: result.mutate_relationship_type,
            node_count: result.node_count,
            relationship_count: result.relationship_count,
        })
    }

    pub fn graph_name(&self) -> String {
        self.config.mutate_graph_name.clone()
    }
}
