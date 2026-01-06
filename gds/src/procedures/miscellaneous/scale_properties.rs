//! ScaleProperties procedure facade.
//!
//! Provides stream/stats/min-max scaling helpers mirroring the Java
//! miscellaneous facade surface. This intentionally keeps the logic simple
//! and avoids optimizations; the applications layer already wires modes.

use crate::algo::common::scaling::MinMaxScaler;
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::projection::eval::procedure::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::NodePropertyValues;
use crate::types::ValueType;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
pub struct ScalePropertiesStreamRow {
	pub node_id: u64,
	pub value: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScalePropertiesStats {
	pub scaler: String,
	pub stats: HashMap<String, Vec<f64>>,
}

/// Min-max scaling facade (single property, Java-compatible surface).
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
			return Err(AlgorithmError::Execution(
				"concurrency must be > 0".to_string(),
			));
		}
		Ok(())
	}

	fn property_values(&self) -> Result<Arc<dyn NodePropertyValues>> {
		self.validate()?;

		let pv = GraphStore::node_property_values(self.graph_store.as_ref(), &self.source_property)
			.map_err(|e| {
				AlgorithmError::Execution(format!(
					"missing node property '{}': {e}",
					self.source_property
				))
			})?;

		match pv.value_type() {
			ValueType::Long | ValueType::Double => Ok(pv),
			other => Err(AlgorithmError::Execution(format!(
				"scaleProperties expects Long/Double node property (got {other:?})",
			))),
		}
	}

	fn min_max(&self, pv: &Arc<dyn NodePropertyValues>) -> Result<(f64, f64)> {
		let node_count = GraphStore::node_count(self.graph_store.as_ref()) as u64;
		let mut min = f64::MAX;
		let mut max = f64::MIN;

		for node_id in 0..node_count {
			let value = match pv.value_type() {
				ValueType::Long => pv.long_value(node_id).unwrap_or(0) as f64,
				ValueType::Double => pv.double_value(node_id).unwrap_or(0.0),
				_ => 0.0,
			};
			if value < min {
				min = value;
			}
			if value > max {
				max = value;
			}
		}

		if min == f64::MAX || max == f64::MIN {
			return Err(AlgorithmError::Execution(
				"no values found while scaling".to_string(),
			));
		}

		Ok((min, max))
	}

	pub fn stream(&self) -> Result<Box<dyn Iterator<Item = ScalePropertiesStreamRow>>> {
		let pv = self.property_values()?;
		let node_count = GraphStore::node_count(self.graph_store.as_ref()) as u64;
		let (min, max) = self.min_max(&pv)?;
		let range = (max - min).abs();

		let rows: Vec<ScalePropertiesStreamRow> = if range == 0.0 {
			(0..node_count)
				.map(|node_id| ScalePropertiesStreamRow {
					node_id,
					value: 0.0,
				})
				.collect()
		} else {
			let property_fn: Box<dyn Fn(u64) -> f64 + Send + Sync> = match pv.value_type() {
				ValueType::Long => Box::new(move |node_id: u64| pv.long_value(node_id).unwrap_or(0) as f64),
				_ => Box::new(move |node_id: u64| pv.double_value(node_id).unwrap_or(0.0)),
			};

			let scaler: Box<dyn crate::algo::common::scaling::Scaler> =
				MinMaxScaler::create(node_count, &property_fn, self.concurrency);

			(0..node_count)
				.map(|node_id| ScalePropertiesStreamRow {
					node_id,
					value: scaler.scale_property(node_id, property_fn.as_ref()),
				})
				.collect()
		};

		Ok(Box::new(rows.into_iter()))
	}

	pub fn stats(&self) -> Result<ScalePropertiesStats> {
		let pv = self.property_values()?;
		let (min, max) = self.min_max(&pv)?;
		let mut stats = HashMap::new();
		stats.insert("min".to_string(), vec![min]);
		stats.insert("max".to_string(), vec![max]);
		stats.insert("maxMinDiff".to_string(), vec![max - min]);

		Ok(ScalePropertiesStats {
			scaler: "minMax".to_string(),
			stats,
		})
	}

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

