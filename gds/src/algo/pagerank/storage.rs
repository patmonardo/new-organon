//! PageRank storage runtime
//!
//! Storage owns graph access and orchestration; computation remains pure state.

use crate::algo::pagerank::{PageRankComputationRuntime, PageRankRunResult};
use crate::core::utils::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::Graph;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

pub struct PageRankStorageRuntime<'a, G: GraphStore> {
    graph_store: &'a G,
    graph: Arc<dyn Graph>,
}

impl<'a, G: GraphStore> PageRankStorageRuntime<'a, G> {
    pub fn with_relationship_types_and_orientation(
        graph_store: &'a G,
        rel_types: &HashSet<RelationshipType>,
        orientation: Orientation,
    ) -> Result<Self, AlgorithmError> {
        let graph = graph_store
            .get_graph_with_types_and_orientation(rel_types, orientation)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        Ok(Self { graph_store, graph })
    }

    pub fn with_orientation(
        graph_store: &'a G,
        orientation: Orientation,
    ) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = graph_store.relationship_types();
        Self::with_relationship_types_and_orientation(graph_store, &rel_types, orientation)
    }

    pub fn graph_store(&self) -> &'a G {
        self.graph_store
    }

    pub fn graph(&self) -> Arc<dyn Graph> {
        Arc::clone(&self.graph)
    }

    pub fn run(
        &self,
        computation: &PageRankComputationRuntime,
        concurrency: usize,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> PageRankRunResult {
        let node_count = self.graph.node_count() as usize;

        let mut out_degree: Vec<usize> = vec![0; node_count];
        for i in 0..node_count {
            out_degree[i] = self.graph.degree(i as i64);
        }

        let fallback = self.graph.default_property_value();
        let stream_neighbors = |source: usize, push: &mut dyn FnMut(usize)| {
            for cursor in self.graph.stream_relationships(source as i64, fallback) {
                push(cursor.target_id() as usize);
            }
        };

        let result = computation.run(node_count, &out_degree, concurrency, &stream_neighbors);

        progress_tracker.log_progress(computation.max_iterations());

        result
    }
}
