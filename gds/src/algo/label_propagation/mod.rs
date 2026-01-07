//! Label Propagation (Java GDS parity)
//!
//! Standard algorithm module layout:
//! - `spec`: config + result + executor AlgorithmSpec integration
//! - `storage`: GraphStore-facing accessors
//! - `computation`: core label propagation runtime

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

pub use computation::LabelPropComputationRuntime;
pub use spec::{
    LABEL_PROPAGATIONAlgorithmSpec, LabelPropAlgorithmSpec, LabelPropConfig, LabelPropResult,
};
pub use storage::LabelPropStorageRuntime;
