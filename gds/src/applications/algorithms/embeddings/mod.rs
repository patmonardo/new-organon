//! Node embeddings algorithm dispatch handlers.
//!
//! Handlers for node embedding algorithms that delegate to the facade layer.

pub mod fast_rp;
pub mod gat;
pub mod hash_gnn;
#[cfg(feature = "node2vec")]
pub mod node2vec;
