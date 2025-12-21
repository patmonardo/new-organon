//! Embeddings facades.

pub mod fast_rp;
pub mod hash_gnn;
#[cfg(feature = "node2vec")]
pub mod node2vec;

pub use fast_rp::FastRPBuilder;
pub use hash_gnn::HashGNNBuilder;
#[cfg(feature = "node2vec")]
pub use node2vec::Node2VecBuilder;
