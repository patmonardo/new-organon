use crate::collections::HugeObjectArray;

/// Result of FastRP.
///
/// Java: `record FastRPResult(HugeObjectArray<float[]> embeddings){}`
pub struct FastRPResult {
    pub embeddings: HugeObjectArray<Vec<f32>>,
}


