//! Closeness Centrality Storage Runtime
//!
//! Storage-first controller for closeness centrality.
//!
//! Responsibilities:
//! - Build an oriented graph view from a `GraphStore`
//! - Provide neighbor access
//! - Own the top-level pipeline (farness -> closeness)

use crate::concurrency::{TerminatedException, TerminationFlag};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::NodeId;
use crate::types::graph::Graph;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

use super::ClosenessCentralityComputationRuntime;

pub struct ClosenessCentralityStorageRuntime<'a, G: GraphStore> {
    graph_store: &'a G,
    graph: Arc<dyn Graph>,
}

impl<'a, G: GraphStore> ClosenessCentralityStorageRuntime<'a, G> {
    pub fn new(graph_store: &'a G, orientation: Orientation) -> Result<Self, AlgorithmError> {
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

    /// Storage-owned closeness pipeline.
    pub fn compute_parallel(
        &self,
        computation: &ClosenessCentralityComputationRuntime,
        wasserman_faust: bool,
        concurrency: usize,
        termination: &TerminationFlag,
        on_farness_sources_done: Arc<dyn Fn(usize) + Send + Sync>,
        on_closeness_nodes_done: Arc<dyn Fn(usize) + Send + Sync>,
    ) -> Result<Vec<f64>, TerminatedException> {
        let node_count = self.node_count();
        if node_count == 0 {
            return Ok(Vec::new());
        }

        let neighbors = |n: usize| self.neighbors(n);

        let (farness, component) = computation.compute_farness_parallel(
            node_count,
            concurrency,
            termination,
            on_farness_sources_done,
            &neighbors,
        )?;

        computation.compute_closeness_parallel(
            node_count,
            wasserman_faust,
            concurrency,
            termination,
            &farness,
            &component,
            on_closeness_nodes_done,
        )
    }
}
