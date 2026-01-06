//! Yen's K-Shortest Paths Facade
//!
//! Single-pair K-shortest paths using Yen's algorithm.
//!
//! This facade runs the translated Yen's runtime against a live `DefaultGraphStore`.

use crate::core::utils::progress::Tasks;
use crate::mem::MemoryRange;
use crate::algo::common::prelude::{PathFindingResult, PathResultBuilder};
use crate::algo::common::result_builders::ResultBuilder;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::{PathResult, Result};
use crate::algo::yens::{YensComputationRuntime, YensStorageRuntime};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Statistics about Yen's execution
#[derive(Debug, Clone, serde::Serialize)]
pub struct YensStats {
    pub paths_found: u64,
    pub computation_time_ms: u64,
    pub execution_time_ms: u64,
}

/// Yen's algorithm builder - fluent configuration
pub struct YensBuilder {
    graph_store: Arc<DefaultGraphStore>,
    source: Option<u64>,
    target: Option<u64>,
    k: usize,
    weight_property: String,
    relationship_types: Vec<String>,
    direction: String,
    track_relationships: bool,
    concurrency: usize,
}

impl YensBuilder {
    /// Create a new Yen's builder bound to a live graph store.
    ///
    /// Defaults:
    /// - source/target: None (must be set)
    /// - k: 3
    /// - weight_property: "weight"
    /// - relationship_types: all types
    /// - direction: "outgoing"
    /// - track_relationships: false
    /// - concurrency: 1
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            source: None,
            target: None,
            k: 3,
            weight_property: "weight".to_string(),
            relationship_types: vec![],
            direction: "outgoing".to_string(),
            track_relationships: false,
            concurrency: 1,
        }
    }

    pub fn source(mut self, source: u64) -> Self {
        self.source = Some(source);
        self
    }

    pub fn target(mut self, target: u64) -> Self {
        self.target = Some(target);
        self
    }

    pub fn k(mut self, k: usize) -> Self {
        self.k = k;
        self
    }

    pub fn weight_property(mut self, property: &str) -> Self {
        self.weight_property = property.to_string();
        self
    }

    pub fn relationship_types(mut self, relationship_types: Vec<String>) -> Self {
        self.relationship_types = relationship_types;
        self
    }

    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = direction.to_string();
        self
    }

    pub fn track_relationships(mut self, enabled: bool) -> Self {
        self.track_relationships = enabled;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    fn validate(&self) -> Result<()> {
        if self.source.is_none() {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "source node must be specified".to_string(),
                ),
            );
        }

        if self.target.is_none() {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "target node must be specified".to_string(),
                ),
            );
        }

        if self.k == 0 {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "k must be > 0".to_string(),
                ),
            );
        }

        if self.concurrency == 0 {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "concurrency must be > 0".to_string(),
                ),
            );
        }

        match self.direction.to_ascii_lowercase().as_str() {
            "outgoing" | "incoming" => {}
            other => {
                return Err(
                    crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                        "direction must be 'outgoing' or 'incoming' (got '{other}')"
                    )),
                );
            }
        }

        ConfigValidator::non_empty_string(&self.weight_property, "weight_property")?;

        Ok(())
    }

    fn checked_node_id(value: u64, field: &str) -> Result<NodeId> {
        NodeId::try_from(value).map_err(|_| {
            crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                "{field} must fit into i64 (got {value})",
            ))
        })
    }

    fn checked_u64(value: NodeId, context: &str) -> Result<u64> {
        u64::try_from(value).map_err(|_| {
            crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                "Yen's returned invalid node id for {context}: {value}",
            ))
        })
    }

    fn compute(self) -> Result<PathFindingResult> {
        self.validate()?;

        let source_u64 = self.source.expect("validate ensures source is set");
        let target_u64 = self.target.expect("validate ensures target is set");
        let source_node = Self::checked_node_id(source_u64, "source")?;
        let target_node = Self::checked_node_id(target_u64, "target")?;

        let rel_types: HashSet<RelationshipType> = if self.relationship_types.is_empty() {
            self.graph_store.relationship_types()
        } else {
            RelationshipType::list_of(self.relationship_types.clone())
                .into_iter()
                .collect()
        };

        let (orientation, direction_byte) = match self.direction.to_ascii_lowercase().as_str() {
            "incoming" => (Orientation::Reverse, 1u8),
            _ => (Orientation::Natural, 0u8),
        };

        let selectors: HashMap<RelationshipType, String> = rel_types
            .iter()
            .map(|t| (t.clone(), self.weight_property.clone()))
            .collect();

        let graph_view = self
            .graph_store
            .get_graph_with_types_selectors_and_orientation(&rel_types, &selectors, orientation)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let storage = YensStorageRuntime::new(
            source_node,
            target_node,
            self.k,
            self.track_relationships,
            self.concurrency,
        );

        let mut computation = YensComputationRuntime::new(
            source_node,
            target_node,
            self.k,
            self.track_relationships,
            self.concurrency,
        );

        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("yens".to_string(), self.k),
            self.concurrency,
        );

        let start = std::time::Instant::now();
        let result = storage.compute_yens(
            &mut computation,
            Some(graph_view.as_ref()),
            direction_byte,
            &mut progress_tracker,
        )?;

        let paths: Vec<crate::algo::common::result_builders::PathResult> = result
            .paths
            .into_iter()
            .map(|p| {
                let source = Self::checked_u64(p.source_node, "source")?;
                let target = Self::checked_u64(p.target_node, "target")?;
                let path = p
                    .node_ids
                    .into_iter()
                    .map(|node_id| Self::checked_u64(node_id, "path"))
                    .collect::<Result<Vec<_>>>()?;

                Ok(crate::algo::common::result_builders::PathResult {
                    source,
                    target,
                    path,
                    cost: p.total_cost,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        // Create execution metadata
        let mut additional = std::collections::HashMap::new();
        additional.insert(
            "computation_time_ms".to_string(),
            result.computation_time_ms.to_string(),
        );
        additional.insert("k".to_string(), self.k.to_string());
        additional.insert(
            "track_relationships".to_string(),
            self.track_relationships.to_string(),
        );

        let metadata = crate::algo::common::result_builders::ExecutionMetadata {
            execution_time: start.elapsed(),
            iterations: None,
            converged: None,
            additional,
        };

        // Build result using upgraded result builder
        let path_result = PathResultBuilder::new()
            .with_paths(paths)
            .with_metadata(metadata)
            .build()
            .map_err(
                |e: crate::algo::common::result_builders::ResultBuilderError| {
                    crate::projection::eval::procedure::AlgorithmError::Execution(e.to_string())
                },
            )?;

        Ok(path_result)
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PathResult>>> {
        let result = self.compute()?;
        let paths = result.paths.into_iter().map(|p| PathResult {
            source: p.source,
            target: p.target,
            path: p.path,
            cost: p.cost,
        });
        Ok(Box::new(paths))
    }

    pub fn stats(self) -> Result<YensStats> {
        let result = self.compute()?;
        let computation_time_ms = result
            .metadata
            .additional
            .get("computation_time_ms")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        Ok(YensStats {
            paths_found: result.paths.len() as u64,
            computation_time_ms,
            execution_time_ms: result.metadata.execution_time.as_millis() as u64,
        })
    }

    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "Yen's mutate/write is not implemented yet".to_string(),
            ),
        )
    }

    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "Yen's mutate/write is not implemented yet".to_string(),
            ),
        )
    }

    /// Estimate memory requirements for Yen's K-shortest paths execution
    ///
    /// Returns a memory range estimate based on path storage, priority queues, and graph overhead.
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();
        let k = self.k;

        // Priority queue for candidate paths (can grow large)
        let queue_memory = k * node_count * 16; // path + cost per candidate

        // Path storage (storing k shortest paths)
        let path_storage_memory = k * node_count * 8; // worst case: full paths

        // Distance arrays for each candidate path computation
        let distance_arrays_memory = k * node_count * 8;

        // Graph structure overhead
        let avg_degree = 10.0;
        let relationship_count = (node_count as f64 * avg_degree) as usize;
        let graph_overhead = relationship_count * 16;

        let total_memory =
            queue_memory + path_storage_memory + distance_arrays_memory + graph_overhead;
        let overhead = total_memory / 5;
        MemoryRange::of_range(total_memory, total_memory + overhead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    #[test]
    fn test_builder_validation() {
        let config = RandomGraphConfig {
            seed: Some(99),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 0.7)],
            ..RandomGraphConfig::default()
        };

        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());

        assert!(YensBuilder::new(Arc::clone(&store)).validate().is_err());
        assert!(YensBuilder::new(Arc::clone(&store))
            .source(0)
            .validate()
            .is_err());
        assert!(YensBuilder::new(Arc::clone(&store))
            .source(0)
            .target(1)
            .k(0)
            .validate()
            .is_err());
    }

    #[test]
    fn test_stream_smoke() {
        let config = RandomGraphConfig {
            seed: Some(101),
            node_count: 10,
            relationships: vec![RandomRelationshipConfig::new("REL", 0.8)],
            ..RandomGraphConfig::default()
        };

        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let graph = crate::procedures::Graph::new(store);

        let _rows: Vec<_> = graph
            .yens()
            .source(0)
            .target(3)
            .k(3)
            .relationship_types(vec!["REL".to_string()])
            .weight_property("weight")
            .stream()
            .unwrap()
            .collect();
    }
}
