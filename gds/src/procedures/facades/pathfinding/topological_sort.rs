//! Topological Sort Facade
//!
//! Orders nodes in a directed acyclic graph (DAG) such that for every edge (u, v),
//! u appears before v. Optionally computes longest path distances.

use crate::procedures::topological_sort::computation::TopologicalSortComputationRuntime;
use crate::procedures::facades::traits::Result;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Result row for topological sort stream mode
#[derive(Debug, Clone, PartialEq)]
pub struct TopologicalSortRow {
    pub node_id: u64,
    pub max_distance: Option<f64>,
}

/// Statistics for topological sort computation
#[derive(Debug, Clone)]
pub struct TopologicalSortStats {
    pub node_count: usize,
    pub execution_time_ms: u64,
}

/// Topological Sort algorithm builder
#[derive(Clone)]
pub struct TopologicalSortBuilder {
    graph_store: Arc<DefaultGraphStore>,
    compute_max_distance: bool,
}

impl TopologicalSortBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            compute_max_distance: false,
        }
    }

    pub fn compute_max_distance(mut self, value: bool) -> Self {
        self.compute_max_distance = value;
        self
    }

    fn checked_node_id(value: usize) -> Result<NodeId> {
        NodeId::try_from(value as i64).map_err(|_| {
            crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                "node_id must fit into i64 (got {})",
                value
            ))
        })
    }

    fn compute(&self) -> Result<(Vec<u64>, Option<Vec<f64>>, std::time::Duration)> {
        let start = Instant::now();

        // Topological sort works on directed graphs (Natural orientation)
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string()))?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((Vec::new(), None, start.elapsed()));
        }

        let fallback = graph_view.default_property_value();

        // Get neighbors with weights
        let get_neighbors = |node_idx: usize| -> Vec<(usize, f64)> {
            let node_id = match Self::checked_node_id(node_idx) {
                Ok(value) => value,
                Err(_) => return Vec::new(),
            };

            graph_view
                .stream_relationships(node_id, fallback)
                .filter_map(|cursor| {
                    let target = cursor.target_id();
                    if target < 0 {
                        return None;
                    }
                    let weight = cursor.property();
                    Some((target as usize, weight))
                })
                .collect()
        };

        let mut runtime = TopologicalSortComputationRuntime::new(node_count, self.compute_max_distance);
        let result = runtime.compute(node_count, get_neighbors);

        Ok((result.sorted_nodes, result.max_source_distances, start.elapsed()))
    }

    /// Stream mode: yields (node_id, max_distance) for each node in topological order
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = TopologicalSortRow>>> {
        let (sorted_nodes, max_distances, _elapsed) = self.compute()?;

        let rows: Vec<TopologicalSortRow> = sorted_nodes
            .into_iter()
            .map(|node_id| TopologicalSortRow {
                node_id,
                max_distance: max_distances.as_ref().map(|d| d[node_id as usize]),
            })
            .collect();

        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: returns aggregated statistics
    pub fn stats(&self) -> Result<TopologicalSortStats> {
        let (sorted_nodes, _max_distances, elapsed) = self.compute()?;

        Ok(TopologicalSortStats {
            node_count: sorted_nodes.len(),
            execution_time_ms: elapsed.as_millis() as u64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::procedures::facades::Graph;
    use crate::projection::RelationshipType;
    use crate::types::graph::{RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::schema::{Direction, MutableGraphSchema};
    use std::collections::HashMap;

    fn store_from_directed_edges(node_count: usize, edges: &[(usize, usize)]) -> DefaultGraphStore {
        let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];

        for &(a, b) in edges {
            outgoing[a].push(b as i64);
            incoming[b].push(a as i64);
        }

        let rel_type = RelationshipType::of("REL");

        let mut schema_builder = MutableGraphSchema::empty();
        schema_builder
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Directed);
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
    fn facade_computes_topological_order() {
        // Simple DAG: 0 -> 1 -> 2
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph
            .topological_sort()
            .stream()
            .unwrap()
            .collect();

        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].node_id, 0);
        assert_eq!(rows[1].node_id, 1);
        assert_eq!(rows[2].node_id, 2);
    }

    #[test]
    fn facade_computes_stats() {
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2)]);
        let graph = Graph::new(Arc::new(store));

        let stats = graph
            .topological_sort()
            .stats()
            .unwrap();

        assert_eq!(stats.node_count, 3);
        assert!(stats.execution_time_ms < 1000);
    }

    #[test]
    fn facade_computes_max_distances() {
        // Diamond DAG
        let store = store_from_directed_edges(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph
            .topological_sort()
            .compute_max_distance(true)
            .stream()
            .unwrap()
            .collect();

        assert_eq!(rows.len(), 4);

        // All nodes should have distances computed
        assert!(rows.iter().all(|r| r.max_distance.is_some()));

        // Node 0 (source) should have distance 0
        let node_0 = rows.iter().find(|r| r.node_id == 0).unwrap();
        assert_eq!(node_0.max_distance, Some(0.0));
    }
}
