//! Indirect Exposure
//!
//! Translation source: `org.neo4j.gds.indirectExposure` (Pregel-based traversal).
//!
//! Layout:
//! - `spec`: config/result + catalog marker
//! - `storage`: GraphStore-facing orchestration (projection, precomputation, Pregel wiring)
//! - `computation`: Pregel kernels (init/compute/schema)

pub mod computation;
pub mod spec;
pub mod storage;

pub use computation::IndirectExposureComputationRuntime;
pub use spec::{IndirectExposureAlgorithmSpec, IndirectExposureConfig, IndirectExposureResult};
pub use storage::IndirectExposureStorageRuntime;
