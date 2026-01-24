//! Java: `GraphSageResult`.

use crate::collections::HugeObjectArray;

#[allow(dead_code)]
pub struct GraphSageResult {
    pub embeddings: HugeObjectArray<Vec<f64>>,
}
