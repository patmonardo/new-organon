//! Java: `EmbeddingsToNodePropertyValues`.
//!
//! Rust GDS does not use the Java `NodePropertyValues` interface directly.
//! We keep conversion helpers at the data-structure level.

use crate::collections::HugeObjectArray;
use crate::core::utils::paged::HugeAtomicBitSet;
use std::sync::Arc;

use super::hash_gnn_result::HashGNNEmbeddings;

pub struct EmbeddingsToNodePropertyValues;

impl EmbeddingsToNodePropertyValues {
    pub fn from_dense(dense: HugeObjectArray<Vec<f64>>) -> HashGNNEmbeddings {
        HashGNNEmbeddings::Dense { embeddings: dense }
    }

    pub fn from_binary(
        binary: HugeObjectArray<Option<Arc<HugeAtomicBitSet>>>,
        embedding_dimension: usize,
    ) -> HashGNNEmbeddings {
        HashGNNEmbeddings::Binary {
            embeddings: binary,
            embedding_dimension,
        }
    }
}
