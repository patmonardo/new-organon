//! ScaleProperties procedure component.
//!
//! Wires the algorithm storage/computation runtimes (multi-property, configurable scaler)
//! and exposes stream/stats surfaces. Mutate/write are intentionally unimplemented.

use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::scale_properties::{
    ScalePropertiesComputationRuntime, ScalePropertiesConfig, ScalePropertiesMutationSummary,
    ScalePropertiesResult, ScalePropertiesScaler, ScalePropertiesStats,
    ScalePropertiesStorageRuntime, ScalePropertiesStreamRow, ScalePropertiesWriteSummary,
};
use crate::collections::backends::vec::VecDoubleArray;
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::NodeLabel;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultDoubleArrayNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use std::collections::HashSet;
use std::sync::Arc;
#[derive(Debug, Clone)]
pub struct ScalePropertiesMutateResult {
    pub summary: ScalePropertiesMutationSummary,
    pub updated_store: Arc<DefaultGraphStore>,
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

    fn compute(&self) -> Result<ScalePropertiesResult> {
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

    pub fn mutate(&self, property_name: &str) -> Result<ScalePropertiesMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let start_time = std::time::Instant::now();
        let result = self.compute()?;

        let node_count = result.scaled_properties.len();
        let mut data: Vec<Option<Vec<f64>>> = Vec::with_capacity(node_count);
        for row in result.scaled_properties {
            data.push(Some(row));
        }

        let backend = VecDoubleArray::from(data);
        let values = DefaultDoubleArrayNodePropertyValues::<VecDoubleArray>::from_collection(
            backend, node_count,
        );
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!(
                    "scaleProperties mutate failed to add property: {e}"
                ))
            })?;

        let nodes_updated = node_count as u64;
        let summary = ScalePropertiesMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: start_time.elapsed().as_millis() as u64,
        };
        Ok(ScalePropertiesMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    pub fn write(&self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let start_time = std::time::Instant::now();
        let result = self.compute()?;
        let nodes_written = result.scaled_properties.len() as u64;

        let summary = ScalePropertiesWriteSummary {
            nodes_written,
            property_name: property_name.to_string(),
            execution_time_ms: start_time.elapsed().as_millis() as u64,
        };
        Ok(WriteResult::new(
            summary.nodes_written,
            summary.property_name,
            std::time::Duration::from_millis(summary.execution_time_ms),
        ))
    }
}
