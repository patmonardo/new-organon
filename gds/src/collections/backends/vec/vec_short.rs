//! VecShort: Vec-based i16 Collections implementation
#[derive(Debug, Clone)]
pub struct VecShort {
    pub data: Vec<i16>,
}
impl Default for VecShort {
    fn default() -> Self {
        Self::new()
    }
}

impl VecShort {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}
use crate::types::ValueType;
use crate::vec_collections;
vec_collections!(
    VecShort,
    i16,
    ValueType::Short,
    0i16,
    to_f64 = |x: i16| x as f64,
    kind = Ord
);

use crate::collections::PropertyValuesAdapter;

// Implement PropertyValuesAdapter (marker trait)
impl PropertyValuesAdapter<i16> for VecShort {}
