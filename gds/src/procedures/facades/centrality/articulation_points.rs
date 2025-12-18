//! Articulation Points Facade
//!
//! Articulation points (cut vertices) are nodes whose removal increases the
//! number of connected components in an undirected graph.
//!
//! This facade is the "live wiring" layer: it binds the algorithm runtime to a
//! `DefaultGraphStore` graph view.

use crate::procedures::articulation_points::computation::ArticulationPointsComputationRuntime;
use crate::procedures::facades::traits::{AlgorithmRunner, Result, StatsResults, StreamResults};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Result row for articulation points stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArticulationPointRow {
    pub node_id: u64,
}

/// Statistics for articulation points computation.
#[derive(Debug, Clone)]
pub struct ArticulationPointsStats {
    pub articulation_point_count: u64,
    pub execution_time_ms: u64,
}

/// Articulation points facade bound to a live graph store.
#[derive(Clone)]
pub struct ArticulationPointsFacade {
    graph_store: Arc<DefaultGraphStore>,
}

impl ArticulationPointsFacade {
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

    fn compute_bitset(&self) -> Result<(crate::collections::BitSet, std::time::Duration)> {
        let start = Instant::now();

        // Articulation points are defined on undirected connectivity.
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((crate::collections::BitSet::new(0), start.elapsed()));
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

        let mut runtime = ArticulationPointsComputationRuntime::new(node_count);
        let result = runtime.compute(node_count, get_neighbors);

        Ok((result.articulation_points, start.elapsed()))
    }
}

impl AlgorithmRunner for ArticulationPointsFacade {
    fn algorithm_name(&self) -> &'static str {
        "articulationPoints"
    }

    fn description(&self) -> &'static str {
        "Find cut vertices (articulation points) in an undirected graph"
    }
}

impl StreamResults<ArticulationPointRow> for ArticulationPointsFacade {
    fn stream(&self) -> Result<Box<dyn Iterator<Item = ArticulationPointRow>>> {
        let (bitset, _elapsed) = self.compute_bitset()?;

        // Emit only set bits as rows.
        let mut out: Vec<ArticulationPointRow> = Vec::with_capacity(bitset.cardinality());
        let mut idx = bitset.next_set_bit(0);
        while let Some(i) = idx {
            out.push(ArticulationPointRow { node_id: i as u64 });
            idx = bitset.next_set_bit(i + 1);
        }

        Ok(Box::new(out.into_iter()))
    }
}

impl StatsResults for ArticulationPointsFacade {
    type Stats = ArticulationPointsStats;

    fn stats(&self) -> Result<Self::Stats> {
        let (bitset, elapsed) = self.compute_bitset()?;
        Ok(ArticulationPointsStats {
            articulation_point_count: bitset.cardinality() as u64,
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
    fn facade_finds_articulation_points_on_path() {
        // 0-1-2-3-4 => 1,2,3
        let store = store_from_undirected_edges(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph.articulation_points().stream().unwrap().collect();
        let ids: Vec<u64> = rows.into_iter().map(|r| r.node_id).collect();

        assert!(ids.contains(&1));
        assert!(ids.contains(&2));
        assert!(ids.contains(&3));
        assert!(!ids.contains(&0));
        assert!(!ids.contains(&4));
    }

    #[test]
    fn facade_cycle_has_no_articulation_points() {
        // 0-1-2-3-0
        let store = store_from_undirected_edges(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph.articulation_points().stream().unwrap().collect();
        assert!(rows.is_empty());
    }
}
