//! Topological Sort Facade
//!
//! Orders nodes in a directed acyclic graph (DAG) such that for every edge (u, v),
//! u appears before v. Optionally computes longest path distances.

use crate::algo::topological_sort::computation::TopologicalSortComputationRuntime;
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

// Import upgraded systems
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, ProgressTracker, TaskRegistryFactory, Tasks,
};

/// Result row for topological sort stream mode
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct TopologicalSortRow {
    pub node_id: NodeId,
    pub max_distance: Option<f64>,
}

/// Statistics for topological sort computation
#[derive(Debug, Clone, serde::Serialize)]
pub struct TopologicalSortStats {
    pub node_count: usize,
    pub execution_time_ms: u64,
}

/// Topological Sort algorithm builder
pub struct TopologicalSortBuilder {
    graph_store: Arc<DefaultGraphStore>,
    compute_max_distance: bool,
    concurrency: usize,
    /// Progress tracking components
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

impl TopologicalSortBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            compute_max_distance: false,
            concurrency: 4,
            task_registry_factory: None,
            user_log_registry_factory: None,
        }
    }

    pub fn compute_max_distance(mut self, value: bool) -> Self {
        self.compute_max_distance = value;
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// Topological sort benefits from parallelism in large graphs.
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

    fn compute(self) -> Result<(Vec<NodeId>, Option<Vec<f64>>, std::time::Duration)> {
        self.validate()?;

        // Set up progress tracking
        let _task_registry_factory = self
            .task_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));
        let _user_log_registry_factory = self
            .user_log_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));

        let start = Instant::now();

        // Topological sort works on directed graphs (Natural orientation)
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((Vec::new(), None, start.elapsed()));
        }

        let mut progress_tracker =
            crate::core::utils::progress::TaskProgressTracker::with_concurrency(
                Tasks::leaf_with_volume("topological_sort".to_string(), node_count),
                self.concurrency,
            );
        progress_tracker.begin_subtask_with_volume(node_count);

        let fallback = graph_view.default_property_value();

        // Get neighbors with weights
        let get_neighbors = |node_idx: NodeId| -> Vec<(NodeId, f64)> {
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

        let mut runtime =
            TopologicalSortComputationRuntime::new(node_count, self.compute_max_distance);
        let result = runtime.compute(node_count, get_neighbors);

        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        Ok((
            result.sorted_nodes,
            result.max_source_distances,
            start.elapsed(),
        ))
    }

    /// Stream mode: yields (node_id, max_distance) for each node in topological order
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = TopologicalSortRow>>> {
        let (sorted_nodes, max_distances, _elapsed) = self.compute()?;

        let rows: Vec<TopologicalSortRow> = sorted_nodes
            .into_iter()
            .map(|node_id| TopologicalSortRow {
                node_id,
                max_distance: max_distances
                    .as_ref()
                    .and_then(|d| d.get(node_id as usize).copied()),
            })
            .collect();

        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: returns aggregated statistics
    pub fn stats(self) -> Result<TopologicalSortStats> {
        let (sorted_nodes, _max_distances, elapsed) = self.compute()?;

        Ok(TopologicalSortStats {
            node_count: sorted_nodes.len(),
            execution_time_ms: elapsed.as_millis() as u64,
        })
    }

    /// Mutate mode: Compute and update in-memory graph projection
    ///
    /// Stores topological order and distances as node properties.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// let builder = graph.topological_sort().compute_max_distance(true);
    /// let result = builder.mutate("topological_order")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "TopologicalSort mutate/write is not implemented yet".to_string(),
            ),
        )
    }

    /// Write mode: Compute and persist to storage
    ///
    /// Persists topological sort results to storage backend.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// let builder = graph.topological_sort().compute_max_distance(true);
    /// let result = builder.write("topological_sort")?;
    /// println!("Wrote {} nodes", result.nodes_written);
    /// ```
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "TopologicalSort mutate/write is not implemented yet".to_string(),
            ),
        )
    }

    /// Estimate memory requirements for topological sort execution
    ///
    /// Returns a memory range estimate based on:
    /// - Node ordering arrays
    /// - Distance arrays (if computing max distances)
    /// - Graph structure overhead
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// let builder = graph.topological_sort();
    /// let memory = builder.estimate_memory();
    /// println!("Estimated memory: {} bytes", memory.max());
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Node ordering array: node_count * 8 bytes (u64 per node)
        let ordering_memory = node_count * 8;

        // Distance array: node_count * 8 bytes (f64 per node, if computing distances)
        let distance_memory = if self.compute_max_distance {
            node_count * 8
        } else {
            0
        };

        // Graph structure overhead (adjacency lists, etc.)
        let avg_degree = 10.0; // Conservative estimate
        let relationship_count = (node_count as f64 * avg_degree) as usize;
        let graph_overhead = relationship_count * 16; // ~16 bytes per relationship

        let total_memory = ordering_memory + distance_memory + graph_overhead;

        // Add 20% overhead for algorithm-specific structures
        let overhead = total_memory / 5;
        let total_with_overhead = total_memory + overhead;

        MemoryRange::of_range(total_memory, total_with_overhead)
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
    fn facade_computes_topological_order() {
        // Simple DAG: 0 -> 1 -> 2
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph.topological_sort().stream().unwrap().collect();

        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].node_id, 0);
        assert_eq!(rows[1].node_id, 1);
        assert_eq!(rows[2].node_id, 2);
    }

    #[test]
    fn facade_computes_stats() {
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2)]);
        let graph = Graph::new(Arc::new(store));

        let stats = graph.topological_sort().stats().unwrap();

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
