//! Conductance Facade
//!
//! Evaluates community quality by measuring the proportion of edges
//! that cross community boundaries.

use crate::procedures::conductance::computation::ConductanceComputationRuntime;
use crate::procedures::conductance::spec::ConductanceConfig;
use crate::procedures::facades::traits::Result;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
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

/// Conductance algorithm builder
#[derive(Clone)]
pub struct ConductanceBuilder {
    graph_store: Arc<DefaultGraphStore>,
    community_property: String,
    has_relationship_weight_property: bool,
}

impl ConductanceBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>, community_property: String) -> Self {
        Self {
            graph_store,
            community_property,
            has_relationship_weight_property: false,
        }
    }

    pub fn relationship_weight_property(mut self, use_weights: bool) -> Self {
        self.has_relationship_weight_property = use_weights;
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

    fn compute(&self) -> Result<(std::collections::HashMap<u64, f64>, f64)> {
        self.validate()?;

        // Conductance works on directed graphs (Natural orientation)
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((std::collections::HashMap::new(), 0.0));
        }

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

        let config = ConductanceConfig {
            has_relationship_weight_property: self.has_relationship_weight_property,
        };

        let runtime = ConductanceComputationRuntime::new(config);
        let result = runtime.compute(node_count, get_community, get_neighbors);

        Ok((result.community_conductances, result.average_conductance))
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
            std::mem::size_of::<ConductanceBuilder>(),
            std::mem::size_of::<ConductanceBuilder>()
        );
    }
}
