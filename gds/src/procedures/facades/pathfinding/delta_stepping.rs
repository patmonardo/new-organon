//! Delta Stepping Facade
//!
//! Single-source shortest paths using the Delta Stepping binning strategy.
//!
//! This facade runs the translated Delta Stepping runtime against a live
//! `DefaultGraphStore`.

use crate::procedures::delta_stepping::{DeltaSteppingComputationRuntime, DeltaSteppingStorageRuntime};
use crate::procedures::facades::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::facades::traits::{PathResult, Result};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Statistics about Delta Stepping execution
#[derive(Debug, Clone)]
pub struct DeltaSteppingStats {
    pub paths_found: u64,
    pub computation_time_ms: u64,
    pub execution_time_ms: u64,
}

/// Delta Stepping algorithm builder - fluent configuration
pub struct DeltaSteppingBuilder {
    graph_store: Arc<DefaultGraphStore>,
    source: Option<u64>,
    delta: f64,
    weight_property: String,
    relationship_types: Vec<String>,
    direction: String,
    store_predecessors: bool,
    concurrency: usize,
}

impl DeltaSteppingBuilder {
    /// Create a new Delta Stepping builder bound to a live graph store.
    ///
    /// Defaults:
    /// - source: None (must be set)
    /// - delta: 1.0
    /// - weight_property: "weight"
    /// - relationship_types: all types
    /// - direction: "outgoing"
    /// - store_predecessors: true
    /// - concurrency: 4
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            source: None,
            delta: 1.0,
            weight_property: "weight".to_string(),
            relationship_types: vec![],
            direction: "outgoing".to_string(),
            store_predecessors: true,
            concurrency: 4,
        }
    }

    pub fn source(mut self, source: u64) -> Self {
        self.source = Some(source);
        self
    }

    pub fn delta(mut self, delta: f64) -> Self {
        self.delta = delta;
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

    pub fn store_predecessors(mut self, enabled: bool) -> Self {
        self.store_predecessors = enabled;
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

        if self.delta <= 0.0 {
            return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                "delta must be > 0".to_string(),
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
                "Delta Stepping returned invalid node id for {context}: {value}",
            ))
        })
    }

    fn compute(self) -> Result<(Vec<PathResult>, DeltaSteppingStats)> {
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

        let mut storage = DeltaSteppingStorageRuntime::new(
            source_node,
            self.delta,
            self.concurrency,
            self.store_predecessors,
        );

        let mut computation = DeltaSteppingComputationRuntime::new(
            source_node,
            self.delta,
            self.concurrency,
            self.store_predecessors,
        );

        let start = std::time::Instant::now();
        let result = storage.compute_delta_stepping(
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
                .map(|node_id| Self::checked_u64(node_id, "path"))
                .collect::<Result<Vec<_>>>()?;

            rows.push(PathResult {
                source,
                target,
                path,
                cost: p.total_cost(),
            });
        }

        let stats = DeltaSteppingStats {
            paths_found: rows.len() as u64,
            computation_time_ms: result.computation_time_ms,
            execution_time_ms: start.elapsed().as_millis() as u64,
        };

        Ok((rows, stats))
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PathResult>>> {
        let (rows, _) = self.compute()?;
        Ok(Box::new(rows.into_iter()))
    }

    pub fn stats(self) -> Result<DeltaSteppingStats> {
        let (_, stats) = self.compute()?;
        Ok(stats)
    }

    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(crate::projection::eval::procedure::AlgorithmError::Execution(
            "Delta Stepping mutate/write is not implemented yet".to_string(),
        ))
    }

    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(crate::projection::eval::procedure::AlgorithmError::Execution(
            "Delta Stepping mutate/write is not implemented yet".to_string(),
        ))
    }
}
