//! PageRank storage runtime
//!
//! PageRank operates directly over an oriented `Graph` view.
//! This storage runtime is intentionally thin and exists to conform
//! to the standard algorithm module layout.

use crate::projection::eval::procedure::AlgorithmError;
use crate::types::prelude::GraphStore;

#[derive(Debug, Clone, Copy)]
pub struct PageRankStorageRuntime;

impl PageRankStorageRuntime {
    pub fn new<G: GraphStore>(_graph_store: &G) -> Result<Self, AlgorithmError> {
        Ok(Self)
    }
}
