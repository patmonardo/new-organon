//! ApproxMaxKCut Facade
//!
//! Partitions nodes into k communities to maximize (or minimize) the
//! weight of edges crossing between communities using GRASP.

use crate::core::utils::progress::{TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::algo::approx_max_kcut::computation::ApproxMaxKCutComputationRuntime;
use crate::algo::approx_max_kcut::storage::ApproxMaxKCutStorageRuntime;
use crate::algo::approx_max_kcut::spec::ApproxMaxKCutConfig;
use crate::concurrency::TerminationFlag;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::sync::Arc;

/// Result row for approx max k-cut stream mode
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ApproxMaxKCutRow {
    /// Node ID
    pub node_id: u64,
    /// Assigned community (0 to k-1)
    pub community: u8,
}

/// Statistics for approx max k-cut computation
#[derive(Debug, Clone, serde::Serialize)]
pub struct ApproxMaxKCutStats {
    /// Total cut cost achieved
    pub cut_cost: f64,
    /// Number of communities
    pub k: u8,
    /// Number of nodes processed
    pub node_count: usize,
}

/// ApproxMaxKCut algorithm facade
#[derive(Clone)]
pub struct ApproxMaxKCutFacade {
    graph_store: Arc<DefaultGraphStore>,
    k: u8,
    iterations: usize,
    random_seed: u64,
    minimize: bool,
    has_relationship_weight_property: bool,
    min_community_sizes: Vec<usize>,
    concurrency: usize,
    task_registry: Option<TaskRegistry>,
}

impl ApproxMaxKCutFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            k: 2,
            iterations: 8,
            random_seed: 0,
            minimize: false,
            has_relationship_weight_property: false,
            min_community_sizes: vec![0, 0],
            concurrency: 4,
            task_registry: None,
        }
    }

    pub fn k(mut self, k: u8) -> Self {
        self.k = k;
        // Resize min_community_sizes to match k
        self.min_community_sizes.resize(k as usize, 0);
        self
    }

    pub fn iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn random_seed(mut self, seed: u64) -> Self {
        self.random_seed = seed;
        self
    }

    pub fn minimize(mut self, minimize: bool) -> Self {
        self.minimize = minimize;
        self
    }

    pub fn relationship_weight_property(mut self, use_weights: bool) -> Self {
        self.has_relationship_weight_property = use_weights;
        self
    }

    pub fn min_community_sizes(mut self, sizes: Vec<usize>) -> Self {
        self.min_community_sizes = sizes;
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(self.k as f64, 2.0, 127.0, "k")?;
        ConfigValidator::in_range(self.iterations as f64, 1.0, 1000.0, "iterations")?;
        ConfigValidator::in_range(self.concurrency as f64, 1.0, 1024.0, "concurrency")?;

        if self.min_community_sizes.len() != self.k as usize {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                    "min_community_sizes length ({}) must equal k ({})",
                    self.min_community_sizes.len(),
                    self.k
                )),
            );
        }

        Ok(())
    }

    fn compute(&self) -> Result<(Vec<u8>, f64, usize)> {
        self.validate()?;
        let node_count = self.graph_store.node_count();
        if node_count == 0 {
            return Ok((Vec::new(), 0.0, 0));
        }

        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume(
                "approx_max_kcut".to_string(),
                node_count.saturating_add(self.iterations),
            ),
            self.concurrency,
        );
        let termination_flag = TerminationFlag::default();

        let config = ApproxMaxKCutConfig {
            k: self.k,
            iterations: self.iterations,
            random_seed: self.random_seed,
            minimize: self.minimize,
            has_relationship_weight_property: self.has_relationship_weight_property,
            min_community_sizes: self.min_community_sizes.clone(),
        };

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
            .map_err(crate::projection::eval::procedure::AlgorithmError::Execution)?;

        Ok((result.communities, result.cut_cost, node_count))
    }

    /// Stream mode: yields community assignment per node
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = ApproxMaxKCutRow>>> {
        let (communities, _cost, _node_count) = self.compute()?;

        Ok(Box::new(
            communities
                .into_iter()
                .enumerate()
                .map(|(node_idx, community)| ApproxMaxKCutRow {
                    node_id: node_idx as u64,
                    community,
                }),
        ))
    }

    /// Stats mode: returns aggregated statistics
    pub fn stats(&self) -> Result<ApproxMaxKCutStats> {
        let (_communities, cost, node_count) = self.compute()?;

        Ok(ApproxMaxKCutStats {
            cut_cost: cost,
            k: self.k,
            node_count,
        })
    }

    /// Mutate mode: writes labels back to the graph store.
    pub fn mutate(self) -> Result<MutationResult> {
        // Note: mutation logic is deferred.
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "mutate not yet implemented".to_string(),
            ),
        )
    }

    /// Write mode: writes labels to a new graph.
    pub fn write(self) -> Result<WriteResult> {
        // Note: write logic is deferred.
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "write not yet implemented".to_string(),
            ),
        )
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
    use crate::procedures::Graph;
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
        let graph = Graph::new(Arc::new(store));

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
        let graph = Graph::new(Arc::new(store));

        let stats = graph.approx_max_kcut().k(2).iterations(3).stats().unwrap();

        assert_eq!(stats.k, 2);
        assert_eq!(stats.node_count, 4);
        assert!(stats.cut_cost >= 0.0);
    }
}
