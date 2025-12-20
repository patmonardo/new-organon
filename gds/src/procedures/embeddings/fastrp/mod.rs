//! FastRP node embeddings.
//!
//! Translation source (historical): `org.neo4j.gds.embeddings.fastrp.*`

pub mod computation;
pub mod spec;
pub mod storage;

pub use computation::FastRPComputationRuntime;
pub use spec::{FastRPAlgorithmSpec, FastRPConfig, FastRPResult};
pub use storage::FastRPStorageRuntime;
