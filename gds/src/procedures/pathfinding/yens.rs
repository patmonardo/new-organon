//! Yen's K-Shortest Paths Facade
//!
//! Single-pair K-shortest paths using Yen's algorithm.
//!
//! This facade runs the translated Yen's runtime against a live `DefaultGraphStore`.

use crate::algo::algorithms::pathfinding::{PathFindingResult, PathResult};
use crate::algo::algorithms::Result;
use crate::algo::yens::{
    YensComputationRuntime, YensConfig, YensMutateResult, YensMutationSummary, YensResult,
    YensResultBuilder, YensStats, YensStorageRuntime, YensWriteSummary,
};
use crate::core::utils::progress::{TaskProgressTracker, Tasks};
use crate::mem::MemoryRange;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// Additional import for error handling
use crate::projection::eval::algorithm::AlgorithmError;

/// Yen's algorithm facade - fluent configuration
pub struct YensFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: YensConfig,
    concurrency: usize,
}

/// Backwards-compatible alias (builder-style naming).
pub type YensBuilder = YensFacade;

impl YensFacade {
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
            config: YensConfig::default(),
            concurrency: 1,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: YensConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
            concurrency: 1,
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: YensConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: YensConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    pub fn source(mut self, source: u64) -> Self {
        self.config.source_node = i64::try_from(source).unwrap_or(-1);
        self
    }

    pub fn source_node(mut self, source: NodeId) -> Self {
        self.config.source_node = source;
        self
    }

    pub fn target(mut self, target: u64) -> Self {
        self.config.target_node = i64::try_from(target).unwrap_or(-1);
        self
    }

    pub fn target_node(mut self, target: NodeId) -> Self {
        self.config.target_node = target;
        self
    }

    pub fn k(mut self, k: usize) -> Self {
        self.config.k = k;
        self
    }

    pub fn weight_property(mut self, property: &str) -> Self {
        self.config.weight_property = property.to_string();
        self
    }

    pub fn relationship_types(mut self, relationship_types: Vec<String>) -> Self {
        self.config.relationship_types = relationship_types;
        self
    }

    pub fn direction(mut self, direction: &str) -> Self {
        self.config.direction = direction.to_string();
        self
    }

    pub fn track_relationships(mut self, enabled: bool) -> Self {
        self.config.track_relationships = enabled;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self.concurrency = concurrency;
        self
    }

    fn compute(self) -> Result<PathFindingResult> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        let source_node = self.config.source_node;
        let target_node = self.config.target_node;

        let rel_types: HashSet<RelationshipType> = if self.config.relationship_types.is_empty() {
            self.graph_store.relationship_types()
        } else {
            RelationshipType::list_of(self.config.relationship_types.clone())
                .into_iter()
                .collect()
        };

        let (orientation, direction_byte) =
            match self.config.direction.to_ascii_lowercase().as_str() {
                "incoming" => (Orientation::Reverse, 1u8),
                _ => (Orientation::Natural, 0u8),
            };

        let selectors: HashMap<RelationshipType, String> = rel_types
            .iter()
            .map(|t| (t.clone(), self.config.weight_property.clone()))
            .collect();

        let graph_view = self
            .graph_store
            .get_graph_with_types_selectors_and_orientation(&rel_types, &selectors, orientation)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let storage = YensStorageRuntime::new(
            source_node,
            target_node,
            self.config.k,
            self.config.track_relationships,
            self.config.concurrency,
        );

        let mut computation = YensComputationRuntime::new(
            source_node,
            target_node,
            self.config.k,
            self.config.track_relationships,
            self.config.concurrency,
        );

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("yens".to_string(), self.config.k),
            self.config.concurrency,
        );

        let start = std::time::Instant::now();
        let result: YensResult = storage.compute_yens(
            &mut computation,
            Some(graph_view.as_ref()),
            direction_byte,
            &mut progress_tracker,
        )?;
        YensResultBuilder::result(
            result,
            start.elapsed(),
            self.config.k,
            self.config.track_relationships,
        )
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PathResult>>> {
        let result = self.compute()?;
        Ok(Box::new(result.paths.into_iter()))
    }

    pub fn stats(self) -> Result<YensStats> {
        let result = self.compute()?;
        let computation_time_ms = result
            .metadata
            .additional
            .get("computation_time_ms")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        Ok(YensStats {
            paths_found: result.paths.len() as u64,
            computation_time_ms,
            execution_time_ms: result.metadata.execution_time.as_millis() as u64,
        })
    }

    pub fn mutate(self, property_name: &str) -> Result<YensMutateResult> {
        if property_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "property_name cannot be empty".to_string(),
            ));
        }
        let graph_store = Arc::clone(&self.graph_store);
        let result = self.compute()?;
        let paths = result.paths;

        let updated_store = crate::algo::algorithms::pathfinding::build_path_relationship_store(
            graph_store.as_ref(),
            property_name,
            &paths,
        )?;

        let summary = YensMutationSummary {
            nodes_updated: paths.len() as u64,
            property_name: property_name.to_string(),
            execution_time_ms: result.metadata.execution_time.as_millis() as u64,
        };

        Ok(YensMutateResult {
            summary,
            updated_store,
        })
    }

    pub fn write(self, property_name: &str) -> Result<YensWriteSummary> {
        if property_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "property_name cannot be empty".to_string(),
            ));
        }
        let res = self.mutate(property_name)?;
        Ok(YensWriteSummary {
            nodes_written: res.summary.nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: res.summary.execution_time_ms,
        })
    }

    /// Estimate memory requirements for Yen's K-shortest paths execution
    ///
    /// Returns a memory range estimate based on path storage, priority queues, and graph overhead.
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();
        let k = self.config.k;

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
    use crate::procedures::GraphFacade;
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

        assert!(YensBuilder::new(Arc::clone(&store))
            .source_node(-1)
            .config
            .validate()
            .is_err());
        assert!(YensBuilder::new(Arc::clone(&store))
            .target_node(-1)
            .config
            .validate()
            .is_err());
        assert!(YensBuilder::new(Arc::clone(&store))
            .source(0)
            .target(1)
            .k(0)
            .config
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
        let graph = GraphFacade::new(store);

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
