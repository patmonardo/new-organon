//! Embeddings procedures (node/graph embeddings).
//!
//! This module hosts embedding algorithms implemented in the standard
//! `spec/storage/computation` shape used across `procedures/`.

pub mod fastrp;
pub mod hashgnn;
#[cfg(feature = "node2vec")]
pub mod node2vec;

pub use fastrp::{FastRPAlgorithmSpec, FastRPConfig, FastRPResult};
pub use hashgnn::{HashGNNAlgorithmSpec, HashGNNConfig, HashGNNEmbeddings, HashGNNResult};
#[cfg(feature = "node2vec")]
pub use node2vec::{Node2VecAlgorithmSpec, Node2VecConfig, Node2VecResult};
