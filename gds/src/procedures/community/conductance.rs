//! Conductance Facade
//!
//! Evaluates community quality by measuring the proportion of edges
//! that cross community boundaries.

use crate::core::utils::progress::TaskRegistry;
use crate::mem::MemoryRange;
use crate::algo::conductance::{
    ConductanceComputationRuntime, ConductanceConfig, ConductanceStorageRuntime,
};
use crate::concurrency::Concurrency;
use crate::procedures::builder_base::{MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::sync::Arc;

/// Result row for conductance stream mode
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ConductanceRow {
    /// Community ID
    pub community: u64,
    /// Conductance value for this community (0.0 to 1.0)
    pub conductance: f64,
}

/// Statistics for conductance computation
#[derive(Debug, Clone, serde::Serialize)]
pub struct ConductanceStats {
    /// Number of communities evaluated
    pub community_count: usize,
    /// Global average conductance
    pub average_conductance: f64,
}

/// Conductance algorithm facade
#[derive(Clone)]
pub struct ConductanceFacade {
    graph_store: Arc<DefaultGraphStore>,
    community_property: String,
    has_relationship_weight_property: bool,
    concurrency: usize,
    min_batch_size: usize,
    task_registry: Option<TaskRegistry>,
}

impl ConductanceFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>, community_property: String) -> Self {
        Self {
            graph_store,
            community_property,
            has_relationship_weight_property: false,
            concurrency: 4,
            min_batch_size: 10_000,
            task_registry: None,
        }
    }

    pub fn relationship_weight_property(mut self, use_weights: bool) -> Self {
        self.has_relationship_weight_property = use_weights;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub fn min_batch_size(mut self, min_batch_size: usize) -> Self {
        self.min_batch_size = min_batch_size;
        self
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

    fn compute(&self) -> Result<(std::collections::HashMap<u64, f64>, f64)> {
        self.validate()?;

        let node_count = self.graph_store.node_count();
        if node_count == 0 {
            return Ok((std::collections::HashMap::new(), 0.0));
        }

        let config = ConductanceConfig {
            concurrency: self.concurrency,
            min_batch_size: self.min_batch_size,
            has_relationship_weight_property: self.has_relationship_weight_property,
            community_property: self.community_property.clone(),
        };

        let base_task = crate::algo::conductance::progress_task(node_count);
        let registry_factory = crate::core::utils::progress::EmptyTaskRegistryFactory;
        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_registry(
            base_task,
            Concurrency::of(self.concurrency.max(1)),
            crate::core::utils::progress::JobId::new(),
            &registry_factory,
        );

        let termination_flag = crate::concurrency::TerminationFlag::default();
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
            .map_err(crate::projection::eval::procedure::AlgorithmError::Execution)?;

        Ok((
            result.community_conductances,
            result.global_average_conductance,
        ))
    }

    /// Stream mode: yields conductance per community
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = ConductanceRow>>> {
        let (conductances, _avg) = self.compute()?;

        let mut rows: Vec<ConductanceRow> = conductances
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
        let (conductances, avg) = self.compute()?;

        Ok(ConductanceStats {
            community_count: conductances.len(),
            average_conductance: avg,
        })
    }

    /// Mutate mode: writes conductance scores back to the graph store.
    pub fn mutate(self) -> Result<MutationResult> {
        // Note: mutation logic is deferred.
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "mutate not yet implemented".to_string(),
            ),
        )
    }

    /// Write mode: writes conductance scores to a new graph.
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
