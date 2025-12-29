//! Java: `GraphSageResult`.

use crate::collections::HugeObjectArray;

pub struct GraphSageResult {
    pub embeddings: HugeObjectArray<Vec<f64>>,
}
