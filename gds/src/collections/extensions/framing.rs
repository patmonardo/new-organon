//! Collections Framing Extensions
//!
//! Provides DataFrame-style framing over a 1D collection, treating it as
//! a 2D array with row/column access semantics.

use crate::collections::Collections;
use crate::config::{CollectionsBackend, Extension};
use crate::types::ValueType;
use std::marker::PhantomData;

/// Memory order for framing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameOrder {
    /// Row-major layout (row * cols + col)
    RowMajor,
    /// Column-major layout (col * rows + row)
    ColMajor,
}

/// Frame shape metadata
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameShape {
    pub rows: usize,
    pub cols: usize,
    pub order: FrameOrder,
}

impl FrameShape {
    pub fn cell_count(&self) -> usize {
        self.rows.saturating_mul(self.cols)
    }
}

/// Framing configuration
#[derive(Debug, Clone)]
pub struct FramingConfig {
    pub shape: FrameShape,
    pub strict_bounds: bool,
}

impl Default for FramingConfig {
    fn default() -> Self {
        Self {
            shape: FrameShape {
                rows: 0,
                cols: 0,
                order: FrameOrder::RowMajor,
            },
            strict_bounds: true,
        }
    }
}

/// Framing extension trait for Collections
pub trait FramingSupport<T> {
    /// Enable framing with the provided configuration
    fn enable_framing(&mut self, config: FramingConfig) -> Result<(), FramingError>;

    /// Disable framing
    fn disable_framing(&mut self);

    /// Check if framing is enabled
    fn is_framing_enabled(&self) -> bool;

    /// Get current frame shape
    fn frame_shape(&self) -> Option<FrameShape>;

    /// Get a cell value by row/column
    fn get_cell(&self, row: usize, col: usize) -> Result<T, FramingError>;

    /// Set a cell value by row/column
    fn set_cell(&mut self, row: usize, col: usize, value: T) -> Result<(), FramingError>;

    /// Extract a row as values
    fn row_values(&self, row: usize) -> Result<Vec<T>, FramingError>;

    /// Extract a column as values
    fn col_values(&self, col: usize) -> Result<Vec<T>, FramingError>;
}

/// Frame-aware collection wrapper
pub struct FrameCollection<T, C>
where
    C: Collections<T>,
{
    inner: C,
    framing_config: Option<FramingConfig>,
    is_framing_enabled: bool,
    _phantom: PhantomData<T>,
}

impl<T, C> FrameCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    pub fn new(inner: C) -> Self {
        Self {
            inner,
            framing_config: None,
            is_framing_enabled: false,
            _phantom: PhantomData,
        }
    }

    pub fn with_framing_config(inner: C, config: FramingConfig) -> Self {
        Self {
            inner,
            framing_config: Some(config),
            is_framing_enabled: false,
            _phantom: PhantomData,
        }
    }

    fn ensure_shape(&self) -> Result<FrameShape, FramingError> {
        if !self.is_framing_enabled {
            return Err(FramingError::FramingNotEnabled);
        }
        let config = self
            .framing_config
            .as_ref()
            .ok_or(FramingError::MissingConfig)?;
        let shape = config.shape;
        if config.strict_bounds {
            let needed = shape.cell_count();
            if needed > self.inner.len() {
                return Err(FramingError::ShapeMismatch {
                    expected: needed,
                    available: self.inner.len(),
                });
            }
        }
        Ok(shape)
    }

    fn index_for(&self, row: usize, col: usize, shape: FrameShape) -> Result<usize, FramingError> {
        if row >= shape.rows || col >= shape.cols {
            return Err(FramingError::OutOfBounds { row, col });
        }
        let idx = match shape.order {
            FrameOrder::RowMajor => row * shape.cols + col,
            FrameOrder::ColMajor => col * shape.rows + row,
        };
        Ok(idx)
    }
}

impl<T, C> Collections<T> for FrameCollection<T, C>
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
        &[Extension::Framing]
    }

    fn extensions(&self) -> &[Extension] {
        &[Extension::Framing]
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

impl<T, C> FramingSupport<T> for FrameCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    fn enable_framing(&mut self, config: FramingConfig) -> Result<(), FramingError> {
        if config.shape.rows == 0 || config.shape.cols == 0 {
            return Err(FramingError::InvalidShape);
        }
        if config.strict_bounds && config.shape.cell_count() > self.inner.len() {
            return Err(FramingError::ShapeMismatch {
                expected: config.shape.cell_count(),
                available: self.inner.len(),
            });
        }
        self.framing_config = Some(config);
        self.is_framing_enabled = true;
        Ok(())
    }

    fn disable_framing(&mut self) {
        self.framing_config = None;
        self.is_framing_enabled = false;
    }

    fn is_framing_enabled(&self) -> bool {
        self.is_framing_enabled
    }

    fn frame_shape(&self) -> Option<FrameShape> {
        self.framing_config.as_ref().map(|c| c.shape)
    }

    fn get_cell(&self, row: usize, col: usize) -> Result<T, FramingError> {
        let shape = self.ensure_shape()?;
        let idx = self.index_for(row, col, shape)?;
        self.inner.get(idx).ok_or(FramingError::MissingValue(idx))
    }

    fn set_cell(&mut self, row: usize, col: usize, value: T) -> Result<(), FramingError> {
        let shape = self.ensure_shape()?;
        let idx = self.index_for(row, col, shape)?;
        self.inner.set(idx, value);
        Ok(())
    }

    fn row_values(&self, row: usize) -> Result<Vec<T>, FramingError> {
        let shape = self.ensure_shape()?;
        if row >= shape.rows {
            return Err(FramingError::OutOfBounds { row, col: 0 });
        }
        let mut values = Vec::with_capacity(shape.cols);
        for col in 0..shape.cols {
            values.push(self.get_cell(row, col)?);
        }
        Ok(values)
    }

    fn col_values(&self, col: usize) -> Result<Vec<T>, FramingError> {
        let shape = self.ensure_shape()?;
        if col >= shape.cols {
            return Err(FramingError::OutOfBounds { row: 0, col });
        }
        let mut values = Vec::with_capacity(shape.rows);
        for row in 0..shape.rows {
            values.push(self.get_cell(row, col)?);
        }
        Ok(values)
    }
}

/// Framing error types
#[derive(Debug, thiserror::Error)]
pub enum FramingError {
    #[error("Framing not enabled")]
    FramingNotEnabled,
    #[error("Missing framing config")]
    MissingConfig,
    #[error("Invalid frame shape")]
    InvalidShape,
    #[error("Frame shape requires {expected} cells but only {available} available")]
    ShapeMismatch { expected: usize, available: usize },
    #[error("Cell out of bounds at row {row}, col {col}")]
    OutOfBounds { row: usize, col: usize },
    #[error("Missing value at index: {0}")]
    MissingValue(usize),
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
}
