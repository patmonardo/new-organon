//! ScaleProperties procedure component.
//!
//! Wires the algorithm storage/computation runtimes (multi-property, configurable scaler)
//! and exposes stream/stats surfaces. Mutate/write are intentionally unimplemented.

use crate::algo::scale_properties::{
    ScalePropertiesComputationRuntime, ScalePropertiesConfig, ScalePropertiesScaler,
    ScalePropertiesStorageRuntime,
};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::projection::eval::procedure::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
pub struct ScalePropertiesStreamRow {
    pub node_id: u64,
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScalePropertiesStats {
    pub scaler: String,
    pub stats: HashMap<String, HashMap<String, Vec<f64>>>,
}

/// ScaleProperties procedure facade (multi-property, configurable scaler).
pub struct ScalePropertiesFacade {
    graph_store: Arc<DefaultGraphStore>,
    node_properties: Vec<String>,
    scaler: ScalePropertiesScaler,
    concurrency: usize,
}

impl ScalePropertiesFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            node_properties: Vec::new(),
            scaler: ScalePropertiesScaler::MinMax,
            concurrency: 4,
        }
    }

    /// Set node properties to scale. Array properties will be flattened.
    pub fn node_properties(mut self, props: Vec<String>) -> Self {
        self.node_properties = props;
        self
    }

    /// Select scaler variant.
    pub fn scaler(mut self, scaler: ScalePropertiesScaler) -> Self {
        self.scaler = scaler;
        self
    }

    /// Concurrency hint for stats/scaling.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    fn validate(&self) -> Result<()> {
        if self.node_properties.is_empty() {
            return Err(AlgorithmError::Execution(
                "node_properties must not be empty".to_string(),
            ));
        }
        ConfigValidator::in_range(self.concurrency as f64, 1.0, 1_000_000.0, "concurrency")?;
        Ok(())
    }

    fn compute(&self) -> Result<crate::algo::scale_properties::ScalePropertiesResult> {
        self.validate()?;

        let config = ScalePropertiesConfig {
            node_properties: self.node_properties.clone(),
            scaler: self.scaler.clone(),
            concurrency: self.concurrency,
        };

        let mut computation = ScalePropertiesComputationRuntime::new();
        let storage = ScalePropertiesStorageRuntime::new();
        storage.compute(self.graph_store.as_ref(), &config, &mut computation)
    }

    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = ScalePropertiesStreamRow>>> {
        let result = self.compute()?;
        let iter = result
            .scaled_properties
            .into_iter()
            .enumerate()
            .map(|(node_id, values)| ScalePropertiesStreamRow {
                node_id: node_id as u64,
                values,
            });
        Ok(Box::new(iter))
    }

    pub fn stats(&self) -> Result<ScalePropertiesStats> {
        let result = self.compute()?;
        Ok(ScalePropertiesStats {
            scaler: format!("{:?}", self.scaler),
            stats: result.scaler_statistics,
        })
    }

    pub fn estimate_memory(&self) -> MemoryRange {
        // Rough estimate: scaled values + stats overhead + concurrency cushion.
        let node_count = GraphStore::node_count(self.graph_store.as_ref());
        let approx_dimension = 128usize.max(self.node_properties.len());
        let scaled = node_count * approx_dimension * std::mem::size_of::<f64>();
        let stats_overhead = 128 * 1024;
        let concurrency_overhead = self.concurrency * 8 * 1024;
        let total = scaled + stats_overhead + concurrency_overhead;
        MemoryRange::of_range(total, total + total / 4)
    }

    pub fn mutate(&self, _property: &str) -> Result<MutationResult> {
        Err(AlgorithmError::Execution(
            "scaleProperties mutate is not implemented yet".to_string(),
        ))
    }

    pub fn write(&self, _property: &str) -> Result<WriteResult> {
        Err(AlgorithmError::Execution(
            "scaleProperties write is not implemented yet".to_string(),
        ))
    }
}
