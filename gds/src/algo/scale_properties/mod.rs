//! Scale Properties algorithm module.
//!
//! Translation source: `org.neo4j.gds.scaleproperties.*`.
//! Scales one or more node properties using configurable scaler variants
//! and returns a per-node vector of scaled values plus scaler statistics.

pub mod computation;
pub mod spec;
pub mod storage;

pub use computation::ScalePropertiesComputationRuntime;
pub use spec::{
    ScalePropertiesAlgorithmSpec, ScalePropertiesConfig, ScalePropertiesResult,
    ScalePropertiesScaler,
};
pub use storage::ScalePropertiesStorageRuntime;
