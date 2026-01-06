//! Walking algorithms
//!
//! Translation source: `org.neo4j.gds.walking` (CollapsePath and helpers).
//!
//! These utilities operate on path templates (ordered relationship-type
//! sequences) and build collapsed relationships that connect the start and
//! end of each matched template instance.

pub mod computation;
pub mod spec;
pub mod storage;

pub use computation::CollapsePathComputationRuntime;
pub use spec::{CollapsePathAlgorithmSpec, CollapsePathConfig, CollapsePathResult};
pub use storage::CollapsePathStorageRuntime;
