//! All Shortest Paths Facade
//!
//! Computes shortest path distances for all (source, target) pairs.
//! Supports unweighted (BFS) and weighted (Dijkstra) variants.

use crate::procedures::all_shortest_paths::{AlgorithmType, AllShortestPathsStorageRuntime};
use crate::procedures::facades::builder_base::ConfigValidator;
use crate::procedures::facades::traits::Result;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// A single all-pairs shortest path distance row.
#[derive(Debug, Clone)]
pub struct AllShortestPathsRow {
    pub source: u64,
    pub target: u64,
    pub distance: f64,
}

/// Aggregated statistics for an all-shortest-paths run.
#[derive(Debug, Clone)]
pub struct AllShortestPathsStats {
    pub node_count: u64,
    pub result_count: u64,
    pub max_distance: f64,
    pub min_distance: f64,
    pub infinite_distances: u64,
    pub execution_time_ms: u64,
}

/// Facade builder for all-shortest-paths.
///
/// Defaults:
/// - algorithm_type: Unweighted
/// - relationship_types: all
/// - direction: "outgoing"
/// - weight_property: "weight"
/// - concurrency: num_cpus
/// - max_results: None
pub struct AllShortestPathsBuilder {
    graph_store: Arc<DefaultGraphStore>,
    algorithm_type: AlgorithmType,
    relationship_types: Vec<String>,
    direction: String,
    weight_property: String,
    concurrency: usize,
    max_results: Option<usize>,
}

impl AllShortestPathsBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            algorithm_type: AlgorithmType::Unweighted,
            relationship_types: vec![],
            direction: "outgoing".to_string(),
            weight_property: "weight".to_string(),
            concurrency: num_cpus::get(),
            max_results: None,
        }
    }

    fn checked_u64(value: NodeId, context: &str) -> Result<u64> {
        u64::try_from(value).map_err(|_| {
            crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                "AllShortestPaths returned invalid node id for {context}: {value}",
            ))
        })
    }

    fn validate(&self) -> Result<()> {
        if self.concurrency == 0 {
            return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                "concurrency must be > 0".to_string(),
            ));
        }
        ConfigValidator::non_empty_string(&self.direction, "direction")?;
        ConfigValidator::non_empty_string(&self.weight_property, "weight_property")?;
        if let Some(max) = self.max_results {
            if max == 0 {
                return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                    "max_results must be > 0".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn compute(self) -> Result<(Vec<AllShortestPathsRow>, AllShortestPathsStats)> {
        self.validate()?;

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

        let storage = AllShortestPathsStorageRuntime::with_settings(
            Arc::clone(&graph_view),
            self.algorithm_type,
            self.concurrency,
        );

        let start = std::time::Instant::now();
        let receiver = storage.compute_all_shortest_paths_streaming(direction_byte)?;

        let node_count = graph_view.node_count() as u64;
        let mut rows: Vec<AllShortestPathsRow> = Vec::new();

        let mut max_distance = 0.0;
        let mut min_distance = f64::INFINITY;
        let mut infinite_distances: u64 = 0;

        for result in receiver.into_iter() {
            if let Some(max) = self.max_results {
                if rows.len() >= max {
                    break;
                }
            }

            if result.distance.is_infinite() {
                infinite_distances += 1;
            } else {
                if result.distance > max_distance {
                    max_distance = result.distance;
                }
                if result.distance < min_distance {
                    min_distance = result.distance;
                }
            }

            let source = Self::checked_u64(result.source, "source")?;
            let target = Self::checked_u64(result.target, "target")?;

            rows.push(AllShortestPathsRow {
                source,
                target,
                distance: result.distance,
            });
        }

        if min_distance == f64::INFINITY {
            min_distance = 0.0;
        }

        let stats = AllShortestPathsStats {
            node_count,
            result_count: rows.len() as u64,
            max_distance,
            min_distance,
            infinite_distances,
            execution_time_ms: start.elapsed().as_millis() as u64,
        };

        Ok((rows, stats))
    }

    /// Select unweighted (BFS) shortest paths.
    pub fn unweighted(mut self) -> Self {
        self.algorithm_type = AlgorithmType::Unweighted;
        self
    }

    /// Select weighted (Dijkstra) shortest paths.
    pub fn weighted(mut self) -> Self {
        self.algorithm_type = AlgorithmType::Weighted;
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

    /// Set weight property (used for weighted runs).
    pub fn weight_property(mut self, property: &str) -> Self {
        self.weight_property = property.to_string();
        self
    }

    /// Set concurrency.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Cap the number of streamed results.
    pub fn max_results(mut self, max_results: Option<usize>) -> Self {
        self.max_results = max_results;
        self
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
}

#[cfg(test)]
mod tests {
    use super::*;
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
        assert_eq!(builder.algorithm_type, AlgorithmType::Unweighted);
        assert!(builder.relationship_types.is_empty());
        assert_eq!(builder.direction, "outgoing");
        assert_eq!(builder.weight_property, "weight");
        assert!(builder.concurrency > 0);
        assert!(builder.max_results.is_none());
    }

    #[test]
    fn test_stream_smoke() {
        let store = store();
        let rows: Vec<_> = crate::procedures::facades::graph::Graph::new(store)
            .all_shortest_paths()
            .unweighted()
            .max_results(Some(50))
            .stream()
            .unwrap()
            .collect();

        assert!(!rows.is_empty());
    }

    #[test]
    fn test_stats_smoke() {
        let store = store();
        let stats = crate::procedures::facades::graph::Graph::new(store)
            .all_shortest_paths()
            .weighted()
            .max_results(Some(50))
            .stats()
            .unwrap();

        assert!(stats.node_count > 0);
        assert!(stats.result_count > 0);
    }
}
