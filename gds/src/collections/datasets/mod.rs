//! Collections datasets.
//!
//! High-level dataset management and Python-like expression helpers live here.
//! This is where we can map dataset families (e.g. pytorch-geometric) into
//! a consistent registry and reuse DataFrame expressions without adding
//! Rust-heavy call sites.

pub mod catalog;
pub mod dataset;
pub mod download;
pub mod error;
pub mod expr;
pub mod extract;
pub mod io;
pub mod registry;
pub mod streaming;

pub use catalog::DatasetCatalog;
pub use dataset::Dataset;
pub use download::{
    copy_local, download_if_missing, download_to_dir, download_url, DownloadReport,
};
pub use error::DatasetIoError;
pub use expr::*;
pub use extract::{extract_archive, ExtractReport};
pub use io::detect_format_from_path;
pub use registry::{DatasetArtifact, DatasetMetadata, DatasetRegistry, DatasetSplit};
pub use streaming::{StreamingBatchIter, StreamingDataset};
