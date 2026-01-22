//! CELF Facade
//!
//! Live wiring for Cost-Effective Lazy Forward influence maximization.

use crate::algo::celf::computation::CELFComputationRuntime;
use crate::algo::celf::spec::CELFConfig;
use crate::algo::celf::storage::CELFStorageRuntime;
use crate::collections::backends::vec::VecDouble;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::{ProgressTracker, TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{MutationResult, WriteResult};
use crate::procedures::traits::{AlgorithmRunner, Result};
use crate::projection::eval::procedure::AlgorithmError;
use crate::types::graph_store::GraphStore;
use crate::types::prelude::DefaultGraphStore;
use crate::types::properties::node::impls::default_node_property_values::DefaultDoubleNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;

/// Result row for CELF stream mode
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct CELFRow {
    pub node_id: u64,
    pub spread: f64,
}

/// Statistics for CELF computation
#[derive(Debug, Clone, serde::Serialize)]
pub struct CELFStats {
    pub seed_count: usize,
    pub total_spread: f64,
    pub execution_time_ms: u64,
}

/// Result for CELF mutate mode
#[derive(Debug, Clone)]
pub struct CELFMutateResult {
    pub summary: MutationResult,
    pub updated_store: Arc<DefaultGraphStore>,
}

/// CELF facade bound to a live graph store
#[derive(Clone)]
pub struct CELFFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: CELFConfig,
    concurrency: usize,
    task_registry: Option<TaskRegistry>,
}

impl CELFFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: CELFConfig::default(),
            concurrency: 4,
            task_registry: None,
        }
    }

    pub fn with_config(mut self, config: CELFConfig) -> Self {
        self.config = config;
        self
    }

    pub fn seed_set_size(mut self, size: usize) -> Self {
        self.config.seed_set_size = size;
        self
    }

    pub fn monte_carlo_simulations(mut self, simulations: usize) -> Self {
        self.config.monte_carlo_simulations = simulations;
        self
    }

    pub fn propagation_probability(mut self, prob: f64) -> Self {
        self.config.propagation_probability = prob;
        self
    }

    pub fn batch_size(mut self, size: usize) -> Self {
        self.config.batch_size = size;
        self
    }

    pub fn random_seed(mut self, seed: u64) -> Self {
        self.config.random_seed = seed;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    fn compute_seed_set(&self) -> Result<(HashMap<u64, f64>, std::time::Duration)> {
        let start = Instant::now();

        let storage = CELFStorageRuntime::new(&*self.graph_store)?;
        let node_count = storage.node_count();
        if node_count == 0 {
            return Ok((HashMap::new(), start.elapsed()));
        }

        let mut progress_tracker =
            crate::core::utils::progress::TaskProgressTracker::with_concurrency(
                Tasks::leaf_with_volume("celf".to_string(), self.config.seed_set_size),
                self.concurrency,
            );
        progress_tracker.begin_subtask_with_volume(self.config.seed_set_size);

        let runtime = CELFComputationRuntime::new(self.config.clone(), node_count);
        let termination = TerminationFlag::running_true();

        let seed_set = storage
            .compute_celf(&runtime, &termination)
            .map_err(|e| AlgorithmError::Execution(format!("CELF terminated: {e}")))?;

        progress_tracker.log_progress(self.config.seed_set_size);
        progress_tracker.end_subtask();

        Ok((seed_set, start.elapsed()))
    }

    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = CELFRow>>> {
        let (seed_set, _elapsed) = self.compute_seed_set()?;

        let mut rows: Vec<CELFRow> = seed_set
            .into_iter()
            .map(|(node_id, spread)| CELFRow { node_id, spread })
            .collect();

        // Sort by spread descending, then by node_id ascending
        rows.sort_by(|a, b| {
            b.spread
                .partial_cmp(&a.spread)
                .unwrap()
                .then_with(|| a.node_id.cmp(&b.node_id))
        });

        Ok(Box::new(rows.into_iter()))
    }

    pub fn stats(&self) -> Result<CELFStats> {
        let (seed_set, elapsed) = self.compute_seed_set()?;
        let total_spread: f64 = seed_set.values().sum();

        Ok(CELFStats {
            seed_count: seed_set.len(),
            total_spread,
            execution_time_ms: elapsed.as_millis() as u64,
        })
    }

    pub fn mutate(self, property_name: &str) -> Result<CELFMutateResult> {
        let start = Instant::now();
        let (seed_set, _elapsed) = self.compute_seed_set()?;

        let node_count = self.graph_store.node_count();
        let nodes_updated = node_count as u64;

        let mut scores = vec![0.0; node_count];
        for (node_id, spread) in seed_set {
            let idx = node_id as usize;
            if idx < scores.len() {
                scores[idx] = spread;
            }
        }

        let backend = VecDouble::from(scores);
        let values = DefaultDoubleNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("CELF mutate failed to add property: {e}"))
            })?;

        let summary =
            MutationResult::new(nodes_updated, property_name.to_string(), start.elapsed());

        Ok(CELFMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        let start = Instant::now();
        let (seed_set, _elapsed) = self.compute_seed_set()?;
        let node_count = self.graph_store.node_count();
        let nodes_written = node_count as u64;
        let _ = seed_set;
        Ok(WriteResult::new(
            nodes_written,
            property_name.to_string(),
            start.elapsed(),
        ))
    }

    pub fn estimate_memory(&self) -> MemoryRange {
        // Estimate memory for CELF computation
        // - HashMap for seed set: seed_set_size * (8 + 8) bytes
        // - Monte Carlo simulations: monte_carlo_simulations * node_count * 8 bytes
        // - Graph view overhead: roughly node_count * 16 bytes
        let seed_set_memory = self.config.seed_set_size * 16;
        let simulation_memory =
            self.config.monte_carlo_simulations * self.graph_store.node_count() * 8;
        let graph_memory = self.graph_store.node_count() * 16;

        let total = seed_set_memory + simulation_memory + graph_memory;
        MemoryRange::of_range(total, total * 2) // Conservative upper bound
    }
}

