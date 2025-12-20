//! Random walk generation task for Node2Vec.
//!
//! Java: `Node2VecRandomWalkTask implements Runnable`
//!
//! In Rust GDS we currently generate walks in `Node2Vec::compute()` directly.
//! This module exists to preserve the Java class decomposition for future parallelization.

#[derive(Debug, Clone)]
pub struct Node2VecRandomWalkTask;


