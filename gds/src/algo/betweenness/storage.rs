//! Betweenness Centrality storage runtime
//!
//! Responsible for building a graph view from a GraphStore with the correct
//! orientation and optional relationship weight property.

use crate::concurrency::{TerminatedException, TerminationFlag};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::NodeId;
use crate::types::graph::Graph;
use crate::types::prelude::GraphStore;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::algo::betweenness::{
    BetweennessCentralityComputationResult, BetweennessCentralityComputationRuntime,
};

pub struct BetweennessCentralityStorageRuntime<'a, G: GraphStore> {
    graph_store: &'a G,
    graph: Arc<dyn Graph>,
    has_weights: bool,
}

impl<'a, G: GraphStore> BetweennessCentralityStorageRuntime<'a, G> {
    pub fn new(
        graph_store: &'a G,
        orientation: Orientation,
        relationship_weight_property: Option<&str>,
    ) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = graph_store.relationship_types();

        let has_weights = relationship_weight_property.is_some();

        let graph = if let Some(weight_prop) = relationship_weight_property {
            let selectors: HashMap<RelationshipType, String> = rel_types
                .iter()
                .map(|t| (t.clone(), weight_prop.to_string()))
                .collect();

            graph_store
                .get_graph_with_types_selectors_and_orientation(&rel_types, &selectors, orientation)
                .map_err(|e| AlgorithmError::Graph(e.to_string()))?
        } else {
            graph_store
                .get_graph_with_types_and_orientation(&rel_types, orientation)
                .map_err(|e| AlgorithmError::Graph(e.to_string()))?
        };

        Ok(Self {
            graph_store,
            graph,
            has_weights,
        })
    }

    pub fn has_weights(&self) -> bool {
        self.has_weights
    }

    pub fn graph_store(&self) -> &'a G {
        self.graph_store
    }

    pub fn graph(&self) -> &Arc<dyn Graph> {
        &self.graph
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn relationship_count(&self) -> usize {
        self.graph.relationship_count()
    }

    pub fn degree(&self, node_idx: usize) -> usize {
        let node_id = match NodeId::try_from(node_idx as i64) {
            Ok(id) => id,
            Err(_) => return 0,
        };
        self.graph.degree(node_id)
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

    pub fn neighbors_weighted(&self, node_idx: usize) -> Vec<(usize, f64)> {
        let node_id = match NodeId::try_from(node_idx as i64) {
            Ok(id) => id,
            Err(_) => return Vec::new(),
        };

        let fallback = self.graph.default_property_value();
        self.graph
            .stream_relationships(node_id, fallback)
            .map(|cursor| (cursor.target_id(), cursor.property()))
            .filter(|(target, _w)| *target >= 0)
            .map(|(target, w)| (target as usize, w))
            .collect()
    }

    /// Select source nodes for Brandes traversal.
    ///
    /// - `strategy`: "all" or "random_degree".
    /// - `sampling_size`: number of sources to pick (defaults to all nodes).
    pub fn select_sources(
        &self,
        strategy: &str,
        sampling_size: Option<usize>,
        seed: u64,
    ) -> Vec<usize> {
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;
        use std::cmp::Ordering;
        use std::collections::BinaryHeap;

        let node_count = self.node_count();
        if node_count == 0 {
            return Vec::new();
        }

        let requested = sampling_size.unwrap_or(node_count).min(node_count).max(1);
        if requested == node_count {
            return (0..node_count).collect();
        }

        let strat = strategy.to_lowercase().replace('_', "").replace('-', "");
        if strat != "randomdegree" {
            return (0..requested).collect();
        }

        #[derive(Debug, Clone)]
        struct SampleItem {
            key: f64,
            node: usize,
        }

        impl PartialEq for SampleItem {
            fn eq(&self, other: &Self) -> bool {
                self.key.eq(&other.key)
            }
        }
        impl Eq for SampleItem {}
        impl PartialOrd for SampleItem {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                // Max-heap by key so we can pop the largest and keep K smallest keys.
                other.key.partial_cmp(&self.key)
            }
        }
        impl Ord for SampleItem {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(other).unwrap_or(Ordering::Equal)
            }
        }

        // Degree-weighted sampling without replacement using Efraimidis–Spirakis keys.
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut heap: BinaryHeap<SampleItem> = BinaryHeap::new();

        for node in 0..node_count {
            let degree = self.degree(node).max(1) as f64;
            let mut u: f64 = rand::Rng::gen(&mut rng);
            if !u.is_finite() || u <= 0.0 {
                u = f64::MIN_POSITIVE;
            }
            let key = -u.ln() / degree;

            heap.push(SampleItem { key, node });
            if heap.len() > requested {
                heap.pop();
            }
        }

        let mut sources: Vec<usize> = heap.into_iter().map(|s| s.node).collect();
        sources.sort_unstable();
        sources
    }

    /// Execute betweenness centrality for the given sources, dispatching to weighted/unweighted.
    pub fn compute_betweenness(
        &self,
        computation: &mut BetweennessCentralityComputationRuntime,
        sources: &[usize],
        divisor: f64,
        concurrency: usize,
        termination: &TerminationFlag,
        on_source_done: Arc<dyn Fn() + Send + Sync>,
    ) -> Result<BetweennessCentralityComputationResult, TerminatedException> {
        if self.has_weights {
            let neigh = |n: usize| self.neighbors_weighted(n);
            computation.compute_parallel_weighted(
                sources,
                divisor,
                concurrency,
                termination,
                on_source_done,
                &neigh,
            )?;
        } else {
            let neigh = |n: usize| self.neighbors(n);
            computation.compute_parallel_unweighted(
                sources,
                divisor,
                concurrency,
                termination,
                on_source_done,
                &neigh,
            )?;
        }

        Ok(computation.finalize_result())
    }
}
