//! IndexInverse algorithm module.
//!
//! Translation source: `org.neo4j.gds.indexInverse.*`.
//! Builds inverse relationship indices for selected relationship types by
//! constructing a new graph store with populated incoming adjacency lists.

pub mod computation;
pub mod spec;
pub mod storage;

pub use computation::IndexInverseComputationRuntime;
pub use spec::{IndexInverseAlgorithmSpec, IndexInverseConfig, IndexInverseResult};
pub use storage::IndexInverseStorageRuntime;