impl AlgorithmRunner for CELFFacade {
    fn algorithm_name(&self) -> &'static str {
        "celf"
    }

    fn description(&self) -> &'static str {
        "Cost-Effective Lazy Forward influence maximization (Independent Cascade)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::procedures::GraphFacade;
    use crate::projection::RelationshipType;
    use crate::types::graph::{RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::properties::PropertyValues;
    use crate::types::schema::{Direction, MutableGraphSchema};
    use std::collections::HashMap;

    fn store_from_directed_edges(node_count: usize, edges: &[(usize, usize)]) -> DefaultGraphStore {
        let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];

        for &(src, tgt) in edges {
            outgoing[src].push(tgt as i64);
            incoming[tgt].push(src as i64);
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
    fn facade_finds_seed_nodes_on_star() {
        // 0 -> 1, 0 -> 2, 0 -> 3
        // Node 0 is the hub with maximum influence
        let store = store_from_directed_edges(4, &[(0, 1), (0, 2), (0, 3)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph
            .celf()
            .seed_set_size(1)
            .monte_carlo_simulations(10)
            .propagation_probability(1.0)
            .stream()
            .unwrap()
            .collect();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].node_id, 0);
    }

    #[test]
    fn facade_respects_seed_set_size() {
        let store = store_from_directed_edges(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph
            .celf()
            .seed_set_size(3)
            .monte_carlo_simulations(10)
            .propagation_probability(0.5)
            .stream()
            .unwrap()
            .collect();

        assert_eq!(rows.len(), 3);
    }

    #[test]
    fn mutate_adds_seed_spread_property() {
        let store = store_from_directed_edges(4, &[(0, 1), (0, 2), (0, 3)]);
        let graph = Graph::new(Arc::new(store));

        let result = graph
            .celf()
            .seed_set_size(1)
            .monte_carlo_simulations(10)
            .propagation_probability(1.0)
            .random_seed(42)
            .mutate("celf_spread")
            .unwrap();

        let values = result
            .updated_store
            .node_property_values("celf_spread")
            .unwrap();

        assert_eq!(values.element_count(), 4);
        assert!(values.double_value(0).unwrap() > 0.0);
        assert_eq!(values.double_value(1).unwrap(), 0.0);
        assert_eq!(values.double_value(2).unwrap(), 0.0);
        assert_eq!(values.double_value(3).unwrap(), 0.0);
    }
}
