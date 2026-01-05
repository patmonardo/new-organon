//! Conductance quality metric.
//!
//! Java parity reference: `org.neo4j.gds.conductance.*`
//!
//! Conductance evaluates a division of nodes into communities based on the
//! proportion of relationship weight that crosses community boundaries.
//!
//! $$\phi(C) = \frac{\text{external}(C)}{\text{external}(C) + \text{internal}(C)}$$
//!
//! Where `internal(C)` is the total weight of relationships whose source and target
//! are both in community `C`, and `external(C)` is the total weight of relationships
//! whose source is in `C` but whose target is outside `C`.
//!
//! This module follows the repository algorithm layout:
//! - `spec.rs`        configuration + result types
//! - `storage.rs`     GraphStore adapter (projection + property access)
//! - `computation.rs` core algorithm runtime

pub mod computation;
#[cfg(test)]
mod integration_tests;
pub mod spec;
pub mod storage;

pub use computation::ConductanceComputationRuntime;
pub use spec::{ConductanceConfig, ConductanceResult};
pub use storage::ConductanceStorageRuntime;

use crate::core::utils::progress::tasks::{Task, Tasks};
use std::sync::Arc;

/// Java parity: progress task hierarchy (see `ConductanceAlgorithmFactory.progressTask`).
pub fn progress_task(node_count: usize) -> Task {
	Tasks::task(
		"Conductance".to_string(),
		vec![
			Arc::new(Tasks::leaf_with_volume("count relationships".to_string(), node_count).base().clone()),
			Arc::new(Tasks::leaf("accumulate counts".to_string()).base().clone()),
			Arc::new(
				Tasks::leaf("perform conductance computations".to_string())
					.base()
					.clone(),
			),
		],
	)
}
