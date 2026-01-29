//! Collections Chunking Extensions
//!
//! Provides chunked access patterns similar to Polars ChunkedArray semantics.
//!
//! Polars Series integration lives in `collections::dataframe::chunked`.
//! Chunking is purely a view over the underlying collection, enabling
//! chunk-level iteration and sizing policy without changing storage backend.

use crate::collections::Collections;
use crate::config::{CollectionsBackend, Extension};
use crate::types::ValueType;
use std::marker::PhantomData;

/// Chunking extension trait for Collections
pub trait ChunkingSupport<T> {
    /// Enable chunking with the provided configuration
    fn enable_chunking(&mut self, config: ChunkingConfig) -> Result<(), ChunkingError>;

    /// Disable chunking
    fn disable_chunking(&mut self);

    /// Check if chunking is enabled
    fn is_chunking_enabled(&self) -> bool;

    /// Get configured chunk size
    fn chunk_size(&self) -> Option<usize>;

    /// Get number of chunks
    fn chunk_count(&self) -> usize;

    /// Get chunk ranges
    fn chunk_ranges(&self) -> Result<Vec<ChunkRange>, ChunkingError>;

    /// Get a chunk's values
    fn get_chunk(&self, chunk_index: usize) -> Result<Vec<T>, ChunkingError>;

    /// Iterate over chunks and pass each chunk to the provided callback
    fn for_each_chunk<F>(&self, f: F) -> Result<(), ChunkingError>
    where
        F: FnMut(ChunkRange, Vec<T>);
}

/// Chunk range metadata
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkRange {
    pub start: usize,
    pub end: usize,
}

impl ChunkRange {
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
}

/// Chunking configuration
#[derive(Debug, Clone)]
pub struct ChunkingConfig {
    pub chunk_size: usize,
    pub max_chunks: Option<usize>,
    pub prefer_power_of_two: bool,
    pub allow_tail: bool,
}

impl Default for ChunkingConfig {
    fn default() -> Self {
        Self {
            chunk_size: 1024,
            max_chunks: None,
            prefer_power_of_two: true,
            allow_tail: true,
        }
    }
}

/// Chunked collection wrapper
pub struct ChunkedCollection<T, C>
where
    C: Collections<T>,
{
    inner: C,
    chunking_config: Option<ChunkingConfig>,
    is_chunking_enabled: bool,
    _phantom: PhantomData<T>,
}

impl<T, C> ChunkedCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    pub fn new(inner: C) -> Self {
        Self {
            inner,
            chunking_config: None,
            is_chunking_enabled: false,
            _phantom: PhantomData,
        }
    }

    pub fn with_chunking_config(inner: C, config: ChunkingConfig) -> Self {
        Self {
            inner,
            chunking_config: Some(config),
            is_chunking_enabled: false,
            _phantom: PhantomData,
        }
    }

    fn normalize_chunk_size(config: &ChunkingConfig) -> usize {
        if config.prefer_power_of_two {
            ChunkingUtils::nearest_power_of_two(config.chunk_size)
        } else {
            config.chunk_size
        }
    }

    fn compute_ranges(&self) -> Result<Vec<ChunkRange>, ChunkingError> {
        if !self.is_chunking_enabled {
            return Err(ChunkingError::ChunkingNotEnabled);
        }

        let config = self
            .chunking_config
            .as_ref()
            .ok_or(ChunkingError::MissingConfig)?;

        if config.chunk_size == 0 {
            return Err(ChunkingError::InvalidChunkSize(
                "chunk_size must be > 0".to_string(),
            ));
        }

        let chunk_size = Self::normalize_chunk_size(config);
        let len = self.inner.len();
        if len == 0 {
            return Ok(vec![]);
        }

        let mut ranges = Vec::new();
        let mut start = 0usize;
        while start < len {
            let end = (start + chunk_size).min(len);
            if end == len && !config.allow_tail && end - start < chunk_size {
                break;
            }

            ranges.push(ChunkRange { start, end });
            start = end;

            if let Some(max_chunks) = config.max_chunks {
                if ranges.len() >= max_chunks {
                    break;
                }
            }
        }

        Ok(ranges)
    }
}

