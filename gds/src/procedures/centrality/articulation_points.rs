//! Articulation Points Facade
//!
//! Articulation points (cut vertices) are nodes whose removal increases the
//! number of connected components in an undirected graph.
//!
//! This facade is the "live wiring" layer: it binds the algorithm runtime to a
//! `DefaultGraphStore` graph view.

use crate::algo::articulation_points::computation::{
    ArticulationPointsComputationRuntime, STACK_EVENT_SIZE_BYTES,
};
use crate::algo::articulation_points::storage::ArticulationPointsStorageRuntime;
use crate::core::utils::progress::ProgressTracker;
use crate::core::utils::progress::{EmptyTaskRegistryFactory, TaskRegistryFactory, Tasks};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, WriteResult};
use crate::procedures::traits::{AlgorithmRunner, Result};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Result row for articulation points stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub struct ArticulationPointRow {
    pub node_id: u64,
}

/// Statistics for articulation points computation.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ArticulationPointsStats {
    pub articulation_point_count: u64,
    pub execution_time_ms: u64,
}

/// Articulation points facade bound to a live graph store.
#[derive(Clone)]
pub struct ArticulationPointsFacade {
    graph_store: Arc<DefaultGraphStore>,
    concurrency: usize,
    task_registry: Arc<dyn TaskRegistryFactory>,
}

impl ArticulationPointsFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            concurrency: 4,
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        }
    }

    /// Set concurrency level for parallel computation.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Set the task registry factory for progress tracking and concurrency control.
    pub fn task_registry(mut self, task_registry: Arc<dyn TaskRegistryFactory>) -> Self {
        self.task_registry = task_registry;
        self
    }

    /// Validate the facade configuration.
    ///
    /// # Returns
    /// Ok(()) if configuration is valid, Err otherwise
    ///
    /// # Errors
    /// Returns an error if concurrency is not positive
    pub fn validate(&self) -> Result<()> {
        if self.concurrency == 0 {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "concurrency must be positive".to_string(),
                ),
            );
        }
        Ok(())
    }

    /// Run the algorithm and return the articulation points as a bitset
    pub fn run(&self) -> Result<crate::collections::BitSet> {
        // Articulation points are defined on undirected connectivity.
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let node_count = graph_view.node_count();
        let _relationship_count = graph_view.relationship_count();
        if node_count == 0 {
            return Ok(crate::collections::BitSet::new(0));
        }

        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_registry(
            Tasks::leaf_with_volume("articulation_points".to_string(), node_count)
                .base()
                .clone(),
            crate::concurrency::Concurrency::of(self.concurrency.max(1)),
            crate::core::utils::progress::JobId::new(),
            self.task_registry.as_ref(),
        );
        progress_tracker.begin_subtask_with_volume(node_count);

        // Create both runtimes (factory pattern)
        let storage = ArticulationPointsStorageRuntime::new(&*self.graph_store)?;
        let mut computation = ArticulationPointsComputationRuntime::new(node_count);

        // Call storage.compute_articulation_points - Applications talk only to procedures
        let result = storage.compute_articulation_points(
            &mut computation,
            Some(graph_view.as_ref()),
            &mut progress_tracker,
        )?;

        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        Ok(result.articulation_points)
    }

    /// Estimate memory requirements for articulation points computation.
    ///
    /// # Returns
    /// Memory range estimate (min/max bytes)
    ///
    /// # Example
    /// ```ignore
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::ArticulationPointsFacade;
    /// let facade = ArticulationPointsFacade::new(graph);
    /// let memory = facade.estimate_memory();
    /// println!("Will use between {} and {} bytes", memory.min(), memory.max());
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();
        let relationship_count = self.graph_store.relationship_count();

        let bitset_bytes = (node_count + 7) / 8;
        let visited_bytes = bitset_bytes;
        let articulation_bytes = bitset_bytes;

        // tin/low/children: i64 per node (HugeLongArray is long-backed).
        let per_node_arrays_bytes = node_count.saturating_mul(3).saturating_mul(8);

        // Java parity: DFS event stack sized by relationship count.
        let stack_bytes = relationship_count.saturating_mul(STACK_EVENT_SIZE_BYTES);

        let total_memory = visited_bytes
            .saturating_add(articulation_bytes)
            .saturating_add(per_node_arrays_bytes)
            .saturating_add(stack_bytes);

        // Conservative overhead for Vec/BitSet headers + allocator slack.
        let total_with_overhead = total_memory.saturating_add(total_memory / 5);

        MemoryRange::of_range(total_memory, total_with_overhead)
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
        let relationship_count = graph_view.relationship_count();
        if node_count == 0 {
            return Ok((crate::collections::BitSet::new(0), start.elapsed()));
        }

        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_registry(
            Tasks::leaf_with_volume("articulation_points".to_string(), node_count)
                .base()
                .clone(),
            crate::concurrency::Concurrency::of(self.concurrency.max(1)),
            crate::core::utils::progress::JobId::new(),
            self.task_registry.as_ref(),
        );
        progress_tracker.begin_subtask_with_volume(node_count);

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
        let result =
            runtime.compute_with_relationship_count(node_count, relationship_count, get_neighbors);

        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        Ok((result.articulation_points, start.elapsed()))
    }

    /// Stream mode: Get articulation points for each node
    ///
    /// Returns an iterator over articulation point rows.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// let results = graph.articulation_points().stream()?.collect::<Vec<_>>();
    /// ```
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = ArticulationPointRow>>> {
        self.validate()?;
        let bitset = self.run()?;

        // Emit only set bits as rows.
        let mut out: Vec<ArticulationPointRow> = Vec::with_capacity(bitset.cardinality());
        let mut idx = bitset.next_set_bit(0);
        while let Some(i) = idx {
            out.push(ArticulationPointRow { node_id: i as u64 });
            idx = bitset.next_set_bit(i + 1);
        }

        Ok(Box::new(out.into_iter()))
    }

    /// Stats mode: Get aggregated statistics
    ///
    /// Returns articulation point count and execution time.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// let stats = graph.articulation_points().stats()?;
    /// println!("Found {} articulation points", stats.articulation_point_count);
    /// ```
    pub fn stats(&self) -> Result<ArticulationPointsStats> {
        self.validate()?;
        let (bitset, elapsed) = self.compute_bitset()?;

        Ok(ArticulationPointsStats {
            articulation_point_count: bitset.cardinality() as u64,
            execution_time_ms: elapsed.as_millis() as u64,
        })
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores articulation point status as a node property (1.0 for articulation points, 0.0 otherwise).
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// let result = graph.articulation_points().mutate("is_articulation_point")?;
    /// println!("Computed and stored for {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(
        self,
        property_name: &str,
    ) -> Result<crate::procedures::builder_base::MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "Articulation Points mutate/write is not implemented yet".to_string(),
            ),
        )
    }

    /// Write mode is not implemented yet for Articulation Points.
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "Articulation Points mutate/write is not implemented yet".to_string(),
            ),
        )
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
