//! ScaleProperties computation runtime.
//!
//! Owns the scaling loop and statistics aggregation. Storage prepares the
//! plan (validated property scalers) and delegates execution here.

use crate::algo::algorithms::scaling::Scaler;
use crate::algo::scale_properties::spec::ScalePropertiesResult;
use std::collections::HashMap;
use std::sync::Arc;

pub type PropertyFn = Arc<dyn Fn(u64) -> f64 + Send + Sync>;

pub struct ElementScaler {
    pub(crate) property_fn: PropertyFn,
    pub(crate) scaler: Box<dyn Scaler>,
}

impl ElementScaler {
    pub(crate) fn new(property_fn: PropertyFn, scaler: Box<dyn Scaler>) -> Self {
        Self {
            property_fn,
            scaler,
        }
    }
}

pub struct PropertyScaler {
    pub(crate) name: String,
    pub(crate) elements: Vec<ElementScaler>,
}

impl PropertyScaler {
    pub fn new(name: String, elements: Vec<ElementScaler>) -> Self {
        Self { name, elements }
    }

    pub fn dimension(&self) -> usize {
        self.elements.len()
    }

    pub fn scale_into(&self, node_id: u64, target: &mut [f64], offset: usize) {
        for (idx, element) in self.elements.iter().enumerate() {
            target[offset + idx] = element
                .scaler
                .scale_property(node_id, element.property_fn.as_ref());
        }
    }

    pub fn statistics(&self) -> HashMap<String, Vec<f64>> {
        let mut stats: HashMap<String, Vec<f64>> = HashMap::new();
        for element in &self.elements {
            for (name, values) in element.scaler.statistics().as_map() {
                stats
                    .entry(name.clone())
                    .or_insert_with(Vec::new)
                    .extend(values.iter().copied());
            }
        }
        stats
    }
}

pub struct ScalePropertiesPlan {
    pub node_count: usize,
    pub property_scalers: Vec<PropertyScaler>,
}

#[derive(Default, Clone)]
pub struct ScalePropertiesComputationRuntime;

impl ScalePropertiesComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn compute(&self, plan: ScalePropertiesPlan) -> ScalePropertiesResult {
        let total_dimension: usize = plan.property_scalers.iter().map(|p| p.dimension()).sum();

        let mut scaled_properties = vec![vec![0.0; total_dimension]; plan.node_count];
        let mut scaler_statistics = HashMap::new();

        let mut offset = 0;
        for property_scaler in &plan.property_scalers {
            for (node_idx, row) in scaled_properties.iter_mut().enumerate() {
                property_scaler.scale_into(node_idx as u64, row, offset);
            }
            offset += property_scaler.dimension();

            scaler_statistics.insert(property_scaler.name.clone(), property_scaler.statistics());
        }

        ScalePropertiesResult {
            scaled_properties,
            scaler_statistics,
        }
    }
}
