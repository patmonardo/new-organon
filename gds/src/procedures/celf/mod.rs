//! CELF (Cost-Effective Lazy Forward) Algorithm Module
//!
//! Influence Maximization using the Independent Cascade model.

pub mod computation;
pub mod spec;
pub mod storage;

pub use computation::CELFComputationRuntime;
pub use spec::{CELFAlgorithmSpec, CELFConfig, CELFResult};
