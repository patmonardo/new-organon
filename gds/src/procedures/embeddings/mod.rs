//! Embeddings procedures (node/graph embeddings).
//!
//! This module hosts embedding algorithms implemented in the standard
//! `spec/storage/computation` shape used across `procedures/`.

pub mod fastrp;
pub mod gat;
pub mod graphsage;
pub mod hashgnn;
pub mod node2vec;

pub use fastrp::{FastRPAlgorithmSpec, FastRPConfig, FastRPResult};
pub use gat::config::GATConfig;
pub use gat::spec::GATAlgorithmSpec;
pub use gat::types::GATResult;
pub use graphsage::algo::spec::{GRAPHSAGEAlgorithmSpec, GraphSageConfig, GraphSageResult};
pub use hashgnn::{HashGNNAlgorithmSpec, HashGNNConfig, HashGNNEmbeddings, HashGNNResult};
pub use node2vec::{Node2VecAlgorithmSpec, Node2VecConfig, Node2VecResult};
