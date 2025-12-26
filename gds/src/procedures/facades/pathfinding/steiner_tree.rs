//! Steiner Tree Facade
//!
//! Computes minimum Steiner trees connecting source nodes to terminal nodes.
//! Uses approximation algorithms with delta-stepping and rerouting optimizations.

use crate::procedures::facades::builder_base::ConfigValidator;
use crate::procedures::facades::traits::Result;
use crate::procedures::steiner_tree::computation::SteinerTreeComputationRuntime;
use crate::procedures::steiner_tree::SteinerTreeConfig;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;

/// Result row for Steiner tree stream mode
#[derive(Debug, Clone, serde::Serialize)]
pub struct SteinerTreeRow {
    pub node: u64,
    pub parent: Option<u64>,
    pub cost_to_parent: f64,
}

/// Statistics for Steiner tree computation
#[derive(Debug, Clone, serde::Serialize)]
pub struct SteinerTreeStats {
    pub effective_node_count: u64,
    pub effective_target_nodes_count: u64,
    pub total_cost: f64,
    pub computation_time_ms: u64,
}

/// Steiner Tree algorithm builder
#[derive(Clone)]
pub struct SteinerTreeBuilder {
    graph_store: Arc<DefaultGraphStore>,
    source_node: u64,
    target_nodes: Vec<u64>,
    relationship_weight_property: Option<String>,
    delta: f64,
    apply_rerouting: bool,
}

impl SteinerTreeBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            source_node: 0,
            target_nodes: Vec::new(),
            relationship_weight_property: None,
            delta: 1.0,
            apply_rerouting: true,
        }
    }

    pub fn source_node(mut self, source: u64) -> Self {
        self.source_node = source;
        self
    }

    pub fn target_nodes(mut self, targets: Vec<u64>) -> Self {
        self.target_nodes = targets;
        self
    }

    pub fn relationship_weight_property(mut self, property: &str) -> Self {
        self.relationship_weight_property = Some(property.to_string());
        self
    }

    pub fn delta(mut self, delta: f64) -> Self {
        self.delta = delta;
        self
    }

    pub fn apply_rerouting(mut self, apply: bool) -> Self {
        self.apply_rerouting = apply;
        self
    }

    fn validate(&self) -> Result<()> {
        if self.target_nodes.is_empty() {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "target_nodes must not be empty".to_string(),
                ),
            );
        }

        ConfigValidator::in_range(self.delta, 0.0, 100.0, "delta")?;

        Ok(())
    }

    fn compute(&self) -> Result<(Vec<SteinerTreeRow>, SteinerTreeStats)> {
        self.validate()?;
        let start = std::time::Instant::now();

        // Steiner tree works on undirected graphs
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((Vec::new(), SteinerTreeStats {
                effective_node_count: 0,
                effective_target_nodes_count: 0,
                total_cost: 0.0,
                computation_time_ms: start.elapsed().as_millis() as u64,
            }));
        }

        let fallback = graph_view.default_property_value();

        // Get neighbors with weights
        let get_neighbors = |node_idx: usize| -> Vec<(usize, f64)> {
            graph_view
                .stream_relationships(node_idx as i64, fallback)
                .filter_map(|cursor| {
                    let target = cursor.target_id();
                    if target >= 0 {
                        let weight = 1.0; // TODO: get actual weight property when available
                        Some((target as usize, weight))
                    } else {
                        None
                    }
                })
                .collect()
        };

        let config = SteinerTreeConfig {
            source_node: self.source_node,
            target_nodes: self.target_nodes.clone(),
            relationship_weight_property: self.relationship_weight_property.clone(),
            delta: self.delta,
            apply_rerouting: self.apply_rerouting,
        };

        let runtime = SteinerTreeComputationRuntime::new(config);
        let result = runtime.compute(node_count, get_neighbors);

        let mut rows = Vec::new();
        for (node_idx, &parent) in result.parent_array.iter().enumerate() {
            if parent >= 0 {
                rows.push(SteinerTreeRow {
                    node: node_idx as u64,
                    parent: Some(parent as u64),
                    cost_to_parent: result.relationship_to_parent_cost[node_idx],
                });
            } else if parent == -1 {
                // Root node
                rows.push(SteinerTreeRow {
                    node: node_idx as u64,
                    parent: None,
                    cost_to_parent: 0.0,
                });
            }
            // Skip pruned nodes (parent == -2)
        }

        let stats = SteinerTreeStats {
            effective_node_count: result.effective_node_count,
            effective_target_nodes_count: result.effective_target_nodes_count,
            total_cost: result.total_cost,
            computation_time_ms: start.elapsed().as_millis() as u64,
        };

        Ok((rows, stats))
    }

    /// Stream mode: yields tree edges
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = SteinerTreeRow>>> {
        let (rows, _) = self.compute()?;
        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: aggregated tree stats
    pub fn stats(&self) -> Result<SteinerTreeStats> {
        let (_, stats) = self.compute()?;
        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(13),
            node_count: 10,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_builder_defaults() {
        let builder = SteinerTreeBuilder::new(store());
        assert_eq!(builder.source_node, 0);
        assert!(builder.target_nodes.is_empty());
        assert!(builder.relationship_weight_property.is_none());
        assert_eq!(builder.delta, 1.0);
        assert!(builder.apply_rerouting);
    }

    #[test]
    fn test_stream_smoke() {
        let store = store();
        let rows: Vec<_> = crate::procedures::facades::graph::Graph::new(store)
            .steiner_tree()
            .source_node(0)
            .target_nodes(vec![5, 7])
            .stream()
            .unwrap()
            .collect();

        assert!(!rows.is_empty());
    }

    #[test]
    fn test_stats_smoke() {
        let store = store();
        let stats = crate::procedures::facades::graph::Graph::new(store)
            .steiner_tree()
            .source_node(0)
            .target_nodes(vec![5, 7])
            .stats()
            .unwrap();

        assert!(stats.effective_target_nodes_count > 0);
    }
}
