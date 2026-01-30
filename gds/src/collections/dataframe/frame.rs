//! Framing adapter for Polars DataFrame (Collections framing API).

use polars::prelude::{AnyValue, DataFrame};

use crate::collections::extensions::framing::{
    FrameShape, FramingConfig, FramingError, FramingSupport,
};

/// Framing adapter over a Polars DataFrame.
#[derive(Debug, Clone)]
pub struct PolarsDataFrame {
    df: DataFrame,
    framing_config: Option<FramingConfig>,
    is_framing_enabled: bool,
}

impl PolarsDataFrame {
    pub fn new(df: DataFrame) -> Self {
        Self {
            df,
            framing_config: None,
            is_framing_enabled: false,
        }
    }

    pub fn dataframe(&self) -> &DataFrame {
        &self.df
    }

    pub fn into_inner(self) -> DataFrame {
        self.df
    }

    fn resolve_shape(&self) -> Result<FrameShape, FramingError> {
        let config = self
            .framing_config
            .as_ref()
            .ok_or(FramingError::MissingConfig)?;

        let mut shape = config.shape;
        let rows = self.df.height();
        let cols = self.df.width();

        if shape.rows == 0 {
            shape.rows = rows;
        }
        if shape.cols == 0 {
            shape.cols = cols;
        }

        if shape.rows != rows || shape.cols != cols {
            return Err(FramingError::InvalidShape);
        }

        Ok(shape)
    }

    fn ensure_shape(&self) -> Result<FrameShape, FramingError> {
        if !self.is_framing_enabled {
            return Err(FramingError::FramingNotEnabled);
        }
        self.resolve_shape()
    }
}

impl From<DataFrame> for PolarsDataFrame {
    fn from(df: DataFrame) -> Self {
        Self::new(df)
    }
}

impl FramingSupport<AnyValue<'static>> for PolarsDataFrame {
    fn enable_framing(&mut self, config: FramingConfig) -> Result<(), FramingError> {
        self.framing_config = Some(config);
        self.is_framing_enabled = true;
        self.resolve_shape()?;
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
        if !self.is_framing_enabled {
            return None;
        }
        self.resolve_shape().ok()
    }

    fn get_cell(&self, row: usize, col: usize) -> Result<AnyValue<'static>, FramingError> {
        let shape = self.ensure_shape()?;
        if row >= shape.rows || col >= shape.cols {
            return Err(FramingError::OutOfBounds { row, col });
        }
        let row_values = self.df.get(row).ok_or(FramingError::MissingValue(row))?;
        let value = row_values
            .get(col)
            .ok_or(FramingError::MissingValue(row))?
            .clone()
            .into_static();
        Ok(value)
    }

    fn set_cell(
        &mut self,
        _row: usize,
        _col: usize,
        _value: AnyValue<'static>,
    ) -> Result<(), FramingError> {
        Err(FramingError::UnsupportedOperation(
            "Polars DataFrame framing is read-only; mutate via Polars APIs".to_string(),
        ))
    }

    fn row_values(&self, row: usize) -> Result<Vec<AnyValue<'static>>, FramingError> {
        let shape = self.ensure_shape()?;
        if row >= shape.rows {
            return Err(FramingError::OutOfBounds { row, col: 0 });
        }
        let values = self
            .df
            .get(row)
            .ok_or(FramingError::MissingValue(row))?
            .into_iter()
            .map(|value| value.into_static())
            .collect();
        Ok(values)
    }

    fn col_values(&self, col: usize) -> Result<Vec<AnyValue<'static>>, FramingError> {
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
