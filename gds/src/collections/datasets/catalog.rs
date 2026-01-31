//! Dataset catalog integration with CollectionsCatalogDisk.

use std::path::{Path, PathBuf};

use crate::collections::catalog::{
    CatalogError, CollectionsCatalogDisk, CollectionsCatalogDiskEntry, CollectionsIoFormat,
    CollectionsIoPolicy,
};
use crate::collections::catalog::schema::CollectionsSchema;
use crate::collections::dataframe::PolarsDataFrameCollection;
use crate::collections::datasets::dataset::Dataset;
use crate::collections::datasets::io::detect_format_from_path;
use crate::config::CollectionsBackend;
use crate::types::ValueType;

/// Dataset-first wrapper for the disk catalog.
#[derive(Debug, Clone)]
pub struct DatasetCatalog {
    catalog: CollectionsCatalogDisk,
}

impl DatasetCatalog {
    pub fn load(root: impl Into<PathBuf>) -> Result<Self, CatalogError> {
        let catalog = CollectionsCatalogDisk::load(root)?;
        Ok(Self { catalog })
    }

    pub fn catalog(&self) -> &CollectionsCatalogDisk {
        &self.catalog
    }

    pub fn catalog_mut(&mut self) -> &mut CollectionsCatalogDisk {
        &mut self.catalog
    }

    pub fn save(&self) -> Result<(), CatalogError> {
        self.catalog.save()
    }

    pub fn register_table_path(
        &mut self,
        name: &str,
        data_path: impl AsRef<Path>,
        format: Option<CollectionsIoFormat>,
        schema: Option<CollectionsSchema>,
    ) -> Result<CollectionsCatalogDiskEntry, CatalogError> {
        let data_path = data_path.as_ref();
        let format = format.unwrap_or_else(|| detect_format_from_path(data_path));
        let relative_path = self.ensure_relative_path(data_path)?;
        let entry = self.build_table_entry(name, format, schema, relative_path);
        self.catalog.register(entry.clone())?;
        Ok(entry)
    }

    pub fn ingest_table(
        &mut self,
        name: &str,
        table: &PolarsDataFrameCollection,
        format: CollectionsIoFormat,
    ) -> Result<CollectionsCatalogDiskEntry, CatalogError> {
        let schema = Some(CollectionsSchema::from_polars(table.dataframe()));
        let data_path = self.catalog.default_data_path(name, format);
        let entry = self.build_table_entry(name, format, schema, data_path);
        self.catalog.register(entry.clone())?;
        self.catalog.write_table(&entry, table)?;
        Ok(entry)
    }

    pub fn load_table(&self, name: &str) -> Result<Dataset, CatalogError> {
        let entry = self
            .catalog
            .get(name)
            .ok_or_else(|| CatalogError::NotFound(name.to_string()))?;
        let table = self.catalog.read_table(entry)?;
        Ok(Dataset::named(name.to_string(), table))
    }

    fn ensure_relative_path(&self, path: &Path) -> Result<String, CatalogError> {
        if path.is_absolute() {
            let root = self.catalog.root();
            if !path.starts_with(root) {
                return Err(CatalogError::Io(format!(
                    "Path {:?} is outside catalog root {:?}",
                    path, root
                )));
            }
            let relative = path.strip_prefix(root).map_err(|e| CatalogError::Io(e.to_string()))?;
            return Ok(relative.to_string_lossy().to_string());
        }
        Ok(path.to_string_lossy().to_string())
    }

    fn build_table_entry(
        &self,
        name: &str,
        format: CollectionsIoFormat,
        schema: Option<CollectionsSchema>,
        data_path: impl Into<String>,
    ) -> CollectionsCatalogDiskEntry {
        let io_policy = CollectionsIoPolicy {
            format,
            ..Default::default()
        };
        CollectionsCatalogDiskEntry {
            name: name.to_string(),
            value_type: ValueType::Unknown,
            schema,
            backend: CollectionsBackend::Arrow,
            extensions: Vec::new(),
            io_policy,
            data_path: data_path.into(),
        }
    }
}
