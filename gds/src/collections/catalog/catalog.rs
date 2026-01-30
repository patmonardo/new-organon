//! Collections Catalog Logic
//!
//! Catalog trait and in-memory implementation, plus config-to-policy helpers.

use crate::collections::catalog::types::{
    CatalogError, CollectionsCatalogEntry, CollectionsIoFormat, CollectionsIoPolicy,
};
use crate::config::{CollectionsConfig, DataSourceType};
use std::collections::HashMap;

/// Collections catalog trait
pub trait CollectionsCatalog {
    fn register(&mut self, entry: CollectionsCatalogEntry) -> Result<(), CatalogError>;
    fn get(&self, name: &str) -> Option<&CollectionsCatalogEntry>;
    fn list(&self) -> Vec<&CollectionsCatalogEntry>;
    fn remove(&mut self, name: &str) -> Option<CollectionsCatalogEntry>;
}

/// In-memory implementation of the Collections catalog
#[derive(Default)]
pub struct InMemoryCollectionsCatalog {
    entries: HashMap<String, CollectionsCatalogEntry>,
}

impl InMemoryCollectionsCatalog {
    pub fn new() -> Self {
        Self::default()
    }
}

impl CollectionsCatalog for InMemoryCollectionsCatalog {
    fn register(&mut self, entry: CollectionsCatalogEntry) -> Result<(), CatalogError> {
        if self.entries.contains_key(&entry.name) {
            return Err(CatalogError::AlreadyExists(entry.name));
        }
        self.entries.insert(entry.name.clone(), entry);
        Ok(())
    }

    fn get(&self, name: &str) -> Option<&CollectionsCatalogEntry> {
        self.entries.get(name)
    }

    fn list(&self) -> Vec<&CollectionsCatalogEntry> {
        self.entries.values().collect()
    }

    fn remove(&mut self, name: &str) -> Option<CollectionsCatalogEntry> {
        self.entries.remove(name)
    }
}

/// Convert a CollectionsConfig into a catalog entry with an IO policy.
pub fn entry_from_config<T>(
    name: impl Into<String>,
    config: &CollectionsConfig<T>,
) -> CollectionsCatalogEntry {
    let io_policy = io_policy_from_config(config);
    CollectionsCatalogEntry {
        name: name.into(),
        value_type: config.element_type.value_type,
        schema: None,
        backend: config.backend.primary,
        extensions: config.extensions.enabled.clone(),
        dataset: config.dataset.clone(),
        io_policy,
    }
}

/// Derive an IO policy from CollectionsConfig.
pub fn io_policy_from_config<T>(config: &CollectionsConfig<T>) -> CollectionsIoPolicy {
    let mut policy = CollectionsIoPolicy::default();
    if let Some(dataset) = &config.dataset {
        policy.location = dataset.data_source.url.clone();
        policy.format = match dataset.data_source.source_type {
            DataSourceType::File => CollectionsIoFormat::Auto,
            DataSourceType::Database => CollectionsIoFormat::Database,
            DataSourceType::API => CollectionsIoFormat::Json,
            DataSourceType::Stream => CollectionsIoFormat::ArrowIpc,
        };
    }

    policy
}
