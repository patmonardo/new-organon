//! Core substrate (new): product structs + capability traits.
//!
//! This module is intentionally *not* the Java-faithful translation under `core/`.
//! It is the minimal Rust-native substrate meant to back Projection Factory/Eval.
//!
//! Hard rule: do not introduce a `substrate/utils` mega-module.

pub mod graph;
pub mod compute_surface;
pub mod form_store;
pub mod lifecycle;
pub mod reality_fabric;
pub mod surface;
pub mod store;

pub use graph::*;
pub use compute_surface::*;
pub use form_store::*;
pub use lifecycle::*;
pub use reality_fabric::*;
pub use surface::*;
pub use store::*;

/// Internal contiguous node identifier for traversal.
///
/// This is a substrate-level identity, not an “original DB id”.
pub type NodeId = u32;

/// Relationship type identifier in the substrate.
///
/// We keep this numeric for performance; mapping from names is a layer above.
pub type RelTypeId = u32;

/// Substrate-level errors.
#[derive(Debug, thiserror::Error)]
pub enum SubstrateError {
    #[error("unsupported operation: {0}")]
    Unsupported(&'static str),

    #[error("invalid input: {0}")]
    Invalid(&'static str),
}

pub type SubstrateResult<T> = Result<T, SubstrateError>;
