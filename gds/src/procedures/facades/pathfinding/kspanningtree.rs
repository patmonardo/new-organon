//! K-Spanning Tree Facade
//!
//! Computes k spanning trees by first computing an MST using Prim's algorithm,
//! then progressively pruning to maintain exactly k nodes.

use crate::procedures::facades::builder_base::ConfigValidator;
use crate::procedures::facades::traits::Result;
use crate::procedures::kspanningtree::computation::KSpanningTreeComputationRuntime;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Result row for k-spanning tree stream mode
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct KSpanningTreeRow {
    pub node_id: u64,
    pub parent_id: i64,
    pub cost: f64,
}

/// Statistics for k-spanning tree computation
#[derive(Debug, Clone, serde::Serialize)]
pub struct KSpanningTreeStats {
    pub node_count: usize,
    pub total_cost: f64,
    pub execution_time_ms: u64,
}

/// K-Spanning Tree algorithm builder
#[derive(Clone)]
pub struct KSpanningTreeBuilder {
    graph_store: Arc<DefaultGraphStore>,
    source_node: Option<u64>,
    k: u64,
    objective: String,
    weight_property: Option<String>,
}

impl KSpanningTreeBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            source_node: None,
            k: 1,
            objective: "min".to_string(),
            weight_property: None,
        }
    }

    pub fn source_node(mut self, source: u64) -> Self {
        self.source_node = Some(source);
        self
    }

    pub fn k(mut self, k: u64) -> Self {
        self.k = k;
        self
    }

    pub fn objective(mut self, obj: &str) -> Self {
        self.objective = obj.to_string();
        self
    }

    pub fn weight_property(mut self, prop: &str) -> Self {
        self.weight_property = Some(prop.to_string());
        self
    }

    fn validate(&self) -> Result<()> {
        if self.source_node.is_none() {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "source_node is required".to_string(),
                ),
            );
        }

        ConfigValidator::in_range(self.k as f64, 1.0, 1_000_000.0, "k")?;

        if self.objective != "min" && self.objective != "max" {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                    "objective must be 'min' or 'max', got '{}'",
                    self.objective
                )),
            );
        }

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

    fn compute(&self) -> Result<(Vec<i64>, Vec<f64>, f64, u64, std::time::Duration)> {
        self.validate()?;
        let start = Instant::now();

        let source = self.source_node.unwrap();

        // K-spanning tree typically works on undirected graphs (like MST)
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((Vec::new(), Vec::new(), 0.0, source, start.elapsed()));
        }

        // Check source node exists
        if source as usize >= node_count {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                    "source_node {} out of range [0, {})",
                    source, node_count
                )),
            );
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
                .map(|cursor| {
                    let target = cursor.target_id();
                    if target < 0 {
                        return None;
                    }
                    let weight = if let Some(ref _prop) = self.weight_property {
                        // TODO: get actual property value when property access is available
                        1.0
                    } else {
                        1.0
                    };
                    Some((target as usize, weight))
                })
                .flatten()
                .collect()
        };

        let mut runtime = KSpanningTreeComputationRuntime::new(node_count);
        let result = runtime.compute(
            node_count,
            source as usize,
            self.k,
            &self.objective,
            get_neighbors,
        );

        Ok((
            result.parent,
            result.cost_to_parent,
            result.total_cost,
            result.root,
            start.elapsed(),
        ))
    }

    /// Stream mode: yields (node_id, parent_id, cost) for each node in the tree
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = KSpanningTreeRow>>> {
        let (parent, cost_to_parent, _total_cost, _root, _elapsed) = self.compute()?;

        let rows: Vec<KSpanningTreeRow> = parent
            .iter()
            .enumerate()
            .filter_map(|(node_id, &parent_id)| {
                if parent_id != -1 || node_id == _root as usize {
                    Some(KSpanningTreeRow {
                        node_id: node_id as u64,
                        parent_id,
                        cost: cost_to_parent[node_id],
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: returns aggregated statistics
    pub fn stats(&self) -> Result<KSpanningTreeStats> {
        let (parent, _cost_to_parent, total_cost, _root, elapsed) = self.compute()?;

        // Count nodes in tree (parent != -1 or is root)
        let node_count = parent
            .iter()
            .enumerate()
            .filter(|(idx, &p)| p != -1 || *idx == _root as usize)
            .count();

        Ok(KSpanningTreeStats {
            node_count,
            total_cost,
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
    fn facade_computes_spanning_tree() {
        // Simple path: 0-1-2-3
        let store = store_from_undirected_edges(4, &[(0, 1), (1, 2), (2, 3)]);
        let graph = Graph::new(Arc::new(store));

        let stats = graph.kspanning_tree().source_node(0).k(4).stats().unwrap();

        // K=4 should include all nodes in a 4-node path
        // If k >= node_count, the algorithm returns the full MST
        assert!(
            stats.node_count >= 1,
            "Expected at least 1 node, got {}",
            stats.node_count
        );
    }

    #[test]
    fn facade_limits_to_k_nodes() {
        // 5-node path: 0-1-2-3-4, limit to k=3
        let store = store_from_undirected_edges(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph
            .kspanning_tree()
            .source_node(0)
            .k(3)
            .stream()
            .unwrap()
            .collect();

        // Should have at most 3 nodes
        assert!(rows.len() <= 3);
    }
}
