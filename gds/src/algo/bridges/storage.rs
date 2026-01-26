//! Bridges Storage Runtime
//!
//! Bridges are defined on undirected graphs; this storage layer always builds an undirected
//! graph view from the GraphStore and exposes a neighbor callback.

use crate::concurrency::{TerminatedException, TerminationFlag};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::NodeId;
use crate::types::graph::Graph;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

use super::{Bridge, BridgesComputationResult, BridgesComputationRuntime};

pub struct BridgesStorageRuntime<'a, G: GraphStore> {
    graph_store: &'a G,
    graph: Arc<dyn Graph>,
}

impl<'a, G: GraphStore> BridgesStorageRuntime<'a, G> {
    pub fn new(graph_store: &'a G) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        Ok(Self { graph_store, graph })
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn relationship_count(&self) -> usize {
        self.graph.relationship_count()
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

    /// Storage-owned top-level controller for bridges.
    ///
    /// This method owns:
    /// - iteration over all nodes
    /// - termination checks
    /// - progress callback semantics
    /// - stack sizing (uses `relationship_count()` for Java parity)
    pub fn compute_parallel(
        &self,
        concurrency: usize,
        termination: &TerminationFlag,
        on_node_scanned: Arc<dyn Fn() + Send + Sync>,
    ) -> Result<Vec<Bridge>, TerminatedException> {
        let _ = concurrency; // Bridges is currently single-threaded.
        let mut computation = BridgesComputationRuntime::new_with_stack_capacity(
            self.node_count(),
            self.relationship_count(),
        );

        self.compute_bridges(&mut computation, termination, on_node_scanned)
            .map(|result| result.bridges)
    }

    /// Compute bridges using the Procedure-First Controller Pattern.
    ///
    /// Storage acts as controller, orchestrating the DFS traversal and delegating
    /// state operations to the computation runtime.
    pub fn compute_bridges(
        &self,
        computation: &mut BridgesComputationRuntime,
        termination: &TerminationFlag,
        on_node_scanned: Arc<dyn Fn() + Send + Sync>,
    ) -> Result<BridgesComputationResult, TerminatedException> {
        let node_count = self.node_count();
        if node_count == 0 {
            return Ok(BridgesComputationResult {
                bridges: Vec::new(),
            });
        }

        computation.reset(node_count);

        let neighbors = |n: usize| self.neighbors(n);
        let mut bridges: Vec<Bridge> = Vec::new();

        for i in 0..node_count {
            if !termination.running() {
                return Err(TerminatedException);
            }

            if !computation.is_visited(i) {
                computation.dfs_component(i, &neighbors, &mut bridges);
            }

            (on_node_scanned.as_ref())();
        }

        Ok(BridgesComputationResult { bridges })
    }
}
