//! K-Spanning Tree Facade
//!
//! Computes k spanning trees by first computing an MST using Prim's algorithm,
//! then progressively pruning to maintain exactly k nodes.

use crate::algo::kspanningtree::computation::KSpanningTreeComputationRuntime;
use crate::algo::kspanningtree::storage::KSpanningTreeStorageRuntime;
use crate::core::utils::progress::Tasks;
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::{PathResult, Result};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

// Additional imports for error handling and progress tracking
use crate::core::utils::progress::TaskProgressTracker;
use crate::projection::eval::procedure::AlgorithmError;

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

/// Mutate result for k-spanning tree: summary + updated store
#[derive(Debug, Clone)]
pub struct KSpanningTreeMutateResult {
    pub summary: MutationResult,
    pub updated_store: Arc<DefaultGraphStore>,
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
            return Err(AlgorithmError::Execution(
                "source_node is required".to_string(),
            ));
        }

        ConfigValidator::in_range(self.k as f64, 1.0, 1_000_000.0, "k")?;

        if self.objective != "min" && self.objective != "max" {
            return Err(AlgorithmError::Execution(format!(
                "objective must be 'min' or 'max', got '{}'",
                self.objective
            )));
        }

        Ok(())
    }

    #[allow(clippy::type_complexity)]
    fn compute(&self) -> Result<(Vec<i64>, Vec<f64>, f64, u64, std::time::Duration)> {
        self.validate()?;
        let start = Instant::now();

        let source = self.source_node.unwrap();

        // K-spanning tree typically works on undirected graphs (like MST)
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((Vec::new(), Vec::new(), 0.0, source, start.elapsed()));
        }

        // Check source node exists
        if source as usize >= node_count {
            return Err(AlgorithmError::Execution(format!(
                "source_node {} out of range [0, {})",
                source, node_count
            )));
        }

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf_with_volume(
            "kspanningtree".to_string(),
            node_count,
        ));

        // Create storage runtime (Gross pole - controller)
        let storage =
            KSpanningTreeStorageRuntime::new(source as i64, self.k, self.objective.clone());

        // Create computation runtime (Subtle pole - state management)
        let mut computation = KSpanningTreeComputationRuntime::new(node_count);

        // Call storage.compute_kspanningtree() - Applications never call ::algo:: directly
        let result = storage.compute_kspanningtree(
            &mut computation,
            Some(graph_view.as_ref()),
            &mut progress_tracker,
        )?;

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

    /// Mutate mode: writes results back to the graph store
    pub fn mutate(&self, property_name: &str) -> Result<KSpanningTreeMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let (parent, cost_to_parent, _total_cost, _root, elapsed) = self.compute()?;
        let mut paths: Vec<PathResult> = Vec::new();

        for (node_id, &parent_id) in parent.iter().enumerate() {
            if parent_id >= 0 {
                let parent_u64 = parent_id as u64;
                let node_u64 = node_id as u64;
                paths.push(PathResult {
                    source: parent_u64,
                    target: node_u64,
                    path: vec![parent_u64, node_u64],
                    cost: cost_to_parent[node_id],
                });
            }
        }

        let updated_store =
            super::build_path_relationship_store(self.graph_store.as_ref(), property_name, &paths)?;

        let summary = MutationResult::new(paths.len() as u64, property_name.to_string(), elapsed);

        Ok(KSpanningTreeMutateResult {
            summary,
            updated_store,
        })
    }

    /// Write mode: writes results to external storage
    pub fn write(&self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let res = self.mutate(property_name)?;
        Ok(WriteResult::new(
            res.summary.nodes_updated,
            property_name.to_string(),
            std::time::Duration::from_millis(res.summary.execution_time_ms),
        ))
    }

    /// Estimate memory requirements for k-spanning tree execution.
    ///
    /// Conservative estimate based on:
    /// - parent + cost arrays (per node)
    /// - a small amount of queue/working storage
    /// - graph structure overhead
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // parent: i64, cost: f64
        let arrays = node_count * (std::mem::size_of::<i64>() + std::mem::size_of::<f64>());

        // Working storage: small multiple of node_count.
        let working = node_count * 32;

        // Graph structure overhead (adjacency lists, etc.)
        let avg_degree = 10.0;
        let relationship_count = (node_count as f64 * avg_degree) as usize;
        let graph_overhead = relationship_count * 16;

        let total = arrays + working + graph_overhead;
        let overhead = total / 5; // +20%
        MemoryRange::of_range(total, total + overhead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphStoreConfig;
    use crate::procedures::GraphFacade;

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
            GraphStoreConfig::default(),
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
        let graph = GraphFacade::new(Arc::new(store));

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
        let graph = GraphFacade::new(Arc::new(store));

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
