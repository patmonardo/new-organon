//! Node2Vec canonical algorithm surface.
//!
//! This folder hosts the standard `spec/storage/computation` split used across
//! `procedures/`, while the translated Java parity layer lives in the parent
//! module (`super`).

pub mod computation;
pub mod spec;
pub mod storage;

pub use computation::Node2VecComputationRuntime;
pub use spec::{EmbeddingInitializerConfig, Node2VecAlgorithmSpec, Node2VecConfig, Node2VecResult};
pub use storage::Node2VecStorageRuntime;
