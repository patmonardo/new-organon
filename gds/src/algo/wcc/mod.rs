//! Weakly Connected Components (WCC) Algorithm
//!
//! **Translation Source**: `org.neo4j.gds.wcc.Wcc` (+ `SampledStrategy`, `UnsampledStrategy`)
//!
//! Computes connected components under *undirected* semantics.
//! For directed graphs, each relationship still connects its endpoints.

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

pub use computation::WccComputationRuntime;
pub use spec::{WCCAlgorithmSpec, WccConfig, WccResult};
pub use storage::WccStorageRuntime;
