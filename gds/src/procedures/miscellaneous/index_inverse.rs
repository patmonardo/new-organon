//! IndexInverse procedure facade.
//!
//! Java GDS exposes utilities around inverse indexing and orientation.
//!
//! In this Rust port, inverse indexing is primarily a graph-store concern.
//! This facade is a stable placeholder surface for future upgrades.

use crate::procedures::traits::Result;
use crate::types::graph_store::GraphName;
use crate::types::prelude::DefaultGraphStore;
use std::sync::Arc;

#[derive(Clone)]
pub struct IndexInverseFacade {
    graph_store: Arc<DefaultGraphStore>,
}

impl IndexInverseFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
        }
    }

    pub fn to_store(self, new_graph_name: impl AsRef<str>) -> Result<DefaultGraphStore> {
        let graph_name = GraphName::new(new_graph_name.as_ref());
        self.graph_store
            .with_inverse_indices(graph_name)
            .map_err(|e| crate::projection::eval::procedure::AlgorithmError::Execution(e.to_string()))
    }
}
