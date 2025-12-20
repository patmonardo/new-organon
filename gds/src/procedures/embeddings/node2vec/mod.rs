//! Node2Vec node embeddings.

pub mod computation;
pub mod spec;
pub mod storage;

mod translated;

pub use computation::Node2VecComputationRuntime;
pub use spec::{EmbeddingInitializerConfig, Node2VecAlgorithmSpec, Node2VecConfig, Node2VecResult};
pub use storage::Node2VecStorageRuntime;
