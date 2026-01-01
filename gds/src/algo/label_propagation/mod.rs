//! Label Propagation Module
//!
//! Implements label propagation algorithm for community detection
//! by iteratively propagating labels through node voting.

pub mod computation;
pub mod spec;
pub mod storage;

#[cfg(test)]
mod integration_tests;

pub use computation::LabelPropComputationRuntime;
pub use spec::{LabelPropAlgorithmSpec, LabelPropConfig, LabelPropResult};
pub use storage::LabelPropStorageRuntime;
