//! CELF (Cost-Effective Lazy Forward) Algorithm Module
//!
//! Influence Maximization using the Independent Cascade model.

pub mod spec;
pub mod storage;
pub mod computation;

pub use spec::{CELFAlgorithmSpec, CELFConfig, CELFResult};
pub use computation::CELFComputationRuntime;
