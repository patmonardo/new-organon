//! Bellman-Ford Facade
//!
//! Single-source shortest paths with negative-cycle detection.
//!
//! This facade runs the translated Bellman-Ford runtime against a live
//! `DefaultGraphStore` (no dummy outputs).

use crate::procedures::bellman_ford::{BellmanFordComputationRuntime, BellmanFordStorageRuntime};
use crate::procedures::facades::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::facades::traits::{PathResult, Result};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Statistics about Bellman-Ford execution
#[derive(Debug, Clone)]
pub struct BellmanFordStats {
    pub paths_found: u64,
    pub negative_cycles_found: u64,
    pub contains_negative_cycle: bool,
    pub execution_time_ms: u64,
}

/// Bellman-Ford algorithm builder - fluent configuration
pub struct BellmanFordBuilder {
    graph_store: Arc<DefaultGraphStore>,
    source: Option<u64>,
    weight_property: String,
    relationship_types: Vec<String>,
    direction: String,
    track_negative_cycles: bool,
    track_paths: bool,
    concurrency: usize,
}

impl BellmanFordBuilder {
    /// Create a new Bellman-Ford builder bound to a live graph store.
    ///
    /// Defaults:
    /// - source: None (must be set)
    /// - weight_property: "weight"
    /// - relationship_types: all types
    /// - direction: "outgoing"
    /// - track_negative_cycles: true
    /// - track_paths: true
    /// - concurrency: 4
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            source: None,
            weight_property: "weight".to_string(),
            relationship_types: vec![],
            direction: "outgoing".to_string(),
            track_negative_cycles: true,
            track_paths: true,
            concurrency: 4,
        }
    }

    pub fn source(mut self, source: u64) -> Self {
        self.source = Some(source);
        self
    }

    pub fn weight_property(mut self, property: &str) -> Self {
        self.weight_property = property.to_string();
        self
    }

    pub fn relationship_types(mut self, relationship_types: Vec<String>) -> Self {
        self.relationship_types = relationship_types;
        self
    }

    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = direction.to_string();
        self
    }

    pub fn track_negative_cycles(mut self, enabled: bool) -> Self {
        self.track_negative_cycles = enabled;
        self
    }

    pub fn track_paths(mut self, enabled: bool) -> Self {
        self.track_paths = enabled;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    fn validate(&self) -> Result<()> {
        if self.source.is_none() {
            return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                "source node must be specified".to_string(),
            ));
        }

        if self.concurrency == 0 {
            return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                "concurrency must be > 0".to_string(),
            ));
        }

        match self.direction.to_ascii_lowercase().as_str() {
            "outgoing" | "incoming" => {}
            other => {
                return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                    format!("direction must be 'outgoing' or 'incoming' (got '{other}')"),
                ));
            }
        }

        ConfigValidator::non_empty_string(&self.weight_property, "weight_property")?;

        Ok(())
    }

    fn checked_node_id(value: u64, field: &str) -> Result<NodeId> {
        NodeId::try_from(value).map_err(|_| {
            crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                "{field} must fit into i64 (got {value})",
            ))
        })
    }

    fn checked_u64(value: NodeId, context: &str) -> Result<u64> {
        u64::try_from(value).map_err(|_| {
            crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                "Bellman-Ford returned invalid node id for {context}: {value}",
            ))
        })
    }

    fn compute(self) -> Result<(Vec<PathResult>, BellmanFordStats)> {
        self.validate()?;

        let source_u64 = self.source.expect("validate ensures source is set");
        let source_node = Self::checked_node_id(source_u64, "source")?;

        let rel_types: HashSet<RelationshipType> = if self.relationship_types.is_empty() {
            self.graph_store.relationship_types()
        } else {
            RelationshipType::list_of(self.relationship_types.clone())
                .into_iter()
                .collect()
        };

        let (orientation, direction_byte) = match self.direction.to_ascii_lowercase().as_str() {
            "incoming" => (Orientation::Reverse, 1u8),
            _ => (Orientation::Natural, 0u8),
        };

        let selectors: HashMap<RelationshipType, String> = rel_types
            .iter()
            .map(|t| (t.clone(), self.weight_property.clone()))
            .collect();

        let graph_view = self
            .graph_store
            .get_graph_with_types_selectors_and_orientation(&rel_types, &selectors, orientation)
            .map_err(|e| crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string()))?;

        let mut storage = BellmanFordStorageRuntime::new(
            source_node,
            self.track_negative_cycles,
            self.track_paths,
            self.concurrency,
        );

        let mut computation = BellmanFordComputationRuntime::new(
            source_node,
            self.track_negative_cycles,
            self.track_paths,
            self.concurrency,
        );

        let start = std::time::Instant::now();
        let result = storage.compute_bellman_ford(
            &mut computation,
            Some(graph_view.as_ref()),
            direction_byte,
        )?;

        let mut rows: Vec<PathResult> = Vec::new();
        for p in result.shortest_paths.iter() {
            let source = Self::checked_u64(p.source_node, "source")?;
            let target = Self::checked_u64(p.target_node, "target")?;
            let path = p
                .node_ids
                .iter()
                .copied()
                .map(|node_id| Self::checked_u64(node_id, "path") )
                .collect::<Result<Vec<_>>>()?;

            rows.push(PathResult {
                source,
                target,
                path,
                cost: p.total_cost,
            });
        }

        let stats = BellmanFordStats {
            paths_found: rows.len() as u64,
            negative_cycles_found: result.negative_cycles.len() as u64,
            contains_negative_cycle: result.contains_negative_cycle,
            execution_time_ms: start.elapsed().as_millis() as u64,
        };

        Ok((rows, stats))
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PathResult>>> {
        let (rows, _) = self.compute()?;
        Ok(Box::new(rows.into_iter()))
    }

    pub fn stats(self) -> Result<BellmanFordStats> {
        let (_, stats) = self.compute()?;
        Ok(stats)
    }

    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(crate::projection::eval::procedure::AlgorithmError::Execution(
            "Bellman-Ford mutate/write is not implemented yet".to_string(),
        ))
    }

    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(crate::projection::eval::procedure::AlgorithmError::Execution(
            "Bellman-Ford mutate/write is not implemented yet".to_string(),
        ))
    }
}
