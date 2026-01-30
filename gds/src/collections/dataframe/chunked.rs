//! Polars Series -> ChunkedArray integration.
//!
//! This layer bridges Polars Series chunking semantics with Collections
//! chunking utilities and configuration.

use polars::prelude::Series;

use crate::collections::extensions::chunking::{
    ChunkRange, ChunkingConfig, ChunkingError, ChunkingUtils,
};

/// Chunked Series view built on Collections chunking configuration.
#[derive(Debug, Clone)]
pub struct PolarsChunkedSeries {
    series: Series,
    chunking_config: Option<ChunkingConfig>,
    is_chunking_enabled: bool,
}

impl PolarsChunkedSeries {
    pub fn new(series: Series) -> Self {
        Self {
            series,
            chunking_config: None,
            is_chunking_enabled: false,
        }
    }

    pub fn series(&self) -> &Series {
        &self.series
    }

    pub fn into_series(self) -> Series {
        self.series
    }

    pub fn enable_chunking(&mut self, config: ChunkingConfig) -> Result<(), ChunkingError> {
        if config.chunk_size == 0 {
            return Err(ChunkingError::InvalidChunkSize(
                "chunk_size must be > 0".to_string(),
            ));
        }
        self.chunking_config = Some(config);
        self.is_chunking_enabled = true;
        Ok(())
    }

    pub fn disable_chunking(&mut self) {
        self.chunking_config = None;
        self.is_chunking_enabled = false;
    }

    pub fn is_chunking_enabled(&self) -> bool {
        self.is_chunking_enabled
    }

    pub fn chunk_size(&self) -> Option<usize> {
        self.chunking_config
            .as_ref()
            .map(|config| normalize_chunk_size(config))
    }

    pub fn chunk_count(&self) -> usize {
        if !self.is_chunking_enabled {
            return 1;
        }
        match self.chunk_ranges() {
            Ok(ranges) => ranges.len(),
            Err(_) => 0,
        }
    }

    pub fn chunk_ranges(&self) -> Result<Vec<ChunkRange>, ChunkingError> {
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

        let chunk_size = normalize_chunk_size(config);
        let len = self.series.len();
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

    /// Get the range for a specific chunk index.
    pub fn chunk_range(&self, chunk_index: usize) -> Result<ChunkRange, ChunkingError> {
        let ranges = self.chunk_ranges()?;
        ranges
            .get(chunk_index)
            .copied()
            .ok_or_else(|| ChunkingError::ChunkOutOfBounds(chunk_index))
    }

    /// Materialize all chunks as a vector of Series slices.
    pub fn chunks(&self) -> Result<Vec<Series>, ChunkingError> {
        let ranges = self.chunk_ranges()?;
        let mut chunks = Vec::with_capacity(ranges.len());
        for range in ranges {
            chunks.push(self.series.slice(range.start as i64, range.len()));
        }
        Ok(chunks)
    }

    /// Get a chunk as a Series slice (Collections-oriented view).
    pub fn get_chunk(&self, chunk_index: usize) -> Result<Series, ChunkingError> {
        let ranges = self.chunk_ranges()?;
        let range = ranges
            .get(chunk_index)
            .ok_or_else(|| ChunkingError::ChunkOutOfBounds(chunk_index))?;

        Ok(self.series.slice(range.start as i64, range.len()))
    }

    /// Iterate over chunks and pass each chunk to the provided callback.
    pub fn for_each_chunk<F>(&self, mut f: F) -> Result<(), ChunkingError>
    where
        F: FnMut(ChunkRange, Series),
    {
        let ranges = self.chunk_ranges()?;
        for (chunk_index, range) in ranges.into_iter().enumerate() {
            let chunk = self.get_chunk(chunk_index)?;
            f(range, chunk);
        }
        Ok(())
    }

    /// Estimate memory usage using Collections chunking utilities.
    pub fn estimate_memory(&self) -> Option<usize> {
        self.chunk_size()
            .map(|chunk| ChunkingUtils::estimate_memory::<u8>(self.series.len(), chunk))
    }
}

impl From<Series> for PolarsChunkedSeries {
    fn from(series: Series) -> Self {
        Self::new(series)
    }
}

fn normalize_chunk_size(config: &ChunkingConfig) -> usize {
    if config.prefer_power_of_two {
        ChunkingUtils::nearest_power_of_two(config.chunk_size)
    } else {
        config.chunk_size
    }
}
