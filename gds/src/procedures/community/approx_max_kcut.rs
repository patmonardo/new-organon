//! ApproxMaxKCut Facade
//!
//! Partitions nodes into k communities to maximize (or minimize) the
//! weight of edges crossing between communities using GRASP.

use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::approx_max_kcut::spec::{
    ApproxMaxKCutConfig, ApproxMaxKCutMutateResult, ApproxMaxKCutMutationSummary,
    ApproxMaxKCutResult, ApproxMaxKCutResultBuilder, ApproxMaxKCutStats,
};
use crate::algo::approx_max_kcut::storage::ApproxMaxKCutStorageRuntime;
use crate::algo::approx_max_kcut::ApproxMaxKCutComputationRuntime;
use crate::collections::backends::vec::VecLong;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::{TaskProgressTracker, TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultLongNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Result row for approx max k-cut stream mode
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ApproxMaxKCutRow {
    /// Node ID
    pub node_id: u64,
    /// Assigned community (0 to k-1)
    pub community: u8,
}

/// ApproxMaxKCut algorithm facade
#[derive(Clone)]
pub struct ApproxMaxKCutFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: ApproxMaxKCutConfig,
    task_registry: Option<TaskRegistry>,
}

impl ApproxMaxKCutFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: ApproxMaxKCutConfig::default(),
            task_registry: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: ApproxMaxKCutConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
            task_registry: None,
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: ApproxMaxKCutConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: ApproxMaxKCutConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    pub fn k(mut self, k: u8) -> Self {
        self.config.k = k;
        // Resize min_community_sizes to match k
        self.config.min_community_sizes.resize(k as usize, 0);
        self
    }

    pub fn iterations(mut self, iterations: usize) -> Self {
        self.config.iterations = iterations;
        self
    }

    pub fn random_seed(mut self, seed: u64) -> Self {
        self.config.random_seed = seed;
        self
    }

    pub fn minimize(mut self, minimize: bool) -> Self {
        self.config.minimize = minimize;
        self
    }

    pub fn relationship_weight_property(mut self, use_weights: bool) -> Self {
        self.config.has_relationship_weight_property = use_weights;
        self
    }

    pub fn min_community_sizes(mut self, sizes: Vec<usize>) -> Self {
        self.config.min_community_sizes = sizes;
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    fn validate(&self) -> Result<()> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    fn compute(&self) -> Result<ApproxMaxKCutResult> {
        self.validate()?;
        let start = Instant::now();
        let node_count = self.graph_store.node_count();
        if node_count == 0 {
            return Ok(ApproxMaxKCutResult {
                communities: Vec::new(),
                cut_cost: 0.0,
                k: self.config.k,
                node_count: 0,
                execution_time: start.elapsed(),
            });
        }

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume(
                "approx_max_kcut".to_string(),
                node_count.saturating_add(self.config.iterations),
            ),
            self.config.concurrency,
        );
        let termination_flag = TerminationFlag::default();

        let config = self.config.clone();

        let storage = ApproxMaxKCutStorageRuntime::new();
        let mut runtime = ApproxMaxKCutComputationRuntime::new(config.clone());
        let result = storage
            .compute_approx_max_kcut(
                &mut runtime,
                self.graph_store.as_ref(),
                &config,
                &mut progress_tracker,
                &termination_flag,
            )
            .map_err(AlgorithmError::Execution)?;

        Ok(ApproxMaxKCutResult {
            communities: result.communities,
            cut_cost: result.cut_cost,
            k: config.k,
            node_count,
            execution_time: start.elapsed(),
        })
    }

    /// Stream mode: yields community assignment per node
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = ApproxMaxKCutRow>>> {
        let result = self.compute()?;

        Ok(Box::new(result.communities.into_iter().enumerate().map(
            |(node_idx, community)| ApproxMaxKCutRow {
                node_id: node_idx as u64,
                community,
            },
        )))
    }

    /// Stats mode: returns aggregated statistics
    pub fn stats(&self) -> Result<ApproxMaxKCutStats> {
        let result = self.compute()?;
        Ok(ApproxMaxKCutResultBuilder::new(result).stats())
    }

    /// Mutate mode: writes labels back to the graph store.
    pub fn mutate(self, property_name: &str) -> Result<ApproxMaxKCutMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let result = self.compute()?;
        let communities = result.communities;
        let node_count = result.node_count;

        let nodes_updated = node_count as u64;

        let longs: Vec<i64> = communities.into_iter().map(|c| c as i64).collect();
        let backend = VecLong::from(longs);
        let values = DefaultLongNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels_set: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels_set, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!(
                    "ApproxMaxKCut mutate failed to add property: {e}"
                ))
            })?;

        let summary = ApproxMaxKCutMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: result.execution_time.as_millis() as u64,
        };

        Ok(ApproxMaxKCutMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode: writes labels to a new graph.
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        let res = self.mutate(property_name)?;
        Ok(WriteResult::new(
            res.summary.nodes_updated,
            property_name.to_string(),
            std::time::Duration::from_millis(res.summary.execution_time_ms),
        ))
    }

    /// Estimate memory usage.
    pub fn estimate_memory(&self) -> Result<MemoryRange> {
        // ApproxMaxKCut builds adjacency + reverse adjacency in the computation runtime.
        // Rough estimate (bytes):
        // - communities (u8) + a few per-node arrays: O(n)
        // - adjacency + reverse adjacency: O(m)
        let node_count = self.graph_store.node_count();
        let relationship_count = self.graph_store.relationship_count();

        // Per node: labels + bookkeeping + Vec headers (conservative).
        let per_node = 96usize;
        // Per relationship: store (usize,f64) pairs in adjacency and reverse (two copies).
        let per_relationship = 48usize;

        let base: usize = 64 * 1024; // fixed overhead
        let total = base
            .saturating_add(node_count.saturating_mul(per_node))
            .saturating_add(relationship_count.saturating_mul(per_relationship));

        Ok(MemoryRange::of_range(total, total.saturating_mul(3)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphStoreConfig;
    use crate::procedures::GraphFacade;
    use crate::projection::RelationshipType;
    use crate::types::graph::{RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::schema::{Direction, MutableGraphSchema};
    use std::collections::HashMap;

    fn store_from_edges(node_count: usize, edges: &[(usize, usize)]) -> DefaultGraphStore {
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
    fn facade_partitions_graph() {
        // Simple clique
        let store = store_from_edges(
            4,
            &[
                (0, 1),
                (1, 0),
                (0, 2),
                (2, 0),
                (1, 2),
                (2, 1),
                (0, 3),
                (3, 0),
                (1, 3),
                (3, 1),
                (2, 3),
                (3, 2),
            ],
        );
        let graph = GraphFacade::new(Arc::new(store));

        let rows: Vec<_> = graph
            .approx_max_kcut()
            .k(2)
            .iterations(5)
            .random_seed(42)
            .stream()
            .unwrap()
            .collect();

        assert_eq!(rows.len(), 4);

        // All nodes should be assigned to a community
        for row in &rows {
            assert!(row.community < 2);
        }
    }

    #[test]
    fn facade_computes_stats() {
        let store = store_from_edges(4, &[(0, 1), (1, 2), (2, 3)]);
        let graph = GraphFacade::new(Arc::new(store));

        let stats = graph.approx_max_kcut().k(2).iterations(3).stats().unwrap();

        assert_eq!(stats.k, 2);
        assert_eq!(stats.node_count, 4);
        assert!(stats.cut_cost >= 0.0);
    }

    #[test]
    fn facade_mutates_store_with_property() {
        let store = store_from_edges(
            4,
            &[
                (0, 1),
                (1, 0),
                (0, 2),
                (2, 0),
                (1, 2),
                (2, 1),
                (0, 3),
                (3, 0),
                (1, 3),
                (3, 1),
                (2, 3),
                (3, 2),
            ],
        );
        let graph = GraphFacade::new(Arc::new(store));

        let mutation_result = graph
            .approx_max_kcut()
            .k(2)
            .iterations(5)
            .random_seed(42)
            .mutate("community")
            .unwrap();

        assert_eq!(mutation_result.summary.nodes_updated, 4);
        assert_eq!(mutation_result.summary.property_name, "community");
        assert!(mutation_result.updated_store.has_node_property("community"));
    }
}
