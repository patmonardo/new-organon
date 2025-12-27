//! Bridges Facade
//!
//! Live wiring for bridge edge detection in undirected graphs.

use crate::procedures::bridges::computation::{Bridge, BridgesComputationRuntime};
use crate::procedures::facades::traits::{AlgorithmRunner, Result, StatsResults, StreamResults};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Result row for bridges stream mode
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct BridgeRow {
    pub from: u64,
    pub to: u64,
}

/// Statistics for bridges computation
#[derive(Debug, Clone, serde::Serialize)]
pub struct BridgesStats {
    pub bridge_count: usize,
    pub execution_time_ms: u64,
}

/// Bridges facade bound to a live graph store
#[derive(Clone)]
pub struct BridgesFacade {
    graph_store: Arc<DefaultGraphStore>,
}

impl BridgesFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self { graph_store }
    }

    fn checked_node_id(value: usize) -> Result<NodeId> {
        NodeId::try_from(value as i64).map_err(|_| {
            crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                "node_id must fit into i64 (got {})",
                value
            ))
        })
    }

    fn compute_bridges(&self) -> Result<(Vec<Bridge>, std::time::Duration)> {
        let start = Instant::now();

        // Bridges are defined on undirected graphs
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((Vec::new(), start.elapsed()));
        }

        let fallback = graph_view.default_property_value();
        let get_neighbors = |node_idx: usize| -> Vec<usize> {
            let node_id = match Self::checked_node_id(node_idx) {
                Ok(value) => value,
                Err(_) => return Vec::new(),
            };

            graph_view
                .stream_relationships(node_id, fallback)
                .map(|cursor| cursor.target_id())
                .filter(|target| *target >= 0)
                .map(|target| target as usize)
                .collect()
        };

        let mut runtime = BridgesComputationRuntime::new(node_count);
        let result = runtime.compute(node_count, get_neighbors);

        Ok((result.bridges, start.elapsed()))
    }
}

impl AlgorithmRunner for BridgesFacade {
    fn algorithm_name(&self) -> &'static str {
        "bridges"
    }

    fn description(&self) -> &'static str {
        "Find bridge edges (cut edges) in an undirected graph"
    }
}

impl StreamResults<BridgeRow> for BridgesFacade {
    fn stream(&self) -> Result<Box<dyn Iterator<Item = BridgeRow>>> {
        let (bridges, _elapsed) = self.compute_bridges()?;

        let rows: Vec<BridgeRow> = bridges
            .into_iter()
            .map(|bridge| BridgeRow {
                from: bridge.from,
                to: bridge.to,
            })
            .collect();

        Ok(Box::new(rows.into_iter()))
    }
}

impl StatsResults for BridgesFacade {
    type Stats = BridgesStats;

    fn stats(&self) -> Result<Self::Stats> {
        let (bridges, elapsed) = self.compute_bridges()?;

        Ok(BridgesStats {
            bridge_count: bridges.len(),
            execution_time_ms: elapsed.as_millis() as u64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::procedures::facades::traits::StreamResults;
    use crate::procedures::facades::Graph;
    use crate::projection::RelationshipType;
    use crate::types::graph::{RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::schema::{Direction, MutableGraphSchema};
    use std::collections::HashMap;

    fn store_from_undirected_edges(
        node_count: usize,
        edges: &[(usize, usize)],
    ) -> DefaultGraphStore {
        let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];

        for &(a, b) in edges {
            outgoing[a].push(b as i64);
            outgoing[b].push(a as i64);
            incoming[a].push(b as i64);
            incoming[b].push(a as i64);
        }

        let rel_type = RelationshipType::of("REL");

        let mut schema_builder = MutableGraphSchema::empty();
        schema_builder
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Undirected);
        let schema = schema_builder.build();

        let mut relationship_topologies = HashMap::new();
        relationship_topologies.insert(
            rel_type,
            RelationshipTopology::new(outgoing, Some(incoming)),
        );

        let original_ids: Vec<i64> = (0..node_count as i64).collect();
        let id_map = SimpleIdMap::from_original_ids(original_ids);

        DefaultGraphStore::new(
            crate::config::GraphStoreConfig::default(),
            GraphName::new("g"),
            DatabaseInfo::new(
                DatabaseId::new("db"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            schema,
            Capabilities::default(),
            id_map,
            relationship_topologies,
        )
    }

    #[test]
    fn facade_finds_bridge_on_path() {
        // 0-1-2-3 => all edges are bridges
        let store = store_from_undirected_edges(4, &[(0, 1), (1, 2), (2, 3)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph.bridges().stream().unwrap().collect();

        assert_eq!(rows.len(), 3);
        // All edges should be bridges
        assert!(rows
            .iter()
            .any(|r| (r.from == 0 && r.to == 1) || (r.from == 1 && r.to == 0)));
        assert!(rows
            .iter()
            .any(|r| (r.from == 1 && r.to == 2) || (r.from == 2 && r.to == 1)));
        assert!(rows
            .iter()
            .any(|r| (r.from == 2 && r.to == 3) || (r.from == 3 && r.to == 2)));
    }

    #[test]
    fn facade_cycle_has_no_bridges() {
        // 0-1-2-3-0 => no bridges (all edges in cycle)
        let store = store_from_undirected_edges(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph.bridges().stream().unwrap().collect();
        assert!(rows.is_empty());
    }

    #[test]
    fn facade_bridge_connects_cycles() {
        // Two cycles connected by a bridge: 0-1-2-0 and 3-4-5-3, connected by 2-3
        let store = store_from_undirected_edges(
            6,
            &[(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 5), (5, 3)],
        );
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph.bridges().stream().unwrap().collect();

        assert_eq!(rows.len(), 1);
        let bridge = &rows[0];
        assert!((bridge.from == 2 && bridge.to == 3) || (bridge.from == 3 && bridge.to == 2));
    }
}
