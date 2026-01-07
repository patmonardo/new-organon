//! PageRank (Java GDS parity, simplified)
//!
//! Standard algorithm module layout:
//! - `spec`: AlgorithmSpec integration (executor)
//! - `storage`: GraphStore-facing accessors (thin for PageRank)
//! - `computation`: core PageRank loop + memory estimation

pub mod computation;
pub mod spec;
pub mod storage;

#[cfg(test)]
pub mod integration_tests;

pub use computation::{
    estimate_pagerank_memory, PageRankComputationRuntime, PageRankMemoryEstimation,
    PageRankRunResult,
};
pub use spec::{PAGERANKAlgorithmSpec, PageRankConfigInput, PageRankResult};
pub use storage::PageRankStorageRuntime;

// Keep public surface stable for `gds/src/algo/mod.rs` re-exports.
pub type PageRankAlgorithmSpec = PAGERANKAlgorithmSpec;
