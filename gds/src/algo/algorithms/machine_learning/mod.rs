//! Machine learning algorithm infrastructure.
//!
//! Currently contains KGE (knowledge-graph embedding) link prediction.
//! The KGE implementation is split into small, documented modules under
//! `machine_learning/kge/` to mirror the Java layout and ease parity work.

pub mod kge;

pub use kge::*;
