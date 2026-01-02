//! CollapsePath procedure facade.
//!
//! This algorithm is part of the Java GDS miscellaneous utilities.
//!
//! The stable surface is defined here; implementation will be wired once
//! we finalize semantics for path-collapsing in our store model.

use crate::procedures::traits::Result;
use crate::types::graph_store::GraphName;
use crate::types::prelude::DefaultGraphStore;
use std::sync::Arc;

#[derive(Clone)]
pub struct CollapsePathFacade {
    graph_store: Arc<DefaultGraphStore>,
    max_hops: Option<usize>,
}

impl CollapsePathFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            max_hops: None,
        }
    }

    pub fn max_hops(mut self, max_hops: usize) -> Self {
        self.max_hops = Some(max_hops);
        self
    }

    pub fn to_store(self, new_graph_name: impl AsRef<str>) -> Result<DefaultGraphStore> {
        let graph_name = GraphName::new(new_graph_name.as_ref());
        self.graph_store
            .collapse_paths_degree2(graph_name, self.max_hops)
            .map_err(|e| crate::projection::eval::procedure::AlgorithmError::Execution(e.to_string()))
    }
}
