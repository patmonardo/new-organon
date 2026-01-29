//! Collections Indexing Extensions
//!
//! Provides index-building and lookup semantics inspired by Polars IndexArray.
//! This extension adds a secondary lookup structure over a collection.

use crate::collections::Collections;
use crate::config::{CollectionsBackend, Extension};
use crate::types::ValueType;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

/// Indexing extension trait for Collections
pub trait IndexingSupport<T> {
    /// Enable indexing with the provided configuration
    fn enable_indexing(&mut self, config: IndexingConfig) -> Result<(), IndexingError>;

    /// Disable indexing
    fn disable_indexing(&mut self);

    /// Check if indexing is enabled
    fn is_indexing_enabled(&self) -> bool;

    /// Rebuild the index from scratch
    fn rebuild_index(&mut self) -> Result<(), IndexingError>;

    /// Lookup all indices for a given value
    fn lookup(&mut self, value: &T) -> Result<Vec<usize>, IndexingError>;

    /// Lookup the first index for a given value
    fn lookup_first(&mut self, value: &T) -> Result<Option<usize>, IndexingError>;

    /// Get index statistics
    fn index_stats(&self) -> Option<IndexStats>;
}

/// Indexing mode selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexingMode {
    /// HashMap-based indexing
    HashMap,
}

/// Indexing configuration
#[derive(Debug, Clone)]
pub struct IndexingConfig {
    pub mode: IndexingMode,
    pub allow_duplicates: bool,
    pub auto_rebuild_on_set: bool,
}

impl Default for IndexingConfig {
    fn default() -> Self {
        Self {
            mode: IndexingMode::HashMap,
            allow_duplicates: true,
            auto_rebuild_on_set: false,
        }
    }
}

/// Index statistics
#[derive(Debug, Clone)]
pub struct IndexStats {
    pub unique_keys: usize,
    pub total_entries: usize,
    pub duplicate_entries: usize,
    pub is_dirty: bool,
}

/// Index-aware collection wrapper
pub struct IndexCollection<T, C>
where
    C: Collections<T>,
{
    inner: C,
    indexing_config: Option<IndexingConfig>,
    is_indexing_enabled: bool,
    index: Option<HashMap<T, Vec<usize>>>,
    index_dirty: bool,
    _phantom: PhantomData<T>,
}

impl<T, C> IndexCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Eq + Hash + Send + Sync,
{
    pub fn new(inner: C) -> Self {
        Self {
            inner,
            indexing_config: None,
            is_indexing_enabled: false,
            index: None,
            index_dirty: false,
            _phantom: PhantomData,
        }
    }

    pub fn with_indexing_config(inner: C, config: IndexingConfig) -> Self {
        Self {
            inner,
            indexing_config: Some(config),
            is_indexing_enabled: false,
            index: None,
            index_dirty: false,
            _phantom: PhantomData,
        }
    }

    fn ensure_index(&mut self) -> Result<(), IndexingError> {
        if !self.is_indexing_enabled {
            return Err(IndexingError::IndexingNotEnabled);
        }

        if self.index.is_none() || self.index_dirty {
            self.rebuild_index()?;
        }

        Ok(())
    }

    fn build_index(&self, config: &IndexingConfig) -> Result<HashMap<T, Vec<usize>>, IndexingError> {
        let mut map: HashMap<T, Vec<usize>> = HashMap::new();
        for idx in 0..self.inner.len() {
            let value = self
                .inner
                .get(idx)
                .ok_or(IndexingError::MissingValue(idx))?;
            let entry = map.entry(value).or_default();
            if !config.allow_duplicates && !entry.is_empty() {
                continue;
            }
            entry.push(idx);
        }
        Ok(map)
    }

    fn compute_stats(index: &HashMap<T, Vec<usize>>, is_dirty: bool) -> IndexStats {
        let unique_keys = index.len();
        let total_entries: usize = index.values().map(|v| v.len()).sum();
        let duplicate_entries = total_entries.saturating_sub(unique_keys);
        IndexStats {
            unique_keys,
            total_entries,
            duplicate_entries,
            is_dirty,
        }
    }
}

impl<T, C> Collections<T> for IndexCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Eq + Hash + Send + Sync,
{
    fn get(&self, index: usize) -> Option<T> {
        self.inner.get(index)
    }

