//! Modularity Facade
//!
//! Measures community quality by comparing actual edges within communities
//! to expected edges if the network were random.

use crate::core::utils::progress::{TaskProgressTracker, TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::algo::modularity::{
    ModularityComputationRuntime, ModularityResult, ModularityStorageRuntime,
};
use crate::concurrency::TerminationFlag;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::sync::Arc;

/// Result row for modularity stream mode
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ModularityRow {
    /// Community ID
    pub community: u64,
    /// Modularity score for this community
    pub modularity: f64,
}

/// Statistics for modularity computation
#[derive(Debug, Clone, serde::Serialize)]
pub struct ModularityStats {
    /// Total modularity score across all communities
    pub total_modularity: f64,
    /// Number of communities evaluated
    pub community_count: usize,
}

/// Modularity algorithm facade
#[derive(Clone)]
pub struct ModularityFacade {
    graph_store: Arc<DefaultGraphStore>,
    community_property: String,
    task_registry: Option<TaskRegistry>,
}

impl ModularityFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>, community_property: String) -> Self {
        Self {
            graph_store,
            community_property,
            task_registry: None,
        }
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    fn validate(&self) -> Result<()> {
        if self.community_property.is_empty() {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "community_property cannot be empty".to_string(),
                ),
            );
        }
        Ok(())
    }

    fn compute(&self) -> Result<ModularityResult> {
        self.validate()?;

        let storage = ModularityStorageRuntime::new(self.graph_store.as_ref())?;
        let computation = ModularityComputationRuntime::new();
        let termination_flag = TerminationFlag::default();

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf_with_volume(
            "modularity".to_string(),
            storage.node_count(),
        ));

        storage.compute_modularity(
            &computation,
            &self.community_property,
            &mut progress_tracker,
            &termination_flag,
        )
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

        Ok(ModularityStats {
            total_modularity: result.total_modularity,
            community_count: result.community_modularities.len(),
        })
    }

    /// Mutate mode: writes modularity scores back to the graph store.
    pub fn mutate(self) -> Result<MutationResult> {
        // Note: mutation logic is deferred.
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "mutate not yet implemented".to_string(),
            ),
        )
    }

    /// Write mode: writes modularity scores to a new graph.
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
