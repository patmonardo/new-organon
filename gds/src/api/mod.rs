//! Minimal API façade for Java-parity translations.
//!
//! Several translated “applications/machinery” modules expect a `crate::api` module
//! (matching the Java package layering). The rest of the crate uses more explicit
//! module paths (`procedures`, `types`, etc.). This module is intentionally small
//! and primarily provides type re-exports/aliases.

use std::sync::Arc;

pub type Graph = crate::procedures::Graph;

/// Named graph identifier (Java parity: `GraphName`).
///
/// Today we keep this as an owned string.
pub type GraphName = String;

/// Graph store handle used by application machinery.
///
/// We use an `Arc` to match `core::loading::GraphResources` and to keep signatures
/// cheap to clone across layers.
pub type GraphStore = Arc<crate::types::graph_store::DefaultGraphStore>;

/// Result store abstraction used by graph-store catalog loaders.
///
/// Many translated signatures use `&mut ResultStore` (without `dyn`). This alias
/// preserves that call shape while still pointing at a trait object.
pub type ResultStore = dyn crate::applications::graph_store_catalog::loaders::ResultStore;

pub mod properties;