impl<T, C> Collections<T> for ChunkedCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    fn get(&self, index: usize) -> Option<T> {
        self.inner.get(index)
    }

    fn set(&mut self, index: usize, value: T) {
        self.inner.set(index, value);
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
        self.inner.sort()
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
        &[Extension::Chunking]
    }

    fn extensions(&self) -> &[Extension] {
        &[Extension::Chunking]
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

impl<T, C> ChunkingSupport<T> for ChunkedCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    fn enable_chunking(&mut self, config: ChunkingConfig) -> Result<(), ChunkingError> {
        if config.chunk_size == 0 {
            return Err(ChunkingError::InvalidChunkSize(
                "chunk_size must be > 0".to_string(),
            ));
        }

        self.chunking_config = Some(config);
        self.is_chunking_enabled = true;
        Ok(())
    }

    fn disable_chunking(&mut self) {
        self.chunking_config = None;
        self.is_chunking_enabled = false;
    }

    fn is_chunking_enabled(&self) -> bool {
        self.is_chunking_enabled
    }

    fn chunk_size(&self) -> Option<usize> {
        self.chunking_config
            .as_ref()
            .map(|c| Self::normalize_chunk_size(c))
    }

    fn chunk_count(&self) -> usize {
        if !self.is_chunking_enabled {
            return 1;
        }
        if let Ok(ranges) = self.compute_ranges() {
            ranges.len()
        } else {
            0
        }
    }

    fn chunk_ranges(&self) -> Result<Vec<ChunkRange>, ChunkingError> {
        self.compute_ranges()
    }

    fn get_chunk(&self, chunk_index: usize) -> Result<Vec<T>, ChunkingError> {
        let ranges = self.compute_ranges()?;
        let range = ranges
            .get(chunk_index)
            .ok_or_else(|| ChunkingError::ChunkOutOfBounds(chunk_index))?;

        let mut values = Vec::with_capacity(range.len());
        for idx in range.start..range.end {
            if let Some(value) = self.inner.get(idx) {
                values.push(value);
            } else {
                return Err(ChunkingError::MissingValue(idx));
            }
        }

        Ok(values)
    }

    fn for_each_chunk<F>(&self, mut f: F) -> Result<(), ChunkingError>
    where
        F: FnMut(ChunkRange, Vec<T>),
    {
        let ranges = self.compute_ranges()?;
        for (chunk_index, range) in ranges.into_iter().enumerate() {
            let values = self.get_chunk(chunk_index)?;
            f(range, values);
        }
        Ok(())
    }
}

/// Chunking error types
#[derive(Debug, thiserror::Error)]
pub enum ChunkingError {
    #[error("Chunking not enabled")]
    ChunkingNotEnabled,
    #[error("Missing chunking config")]
    MissingConfig,
    #[error("Invalid chunk size: {0}")]
    InvalidChunkSize(String),
    #[error("Chunk index out of bounds: {0}")]
    ChunkOutOfBounds(usize),
    #[error("Missing value at index: {0}")]
    MissingValue(usize),
}

/// Chunking utilities
pub struct ChunkingUtils;

impl ChunkingUtils {
    /// Estimate memory usage for chunked collection
    pub fn estimate_memory<T>(element_count: usize, chunk_size: usize) -> usize {
        let chunk_overhead = std::mem::size_of::<ChunkRange>();
        let chunk_count = element_count.div_ceil(chunk_size.max(1));
        let data_size = element_count * std::mem::size_of::<T>();
        data_size + (chunk_count * chunk_overhead)
    }

    /// Adjust chunk size to nearest power of two
    pub fn nearest_power_of_two(mut value: usize) -> usize {
        if value == 0 {
            return 1;
        }
        value = value.next_power_of_two();
        value
    }

    // Intentionally empty for now; reserved for future chunk indexing helpers.
}
