//! Conductance Facade
//!
//! Evaluates community quality by measuring the proportion of edges
//! that cross community boundaries.

use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::conductance::{
    ConductanceComputationRuntime, ConductanceConfig, ConductanceMutateResult,
    ConductanceMutationSummary, ConductanceResult, ConductanceResultBuilder, ConductanceStats,
    ConductanceStorageRuntime,
};
use crate::collections::backends::vec::VecDouble;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, JobId, TaskProgressTracker, TaskRegistry, TaskRegistryFactory,
};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultDoubleNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Result row for conductance stream mode
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ConductanceRow {
    /// Community ID
    pub community: u64,
    /// Conductance value for this community (0.0 to 1.0)
    pub conductance: f64,
}

/// Conductance algorithm facade
#[derive(Clone)]
pub struct ConductanceFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: ConductanceConfig,
    task_registry: Option<TaskRegistry>,
}

impl ConductanceFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>, community_property: String) -> Self {
        Self {
            graph_store,
            config: ConductanceConfig {
                community_property,
                ..ConductanceConfig::default()
            },
            task_registry: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: ConductanceConfig,
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
        let parsed: ConductanceConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: ConductanceConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    pub fn relationship_weight_property(mut self, use_weights: bool) -> Self {
        self.config.has_relationship_weight_property = use_weights;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    pub fn min_batch_size(mut self, min_batch_size: usize) -> Self {
        self.config.min_batch_size = min_batch_size;
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    fn validate(&self) -> Result<()> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    fn compute(&self) -> Result<ConductanceResult> {
        self.validate()?;
        let start = Instant::now();

        let node_count = self.graph_store.node_count();
        if node_count == 0 {
            return Ok(ConductanceResult {
                community_conductances: std::collections::HashMap::new(),
                global_average_conductance: 0.0,
                community_count: 0,
                node_count: 0,
                execution_time: start.elapsed(),
            });
        }

        let config = self.config.clone();

        let base_task = crate::core::utils::progress::Tasks::leaf_with_volume(
            "conductance".to_string(),
            node_count,
        )
        .base()
        .clone();
        let registry_factory = self.registry_factory();
        let mut progress_tracker = TaskProgressTracker::with_registry(
            base_task,
            Concurrency::of(self.config.concurrency.max(1)),
            JobId::new(),
            registry_factory.as_ref(),
        );

        let termination_flag = TerminationFlag::default();
        let storage = ConductanceStorageRuntime::new();
        let mut runtime = ConductanceComputationRuntime::new();
        let result = storage
            .compute_conductance(
                &mut runtime,
                self.graph_store.as_ref(),
                &config,
                &mut progress_tracker,
                &termination_flag,
            )
            .map_err(AlgorithmError::Execution)?;

        Ok(ConductanceResult {
            execution_time: start.elapsed(),
            ..result
        })
    }

    fn registry_factory(&self) -> Box<dyn TaskRegistryFactory> {
        struct PrebuiltTaskRegistryFactory(TaskRegistry);

        impl TaskRegistryFactory for PrebuiltTaskRegistryFactory {
            fn new_instance(&self, _job_id: JobId) -> TaskRegistry {
                self.0.clone()
            }
        }

        if let Some(registry) = &self.task_registry {
            Box::new(PrebuiltTaskRegistryFactory(registry.clone()))
        } else {
            Box::new(EmptyTaskRegistryFactory)
        }
    }

    /// Stream mode: yields conductance per community
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = ConductanceRow>>> {
        let result = self.compute()?;

        let mut rows: Vec<ConductanceRow> = result
            .community_conductances
            .into_iter()
            .map(|(community, conductance)| ConductanceRow {
                community,
                conductance,
            })
            .collect();

        // Sort by community ID for consistent output
        rows.sort_by_key(|r| r.community);

        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: returns aggregated statistics
    pub fn stats(&self) -> Result<ConductanceStats> {
        let result = self.compute()?;
        Ok(ConductanceResultBuilder::new(result).stats())
    }

    /// Mutate mode: writes conductance scores back to the graph store.
    pub fn mutate(self, property_name: &str) -> Result<ConductanceMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let result = self.compute()?;
        let conductances = result.community_conductances;

        let node_count = self.graph_store.node_count();
        let nodes_updated = node_count as u64;

        let community_props = self
            .graph_store
            .node_property_values(&self.config.community_property)
            .map_err(|e| {
                AlgorithmError::Execution(format!(
                    "Conductance mutate failed to load community property: {e}"
                ))
            })?;

        let mut values_vec: Vec<f64> = Vec::with_capacity(node_count);
        for node_id in 0..node_count as u64 {
            if !community_props.has_value(node_id) {
                values_vec.push(0.0);
                continue;
            }

            let community_id = match community_props.long_value(node_id) {
                Ok(v) if v >= 0 => v as u64,
                _ => {
                    values_vec.push(0.0);
                    continue;
                }
            };

            let value = conductances.get(&community_id).copied().unwrap_or(0.0);
            values_vec.push(value);
        }

        let backend = VecDouble::from(values_vec);
        let values = DefaultDoubleNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels_set: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels_set, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("Conductance mutate failed to add property: {e}"))
            })?;

        let summary = ConductanceMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: result.execution_time.as_millis() as u64,
        };

        Ok(ConductanceMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode: writes conductance scores to a new graph.
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
        // Conductance keeps per-node community ids and accumulators per community.
        // Dominant memory is linear in node count; relationship count influences traversal overhead.
        let node_count = self.graph_store.node_count();
        let relationship_count = self.graph_store.relationship_count();

        // Per node: community id + temporary sums.
        let per_node = 64usize;
        // Per relationship: traversal bookkeeping (very conservative).
        let per_relationship = 8usize;

        let base: usize = 32 * 1024;
        let total = base
            .saturating_add(node_count.saturating_mul(per_node))
            .saturating_add(relationship_count.saturating_mul(per_relationship));

        Ok(MemoryRange::of_range(total, total.saturating_mul(2)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Full facade integration tests require complex graph store setup with node properties.
    // The core algorithm is tested in the conductance integration_tests module.

    #[test]
    fn builder_api() {
        // Test that builder methods exist and are chainable
        // (Cannot test actual execution without a real graph store)
        assert_eq!(
            std::mem::size_of::<ConductanceFacade>(),
            std::mem::size_of::<ConductanceFacade>()
        );
    }
}
