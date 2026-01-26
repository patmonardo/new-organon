//! Steiner Tree Facade
//!
//! Computes minimum Steiner trees connecting source nodes to terminal nodes.
//! Uses approximation algorithms with delta-stepping and rerouting optimizations.

use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::steiner_tree::{
    SteinerTreeComputationRuntime, SteinerTreeConfig, SteinerTreeMutateResult, SteinerTreeResult,
    SteinerTreeResultBuilder, SteinerTreeRow, SteinerTreeStats, SteinerTreeStorageRuntime,
};
use crate::mem::MemoryRange;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

// Import upgraded systems
use crate::algo::algorithms::pathfinding::PathResult;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::{TaskRegistryFactory, Tasks};
use crate::projection::eval::algorithm::AlgorithmError;

/// Steiner Tree algorithm builder
pub struct SteinerTreeBuilder {
    graph_store: Arc<DefaultGraphStore>,
    source_node: u64,
    target_nodes: Vec<u64>,
    relationship_weight_property: Option<String>,
    delta: f64,
    apply_rerouting: bool,
    concurrency: usize,
    /// Progress tracking components
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

impl SteinerTreeBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            source_node: 0,
            target_nodes: Vec::new(),
            relationship_weight_property: None,
            delta: 1.0,
            apply_rerouting: true,
            concurrency: 4,
            task_registry_factory: None,
            user_log_registry_factory: None,
        }
    }

    pub fn source_node(mut self, source: u64) -> Self {
        self.source_node = source;
        self
    }

    pub fn target_nodes(mut self, targets: Vec<u64>) -> Self {
        self.target_nodes = targets;
        self
    }

    pub fn relationship_weight_property(mut self, property: &str) -> Self {
        self.relationship_weight_property = Some(property.to_string());
        self
    }

    pub fn delta(mut self, delta: f64) -> Self {
        self.delta = delta;
        self
    }

    pub fn apply_rerouting(mut self, apply: bool) -> Self {
        self.apply_rerouting = apply;
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// Steiner tree benefits from parallelism in large graphs.
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
        if self.target_nodes.is_empty() {
            return Err(AlgorithmError::Execution(
                "target_nodes must not be empty".to_string(),
            ));
        }

        if self.concurrency == 0 {
            return Err(AlgorithmError::Execution(
                "concurrency must be > 0".to_string(),
            ));
        }

        if self.delta <= 0.0 {
            return Err(AlgorithmError::Execution("delta must be > 0".to_string()));
        }

        ConfigValidator::in_range(self.delta, 0.0, 100.0, "delta")?;

        Ok(())
    }

    fn compute(self) -> Result<(SteinerTreeResult, std::time::Duration)> {
        self.validate()?;
        let start = std::time::Instant::now();

        // Steiner tree works on undirected graphs.
        // Use all relationship types by default.
        let rel_types: HashSet<RelationshipType> = self.graph_store.relationship_types();

        let graph_view = if let Some(prop) = self.relationship_weight_property.as_ref() {
            let selectors: HashMap<RelationshipType, String> = rel_types
                .iter()
                .map(|t| (t.clone(), prop.clone()))
                .collect();
            self.graph_store
                .get_graph_with_types_selectors_and_orientation(
                    &rel_types,
                    &selectors,
                    Orientation::Undirected,
                )
                .map_err(|e| AlgorithmError::Graph(e.to_string()))?
        } else {
            self.graph_store
                .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
                .map_err(|e| AlgorithmError::Graph(e.to_string()))?
        };

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((
                SteinerTreeResult {
                    parent_array: Vec::new(),
                    relationship_to_parent_cost: Vec::new(),
                    total_cost: 0.0,
                    effective_node_count: 0,
                    effective_target_nodes_count: 0,
                },
                start.elapsed(),
            ));
        }

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("steiner_tree".to_string(), node_count),
            self.concurrency,
        );

        let source_node: NodeId = NodeId::try_from(self.source_node).map_err(|_| {
            AlgorithmError::Execution(format!(
                "source_node must fit into i64 (got {})",
                self.source_node
            ))
        })?;
        let target_nodes: Vec<NodeId> = self
            .target_nodes
            .iter()
            .map(|&t| {
                NodeId::try_from(t).map_err(|_| {
                    AlgorithmError::Execution(format!("target_nodes must fit into i64 (got {t})"))
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let config = SteinerTreeConfig {
            source_node,
            target_nodes,
            relationship_weight_property: self.relationship_weight_property.clone(),
            delta: self.delta,
            apply_rerouting: self.apply_rerouting,
        };

        let storage = SteinerTreeStorageRuntime::new(config, self.concurrency);
        let mut computation = SteinerTreeComputationRuntime::new(self.delta, node_count);
        let result = storage.compute_steiner_tree(
            &mut computation,
            Some(graph_view.as_ref()),
            &mut progress_tracker,
        )?;
        Ok((result, start.elapsed()))
    }

    /// Stream mode: yields tree edges
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = SteinerTreeRow>>> {
        let (result, elapsed) = self.compute()?;
        let rows = SteinerTreeResultBuilder::new(result, elapsed).rows();
        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: aggregated tree stats
    pub fn stats(self) -> Result<SteinerTreeStats> {
        let (result, elapsed) = self.compute()?;
        Ok(SteinerTreeResultBuilder::new(result, elapsed).stats())
    }

    /// Mutate mode: writes results back to the graph store
    pub fn mutate(self, property_name: &str) -> Result<SteinerTreeMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let graph_store = Arc::clone(&self.graph_store);
        let (result, elapsed) = self.compute()?;
        let builder = SteinerTreeResultBuilder::new(result, elapsed);
        let paths: Vec<PathResult> = builder.paths();

        let updated_store = crate::algo::algorithms::pathfinding::build_path_relationship_store(
            graph_store.as_ref(),
            property_name,
            &paths,
        )?;

        let summary = builder.mutation_summary(property_name, paths.len() as u64);

        Ok(SteinerTreeMutateResult {
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
        // Estimate based on node count and expected tree structure
        let node_count = self.graph_store.node_count();
        let estimated_bytes = node_count * std::mem::size_of::<f64>() * 3; // distances, parents, costs
        Ok(MemoryRange::of_range(
            estimated_bytes / 2,
            estimated_bytes * 2,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::procedures::GraphFacade;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(13),
            node_count: 10,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_builder_defaults() {
        let builder = SteinerTreeBuilder::new(store());
        assert_eq!(builder.source_node, 0);
        assert!(builder.target_nodes.is_empty());
        assert!(builder.relationship_weight_property.is_none());
        assert_eq!(builder.delta, 1.0);
        assert!(builder.apply_rerouting);
    }

    #[test]
    fn test_stream_smoke() {
        let store = store();
        let rows: Vec<_> = GraphFacade::new(store)
            .steiner_tree()
            .source_node(0)
            .target_nodes(vec![5, 7])
            .stream()
            .unwrap()
            .collect();

        assert!(!rows.is_empty());
    }

    #[test]
    fn test_stats_smoke() {
        let store = store();
        let stats = GraphFacade::new(store)
            .steiner_tree()
            .source_node(0)
            .target_nodes(vec![5, 7])
            .stats()
            .unwrap();

        assert!(stats.effective_target_nodes_count > 0);
    }
}
