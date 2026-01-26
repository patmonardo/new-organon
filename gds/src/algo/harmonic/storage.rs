//! Harmonic Centrality Storage Runtime
//!
//! Storage owns graph access, orientation, and concurrency orchestration. Computation
//! remains state-only.

use crate::algo::harmonic::HarmonicComputationRuntime;
use crate::algo::msbfs::{AggregatedNeighborProcessingMsBfs, OMEGA};
use crate::concurrency::virtual_threads::{Executor, WorkerContext};
use crate::concurrency::{Concurrency, TerminatedException, TerminationFlag};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::Graph;
use crate::types::graph::NodeId;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

pub struct HarmonicStorageRuntime<'a, G: GraphStore> {
    graph_store: &'a G,
    graph: Arc<dyn Graph>,
}

impl<'a, G: GraphStore> HarmonicStorageRuntime<'a, G> {
    pub fn with_orientation(
        graph_store: &'a G,
        orientation: Orientation,
    ) -> Result<Self, AlgorithmError> {
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

    /// Controller entrypoint: orchestrates MSBFS batches, termination, and progress reporting.
    pub fn compute_parallel(
        &self,
        computation: &HarmonicComputationRuntime,
        concurrency: usize,
        termination: &TerminationFlag,
        on_sources_done: Arc<dyn Fn(usize) + Send + Sync>,
    ) -> Result<Vec<f64>, TerminatedException> {
        let node_count = self.node_count();
        if node_count == 0 {
            return Ok(Vec::new());
        }

        debug_assert_eq!(node_count, computation.node_count());

        let executor = Executor::new(Concurrency::of(concurrency.max(1)));
        let msbfs_state =
            WorkerContext::new(move || AggregatedNeighborProcessingMsBfs::new(node_count));
        let neighbors = |n: usize| self.neighbors(n);

        let batch_count = (node_count + OMEGA - 1) / OMEGA;
        executor.parallel_for(0, batch_count, termination, |batch_idx| {
            if !termination.running() {
                return;
            }

            let source_offset = batch_idx * OMEGA;
            let source_len = (source_offset + OMEGA).min(node_count) - source_offset;

            msbfs_state.with(|msbfs| {
                computation.run_batch_with_termination(
                    msbfs,
                    source_offset,
                    source_len,
                    termination,
                    &neighbors,
                );
            });

            (on_sources_done.as_ref())(source_len);
        })?;

        Ok(computation.finalize())
    }
}
