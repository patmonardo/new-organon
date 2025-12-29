use crate::collections::HugeObjectArray;
use crate::core::utils::paged::HugeAtomicBitSet;
use std::sync::Arc;

/// HashGNN embeddings output (binary or dense).
pub enum HashGNNEmbeddings {
    Binary {
        embeddings: HugeObjectArray<Option<Arc<HugeAtomicBitSet>>>,
        embedding_dimension: usize,
    },
    Dense {
        embeddings: HugeObjectArray<Vec<f64>>,
    },
}

/// HashGNN result wrapper.
///
/// Java: `record HashGNNResult(NodePropertyValues embeddings) {}`
pub struct HashGNNResult {
    pub embeddings: HashGNNEmbeddings,
}
