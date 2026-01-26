//! CollapsePath procedure facade.
//!
//! CollapsePath procedure facade.
//!
//! Validates configuration, builds storage/computation runtimes, and returns a
//! new graph store with collapsed edges added as a relationship type.

use crate::algo::algorithms::Result;
use crate::algo::walking::{
    CollapsePathComputationRuntime, CollapsePathConfig, CollapsePathStats,
    CollapsePathStorageRuntime,
};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::sync::Arc;

pub struct CollapsePathFacade {
    graph_store: Arc<DefaultGraphStore>,
    path_templates: Vec<Vec<String>>,
    mutate_relationship_type: String,
    mutate_graph_name: String,
    allow_self_loops: bool,
    concurrency: usize,
}

impl CollapsePathFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            path_templates: Vec::new(),
            mutate_relationship_type: "collapsed".to_string(),
            mutate_graph_name: "collapse_path".to_string(),
            allow_self_loops: false,
            concurrency: 4,
        }
    }

    pub fn path_templates<I, P>(mut self, templates: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: IntoIterator,
        P::Item: Into<String>,
    {
        self.path_templates = templates
            .into_iter()
            .map(|path| path.into_iter().map(|s| s.into()).collect())
            .collect();
        self
    }

    pub fn mutate_relationship_type(mut self, rel_type: impl Into<String>) -> Self {
        self.mutate_relationship_type = rel_type.into();
        self
    }

    pub fn mutate_graph_name(mut self, graph_name: impl Into<String>) -> Self {
        self.mutate_graph_name = graph_name.into();
        self
    }

    pub fn allow_self_loops(mut self, allow: bool) -> Self {
        self.allow_self_loops = allow;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Produce a new graph store with collapsed paths.
    pub fn to_store(&self, graph_name: &str) -> Result<DefaultGraphStore> {
        if self.path_templates.is_empty() {
            return Err(AlgorithmError::Execution(
                "pathTemplates must be provided".to_string(),
            ));
        }

        let mut computation = CollapsePathComputationRuntime::new(self.allow_self_loops);
        let storage = CollapsePathStorageRuntime::new(self.concurrency);
        let config = CollapsePathConfig {
            path_templates: self.path_templates.clone(),
            mutate_relationship_type: self.mutate_relationship_type.clone(),
            mutate_graph_name: graph_name.to_string(),
            allow_self_loops: self.allow_self_loops,
            concurrency: self.concurrency,
        };

        storage
            .compute(&self.graph_store, &config, &mut computation)
            .map(|r| r.graph_store)
            .map_err(AlgorithmError::Execution)
    }

    pub fn stats(&self, graph_name: &str) -> Result<CollapsePathStats> {
        let store = self.to_store(graph_name)?;
        Ok(CollapsePathStats {
            graph_name: graph_name.to_string(),
            mutate_relationship_type: self.mutate_relationship_type.clone(),
            node_count: store.node_count() as u64,
            relationship_count: store.relationship_count() as u64,
        })
    }

    pub fn graph_name(&self) -> String {
        self.mutate_graph_name.clone()
    }
}
