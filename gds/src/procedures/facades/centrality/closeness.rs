//! Closeness Centrality Facade
//!
//! **What is it?**: Measures how close a node is to all other nodes
//! **Why care?**: Finds nodes that can reach others quickly (good broadcasters)
//! **Complexity**: O(V*(V+E)) in the worst case (all-pairs BFS)
//!
//! This implementation follows the Neo4j GDS behavior:
//! - Uses MSBFS-style aggregation to compute farness and component size
//! - Centrality formula: `componentSize / farness`
//! - Optional Wasserman–Faust normalization

use crate::procedures::facades::builder_base::ConfigValidator;
use crate::procedures::facades::traits::{CentralityScore, Result};
use crate::procedures::msbfs::AggregatedNeighborProcessingMsBfs;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Statistics about closeness centrality.
#[derive(Debug, Clone)]
pub struct ClosenessCentralityStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub stddev: f64,
    pub p50: f64,
    pub p90: f64,
    pub p99: f64,
    pub isolated_nodes: u64,
    pub execution_time_ms: u64,
}

/// Closeness centrality facade/builder bound to a live graph store.
#[derive(Clone)]
pub struct ClosenessCentralityFacade {
    graph_store: Arc<DefaultGraphStore>,
    wasserman_faust: bool,
    direction: String,
}

impl ClosenessCentralityFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            wasserman_faust: false,
            direction: "both".to_string(),
        }
    }

    /// Enable/disable Wasserman–Faust normalization.
    pub fn wasserman_faust(mut self, enabled: bool) -> Self {
        self.wasserman_faust = enabled;
        self
    }

    /// Direction of traversal: "outgoing", "incoming", or "both".
    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = direction.to_string();
        self
    }

    fn orientation(&self) -> Orientation {
        match self.direction.as_str() {
            "incoming" => Orientation::Reverse,
            "outgoing" => Orientation::Natural,
            _ => Orientation::Undirected,
        }
    }

    fn checked_node_id(value: usize) -> Result<NodeId> {
        NodeId::try_from(value as i64).map_err(|_| {
            crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                "node_id must fit into i64 (got {})",
                value
            ))
        })
    }

    fn compute_scores(&self) -> Result<(Vec<f64>, std::time::Duration)> {
        let start = Instant::now();

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, self.orientation())
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((Vec::new(), start.elapsed()));
        }

        let mut farness = vec![0u64; node_count];
        let mut component = vec![0u64; node_count];

        let mut msbfs = AggregatedNeighborProcessingMsBfs::new(node_count);

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

        for source_offset in (0..node_count).step_by(crate::procedures::msbfs::OMEGA) {
            let source_len =
                (source_offset + crate::procedures::msbfs::OMEGA).min(node_count) - source_offset;

            msbfs.run(
                source_offset,
                source_len,
                false,
                get_neighbors,
                |node_id, depth, sources_mask| {
                    if depth == 0 {
                        return;
                    }
                    let len = sources_mask.count_ones() as u64;
                    farness[node_id] += len * depth as u64;
                    component[node_id] += len;
                },
            );
        }

        let mut centralities = vec![0.0f64; node_count];
        for idx in 0..node_count {
            let far = farness[idx];
            if far == 0 {
                centralities[idx] = 0.0;
                continue;
            }

            let comp = component[idx] as f64;
            let base = comp / far as f64;

            if self.wasserman_faust {
                centralities[idx] = if node_count <= 1 {
                    0.0
                } else {
                    base * (comp / (node_count as f64 - 1.0))
                };
            } else {
                centralities[idx] = base;
            }
        }

        Ok((centralities, start.elapsed()))
    }

    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = CentralityScore>>> {
        let (scores, _elapsed) = self.compute_scores()?;
        let iter = scores
            .into_iter()
            .enumerate()
            .map(|(node_id, score)| CentralityScore {
                node_id: node_id as u64,
                score,
            });
        Ok(Box::new(iter))
    }

    pub fn stats(&self) -> Result<ClosenessCentralityStats> {
        let (scores, elapsed) = self.compute_scores()?;
        if scores.is_empty() {
            return Ok(ClosenessCentralityStats {
                min: 0.0,
                max: 0.0,
                mean: 0.0,
                stddev: 0.0,
                p50: 0.0,
                p90: 0.0,
                p99: 0.0,
                isolated_nodes: 0,
                execution_time_ms: elapsed.as_millis() as u64,
            });
        }

        let isolated_nodes = scores.iter().filter(|v| **v == 0.0).count() as u64;

        let mut sorted = scores.clone();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let min = *sorted.first().unwrap();
        let max = *sorted.last().unwrap();
        let mean = scores.iter().sum::<f64>() / scores.len() as f64;
        let var = scores
            .iter()
            .map(|x| {
                let d = x - mean;
                d * d
            })
            .sum::<f64>()
            / scores.len() as f64;
        let stddev = var.sqrt();

        let percentile = |p: f64| -> f64 {
            let idx =
                ((p.clamp(0.0, 100.0) / 100.0) * (sorted.len() as f64 - 1.0)).round() as usize;
            sorted[idx]
        };

        Ok(ClosenessCentralityStats {
            min,
            max,
            mean,
            stddev,
            p50: percentile(50.0),
            p90: percentile(90.0),
            p99: percentile(99.0),
            isolated_nodes,
            execution_time_ms: elapsed.as_millis() as u64,
        })
    }

    /// Mutate mode is not implemented yet for closeness.
    pub fn mutate(
        &self,
        property_name: &str,
    ) -> Result<crate::procedures::facades::builder_base::MutationResult> {
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "ClosenessCentrality mutate/write is not implemented yet".to_string(),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(7),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_stream_returns_node_count_rows() {
        let facade = ClosenessCentralityFacade::new(store());
        let rows: Vec<_> = facade.stream().unwrap().collect();
        assert_eq!(rows.len(), 8);
    }

    #[test]
    fn test_stats_shape() {
        let facade = ClosenessCentralityFacade::new(store());
        let stats = facade.stats().unwrap();
        assert!(stats.max >= stats.min);
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let facade = ClosenessCentralityFacade::new(store());
        assert!(facade.mutate("").is_err());
        assert!(facade.mutate("closeness").is_err());
    }
}
