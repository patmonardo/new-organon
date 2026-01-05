//! Approximate Maximum K-Cut.
//!
//! This module exists primarily to satisfy the Rust facade in
//! `src/procedures/community/approx_max_kcut.rs`.
//!
//! Java parity reference (conceptual): `org.neo4j.gds.approxmaxkcut.*`.
//!
//! Layout follows the repository algorithm convention:
//! - `spec.rs`        configuration + result types
//! - `storage.rs`     (placeholder) GraphStore adapter
//! - `computation.rs` core algorithm runtime

pub mod computation;
#[cfg(test)]
mod integration_tests;
pub mod spec;
pub mod storage;

pub use computation::ApproxMaxKCutComputationRuntime;
pub use spec::{ApproxMaxKCutConfig, ApproxMaxKCutResult};
pub use storage::ApproxMaxKCutStorageRuntime;

use crate::core::utils::progress::tasks::{LeafTask, Tasks};

/// Progress task used by historical Rust call sites.
pub fn progress_leaf_task(iterations: usize) -> LeafTask {
	Tasks::leaf_with_volume("approx_max_kcut".to_string(), iterations)
}
