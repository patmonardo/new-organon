//! Table builders and IO helpers for Polars-backed DataFrames.

use std::path::Path;

use polars::prelude::{
    Column, CsvReadOptions, CsvWriter, DataFrame, IpcReader, IpcWriter, ParquetReader,
    ParquetWriter, SerReader, SerWriter, Series,
};

use crate::collections::dataframe::collection::PolarsDataFrameCollection;
use crate::collections::dataframe::column::{
    column_bool, column_f32, column_f64, column_i32, column_i64, column_string,
};

/// Builder for creating a DataFrame-backed table without exposing Polars types.
#[derive(Debug, Default)]
pub struct TableBuilder {
    columns: Vec<Column>,
}

impl TableBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_i64_column(mut self, name: &str, values: &[i64]) -> Self {
        self.columns.push(column_i64(name, values));
        self
    }

    pub fn with_i32_column(mut self, name: &str, values: &[i32]) -> Self {
        self.columns.push(column_i32(name, values));
        self
    }

    pub fn with_f64_column(mut self, name: &str, values: &[f64]) -> Self {
        self.columns.push(column_f64(name, values));
        self
    }

    pub fn with_f32_column(mut self, name: &str, values: &[f32]) -> Self {
        self.columns.push(column_f32(name, values));
        self
    }

    pub fn with_bool_column(mut self, name: &str, values: &[bool]) -> Self {
        self.columns.push(column_bool(name, values));
        self
    }

    pub fn with_string_column(mut self, name: &str, values: &[String]) -> Self {
        self.columns.push(column_string(name, values));
        self
    }

    pub fn build(self) -> Result<PolarsDataFrameCollection, polars::error::PolarsError> {
        let df = DataFrame::new(self.columns)?;
        Ok(PolarsDataFrameCollection::from(df))
    }
}

/// Write a table to Parquet (Polars-backed).
pub fn write_table_parquet(
    path: &Path,
    table: &PolarsDataFrameCollection,
) -> Result<(), polars::error::PolarsError> {
    let mut df = table.dataframe().clone();
    let mut file = std::fs::File::create(path)?;
    ParquetWriter::new(&mut file).finish(&mut df)?;
    Ok(())
}

/// Read a table from Parquet (Polars-backed).
pub fn read_table_parquet(
    path: &Path,
) -> Result<PolarsDataFrameCollection, polars::error::PolarsError> {
    let mut file = std::fs::File::open(path)?;
    let df = ParquetReader::new(&mut file).finish()?;
    Ok(PolarsDataFrameCollection::from(df))
}

/// Write a table to CSV (Polars-backed).
pub fn write_table_csv(
    path: &Path,
    table: &PolarsDataFrameCollection,
) -> Result<(), polars::error::PolarsError> {
    let mut df = table.dataframe().clone();
    let mut file = std::fs::File::create(path)?;
    CsvWriter::new(&mut file)
        .include_header(true)
        .finish(&mut df)?;
    Ok(())
}

/// Write a table to Arrow IPC (Polars-backed).
pub fn write_table_ipc(
    path: &Path,
    table: &PolarsDataFrameCollection,
) -> Result<(), polars::error::PolarsError> {
    let mut df = table.dataframe().clone();
    let mut file = std::fs::File::create(path)?;
    IpcWriter::new(&mut file).finish(&mut df)?;
    Ok(())
}

/// Read a table from CSV (Polars-backed).
pub fn read_table_csv(
    path: &Path,
) -> Result<PolarsDataFrameCollection, polars::error::PolarsError> {
    let mut file = std::fs::File::open(path)?;
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .into_reader_with_file_handle(&mut file)
        .finish()?;
    Ok(PolarsDataFrameCollection::from(df))
}

/// Read a table from Arrow IPC (Polars-backed).
pub fn read_table_ipc(
    path: &Path,
) -> Result<PolarsDataFrameCollection, polars::error::PolarsError> {
    let mut file = std::fs::File::open(path)?;
    let df = IpcReader::new(&mut file).finish()?;
    Ok(PolarsDataFrameCollection::from(df))
}

/// Multiply an f64 column by a scalar factor.
pub fn scale_f64_column(
    table: &mut PolarsDataFrameCollection,
    column_name: &str,
    factor: f64,
) -> Result<(), polars::error::PolarsError> {
    let df = table.dataframe_mut();
    let column = df.column(column_name)?;
    let series = column.as_materialized_series();
    let scaled = series
        .f64()?
        .into_no_null_iter()
        .map(|value| value * factor)
        .collect::<Vec<_>>();
    let mut new_series: Series = scaled.into_iter().collect();
    new_series.rename(column_name.into());
    df.replace(column_name, new_series)?;
    Ok(())
}
