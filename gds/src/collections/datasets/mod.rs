//! Collections datasets.
//!
//! High-level dataset management and Python-like expression helpers live here.
//! This is where we can map dataset families (e.g. pytorch-geometric) into
//! a consistent registry and reuse DataFrame expressions without adding
//! Rust-heavy call sites.

pub mod expr;
pub mod registry;

pub use expr::*;
pub use registry::{DatasetArtifact, DatasetMetadata, DatasetRegistry, DatasetSplit};
