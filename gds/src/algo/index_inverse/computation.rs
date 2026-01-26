//! IndexInverse computation runtime.
//!
//! Builds per-relationship-type outgoing/incoming adjacency from a graph view.

use crate::types::graph::MappedNodeId;
use crate::types::graph::Graph;

#[derive(Default, Clone)]
pub struct IndexInverseComputationRuntime;

impl IndexInverseComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn build_topology(
        &self,
        graph: &dyn Graph,
    ) -> (Vec<Vec<MappedNodeId>>, Vec<Vec<MappedNodeId>>) {
        let node_count = graph.node_count();
        let fallback = graph.default_property_value();

        let mut outgoing: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];
        let mut incoming: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];

        for src in 0..node_count {
            for cursor in graph.stream_relationships(src as i64, fallback) {
                let tgt = cursor.target_id();
                if tgt < 0 {
                    continue;
                }
                let u = src as MappedNodeId;
                let v = tgt as MappedNodeId;
                outgoing[src].push(v);
                incoming[v as usize].push(u);
            }
        }

        for adj in outgoing.iter_mut() {
            adj.sort_unstable();
            adj.dedup();
        }
        for adj in incoming.iter_mut() {
            adj.sort_unstable();
            adj.dedup();
        }

        (outgoing, incoming)
    }
}
