//! ToUndirected computation runtime.
//!
//! Translation source: `org.neo4j.gds.undirected.ToUndirected` task logic.
//!
//! Symmetrizes a single relationship type by adding reverse edges for every
//! observed edge, producing a deduplicated edge set.

use crate::types::graph::Graph;
use std::collections::BTreeSet;

pub struct ToUndirectedComputationRuntime;

impl ToUndirectedComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Symmetrize the given graph view, returning unique directed edges that
    /// collectively represent an undirected projection.
    pub fn compute(&self, graph: &dyn Graph, _mutate_relationship_type: &str) -> Vec<(u64, u64)> {
        let node_count = graph.node_count();
        let fallback = graph.default_property_value();

        let mut edges: BTreeSet<(u64, u64)> = BTreeSet::new();

        for source in 0..node_count {
            for cursor in graph.stream_relationships(source as i64, fallback) {
                let target = cursor.target_id();
                if target < 0 {
                    continue;
                }
                let u = source as u64;
                let v = target as u64;
                edges.insert((u, v));
                edges.insert((v, u));
            }
        }

        edges.into_iter().collect()
    }
}
