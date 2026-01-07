//! Embeddings facades.

pub mod fast_rp;
pub mod gat;
pub mod graphsage;
pub mod hash_gnn;
pub mod node2vec;

pub use fast_rp::FastRPBuilder;
pub use gat::GATBuilder;
pub use graphsage::GraphSageBuilder;
pub use hash_gnn::{BinarizeFeaturesConfig, GenerateFeaturesConfig, HashGNNBuilder};
pub use node2vec::Node2VecBuilder;
