//! HashGNN node embeddings (Java: `org.neo4j.gds.embeddings.hashgnn`).
//!
//! This module intentionally mirrors the Java split:
//! - **Support layer** (translated helpers/tasks) in this top-level module.
//! - **Algorithm layer** (`algo/`) which hosts the canonical
//!   `spec/storage/computation` surface used by our executor + facades.

mod binarize_task;
mod densify_task;
mod embeddings_to_node_property_values;
mod generate_features_task;
mod hash_gnn;
mod hash_gnn_companion;
mod hash_gnn_config_transformer;
mod hash_gnn_memory_estimate_definition;
mod hash_gnn_parameters;
mod hash_gnn_result;
mod hash_gnn_task;
mod hash_task;
mod min_hash_task;
mod raw_features_task;

pub mod algo;

pub use algo::{
    BinarizeFeaturesConfig, GenerateFeaturesConfig, HashGNNAlgorithmSpec,
    HashGNNComputationRuntime, HashGNNConfig, HashGNNEmbeddings, HashGNNResult,
    HashGNNStorageRuntime,
};
