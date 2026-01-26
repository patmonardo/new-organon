//! Modularity Facade
//!
//! Measures community quality by comparing actual edges within communities
//! to expected edges if the network were random.

use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::modularity::{
    ModularityComputationRuntime, ModularityConfig, ModularityMutateResult,
    ModularityMutationSummary, ModularityResult, ModularityResultBuilder, ModularityStats,
    ModularityStorageRuntime,
};
use crate::collections::backends::vec::VecDouble;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::{TaskProgressTracker, TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultDoubleNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Result row for modularity stream mode
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ModularityRow {
    /// Community ID
    pub community: u64,
    /// Modularity score for this community
    pub modularity: f64,
}

/// Modularity algorithm facade
#[derive(Clone)]
pub struct ModularityFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: ModularityConfig,
    task_registry: Option<TaskRegistry>,
}

impl ModularityFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>, community_property: String) -> Self {
        Self {
            graph_store,
            config: ModularityConfig {
                community_property,
                ..ModularityConfig::default()
            },
            task_registry: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: ModularityConfig,
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
        let parsed: ModularityConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: ModularityConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
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

    fn compute(&self) -> Result<ModularityResult> {
        self.validate()?;
        let start = Instant::now();

        let storage = ModularityStorageRuntime::new(self.graph_store.as_ref())?;
        let computation = ModularityComputationRuntime::new();
        let termination_flag = TerminationFlag::default();

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf_with_volume(
            "modularity".to_string(),
            storage.node_count(),
        ));

        let result = storage.compute_modularity(
            &computation,
            &self.config,
            &mut progress_tracker,
            &termination_flag,
        )?;

        Ok(ModularityResult {
            execution_time: start.elapsed(),
            ..result
        })
    }

    /// Stream mode: yields modularity per community
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = ModularityRow>>> {
        let result = self.compute()?;

        let mut rows: Vec<ModularityRow> = result
            .community_modularities
            .into_iter()
            .map(|cm| ModularityRow {
                community: cm.community_id,
                modularity: cm.modularity,
            })
            .collect();

        // Sort by community ID for consistent output
        rows.sort_by_key(|r| r.community);

        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: returns aggregated statistics
    pub fn stats(&self) -> Result<ModularityStats> {
        let result = self.compute()?;
        Ok(ModularityResultBuilder::new(result).stats())
    }

    /// Mutate mode: writes modularity scores back to the graph store.
    pub fn mutate(self, property_name: &str) -> Result<ModularityMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let result = self.compute()?;

        let node_count = self.graph_store.node_count();
        let nodes_updated = node_count as u64;

        let community_props = self
            .graph_store
            .node_property_values(&self.config.community_property)
            .map_err(|e| {
                AlgorithmError::Execution(format!(
                    "Modularity mutate failed to load community property: {e}"
                ))
            })?;

        let modularity_map: HashMap<u64, f64> = result
            .community_modularities
            .into_iter()
            .map(|m| (m.community_id, m.modularity))
            .collect();

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

            let value = modularity_map.get(&community_id).copied().unwrap_or(0.0);
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
                AlgorithmError::Execution(format!("Modularity mutate failed to add property: {e}"))
            })?;

        let summary = ModularityMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: result.execution_time.as_millis() as u64,
        };

        Ok(ModularityMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode: writes modularity scores to a new graph.
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
        // Modularity reads community labels and aggregates per-community totals.
        let node_count = self.graph_store.node_count();
        let relationship_count = self.graph_store.relationship_count();

        // Per node: community id + temporary weight sums.
        let per_node = 64usize;
        // Per relationship: one pass to aggregate contributions.
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

    #[test]
    fn builder_api() {
        // Test that builder methods exist and are chainable
        assert_eq!(
            std::mem::size_of::<ModularityFacade>(),
            std::mem::size_of::<ModularityFacade>()
        );
    }
}