    fn set(&mut self, index: usize, value: T) {
        self.inner.set(index, value);
        if self.is_indexing_enabled {
            if let Some(config) = self.indexing_config.as_ref() {
                if config.auto_rebuild_on_set {
                    let _ = self.rebuild_index();
                } else {
                    self.index_dirty = true;
                }
            } else {
                self.index_dirty = true;
            }
        }
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn sum(&self) -> Option<T>
    where
        T: std::iter::Sum,
    {
        self.inner.sum()
    }

    fn min(&self) -> Option<T>
    where
        T: Ord,
    {
        self.inner.min()
    }

    fn max(&self) -> Option<T>
    where
        T: Ord,
    {
        self.inner.max()
    }

    fn mean(&self) -> Option<f64> {
        self.inner.mean()
    }

    fn std_dev(&self) -> Option<f64> {
        self.inner.std_dev()
    }

    fn variance(&self) -> Option<f64> {
        self.inner.variance()
    }

    fn median(&self) -> Option<T>
    where
        T: Ord,
    {
        self.inner.median()
    }

    fn percentile(&self, p: f64) -> Option<T>
    where
        T: Ord,
    {
        self.inner.percentile(p)
    }

    fn binary_search(&self, key: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.inner.binary_search(key)
    }

    fn sort(&mut self)
    where
        T: Ord,
    {
        self.inner.sort();
        self.index_dirty = true;
    }

    fn to_vec(self) -> Vec<T> {
        self.inner.to_vec()
    }

    fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }

    fn is_null(&self, index: usize) -> bool {
        self.inner.is_null(index)
    }

    fn null_count(&self) -> usize {
        self.inner.null_count()
    }

    fn default_value(&self) -> T {
        self.inner.default_value()
    }

    fn backend(&self) -> CollectionsBackend {
        self.inner.backend()
    }

    fn features(&self) -> &[Extension] {
        &[Extension::Indexing]
    }

    fn extensions(&self) -> &[Extension] {
        &[Extension::Indexing]
    }

    fn value_type(&self) -> ValueType {
        self.inner.value_type()
    }

    fn with_capacity(_capacity: usize) -> Self
    where
        Self: Sized,
    {
        Self::new(C::with_capacity(_capacity))
    }

    fn with_defaults(_count: usize, _default_value: T) -> Self
    where
        Self: Sized,
    {
        Self::new(C::with_defaults(_count, _default_value))
    }
}

impl<T, C> IndexingSupport<T> for IndexCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Eq + Hash + Send + Sync,
{
    fn enable_indexing(&mut self, config: IndexingConfig) -> Result<(), IndexingError> {
        self.indexing_config = Some(config);
        self.is_indexing_enabled = true;
        self.index_dirty = true;
        self.rebuild_index()?;
        Ok(())
    }

    fn disable_indexing(&mut self) {
        self.indexing_config = None;
        self.is_indexing_enabled = false;
        self.index = None;
        self.index_dirty = false;
    }

    fn is_indexing_enabled(&self) -> bool {
        self.is_indexing_enabled
    }

    fn rebuild_index(&mut self) -> Result<(), IndexingError> {
        if !self.is_indexing_enabled {
            return Err(IndexingError::IndexingNotEnabled);
        }
        let config = self
            .indexing_config
            .as_ref()
            .ok_or(IndexingError::MissingConfig)?;
        let index = self.build_index(config)?;
        self.index = Some(index);
        self.index_dirty = false;
        Ok(())
    }

    fn lookup(&mut self, value: &T) -> Result<Vec<usize>, IndexingError> {
        self.ensure_index()?;
        let index = self
            .index
            .as_ref()
            .ok_or(IndexingError::IndexUnavailable)?;
        Ok(index.get(value).cloned().unwrap_or_default())
    }

    fn lookup_first(&mut self, value: &T) -> Result<Option<usize>, IndexingError> {
        let entries = self.lookup(value)?;
        Ok(entries.into_iter().next())
    }

    fn index_stats(&self) -> Option<IndexStats> {
        self.index
            .as_ref()
            .map(|index| Self::compute_stats(index, self.index_dirty))
    }
}

/// Indexing error types
#[derive(Debug, thiserror::Error)]
pub enum IndexingError {
    #[error("Indexing not enabled")]
    IndexingNotEnabled,
    #[error("Missing indexing config")]
    MissingConfig,
    #[error("Index unavailable")]
    IndexUnavailable,
    #[error("Missing value at index: {0}")]
    MissingValue(usize),
}

/// Indexing utilities
pub struct IndexingUtils;

impl IndexingUtils {
    /// Estimate memory usage for index
    pub fn estimate_index_memory<T: Hash + Eq>(element_count: usize) -> usize {
        let entry_overhead = std::mem::size_of::<T>() + std::mem::size_of::<usize>();
        element_count * entry_overhead
    }
}
