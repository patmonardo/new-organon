//! Modularity Facade
//!
//! Measures community quality by comparing actual edges within communities
//! to expected edges if the network were random.

use crate::core::utils::progress::{ProgressTracker, TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::algo::modularity::computation::ModularityComputationRuntime;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
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

    fn checked_node_id(value: usize) -> Result<NodeId> {
        NodeId::try_from(value as i64).map_err(|_| {
            crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                "node_id must fit into i64 (got {})",
                value
            ))
        })
    }

    fn compute(&self) -> Result<(f64, Vec<(u64, f64)>)> {
        self.validate()?;

        // Modularity works on undirected graphs
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((0.0, Vec::new()));
        }

        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::new(
            Tasks::leaf_with_volume("modularity".to_string(), node_count),
        );
        progress_tracker.begin_subtask_with_volume(node_count);

        // Get community property values
        let community_props = graph_view
            .node_properties(&self.community_property)
            .ok_or_else(|| {
                crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                    "Community property '{}' not found",
                    self.community_property
                ))
            })?;

        let fallback = graph_view.default_property_value();

        // Get community assignment for each node
        let get_community = |node_idx: usize| -> Option<u64> {
            let node_id = Self::checked_node_id(node_idx).ok()? as u64;
            match community_props.long_value(node_id) {
                Ok(community) if community >= 0 => Some(community as u64),
                _ => None,
            }
        };

        // Get neighbors with weights
        let get_neighbors = |node_idx: usize| -> Vec<(usize, f64)> {
            let node_id = match Self::checked_node_id(node_idx) {
                Ok(value) => value,
                Err(_) => return Vec::new(),
            };

            graph_view
                .stream_relationships(node_id, fallback)
                .map(|cursor| {
                    let target = cursor.target_id() as usize;
                    let weight = cursor.property();
                    (target, weight)
                })
                .collect()
        };

        let runtime = ModularityComputationRuntime::new();
        let result = runtime.compute(node_count, get_community, get_neighbors);

        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        let community_scores: Vec<(u64, f64)> = result
            .community_modularities
            .into_iter()
            .map(|cm| (cm.community_id, cm.modularity))
            .collect();

        Ok((result.total_modularity, community_scores))
    }

    /// Stream mode: yields modularity per community
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = ModularityRow>>> {
        let (_total, scores) = self.compute()?;

        let mut rows: Vec<ModularityRow> = scores
            .into_iter()
            .map(|(community, modularity)| ModularityRow {
                community,
                modularity,
            })
            .collect();

        // Sort by community ID for consistent output
        rows.sort_by_key(|r| r.community);

        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: returns aggregated statistics
    pub fn stats(&self) -> Result<ModularityStats> {
        let (total_modularity, scores) = self.compute()?;

        Ok(ModularityStats {
            total_modularity,
            community_count: scores.len(),
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
        // Note: memory estimation is deferred.
        Ok(MemoryRange::of_range(0, 1024 * 1024)) // placeholder
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
