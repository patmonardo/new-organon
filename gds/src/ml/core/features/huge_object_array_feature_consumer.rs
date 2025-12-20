//! Huge Object Array feature consumer for ML in GDS.
//!
//! Translated from Java GDS ml-core HugeObjectArrayFeatureConsumer.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::FeatureConsumer;
use crate::collections::HugeObjectArray;

/// Feature consumer backed by a HugeObjectArray of feature vectors.
pub struct HugeObjectArrayFeatureConsumer {
    features: HugeObjectArray<Vec<f64>>,
}

impl HugeObjectArrayFeatureConsumer {
    pub fn new(features: HugeObjectArray<Vec<f64>>) -> Self {
        Self { features }
    }

    pub fn features_mut(&mut self) -> &mut HugeObjectArray<Vec<f64>> {
        &mut self.features
    }

    pub fn into_inner(self) -> HugeObjectArray<Vec<f64>> {
        self.features
    }
}

impl FeatureConsumer for HugeObjectArrayFeatureConsumer {
    fn accept_scalar(&mut self, node_offset: u64, offset: usize, value: f64) {
        let row = self.features.get_mut(node_offset as usize);
        row[offset] = value;
    }

    fn accept_array(&mut self, node_offset: u64, offset: usize, values: &[f64]) {
        let row = self.features.get_mut(node_offset as usize);
        row[offset..offset + values.len()].copy_from_slice(values);
    }
}
