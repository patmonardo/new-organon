//! Leiden Community Detection
//!
//! **Java parity reference**: `org.neo4j.gds.leiden.*`
//!
//! This module follows the repository algorithm layout:
//! - `spec.rs`       configuration + result types
//! - `storage.rs`    GraphStore adapter (projection + data prep)
//! - `computation.rs` algorithm runtime

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

pub use computation::LeidenComputationRuntime;
pub use spec::{LeidenConfig, LeidenResult};
pub use storage::LeidenStorageRuntime;
