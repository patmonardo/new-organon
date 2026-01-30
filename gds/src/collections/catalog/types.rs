//! Collections Catalog Types
//!
//! Shared catalog types for disk-first Collections.

use crate::collections::catalog::schema::CollectionsSchema;
use crate::config::{CollectionsBackend, DatasetConfig, Extension};
use crate::types::ValueType;

/// IO formats supported for disk-first Collections
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CollectionsIoFormat {
    Auto,
    ArrowIpc,
    Parquet,
    Csv,
    Json,
    Database,
}

/// Disk-first IO policy derived from CollectionsConfig
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectionsIoPolicy {
    pub format: CollectionsIoFormat,
    pub location: Option<String>,
    pub allow_overwrite: bool,
    pub compression: Option<String>,
    pub batch_size: Option<usize>,
}

impl CollectionsIoFormat {
    pub fn file_extension(self) -> &'static str {
        match self {
            CollectionsIoFormat::Auto => "parquet",
            CollectionsIoFormat::ArrowIpc => "ipc",
            CollectionsIoFormat::Parquet => "parquet",
            CollectionsIoFormat::Csv => "csv",
            CollectionsIoFormat::Json => "json",
            CollectionsIoFormat::Database => "db",
        }
    }
}

impl Default for CollectionsIoPolicy {
    fn default() -> Self {
        Self {
            format: CollectionsIoFormat::Auto,
            location: None,
            allow_overwrite: false,
            compression: None,
            batch_size: None,
        }
    }
}

/// Catalog entry for a Collections dataset
#[derive(Debug, Clone, PartialEq)]
pub struct CollectionsCatalogEntry {
    pub name: String,
    pub value_type: ValueType,
    pub schema: Option<CollectionsSchema>,
    pub backend: CollectionsBackend,
    pub extensions: Vec<Extension>,
    pub dataset: Option<DatasetConfig>,
    pub io_policy: CollectionsIoPolicy,
}

/// On-disk manifest entry for Collections catalog
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectionsCatalogDiskEntry {
    pub name: String,
    pub value_type: ValueType,
    pub schema: Option<CollectionsSchema>,
    pub backend: CollectionsBackend,
    pub extensions: Vec<String>,
    pub io_policy: CollectionsIoPolicy,
    pub data_path: String,
}

impl CollectionsCatalogDiskEntry {
    pub fn from_entry(entry: &CollectionsCatalogEntry, data_path: impl Into<String>) -> Self {
        Self {
            name: entry.name.clone(),
            value_type: entry.value_type,
            schema: entry.schema.clone(),
            backend: entry.backend,
            extensions: entry
                .extensions
                .iter()
                .map(|ext| format!("{ext:?}"))
                .collect(),
            io_policy: entry.io_policy.clone(),
            data_path: data_path.into(),
        }
    }
}

/// On-disk manifest for a Collections catalog
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectionsCatalogManifest {
    pub version: u32,
    pub created_at: String,
    pub entries: Vec<CollectionsCatalogDiskEntry>,
}

/// Catalog error types
#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error("Catalog entry already exists: {0}")]
    AlreadyExists(String),
    #[error("Catalog entry not found: {0}")]
    NotFound(String),
    #[error("Catalog IO error: {0}")]
    Io(String),
    #[error("Catalog parse error: {0}")]
    Parse(String),
    #[error("Catalog polars error: {0}")]
    Polars(String),
}
