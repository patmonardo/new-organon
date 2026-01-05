//! K-Means clustering (Java GDS parity, simplified for this crate)
//!
//! Public API is consumed by:
//! - `gds/src/procedures/community/kmeans.rs`
//! - `gds/src/applications/algorithms/community/kmeans.rs`
//!
//! We keep `KMeansConfig`, `KMeansResult`, `KMeansSamplerType`, and
//! `KMeansComputationRuntime` stable.

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

pub use computation::KMeansComputationRuntime;
pub use spec::{KMeansConfig, KMeansResult, KMeansSamplerType};
pub use storage::KMeansStorageRuntime;
