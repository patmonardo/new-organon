//! Embeddings facades.

pub mod fast_rp;
pub mod hash_gnn;
pub mod node2vec;

pub use fast_rp::FastRPBuilder;
pub use hash_gnn::HashGNNBuilder;
pub use node2vec::Node2VecBuilder;
