//! ScaleProperties procedure facade.
//!
//! This is a lightweight wrapper over the core scaler implementations under
//! `crate::algo::core::scaling`.

use crate::algo::core::scaling::{MinMaxScaler, Scaler};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::ValueType;
use std::sync::Arc;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ScalePropertiesStreamRow {
    pub node_id: u64,
    pub value: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ScalePropertiesStats {
    pub scaler: String,
    pub stats: std::collections::HashMap<String, Vec<f64>>,
}

/// Scales a numeric node property.
///
/// Currently implements min-max scaling.
pub struct ScalePropertiesFacade {
    graph_store: Arc<DefaultGraphStore>,
    source_property: String,
    concurrency: usize,
}

impl ScalePropertiesFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            source_property: String::new(),
            concurrency: 1,
        }
    }

    pub fn source_property(mut self, key: impl Into<String>) -> Self {
        self.source_property = key.into();
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::non_empty_string(&self.source_property, "source_property")?;
        if self.concurrency == 0 {
            return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                "concurrency must be > 0".to_string(),
            ));
        }
        Ok(())
    }

    fn property_fn(&self) -> Result<Box<dyn Fn(u64) -> f64 + Send + Sync>> {
        self.validate()?;

        let pv = GraphStore::node_property_values(self.graph_store.as_ref(), &self.source_property)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                    "missing node property '{}': {e}",
                    self.source_property
                ))
            })?;

        let value_type = pv.value_type();
        match value_type {
            ValueType::Long => Ok(Box::new(move |node_id: u64| {
                pv.long_value(node_id).unwrap_or(0) as f64
            })),
            ValueType::Double => Ok(Box::new(move |node_id: u64| {
                pv.double_value(node_id).unwrap_or(0.0)
            })),
            other => Err(crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                "scaleProperties expects Long/Double node property (got {other:?})",
            ))),
        }
    }

    fn compute(self) -> Result<(Vec<f64>, crate::algo::core::scaling::ScalerStatistics)> {
        let node_count = GraphStore::node_count(self.graph_store.as_ref()) as u64;
        let property_fn = self.property_fn()?;

        let scaler: Box<dyn Scaler> =
            MinMaxScaler::create(node_count, &property_fn, self.concurrency);
        let stats = scaler.statistics().clone();

        let mut values = Vec::with_capacity(node_count as usize);
        for node_id in 0..node_count {
            values.push(scaler.scale_property(node_id, property_fn.as_ref()));
        }
        Ok((values, stats))
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = ScalePropertiesStreamRow>>> {
        let (values, _stats) = self.compute()?;
        let iter = values
            .into_iter()
            .enumerate()
            .map(|(node_id, value)| ScalePropertiesStreamRow {
                node_id: node_id as u64,
                value,
            });
        Ok(Box::new(iter))
    }

    pub fn stats(self) -> Result<ScalePropertiesStats> {
        let (_values, stats) = self.compute()?;
        Ok(ScalePropertiesStats {
            scaler: "minMax".to_string(),
            stats: stats.as_map().clone(),
        })
    }

    /// Memory range estimate (min/max bytes).
    ///
    /// This is a conservative heuristic based on the dominant allocations:
    /// - scaled output values (one `f64` per node)
    /// - temporary vector for streaming/stats (one `f64` per node)
    /// - small fixed overhead for stats and per-worker buffers
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        let scaled_values = node_count * std::mem::size_of::<f64>();
        let scratch_values = node_count * std::mem::size_of::<f64>();

        let stats_overhead = 64 * 1024;
        let concurrency_overhead = self.concurrency * 8 * 1024;

        let total = scaled_values + scratch_values + stats_overhead + concurrency_overhead;
        let total_with_overhead = total + (total / 5);
        MemoryRange::of_range(total, total_with_overhead)
    }

    pub fn mutate(self, _target_property: &str) -> Result<MutationResult> {
        Err(crate::projection::eval::procedure::AlgorithmError::Execution(
            "scaleProperties mutate is not implemented yet".to_string(),
        ))
    }

    pub fn write(self, _target_property: &str) -> Result<WriteResult> {
        Err(crate::projection::eval::procedure::AlgorithmError::Execution(
            "scaleProperties write is not implemented yet".to_string(),
        ))
    }
}
