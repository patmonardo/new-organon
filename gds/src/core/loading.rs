//! Graph loading and resources - minimal catalog-backed implementation.
//!
//! This provides the `GraphResources` type that the machinery template needs.
//! For now, we don't load from a database - we load from the catalog (in-memory).

use std::sync::Arc;

use crate::procedures::GraphFacade as FacadeGraph;
use crate::types::catalog::GraphCatalog;
use crate::types::graph_store::DefaultGraphStore;

/// Graph resources needed by algorithms.
///
/// This is the "loaded" state that the machinery template works with.
/// In Java GDS, this comes from `GraphStoreCatalogService.getGraphResources()`.
///
/// For now, we back this purely from the catalog (no DB loading).
#[derive(Clone)]
pub struct GraphResources {
    /// The underlying graph store
    pub graph_store: Arc<DefaultGraphStore>,
    /// The facade graph (for algorithm access)
    pub graph: FacadeGraph,
    /// Optional result store (Java parity: used for write-back caching).
    ///
    /// Today this is typically `None` for catalog-backed loading.
    pub result_store:
        Option<Arc<dyn crate::applications::graph_store_catalog::loaders::ResultStore>>,
}

impl GraphResources {
    /// Create GraphResources from a graph store.
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        let graph = FacadeGraph::new(Arc::clone(&graph_store));
        Self {
            graph_store,
            graph,
            result_store: None,
        }
    }

    /// Access the underlying store.
    pub fn store(&self) -> &Arc<DefaultGraphStore> {
        &self.graph_store
    }

    /// Access the facade graph.
    pub fn facade(&self) -> &FacadeGraph {
        &self.graph
    }
}

/// Catalog-backed loader for GraphResources.
///
/// This is the minimal "loader" that doesn't touch a database.
/// It simply wraps `catalog.get(name)` → `GraphResources`.
pub struct CatalogLoader;

impl CatalogLoader {
    /// Load GraphResources from a named graph in the catalog.
    ///
    /// Returns `None` if the graph doesn't exist.
    pub fn load(catalog: &dyn GraphCatalog, graph_name: &str) -> Option<GraphResources> {
        catalog.get(graph_name).map(GraphResources::new)
    }

    /// Load GraphResources, returning an error if not found.
    pub fn load_or_err(
        catalog: &dyn GraphCatalog,
        graph_name: &str,
    ) -> Result<GraphResources, LoadError> {
        Self::load(catalog, graph_name).ok_or_else(|| LoadError::NotFound(graph_name.to_string()))
    }
}

/// Errors that can occur during loading.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("Graph not found: {0}")]
    NotFound(String),
}

/// Hooks that can run after loading (placeholder for Java parity).
pub trait PostLoadValidationHook: Send + Sync {
    fn validate(&self, resources: &GraphResources) -> Result<(), String>;
}

/// ETL hooks that can transform after loading (placeholder for Java parity).
pub trait PostLoadETLHook: Send + Sync {
    fn transform(&self, resources: &mut GraphResources) -> Result<(), String>;
}
