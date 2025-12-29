//! Loader module: meta-importers that orchestrate IO → Projection.
//!
//! Keep IO reading and projection factories decoupled; loaders bridge them.

#[cfg(feature = "arrow")]
pub mod arrow_catalog_loader;
pub mod loader;

#[cfg(feature = "arrow")]
pub use arrow_catalog_loader::*;
pub use loader::*;
