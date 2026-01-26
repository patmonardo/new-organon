//! ToUndirected storage runtime.
//!
//! Translation source: `org.neo4j.gds.undirected.ToUndirectedAlgorithmFactory`.
//!
//! Builds a graph view for the requested relationship type and invokes the
//! computation runtime to produce an undirected projection.

use super::spec::{ToUndirectedConfig, ToUndirectedResult};
use super::ToUndirectedComputationRuntime;
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::MappedNodeId;
use crate::types::graph_store::{GraphName, GraphStore};
use crate::types::prelude::DefaultGraphStore;
use crate::types::schema::Direction;
use std::collections::HashSet;

pub struct ToUndirectedStorageRuntime {
    concurrency: usize,
}

impl ToUndirectedStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }

    pub fn compute(
        &self,
        graph_store: &DefaultGraphStore,
        config: &ToUndirectedConfig,
        computation: &mut ToUndirectedComputationRuntime,
    ) -> Result<ToUndirectedResult, String> {
        if config.relationship_type.is_empty() {
            return Err("relationship_type must be provided".to_string());
        }
        if config.mutate_relationship_type.is_empty() {
            return Err("mutate_relationship_type must be provided".to_string());
        }
        if config.mutate_graph_name.is_empty() {
            return Err("mutate_graph_name must be provided".to_string());
        }

        let mut rels = HashSet::new();
        rels.insert(RelationshipType::of(&config.relationship_type));

        let graph = graph_store
            .get_graph_with_types_and_orientation(&rels, Orientation::Natural)
            .map_err(|e| {
                format!(
                    "failed to build graph for relationship type '{}': {e}",
                    config.relationship_type
                )
            })?;

        let edges = computation.compute(graph.as_ref(), &config.mutate_relationship_type);
        let outgoing = build_outgoing(graph_store.node_count(), edges)?;
        let edges_for_result: Vec<(u64, u64)> = outgoing
            .iter()
            .enumerate()
            .flat_map(|(src, targets)| targets.iter().map(move |t| (src as u64, *t as u64)))
            .collect();

        let store = graph_store
            .with_added_relationship_type(
                GraphName::new(&config.mutate_graph_name),
                RelationshipType::of(&config.mutate_relationship_type),
                outgoing,
                Direction::Undirected,
            )
            .map_err(|e| e.to_string())?;

        Ok(ToUndirectedResult {
            graph_name: config.mutate_graph_name.clone(),
            mutate_relationship_type: config.mutate_relationship_type.clone(),
            node_count: store.node_count() as u64,
            relationship_count: store.relationship_count() as u64,
            edges: edges_for_result,
            graph_store: store,
        })
    }

    pub fn concurrency(&self) -> usize {
        self.concurrency
    }
}

fn build_outgoing(
    node_count: usize,
    mut edges: Vec<(u64, u64)>,
) -> Result<Vec<Vec<MappedNodeId>>, String> {
    let mut outgoing: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];
    for (src, tgt) in edges.drain(..) {
        let src_usize = src as usize;
        let tgt_usize = tgt as usize;
        if src_usize >= node_count || tgt_usize >= node_count {
            return Err(format!(
                "edge ({src},{tgt}) is out of bounds for node_count={node_count}"
            ));
        }
        outgoing[src_usize].push(tgt as MappedNodeId);
    }

    for adj in outgoing.iter_mut() {
        adj.sort_unstable();
        adj.dedup();
    }

    Ok(outgoing)
}
