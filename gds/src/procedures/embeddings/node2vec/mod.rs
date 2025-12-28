//! Node2Vec node embeddings.
//!
//! This module follows the "support layer" + "algorithm layer" split:
//! - **Support layer** (translated helpers) in this top-level module.
//! - **Algorithm layer** (`algo/`) which hosts the canonical
//!   `spec/storage/computation` surface used by our executor + facades.

pub mod algo;
pub mod compressed_random_walks;
pub mod node2vec_model;
pub mod node2vec_parameters;
pub mod random_walk_probabilities;
pub mod sampling_walk_parameters;
pub mod train_parameters;

pub use algo::{
    EmbeddingInitializerConfig, Node2VecAlgorithmSpec, Node2VecComputationRuntime,
    Node2VecConfig, Node2VecResult, Node2VecStorageRuntime,
};
