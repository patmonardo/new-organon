//! Spanning Tree (Prim) Facade
//!
//! Computes a minimum or maximum spanning tree rooted at a start node.

use crate::procedures::facades::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::facades::traits::Result;
use crate::procedures::spanning_tree::SpanningTreeStorageRuntime;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Per-node spanning tree row.
#[derive(Debug, Clone)]
pub struct SpanningTreeRow {
    pub node: u64,
    pub parent: Option<u64>,
    pub cost_to_parent: f64,
}

/// Aggregated stats for spanning tree.
#[derive(Debug, Clone)]
pub struct SpanningTreeStats {
    pub effective_node_count: u64,
    pub total_weight: f64,
    pub computation_time_ms: u64,
}

/// Spanning tree facade builder.
///
/// Defaults:
/// - start_node: 0
/// - compute_minimum: true
/// - relationship_types: all
/// - direction: "undirected" (MST semantics)
/// - weight_property: "weight"
/// - concurrency: 1
pub struct SpanningTreeBuilder {
    graph_store: Arc<DefaultGraphStore>,
    start_node_id: u64,
    compute_minimum: bool,
    relationship_types: Vec<String>,
    direction: String,
    weight_property: String,
    concurrency: usize,
}

impl SpanningTreeBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            start_node_id: 0,
            compute_minimum: true,
            relationship_types: vec![],
            direction: "undirected".to_string(),
            weight_property: "weight".to_string(),
            concurrency: 1,
        }
    }

    fn validate(&self) -> Result<()> {
        if self.concurrency == 0 {
            return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                "concurrency must be > 0".to_string(),
            ));
        }
        ConfigValidator::non_empty_string(&self.direction, "direction")?;
        ConfigValidator::non_empty_string(&self.weight_property, "weight_property")?;
        Ok(())
    }

    fn compute(self) -> Result<(Vec<SpanningTreeRow>, SpanningTreeStats)> {
        self.validate()?;

        let start_node_id: u32 = u32::try_from(self.start_node_id).map_err(|_| {
            crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                "start_node_id must fit into u32 (got {})",
                self.start_node_id
            ))
        })?;

        let rel_types: HashSet<RelationshipType> = if self.relationship_types.is_empty() {
            self.graph_store.relationship_types()
        } else {
            RelationshipType::list_of(self.relationship_types.clone())
                .into_iter()
                .collect()
        };

        let (orientation, direction_byte) = match self.direction.to_ascii_lowercase().as_str() {
            "incoming" => (Orientation::Reverse, 1u8),
            "undirected" => (Orientation::Natural, 2u8),
            _ => (Orientation::Natural, 0u8),
        };

        let selectors: HashMap<RelationshipType, String> = rel_types
            .iter()
            .map(|t| (t.clone(), self.weight_property.clone()))
            .collect();

        let graph_view = self
            .graph_store
            .get_graph_with_types_selectors_and_orientation(&rel_types, &selectors, orientation)
            .map_err(|e| crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string()))?;

        let storage = SpanningTreeStorageRuntime::new(start_node_id, self.compute_minimum, self.concurrency);

        let start = std::time::Instant::now();
        let tree = storage.compute_spanning_tree_with_graph(graph_view.as_ref(), direction_byte)?;

        let mut rows = Vec::with_capacity(tree.node_count as usize);
        for node_id in 0..tree.node_count {
            let parent = tree.parent(node_id);
            let parent_u64 = if parent < 0 { None } else { Some(parent as u64) };
            rows.push(SpanningTreeRow {
                node: node_id as u64,
                parent: parent_u64,
                cost_to_parent: tree.cost_to_parent(node_id),
            });
        }

        let stats = SpanningTreeStats {
            effective_node_count: tree.effective_node_count() as u64,
            total_weight: tree.total_weight(),
            computation_time_ms: start.elapsed().as_millis() as u64,
        };

        Ok((rows, stats))
    }

    /// Set the root/start node.
    pub fn start_node(mut self, start_node_id: u64) -> Self {
        self.start_node_id = start_node_id;
        self
    }

    /// Compute a minimum spanning tree.
    pub fn minimum(mut self) -> Self {
        self.compute_minimum = true;
        self
    }

    /// Compute a maximum spanning tree.
    pub fn maximum(mut self) -> Self {
        self.compute_minimum = false;
        self
    }

    /// Set relationship types to consider. Empty means all types.
    pub fn relationship_types(mut self, types: Vec<&str>) -> Self {
        self.relationship_types = types.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// Set traversal direction: "outgoing", "incoming", or "undirected".
    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = direction.to_string();
        self
    }

    /// Set relationship weight property.
    pub fn weight_property(mut self, property: &str) -> Self {
        self.weight_property = property.to_string();
        self
    }

    /// Set concurrency.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Stream mode: yield per-node rows.
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = SpanningTreeRow>>> {
        let (rows, _) = self.compute()?;
        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: aggregated tree stats.
    pub fn stats(self) -> Result<SpanningTreeStats> {
        let (_, stats) = self.compute()?;
        Ok(stats)
    }

    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(crate::projection::eval::procedure::AlgorithmError::Execution(
            "SpanningTree mutate/write is not implemented yet".to_string(),
        ))
    }

    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(crate::projection::eval::procedure::AlgorithmError::Execution(
            "SpanningTree mutate/write is not implemented yet".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(11),
            node_count: 12,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_builder_defaults() {
        let builder = SpanningTreeBuilder::new(store());
        assert_eq!(builder.start_node_id, 0);
        assert!(builder.compute_minimum);
        assert!(builder.relationship_types.is_empty());
        assert_eq!(builder.direction, "undirected");
        assert_eq!(builder.weight_property, "weight");
        assert_eq!(builder.concurrency, 1);
    }

    #[test]
    fn test_stream_smoke() {
        let store = store();
        let rows: Vec<_> = crate::procedures::facades::graph::Graph::new(store)
            .spanning_tree()
            .start_node(0)
            .minimum()
            .stream()
            .unwrap()
            .collect();

        assert!(!rows.is_empty());
    }

    #[test]
    fn test_stats_smoke() {
        let store = store();
        let stats = crate::procedures::facades::graph::Graph::new(store)
            .spanning_tree()
            .start_node(0)
            .minimum()
            .stats()
            .unwrap();

        assert!(stats.effective_node_count > 0);
    }
}
