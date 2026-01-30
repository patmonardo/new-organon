//! Catalog extension facade for Collections.

use std::path::PathBuf;

use crate::collections::catalog::disk::CollectionsCatalogDisk;
use crate::collections::catalog::schema::CollectionsSchema;
use crate::collections::catalog::types::{
    CatalogError, CollectionsCatalogDiskEntry, CollectionsIoFormat, CollectionsIoPolicy,
};
use crate::collections::dataframe::{
    read_table_csv, read_table_ipc, read_table_parquet, write_table_csv, write_table_ipc,
    write_table_parquet, PolarsDataFrameCollection,
};
use crate::config::CollectionsBackend;
use crate::types::ValueType;

/// Configuration for the catalog extension facade.
#[derive(Debug, Clone)]
pub struct CatalogExtensionConfig {
    pub default_format: CollectionsIoFormat,
    pub allow_overwrite: bool,
    pub eager: bool,
    pub infer_schema_on_write: bool,
    pub infer_schema_on_read: bool,
    pub validate_on_read: bool,
    pub auto_save: bool,
}

impl Default for CatalogExtensionConfig {
    fn default() -> Self {
        Self {
            default_format: CollectionsIoFormat::Parquet,
            allow_overwrite: true,
            eager: true,
            infer_schema_on_write: true,
            infer_schema_on_read: false,
            validate_on_read: false,
            auto_save: true,
        }
    }
}

/// Catalog extension that provides a configurable facade over disk catalogs.
#[derive(Debug)]
pub struct CatalogExtension {
    catalog: CollectionsCatalogDisk,
    config: CatalogExtensionConfig,
    pending_schema_refresh: Vec<String>,
}

impl CatalogExtension {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self, CatalogError> {
        let catalog = CollectionsCatalogDisk::load(root)?;
        Ok(Self {
            catalog,
            config: CatalogExtensionConfig::default(),
            pending_schema_refresh: Vec::new(),
        })
    }

    pub fn with_config(mut self, config: CatalogExtensionConfig) -> Self {
        self.config = config;
        self
    }

    pub fn catalog(&self) -> &CollectionsCatalogDisk {
        &self.catalog
    }

    pub fn catalog_mut(&mut self) -> &mut CollectionsCatalogDisk {
        &mut self.catalog
    }

    pub fn config(&self) -> &CatalogExtensionConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut CatalogExtensionConfig {
        &mut self.config
    }

    pub fn register_table(
        &mut self,
        name: &str,
        format: Option<CollectionsIoFormat>,
    ) -> Result<CollectionsCatalogDiskEntry, CatalogError> {
        let format = format.unwrap_or(self.config.default_format);
        let entry = CollectionsCatalogDiskEntry {
            name: name.to_string(),
            value_type: ValueType::Unknown,
            schema: None,
            backend: CollectionsBackend::Vec,
            extensions: Vec::new(),
            io_policy: CollectionsIoPolicy {
                format,
                allow_overwrite: self.config.allow_overwrite,
                ..Default::default()
            },
            data_path: self.catalog.default_data_path(name, format),
        };

        if self.config.allow_overwrite {
            let _ = self.catalog.remove(name);
        }

        self.catalog.register(entry.clone())?;
        Ok(entry)
    }

    pub fn write_table(
        &mut self,
        name: &str,
        table: &PolarsDataFrameCollection,
        format: Option<CollectionsIoFormat>,
    ) -> Result<CollectionsCatalogDiskEntry, CatalogError> {
        let entry = self.register_table(name, format)?;
        let data_path = self.catalog.data_path(&entry);
        if let Some(parent) = data_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| CatalogError::Io(e.to_string()))?;
        }

        match entry.io_policy.format {
            CollectionsIoFormat::Auto | CollectionsIoFormat::Parquet => {
                write_table_parquet(&data_path, table)
                    .map_err(|e| CatalogError::Polars(e.to_string()))?;
            }
            CollectionsIoFormat::Csv => {
                write_table_csv(&data_path, table)
                    .map_err(|e| CatalogError::Polars(e.to_string()))?;
            }
            CollectionsIoFormat::ArrowIpc => {
                write_table_ipc(&data_path, table)
                    .map_err(|e| CatalogError::Polars(e.to_string()))?;
            }
            CollectionsIoFormat::Json => {
                return Err(CatalogError::Polars(
                    "JSON table write not implemented for Collections yet".to_string(),
                ));
            }
            CollectionsIoFormat::Database => {
                return Err(CatalogError::Polars(
                    "Database table write not implemented for Collections yet".to_string(),
                ));
            }
        }

        self.handle_schema_after_write(&entry)?;
        if self.config.auto_save {
            self.catalog.save()?;
        }
        Ok(entry)
    }

    pub fn read_table(&mut self, name: &str) -> Result<PolarsDataFrameCollection, CatalogError> {
        let entry = self
            .catalog
            .get(name)
            .ok_or_else(|| CatalogError::NotFound(name.to_string()))?
            .clone();
        let data_path = self.catalog.data_path(&entry);

        let table = match entry.io_policy.format {
            CollectionsIoFormat::Auto | CollectionsIoFormat::Parquet => {
                read_table_parquet(&data_path).map_err(|e| CatalogError::Polars(e.to_string()))?
            }
            CollectionsIoFormat::Csv => {
                read_table_csv(&data_path).map_err(|e| CatalogError::Polars(e.to_string()))?
            }
            CollectionsIoFormat::ArrowIpc => {
                read_table_ipc(&data_path).map_err(|e| CatalogError::Polars(e.to_string()))?
            }
            CollectionsIoFormat::Json => {
                return Err(CatalogError::Polars(
                    "JSON table read not implemented for Collections yet".to_string(),
                ));
            }
            CollectionsIoFormat::Database => {
                return Err(CatalogError::Polars(
                    "Database table read not implemented for Collections yet".to_string(),
                ));
            }
        };

        if self.config.infer_schema_on_read {
            let inferred = CollectionsSchema::from_polars(table.dataframe());
            if let Some(existing) = entry.schema.as_ref() {
                if self.config.validate_on_read && existing != &inferred {
                    return Err(CatalogError::Parse(format!(
                        "Schema mismatch for entry '{name}'"
                    )));
                }
            } else if self.config.eager {
                self.catalog.refresh_schema(name)?;
                if self.config.auto_save {
                    self.catalog.save()?;
                }
            }
        }

        Ok(table)
    }

    pub fn refresh_schema(&mut self, name: &str) -> Result<(), CatalogError> {
        self.catalog.refresh_schema(name)?;
        if self.config.auto_save {
            self.catalog.save()?;
        }
        Ok(())
    }

    pub fn refresh_all_schemas(&mut self) -> Result<(), CatalogError> {
        self.catalog.refresh_all_schemas()?;
        if self.config.auto_save {
            self.catalog.save()?;
        }
        Ok(())
    }

    pub fn commit(&mut self) -> Result<(), CatalogError> {
        if !self.pending_schema_refresh.is_empty() {
            for name in self.pending_schema_refresh.drain(..) {
                let _ = self.catalog.refresh_schema(&name);
            }
        }
        if self.config.auto_save {
            self.catalog.save()?;
        }
        Ok(())
    }

    fn handle_schema_after_write(
        &mut self,
        entry: &CollectionsCatalogDiskEntry,
    ) -> Result<(), CatalogError> {
        if !self.config.infer_schema_on_write {
            return Ok(());
        }

        if self.config.eager {
            self.catalog.refresh_schema(&entry.name)?;
        } else {
            self.pending_schema_refresh.push(entry.name.clone());
        }
        Ok(())
    }
}
