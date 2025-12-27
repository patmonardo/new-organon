//! CELF Facade
//!
//! Live wiring for Cost-Effective Lazy Forward influence maximization.

use crate::procedures::celf::computation::CELFComputationRuntime;
use crate::procedures::celf::spec::CELFConfig;
use crate::procedures::facades::traits::{AlgorithmRunner, Result, StatsResults, StreamResults};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;

/// Result row for CELF stream mode
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct CELFRow {
    pub node_id: u64,
    pub spread: f64,
}

/// Statistics for CELF computation
#[derive(Debug, Clone, serde::Serialize)]
pub struct CELFStats {
    pub seed_count: usize,
    pub total_spread: f64,
    pub execution_time_ms: u64,
}

/// CELF facade bound to a live graph store
#[derive(Clone)]
pub struct CELFFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: CELFConfig,
}

impl CELFFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: CELFConfig::default(),
        }
    }

    pub fn with_config(mut self, config: CELFConfig) -> Self {
        self.config = config;
        self
    }

    pub fn seed_set_size(mut self, size: usize) -> Self {
        self.config.seed_set_size = size;
        self
    }

    pub fn monte_carlo_simulations(mut self, simulations: usize) -> Self {
        self.config.monte_carlo_simulations = simulations;
        self
    }

    pub fn propagation_probability(mut self, prob: f64) -> Self {
        self.config.propagation_probability = prob;
        self
    }

    pub fn batch_size(mut self, size: usize) -> Self {
        self.config.batch_size = size;
        self
    }

    pub fn random_seed(mut self, seed: u64) -> Self {
        self.config.random_seed = seed;
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

    fn compute_seed_set(&self) -> Result<(HashMap<u64, f64>, std::time::Duration)> {
        let start = Instant::now();

        // CELF typically runs on directed graphs (default NATURAL orientation)
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((HashMap::new(), start.elapsed()));
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

        let runtime = CELFComputationRuntime::new(self.config.clone(), node_count);
        let seed_set = runtime.compute(get_neighbors);

        Ok((seed_set, start.elapsed()))
    }
}

impl AlgorithmRunner for CELFFacade {
    fn algorithm_name(&self) -> &'static str {
        "celf"
    }

    fn description(&self) -> &'static str {
        "Cost-Effective Lazy Forward influence maximization (Independent Cascade)"
    }
}

impl StreamResults<CELFRow> for CELFFacade {
    fn stream(&self) -> Result<Box<dyn Iterator<Item = CELFRow>>> {
        let (seed_set, _elapsed) = self.compute_seed_set()?;

        let mut rows: Vec<CELFRow> = seed_set
            .into_iter()
            .map(|(node_id, spread)| CELFRow { node_id, spread })
            .collect();

        // Sort by spread descending, then by node_id ascending
        rows.sort_by(|a, b| {
            b.spread
                .partial_cmp(&a.spread)
                .unwrap()
                .then_with(|| a.node_id.cmp(&b.node_id))
        });

        Ok(Box::new(rows.into_iter()))
    }
}

impl StatsResults for CELFFacade {
    type Stats = CELFStats;

    fn stats(&self) -> Result<Self::Stats> {
        let (seed_set, elapsed) = self.compute_seed_set()?;
        let total_spread: f64 = seed_set.values().sum();

        Ok(CELFStats {
            seed_count: seed_set.len(),
            total_spread,
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

    fn store_from_directed_edges(node_count: usize, edges: &[(usize, usize)]) -> DefaultGraphStore {
        let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];

        for &(src, tgt) in edges {
            outgoing[src].push(tgt as i64);
            incoming[tgt].push(src as i64);
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
    fn facade_finds_seed_nodes_on_star() {
        // 0 -> 1, 0 -> 2, 0 -> 3
        // Node 0 is the hub with maximum influence
        let store = store_from_directed_edges(4, &[(0, 1), (0, 2), (0, 3)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph
            .celf()
            .seed_set_size(1)
            .monte_carlo_simulations(10)
            .propagation_probability(1.0)
            .stream()
            .unwrap()
            .collect();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].node_id, 0);
    }

    #[test]
    fn facade_respects_seed_set_size() {
        let store = store_from_directed_edges(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph
            .celf()
            .seed_set_size(3)
            .monte_carlo_simulations(10)
            .propagation_probability(0.5)
            .stream()
            .unwrap()
            .collect();

        assert_eq!(rows.len(), 3);
    }
}
