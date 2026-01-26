//! All Shortest Paths Facade
//!
//! Computes shortest path distances for all (source, target) pairs.
//! Supports unweighted (BFS) and weighted (Dijkstra) variants.

use crate::algo::algorithms::pathfinding::PathResult as ProcedurePathResult;
use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::all_shortest_paths::{
    AlgorithmType, AllShortestPathsComputationRuntime, AllShortestPathsMutationSummary,
    AllShortestPathsStats, AllShortestPathsStorageRuntime,
};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// Import upgraded systems
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, TaskProgressTracker, TaskRegistryFactory, Tasks,
};

/// A single all-pairs shortest path distance row.
#[derive(Debug, Clone, serde::Serialize)]
pub struct AllShortestPathsRow {
    pub source: u64,
    pub target: u64,
    pub distance: f64,
}

/// Facade builder for all-shortest-paths.
///
/// Defaults:
/// - weighted: false
/// - relationship_types: all
/// - direction: "outgoing"
/// - weight_property: "weight"
/// - concurrency: 4
/// - max_results: None
pub struct AllShortestPathsBuilder {
    graph_store: Arc<DefaultGraphStore>,
    weighted: bool,
    relationship_types: Vec<String>,
    direction: String,
    weight_property: String,
    concurrency: usize,
    max_results: Option<usize>,
    /// Progress tracking components
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

/// Helper function to convert NodeId to u64
fn checked_u64(value: NodeId, context: &str) -> Result<u64> {
    u64::try_from(value).map_err(|_| {
        AlgorithmError::Execution(format!(
            "AllShortestPaths returned invalid node id for {context}: {value}",
        ))
    })
}

impl AllShortestPathsBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            weighted: false,
            relationship_types: vec![],
            direction: "outgoing".to_string(),
            weight_property: "weight".to_string(),
            concurrency: 4,
            max_results: None,
            task_registry_factory: None,
            user_log_registry_factory: None,
        }
    }

    /// Set whether to use weighted (Dijkstra) or unweighted (BFS) algorithm
    ///
    /// If true, uses Dijkstra's algorithm with edge weights.
    /// If false, uses BFS for unweighted shortest paths.
    /// Default: false
    pub fn weighted(mut self, weighted: bool) -> Self {
        self.weighted = weighted;
        self
    }

    /// Use unweighted BFS algorithm (convenience method)
    pub fn unweighted(self) -> Self {
        self.weighted(false)
    }

    /// Set weight property name
    ///
    /// Property must exist on relationships and contain numeric values.
    /// Only used when weighted=true.
    /// Default: "weight"
    pub fn weight_property(mut self, property: &str) -> Self {
        self.weight_property = property.to_string();
        self
    }

    /// Restrict traversal to the provided relationship types.
    ///
    /// Empty means all relationship types.
    pub fn relationship_types(mut self, relationship_types: Vec<String>) -> Self {
        self.relationship_types = relationship_types;
        self
    }

    /// Set traversal direction.
    ///
    /// Accepted values: "outgoing", "incoming", "undirected".
    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = direction.to_string();
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// All-pairs shortest paths benefits greatly from parallelism.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Set maximum number of results to return
    ///
    /// Limits the output size. None means no limit.
    pub fn max_results(mut self, max_results: usize) -> Self {
        self.max_results = Some(max_results);
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

        match self.direction.to_ascii_lowercase().as_str() {
            "outgoing" | "incoming" | "undirected" => {}
            other => {
                return Err(AlgorithmError::Execution(format!(
                    "direction must be 'outgoing', 'incoming', or 'undirected' (got '{other}')"
                )));
            }
        }

        ConfigValidator::non_empty_string(&self.weight_property, "weight_property")?;
        if let Some(max) = self.max_results {
            if max == 0 {
                return Err(AlgorithmError::Execution(
                    "max_results must be > 0".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn compute(self) -> Result<(Vec<AllShortestPathsRow>, AllShortestPathsStats)> {
        self.validate()?;

        // Set up progress tracking
        let _task_registry_factory = self
            .task_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));
        let _user_log_registry_factory = self
            .user_log_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));

        let algorithm_type = if self.weighted {
            AlgorithmType::Weighted
        } else {
            AlgorithmType::Unweighted
        };

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
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let storage = AllShortestPathsStorageRuntime::with_settings(
            graph_view.as_ref(),
            algorithm_type,
            self.concurrency,
        );

        let mut computation = AllShortestPathsComputationRuntime::new();

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("all_shortest_paths".to_string(), graph_view.node_count()),
            self.concurrency,
        );

        let start = std::time::Instant::now();
        let receiver = storage.compute_all_shortest_paths_streaming(
            &mut computation,
            direction_byte,
            &mut progress_tracker,
        )?;

        let node_count = graph_view.node_count() as u64;
        let mut rows: Vec<AllShortestPathsRow> = Vec::new();

        for result in receiver.into_iter() {
            if let Some(max) = self.max_results {
                if rows.len() >= max {
                    break;
                }
            }

            let source = checked_u64(result.source, "source")?;
            let target = checked_u64(result.target, "target")?;

            rows.push(AllShortestPathsRow {
                source,
                target,
                distance: result.distance,
            });
        }

        let stats = AllShortestPathsStats {
            node_count,
            result_count: rows.len() as u64,
            max_distance: computation.max_distance(),
            min_distance: computation.min_distance(),
            infinite_distances: computation.infinite_distances() as u64,
            execution_time_ms: start.elapsed().as_millis() as u64,
        };

        Ok((rows, stats))
    }

    /// Stream mode: yield distance rows.
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = AllShortestPathsRow>>> {
        let (rows, _) = self.compute()?;
        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: return aggregated statistics.
    pub fn stats(self) -> Result<AllShortestPathsStats> {
        let (_, stats) = self.compute()?;
        Ok(stats)
    }

    /// Mutate mode: Compute and update in-memory graph projection
    ///
    /// Stores all-pairs shortest path distances as node properties.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// let builder = graph.all_shortest_paths();
    /// let result = builder.mutate("distance")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(
        self,
        property_name: &str,
    ) -> Result<crate::algo::all_shortest_paths::AllShortestPathsMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let graph_store = Arc::clone(&self.graph_store);
        let (rows, stats) = self.compute()?;
        let paths: Vec<ProcedurePathResult> = rows
            .iter()
            .map(|row| ProcedurePathResult {
                source: row.source,
                target: row.target,
                path: vec![row.source, row.target],
                cost: row.distance,
            })
            .collect();

        let updated_store = crate::algo::algorithms::pathfinding::build_path_relationship_store(
            graph_store.as_ref(),
            property_name,
            &paths,
        )?;

        let summary = AllShortestPathsMutationSummary {
            nodes_updated: paths.len() as u64,
            property_name: property_name.to_string(),
            execution_time_ms: stats.execution_time_ms,
        };

        Ok(
            crate::algo::all_shortest_paths::AllShortestPathsMutateResult {
                summary,
                updated_store,
            },
        )
    }

    /// Write mode: Compute and persist to storage
    ///
    /// Persists all-pairs shortest path results to storage backend.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// let builder = graph.all_shortest_paths();
    /// let result = builder.write("distances")?;
    /// println!("Wrote {} nodes", result.nodes_written);
    /// ```
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

    /// Estimate memory requirements for all-shortest-paths execution
    ///
    /// Returns a memory range estimate based on:
    /// - Distance matrix storage (node_count² * 8 bytes)
    /// - Priority queues for Dijkstra (if weighted)
    /// - Graph structure overhead
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// let builder = graph.all_shortest_paths();
    /// let memory = builder.estimate_memory();
    /// println!("Estimated memory: {} bytes", memory.max());
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Distance matrix: node_count² * 8 bytes (f64 per pair)
        let distance_matrix_memory = node_count * node_count * 8;

        // Priority queues for weighted algorithm (worst case)
        let priority_queue_memory = if self.weighted {
            node_count * 32 // Similar to Dijkstra
        } else {
            0
        };

        // Graph structure overhead (adjacency lists, etc.)
        let avg_degree = 10.0; // Conservative estimate
        let relationship_count = (node_count as f64 * avg_degree) as usize;
        let graph_overhead = relationship_count * 16; // ~16 bytes per relationship

        let total_memory = distance_matrix_memory + priority_queue_memory + graph_overhead;

        // Add 20% overhead for algorithm-specific structures
        let overhead = total_memory / 5;
        let total_with_overhead = total_memory + overhead;

        MemoryRange::of_range(total_memory, total_with_overhead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::procedures::GraphFacade;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(7),
            node_count: 12,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_builder_defaults() {
        let builder = AllShortestPathsBuilder::new(store());
        assert!(!builder.weighted);
        assert!(builder.relationship_types.is_empty());
        assert_eq!(builder.direction, "outgoing");
        assert_eq!(builder.weight_property, "weight");
        assert_eq!(builder.concurrency, 4);
        assert!(builder.max_results.is_none());
    }

    #[test]
    fn test_stream_smoke() {
        let store = store();
        let rows: Vec<_> = GraphFacade::new(store)
            .all_shortest_paths()
            .weighted(false)
            .max_results(50)
            .stream()
            .unwrap()
            .collect();

        assert!(!rows.is_empty());
    }

    #[test]
    fn test_stats_smoke() {
        let store = store();
        let stats = GraphFacade::new(store)
            .all_shortest_paths()
            .weighted(true)
            .max_results(50)
            .stats()
            .unwrap();

        assert!(stats.node_count > 0);
        assert!(stats.result_count > 0);
    }
}
