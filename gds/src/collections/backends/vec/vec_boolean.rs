//! VecBoolean: Vec-based bool Collections implementation
#[derive(Debug, Clone)]
pub struct VecBoolean {
    pub data: Vec<bool>,
}
impl Default for VecBoolean {
    fn default() -> Self {
        Self::new()
    }
}

impl VecBoolean {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}
use crate::types::ValueType;
use crate::vec_collections;
vec_collections!(VecBoolean, bool, ValueType::Boolean, false, kind = OrdNoAgg);

use crate::collections::PropertyValuesAdapter;

// Implement PropertyValuesAdapter (marker trait)
impl PropertyValuesAdapter<bool> for VecBoolean {}
