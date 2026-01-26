//! Degree Centrality storage runtime
//!
//! Storage owns top-level control:
//! - graph projection (orientation + optional weight selector)
//! - concurrency and partitioning
//! - termination checks
//! - progress callbacks

use crate::algo::degree_centrality::DegreeCentralityComputationRuntime;
use crate::collections::HugeAtomicDoubleArray;
use crate::concurrency::virtual_threads::Executor;
use crate::concurrency::{Concurrency, TerminatedException, TerminationFlag};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::{Orientation as ProjectionOrientation, RelationshipType};
use crate::types::graph::NodeId;
use crate::types::graph::Graph;
use crate::types::prelude::GraphStore;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Natural,
    Reverse,
    Undirected,
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Natural
    }
}

impl Orientation {
    fn to_projection(self) -> ProjectionOrientation {
        match self {
            Orientation::Natural => ProjectionOrientation::Natural,
            Orientation::Reverse => ProjectionOrientation::Reverse,
            Orientation::Undirected => ProjectionOrientation::Undirected,
        }
    }
}

pub struct DegreeCentralityStorageRuntime<'a, G: GraphStore> {
    graph_store: &'a G,
    graph: Arc<dyn Graph>,
    weighted: bool,
}

impl<'a, G: GraphStore> DegreeCentralityStorageRuntime<'a, G> {
    pub fn with_settings(
        graph_store: &'a G,
        orientation: Orientation,
        weighted: bool,
    ) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = graph_store.relationship_types();

        let graph = if weighted {
            // Java parity default weight property name.
            let selectors: HashMap<RelationshipType, String> = rel_types
                .iter()
                .map(|t| (t.clone(), "weight".to_string()))
                .collect();

            graph_store
                .get_graph_with_types_selectors_and_orientation(
                    &rel_types,
                    &selectors,
                    orientation.to_projection(),
                )
                .map_err(|e| AlgorithmError::Graph(e.to_string()))?
        } else {
            graph_store
                .get_graph_with_types_and_orientation(&rel_types, orientation.to_projection())
                .map_err(|e| AlgorithmError::Graph(e.to_string()))?
        };

        Ok(Self {
            graph_store,
            graph,
            weighted,
        })
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn graph_store(&self) -> &'a G {
        self.graph_store
    }

    pub fn weighted(&self) -> bool {
        self.weighted
    }

    fn degree_unweighted(&self, node_idx: usize) -> f64 {
        let node_id = match NodeId::try_from(node_idx as i64) {
            Ok(id) => id,
            Err(_) => return 0.0,
        };
        self.graph.degree(node_id) as f64
    }

    fn degree_weighted(&self, node_idx: usize) -> f64 {
        let node_id = match NodeId::try_from(node_idx as i64) {
            Ok(id) => id,
            Err(_) => return 0.0,
        };

        // Java parity default: missing weight => 0.0.
        let fallback = 0.0;
        self.graph
            .stream_relationships(node_id, fallback)
            .map(|cursor| cursor.property())
            .sum::<f64>()
    }

    pub fn compute_parallel(
        &self,
        computation: &DegreeCentralityComputationRuntime,
        concurrency: usize,
        termination: &TerminationFlag,
        on_nodes_done: Arc<dyn Fn(usize) + Send + Sync>,
    ) -> Result<Vec<f64>, TerminatedException> {
        let node_count = self.node_count();
        if node_count == 0 {
            return Ok(Vec::new());
        }

        let concurrency = concurrency.max(1);
        let out = HugeAtomicDoubleArray::new(node_count);

        let target_batches = concurrency * 8;
        let batch_size = ((node_count + target_batches - 1) / target_batches).max(1);
        let batch_count = (node_count + batch_size - 1) / batch_size;

        let executor = Executor::new(Concurrency::of(concurrency));

        if self.weighted {
            let degree = |n: usize| self.degree_weighted(n);
            executor.parallel_for(0, batch_count, termination, |batch_idx| {
                if !termination.running() {
                    return;
                }

                let start = batch_idx * batch_size;
                let end = (start + batch_size).min(node_count);

                computation.compute_range(start, end, termination, &out, &degree);

                (on_nodes_done.as_ref())(end - start);
            })?;
        } else {
            let degree = |n: usize| self.degree_unweighted(n);
            executor.parallel_for(0, batch_count, termination, |batch_idx| {
                if !termination.running() {
                    return;
                }

                let start = batch_idx * batch_size;
                let end = (start + batch_size).min(node_count);

                computation.compute_range(start, end, termination, &out, &degree);

                (on_nodes_done.as_ref())(end - start);
            })?;
        }

        let mut result = vec![0.0f64; node_count];
        for i in 0..node_count {
            result[i] = out.get(i);
        }

        Ok(result)
    }
}
