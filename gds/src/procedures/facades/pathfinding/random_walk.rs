//! Random Walk Facade
//!
//! Generates random walks from nodes in the graph using biased sampling.
//! Supports node2vec-style exploration with configurable return and in-out factors.

use crate::procedures::facades::builder_base::ConfigValidator;
use crate::procedures::facades::traits::Result;
use crate::procedures::random_walk::computation::RandomWalkComputationRuntime;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Result row for random walk stream mode
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct RandomWalkRow {
    /// The walk as a sequence of node IDs
    pub path: Vec<u64>,
}

/// Statistics for random walk computation
#[derive(Debug, Clone, serde::Serialize)]
pub struct RandomWalkStats {
    pub walk_count: usize,
    pub execution_time_ms: u64,
}

/// Random Walk algorithm builder
#[derive(Clone)]
pub struct RandomWalkBuilder {
    graph_store: Arc<DefaultGraphStore>,
    walks_per_node: usize,
    walk_length: usize,
    return_factor: f64,
    in_out_factor: f64,
    source_nodes: Vec<u64>,
    random_seed: Option<u64>,
}

impl RandomWalkBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            walks_per_node: 10,
            walk_length: 80,
            return_factor: 1.0,
            in_out_factor: 1.0,
            source_nodes: Vec::new(),
            random_seed: None,
        }
    }

    pub fn walks_per_node(mut self, count: usize) -> Self {
        self.walks_per_node = count;
        self
    }

    pub fn walk_length(mut self, length: usize) -> Self {
        self.walk_length = length;
        self
    }

    pub fn return_factor(mut self, factor: f64) -> Self {
        self.return_factor = factor;
        self
    }

    pub fn in_out_factor(mut self, factor: f64) -> Self {
        self.in_out_factor = factor;
        self
    }

    pub fn source_nodes(mut self, nodes: Vec<u64>) -> Self {
        self.source_nodes = nodes;
        self
    }

    pub fn random_seed(mut self, seed: u64) -> Self {
        self.random_seed = Some(seed);
        self
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(
            self.walks_per_node as f64,
            1.0,
            1_000_000.0,
            "walks_per_node",
        )?;

        ConfigValidator::in_range(self.walk_length as f64, 1.0, 1_000_000.0, "walk_length")?;

        ConfigValidator::in_range(self.return_factor, 0.0, 100.0, "return_factor")?;

        ConfigValidator::in_range(self.in_out_factor, 0.0, 100.0, "in_out_factor")?;

        Ok(())
    }

    fn checked_node_id(value: usize) -> Result<NodeId> {
        NodeId::try_from(value as i64).map_err(|_| {
            crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                "node_id must fit into i64 (got {})",
                value
            ))
        })
    }

    fn compute(&self) -> Result<(Vec<Vec<u64>>, std::time::Duration)> {
        self.validate()?;
        let start = Instant::now();

        // Random walk works on directed graphs (Natural orientation)
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((Vec::new(), start.elapsed()));
        }

        let fallback = graph_view.default_property_value();

        // Convert source nodes to internal IDs
        let source_nodes_internal: Vec<usize> = self
            .source_nodes
            .clone()
            .into_iter()
            .map(|n| n as usize)
            .collect();

        // Get neighbors
        let get_neighbors = |node_idx: usize| -> Vec<usize> {
            let node_id = match Self::checked_node_id(node_idx) {
                Ok(value) => value,
                Err(_) => return Vec::new(),
            };

            graph_view
                .stream_relationships(node_id, fallback)
                .filter_map(|cursor| {
                    let target = cursor.target_id();
                    if target >= 0 {
                        Some(target as usize)
                    } else {
                        None
                    }
                })
                .collect()
        };

        let seed = self.random_seed.unwrap_or_else(|| {
            use std::time::SystemTime;
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });

        let runtime = RandomWalkComputationRuntime::new(
            self.walks_per_node,
            self.walk_length,
            self.return_factor,
            self.in_out_factor,
            source_nodes_internal,
            seed,
        );

        let result = runtime.compute(node_count, get_neighbors);

        Ok((result.walks, start.elapsed()))
    }

    /// Stream mode: yields walk sequences
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = RandomWalkRow>>> {
        let (walks, _elapsed) = self.compute()?;

        let rows: Vec<RandomWalkRow> = walks
            .into_iter()
            .map(|path| RandomWalkRow { path })
            .collect();

        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: returns aggregated statistics
    pub fn stats(&self) -> Result<RandomWalkStats> {
        let (walks, elapsed) = self.compute()?;

        Ok(RandomWalkStats {
            walk_count: walks.len(),
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
    fn facade_generates_walks() {
        // Simple path: 0 -> 1 -> 2
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2)]);
        let graph = Graph::new(Arc::new(store));

        let walks: Vec<_> = graph
            .random_walk()
            .walks_per_node(1)
            .walk_length(3)
            .source_nodes(vec![0])
            .random_seed(42)
            .stream()
            .unwrap()
            .collect();

        assert_eq!(walks.len(), 1);
        assert_eq!(walks[0].path, vec![0, 1, 2]);
    }

    #[test]
    fn facade_computes_stats() {
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2)]);
        let graph = Graph::new(Arc::new(store));

        let stats = graph
            .random_walk()
            .walks_per_node(5)
            .walk_length(3)
            .source_nodes(vec![0])
            .stats()
            .unwrap();

        assert_eq!(stats.walk_count, 5);
        assert!(stats.execution_time_ms < 1000);
    }

    #[test]
    fn facade_walks_from_all_nodes() {
        // Triangle
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2), (2, 0)]);
        let graph = Graph::new(Arc::new(store));

        let stats = graph
            .random_walk()
            .walks_per_node(2)
            .walk_length(5)
            .stats()
            .unwrap();

        // 3 nodes * 2 walks per node = 6 walks
        assert_eq!(stats.walk_count, 6);
    }
}
