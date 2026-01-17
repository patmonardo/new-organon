//! Error types for KGE prediction.

#[derive(Debug, thiserror::Error)]
pub enum KgeError {
    #[error("missing node property: {0}")]
    MissingNodeProperty(String),

    #[error("unsupported embedding value type: {0}")]
    UnsupportedEmbeddingValueType(String),

    #[error("relationshipTypeEmbedding length must be > 0")]
    InvalidRelationshipTypeEmbedding,

    #[error("embedding dimension mismatch: expected {expected}, got {actual}")]
    EmbeddingDimensionMismatch { expected: usize, actual: usize },

    #[error("topK must be > 0")]
    InvalidTopK,
}
