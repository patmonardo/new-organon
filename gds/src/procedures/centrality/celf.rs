//! CELF Facade
//!
//! Live wiring for Cost-Effective Lazy Forward influence maximization.

use crate::algo::algorithms::WriteResult;
use crate::algo::celf::storage::CELFStorageRuntime;
use crate::algo::celf::{
    CELFComputationRuntime, CELFConfig, CELFMutateResult, CELFMutationSummary, CELFResult,
    CELFResultBuilder, CELFRow, CELFStats,
};
use crate::collections::backends::vec::VecDouble;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::{ProgressTracker, TaskProgressTracker, TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::algo::algorithms::{AlgorithmRunner, Result};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph_store::GraphStore;
use crate::types::prelude::DefaultGraphStore;
use crate::types::properties::node::DefaultDoubleNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;

/// CELF facade bound to a live graph store
#[derive(Clone)]
pub struct CELFFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: CELFConfig,
    task_registry: Option<TaskRegistry>,
}

impl CELFFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: CELFConfig::default(),
            task_registry: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: CELFConfig,
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
        let parsed: CELFConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: CELFConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
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
        self.config.concurrency = concurrency;
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    fn compute_seed_set(&self) -> Result<(CELFResult, std::time::Duration)> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        let start = Instant::now();

        let storage = CELFStorageRuntime::new(&*self.graph_store)?;
        let node_count = storage.node_count();
        if node_count == 0 {
            return Ok((
                CELFResult {
                    seed_set_nodes: HashMap::new(),
                },
                start.elapsed(),
            ));
        }

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("celf".to_string(), self.config.seed_set_size),
            self.config.concurrency,
        );
        progress_tracker.begin_subtask_with_volume(self.config.seed_set_size);

        let runtime = CELFComputationRuntime::new(self.config.clone(), node_count);
        let termination = TerminationFlag::running_true();

        let seed_set = storage
            .compute_celf(&runtime, &termination)
            .map_err(|e| AlgorithmError::Execution(format!("CELF terminated: {e}")))?;

        progress_tracker.log_progress(self.config.seed_set_size);
        progress_tracker.end_subtask();

        Ok((
            CELFResult {
                seed_set_nodes: seed_set,
            },
            start.elapsed(),
        ))
    }

    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = CELFRow>>> {
        let (result, elapsed) = self.compute_seed_set()?;
        let rows = CELFResultBuilder::new(result, elapsed).rows();
        Ok(Box::new(rows.into_iter()))
    }

    pub fn stats(&self) -> Result<CELFStats> {
        let (result, elapsed) = self.compute_seed_set()?;
        Ok(CELFResultBuilder::new(result, elapsed).stats())
    }

    pub fn mutate(self, property_name: &str) -> Result<CELFMutateResult> {
        let (result, elapsed) = self.compute_seed_set()?;
        let seed_set = result.seed_set_nodes.clone();
        let builder = CELFResultBuilder::new(result, elapsed);

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

        let summary = CELFMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: builder.execution_time_ms(),
        };

        Ok(CELFMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        let _ = self.compute_seed_set()?;
        let node_count = self.graph_store.node_count();
        let nodes_written = node_count as u64;
        Ok(WriteResult::new(
            nodes_written,
            property_name.to_string(),
            std::time::Duration::from_millis(0),
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
    use crate::config::GraphStoreConfig;
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
    fn facade_finds_seed_nodes_on_star() {
        // 0 -> 1, 0 -> 2, 0 -> 3
        // Node 0 is the hub with maximum influence
        let store = store_from_directed_edges(4, &[(0, 1), (0, 2), (0, 3)]);
        let graph = GraphFacade::new(Arc::new(store));

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
        let graph = GraphFacade::new(Arc::new(store));

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
        let graph = GraphFacade::new(Arc::new(store));

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
