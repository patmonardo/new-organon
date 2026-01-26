//! DAG Longest Path Facade
//!
//! Finds longest paths in a directed acyclic graph using topological ordering
//! and dynamic programming.

use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::dag_longest_path::{
    DagLongestPathComputationRuntime, DagLongestPathMutateResult, DagLongestPathResult,
    DagLongestPathResultBuilder, DagLongestPathRow, DagLongestPathStats,
};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

// Import upgraded systems
use crate::algo::algorithms::pathfinding::PathResult;
use crate::core::utils::progress::{
    ProgressTracker, TaskProgressTracker, TaskRegistryFactory, Tasks,
};

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
            return Err(AlgorithmError::Execution(
                "concurrency must be > 0".to_string(),
            ));
        }

        Ok(())
    }

    fn compute(self) -> Result<(DagLongestPathResult, std::time::Duration)> {
        self.validate()?;

        let start = Instant::now();

        // Longest path works on directed graphs (Natural orientation)
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((DagLongestPathResult { paths: Vec::new() }, start.elapsed()));
        }

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
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

        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        Ok((result, start.elapsed()))
    }

    /// Stream mode: yields path rows with source, target, costs, and node sequences
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = DagLongestPathRow>>> {
        let (result, elapsed) = self.compute()?;
        let rows = DagLongestPathResultBuilder::new(result, elapsed).rows();
        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: returns aggregated statistics
    pub fn stats(self) -> Result<DagLongestPathStats> {
        let (result, elapsed) = self.compute()?;
        Ok(DagLongestPathResultBuilder::new(result, elapsed).stats())
    }

    /// Mutate mode: writes results back to the graph store
    pub fn mutate(self, property_name: &str) -> Result<DagLongestPathMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let graph_store = Arc::clone(&self.graph_store);
        let (result, elapsed) = self.compute()?;
        let builder = DagLongestPathResultBuilder::new(result, elapsed);
        let paths: Vec<PathResult> = builder.paths();

        let updated_store = crate::algo::algorithms::pathfinding::build_path_relationship_store(
            graph_store.as_ref(),
            property_name,
            &paths,
        )?;

        let summary = builder.mutation_summary(property_name, paths.len() as u64);

        Ok(DagLongestPathMutateResult {
            summary,
            updated_store,
        })
    }

    /// Write mode: writes results to external storage
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let res = self.mutate(property_name)?;
        Ok(WriteResult::new(
            res.summary.nodes_updated,
            property_name.to_string(),
            std::time::Duration::from_millis(res.summary.execution_time_ms),
        ))
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
    use crate::config::GraphStoreConfig;

    use crate::projection::RelationshipType;
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::schema::{Direction, MutableGraphSchema};
    use crate::RelationshipTopology;
    use crate::SimpleIdMap;
    use std::collections::HashMap;
    use std::sync::Arc;

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
    fn facade_computes_longest_paths() {
        // Simple DAG: 0 -> 1 -> 2
        let store = Arc::new(store_from_directed_edges(3, &[(0, 1), (1, 2)]));
        let builder = DagLongestPathBuilder::new(Arc::clone(&store));
        let rows: Vec<DagLongestPathRow> = builder.stream().unwrap().collect();

        assert!(!rows.is_empty());

        // Find path to node 2
        let path_to_2 = rows.iter().find(|r| r.target_node == 2).unwrap();
        assert_eq!(path_to_2.source_node, 0);
        assert_eq!(path_to_2.node_ids, vec![0, 1, 2]);
    }

    #[test]
    fn facade_computes_stats() {
        let store = Arc::new(store_from_directed_edges(3, &[(0, 1), (1, 2)]));
        let stats = DagLongestPathBuilder::new(Arc::clone(&store))
            .stats()
            .unwrap();

        assert!(stats.path_count > 0);
        assert!(stats.execution_time_ms < 1000);
    }

    #[test]
    fn facade_finds_longest_path_in_diamond() {
        // Diamond: 0 -> 1 -> 3
        //           \-> 2 ->/
        let store = Arc::new(store_from_directed_edges(
            4,
            &[(0, 1), (0, 2), (1, 3), (2, 3)],
        ));
        let builder = DagLongestPathBuilder::new(Arc::clone(&store));
        let rows: Vec<DagLongestPathRow> = builder.stream().unwrap().collect();

        // Find path to node 3
        let path_to_3 = rows.iter().find(|r| r.target_node == 3).unwrap();

        // Should have a path through 2 hops
        assert_eq!(path_to_3.node_ids.len(), 3);
        assert_eq!(path_to_3.node_ids[0], 0);
        assert_eq!(path_to_3.node_ids[2], 3);
    }
}
