//! Knowledge-graph embedding (KGE) link prediction infrastructure.
//!
//! **Translation Source**: `org.neo4j.gds.algorithms.machinelearning` (Java)
//!
//! The module mirrors the Java structure for parity:
//! - `parameters` for input configuration and scoring function
//! - `result` for output aggregation
//! - `scorer` for per-node link scoring logic
//! - `compute` for the top-K prediction driver

pub mod compute;
pub mod error;
pub mod parameters;
pub mod result;

mod scorer;

pub use compute::compute_kge_predict;
pub use error::KgeError;
pub use parameters::{KgeGraph, KgePredictParameters, ScoreFunction};
pub use result::{KgePredictResult, KgePrediction};
