//! Polars DataFrame integration for Collections.

use polars::error::PolarsError;
use polars::prelude::{Column, DataFrame, DataType, Series};

pub use polars::prelude::Column as PolarsColumn;
pub use polars::prelude::DataType as PolarsDataType;
pub use polars::prelude::Series as PolarsSeries;

/// Trait for DataFrame-backed Collections with full Polars access.
pub trait DataFrameCollection: Send + Sync {
    /// Immutable access to the underlying DataFrame.
    fn dataframe(&self) -> &DataFrame;

    /// Mutable access to the underlying DataFrame.
    fn dataframe_mut(&mut self) -> &mut DataFrame;

    /// Number of rows in the DataFrame.
    fn row_count(&self) -> usize {
        self.dataframe().height()
    }

    /// Number of columns in the DataFrame.
    fn column_count(&self) -> usize {
        self.dataframe().width()
    }

    /// Column names.
    fn column_names(&self) -> Vec<String> {
        self.dataframe()
            .get_column_names()
            .iter()
            .map(|name| name.to_string())
            .collect()
    }

    /// Column data types.
    fn dtypes(&self) -> Vec<DataType> {
        self.dataframe()
            .get_columns()
            .iter()
            .map(|series| series.dtype().clone())
            .collect()
    }

    /// Access a column by name.
    fn column(&self, name: &str) -> Result<&Column, PolarsError> {
        self.dataframe().column(name)
    }

    /// Select a subset of columns.
    fn select(&self, columns: &[&str]) -> Result<DataFrame, PolarsError> {
        let selection: Vec<&str> = columns.iter().copied().collect();
        self.dataframe().select(selection)
    }
}

/// Polars-backed DataFrame collection wrapper.
#[derive(Debug, Clone)]
pub struct PolarsDataFrameCollection {
    df: DataFrame,
}

impl PolarsDataFrameCollection {
    pub fn new(df: DataFrame) -> Self {
        Self { df }
    }

    pub fn dataframe(&self) -> &DataFrame {
        &self.df
    }

    pub fn dataframe_mut(&mut self) -> &mut DataFrame {
        &mut self.df
    }

    pub fn from_series(columns: Vec<Series>) -> Result<Self, PolarsError> {
        let cols: Vec<Column> = columns.into_iter().map(Column::from).collect();
        let df = DataFrame::new(cols)?;
        Ok(Self { df })
    }

    pub fn into_inner(self) -> DataFrame {
        self.df
    }
}

impl From<DataFrame> for PolarsDataFrameCollection {
    fn from(df: DataFrame) -> Self {
        Self::new(df)
    }
}

impl DataFrameCollection for PolarsDataFrameCollection {
    fn dataframe(&self) -> &DataFrame {
        &self.df
    }

    fn dataframe_mut(&mut self) -> &mut DataFrame {
        &mut self.df
    }
}
