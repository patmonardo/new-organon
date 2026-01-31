//! Dataset IO helpers for format detection and catalog mapping.

use std::path::Path;

use crate::collections::catalog::types::CollectionsIoFormat;

pub fn detect_format_from_path(path: &Path) -> CollectionsIoFormat {
    let ext = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    match ext.to_ascii_lowercase().as_str() {
        "parquet" => CollectionsIoFormat::Parquet,
        "ipc" | "arrow" => CollectionsIoFormat::ArrowIpc,
        "csv" => CollectionsIoFormat::Csv,
        "json" | "ndjson" => CollectionsIoFormat::Json,
        _ => CollectionsIoFormat::Auto,
    }
}
