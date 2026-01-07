//! DAG Longest Path Facade
//!
//! Finds longest paths in a directed acyclic graph using topological ordering
//! and dynamic programming.

use crate::algo::dag_longest_path::computation::DagLongestPathComputationRuntime;
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

// Import upgraded systems
use crate::core::utils::progress::{ProgressTracker, TaskRegistryFactory, Tasks};

/// Result row for longest path stream mode
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct DagLongestPathRow {
    pub index: u64,
    pub source_node: NodeId,
    pub target_node: NodeId,
    pub total_cost: f64,
    pub node_ids: Vec<NodeId>,
    pub costs: Vec<f64>,
}

/// Statistics for dag longest path computation
#[derive(Debug, Clone, serde::Serialize)]
pub struct DagLongestPathStats {
    pub path_count: usize,
    pub execution_time_ms: u64,
}

/// DAG Longest Path algorithm builder
pub struct DagLongestPathBuilder {
    graph_store: Arc<DefaultGraphStore>,
    concurrency: usize,
    /// Progress tracking components
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

impl DagLongestPathBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            concurrency: 4,
            task_registry_factory: None,
            user_log_registry_factory: None,
        }
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// DAG longest path benefits from parallelism in large graphs.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Set task registry factory for progress tracking
    pub fn task_registry_factory(mut self, factory: Box<dyn TaskRegistryFactory>) -> Self {
        self.task_registry_factory = Some(factory);
        self
    }

    /// Set user log registry factory for progress tracking
    pub fn user_log_registry_factory(mut self, factory: Box<dyn TaskRegistryFactory>) -> Self {
        self.user_log_registry_factory = Some(factory);
        self
    }

    fn validate(&self) -> Result<()> {
        if self.concurrency == 0 {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "concurrency must be > 0".to_string(),
                ),
            );
        }

        Ok(())
    }

    fn compute(self) -> Result<(Vec<DagLongestPathRow>, std::time::Duration)> {
        self.validate()?;

        let start = Instant::now();

        // Longest path works on directed graphs (Natural orientation)
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

        let mut progress_tracker =
            crate::core::utils::progress::TaskProgressTracker::with_concurrency(
                Tasks::leaf_with_volume("dag_longest_path".to_string(), node_count),
                self.concurrency,
            );
        progress_tracker.begin_subtask_with_volume(node_count);

        let fallback = graph_view.default_property_value();

        // Get neighbors with weights
        let get_neighbors = move |node_idx: NodeId| -> Vec<(NodeId, f64)> {
            let node_id = match NodeId::try_from(node_idx) {
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
                    Some((target, weight))
                })
                .collect()
        };

        let mut runtime = DagLongestPathComputationRuntime::new(node_count);
        let result = runtime.compute(node_count, get_neighbors);

        let rows = result
            .paths
            .into_iter()
            .map(|path| DagLongestPathRow {
                index: path.index,
                source_node: path.source_node,
                target_node: path.target_node,
                total_cost: path.total_cost,
                node_ids: path.node_ids,
                costs: path.costs,
            })
            .collect();

        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        Ok((rows, start.elapsed()))
    }

    /// Stream mode: yields path rows with source, target, costs, and node sequences
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = DagLongestPathRow>>> {
        let (rows, _elapsed) = self.compute()?;
        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: returns aggregated statistics
    pub fn stats(self) -> Result<DagLongestPathStats> {
        let (rows, elapsed) = self.compute()?;

        Ok(DagLongestPathStats {
            path_count: rows.len(),
            execution_time_ms: elapsed.as_millis() as u64,
        })
    }

    /// Mutate mode: writes results back to the graph store
    pub fn mutate(self) -> Result<MutationResult> {
        // Note: mutation logic is deferred.
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "mutate mode not yet implemented".to_string(),
            ),
        )
    }

    /// Write mode: writes results to external storage
    pub fn write(self) -> Result<WriteResult> {
        // Note: write logic is deferred.
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "write mode not yet implemented".to_string(),
            ),
        )
    }

    /// Estimate memory usage for the computation
    pub fn estimate_memory(&self) -> Result<MemoryRange> {
        // Estimate based on node count and expected path storage
        let node_count = self.graph_store.node_count();
        let estimated_bytes = node_count * std::mem::size_of::<f64>() * 2; // distances and predecessors
        Ok(MemoryRange::of_range(
            estimated_bytes / 2,
            estimated_bytes * 2,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::procedures::Graph;
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
    fn facade_computes_longest_paths() {
        // Simple DAG: 0 -> 1 -> 2
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph.dag_longest_path().stream().unwrap().collect();

        assert!(!rows.is_empty());

        // Find path to node 2
        let path_to_2 = rows.iter().find(|r| r.target_node == 2).unwrap();
        assert_eq!(path_to_2.source_node, 0);
        assert_eq!(path_to_2.node_ids, vec![0, 1, 2]);
    }

    #[test]
    fn facade_computes_stats() {
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2)]);
        let graph = Graph::new(Arc::new(store));

        let stats = graph.dag_longest_path().stats().unwrap();

        assert!(stats.path_count > 0);
        assert!(stats.execution_time_ms < 1000);
    }

    #[test]
    fn facade_finds_longest_path_in_diamond() {
        // Diamond: 0 -> 1 -> 3
        //           \-> 2 ->/
        let store = store_from_directed_edges(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph.dag_longest_path().stream().unwrap().collect();

        // Find path to node 3
        let path_to_3 = rows.iter().find(|r| r.target_node == 3).unwrap();

        // Should have a path through 2 hops
        assert_eq!(path_to_3.node_ids.len(), 3);
        assert_eq!(path_to_3.node_ids[0], 0);
        assert_eq!(path_to_3.node_ids[2], 3);
    }
}
