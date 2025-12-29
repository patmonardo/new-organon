//! Meta-importer interface: connects IO surfaces to projection factories.
//!
//! This layer is responsible for orchestrating how in-memory sources
//! (e.g., Arrow batches) are handed to projection factories to yield a
//! `GraphStore`. Keep it lightweight and decoupled from specific file IO.

use crate::types::graph_store::DefaultGraphStore;
use thiserror::Error;

/// Simple loader contract for producing a `DefaultGraphStore` from any source.
pub trait GraphStoreLoader {
    fn load(&self) -> Result<DefaultGraphStore, LoaderError>;
}

#[derive(Debug, Error)]
pub enum LoaderError {
    #[error("IO error: {0}")]
    Io(String),

    #[error("Projection error: {0}")]
    Projection(String),

    #[error("Other load error: {0}")]
    Other(String),
}
