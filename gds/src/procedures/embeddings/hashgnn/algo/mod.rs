//! HashGNN canonical algorithm surface.
//!
//! This folder hosts the standard `spec/storage/computation` split used across
//! `procedures/`, while the translated Java parity layer lives in the parent
//! module (`super`).

pub mod computation;
pub mod spec;
pub mod storage;

pub use computation::HashGNNComputationRuntime;
pub use spec::{
    BinarizeFeaturesConfig, GenerateFeaturesConfig, HashGNNAlgorithmSpec, HashGNNConfig,
    HashGNNEmbeddings, HashGNNResult,
};
pub use storage::HashGNNStorageRuntime;
