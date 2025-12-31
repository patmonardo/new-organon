//! Loader for Arrow catalog layout (`nodes.parquet` + `edges.parquet`).
//!
//! Reads parquet files into arrow2 chunks, wraps them as table references,
//! and feeds them to the ArrowNativeFactory to produce a DefaultGraphStore.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::config::CollectionsBackend;
use crate::core::io::arrow::reader::read_parquet_file;
use crate::core::loader::LoaderError;
use crate::projection::factory::arrow::{
    ArrowNativeFactory, ArrowProjectionConfig, ArrowReferenceError, EdgeTableReference,
    NodeTableReference,
};
use crate::projection::factory::GraphStoreFactory;
use crate::types::graph_store::DefaultGraphStore;

/// Loader for a single graph stored under a directory with `nodes.parquet` and `edges.parquet`.
#[derive(Debug, Clone)]
pub struct ArrowCatalogLoader {
    root: PathBuf,
    backend: CollectionsBackend,
}

impl ArrowCatalogLoader {
    /// Create a new loader rooted at `path`. Defaults to Arrow backend.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            backend: CollectionsBackend::Arrow,
        }
    }

    /// Override the collections backend (Vec/Huge/Arrow) for the resulting store.
    pub fn with_backend(mut self, backend: CollectionsBackend) -> Self {
        self.backend = backend;
        self
    }

    fn load_table_reference<T>(
        &self,
        path: &Path,
        make_ref: impl FnOnce(
            Arc<arrow2::chunk::Chunk<Box<dyn arrow2::array::Array>>>,
            Arc<arrow2::datatypes::Schema>,
        ) -> Result<T, ArrowReferenceError>,
    ) -> Result<T, LoaderError> {
        let arrow_data = read_parquet_file(path.to_string_lossy().as_ref())
            .map_err(|e| LoaderError::Io(e.to_string()))?;

        let first = arrow_data
            .chunks
            .into_iter()
            .next()
            .ok_or_else(|| LoaderError::Io(format!("no record batches in {:?}", path)))?;

        make_ref(Arc::new(first), Arc::new(arrow_data.schema))
            .map_err(|e| LoaderError::Projection(e.to_string()))
    }

    /// Load the graph into a DefaultGraphStore.
    pub fn load(&self) -> Result<DefaultGraphStore, LoaderError> {
        let nodes_path = self.root.join("nodes.parquet");
        let edges_path = self.root.join("edges.parquet");

        let node_ref = self.load_table_reference(&nodes_path, |chunk, schema| {
            NodeTableReference::new(
                "nodes",
                Arc::try_unwrap(chunk).unwrap_or_else(|c| (*c).clone()),
                schema,
            )
        })?;

        let edge_ref = self.load_table_reference(&edges_path, |chunk, schema| {
            EdgeTableReference::new(
                "edges",
                Arc::try_unwrap(chunk).unwrap_or_else(|c| (*c).clone()),
                schema,
            )
        })?;

        let factory = ArrowNativeFactory::from_tables(Arc::new(node_ref), Arc::new(edge_ref));

        let cfg = ArrowProjectionConfig {
            collections_backend: self.backend,
            ..Default::default()
        };

        factory
            .build_graph_store(&cfg)
            .map_err(|e| LoaderError::Projection(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    #[cfg(feature = "arrow")]
    fn missing_files_yield_io_error() {
        let tmp_dir = std::env::temp_dir().join("arrow_loader_missing");
        // Ensure directory exists but no parquet files
        let _ = fs::create_dir_all(&tmp_dir);

        let loader = ArrowCatalogLoader::new(&tmp_dir);
        let res = loader.load();
        assert!(matches!(res, Err(LoaderError::Io(_))));

        // Cleanup best-effort
        let _ = fs::remove_dir_all(&tmp_dir);
    }
}
