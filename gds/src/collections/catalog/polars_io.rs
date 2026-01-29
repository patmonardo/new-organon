//! Polars IO helpers for Collections.

use std::fs::File;
use std::path::Path;

use polars::prelude::{
    BooleanChunked, Column, CsvReadOptions, CsvWriter, DataFrame, Float32Chunked, Float64Chunked,
    Int32Chunked, Int64Chunked, IntoSeries, IpcReader, IpcWriter, NewChunkedArray, ParquetReader,
    ParquetWriter, SerReader, SerWriter, Series, StringChunked,
};

use crate::collections::catalog::types::CatalogError;
use crate::collections::Collections;

/// Conversion layer between Collections values and Polars Series.
pub trait PolarsCollectionType: Sized {
    fn to_series(name: &str, values: &[Self]) -> Series;
    fn from_series(series: &Series) -> Result<Vec<Self>, CatalogError>;
}

impl From<polars::error::PolarsError> for CatalogError {
    fn from(err: polars::error::PolarsError) -> Self {
        CatalogError::Polars(err.to_string())
    }
}

fn ensure_no_nulls(null_count: usize, series_name: &str) -> Result<(), CatalogError> {
    if null_count > 0 {
        return Err(CatalogError::Polars(format!(
            "Series '{series_name}' contains nulls which are not supported by Collections IO",
        )));
    }
    Ok(())
}

fn dataframe_from_series(series: Series) -> Result<DataFrame, CatalogError> {
    DataFrame::new(vec![Column::from(series)]).map_err(CatalogError::from)
}

impl PolarsCollectionType for i64 {
    fn to_series(name: &str, values: &[Self]) -> Series {
        Int64Chunked::from_slice(name.into(), values).into_series()
    }

    fn from_series(series: &Series) -> Result<Vec<Self>, CatalogError> {
        let chunk = series.i64().map_err(CatalogError::from)?;
        ensure_no_nulls(chunk.null_count(), series.name())?;
        Ok(chunk.into_no_null_iter().collect())
    }
}

impl PolarsCollectionType for i32 {
    fn to_series(name: &str, values: &[Self]) -> Series {
        Int32Chunked::from_slice(name.into(), values).into_series()
    }

    fn from_series(series: &Series) -> Result<Vec<Self>, CatalogError> {
        let chunk = series.i32().map_err(CatalogError::from)?;
        ensure_no_nulls(chunk.null_count(), series.name())?;
        Ok(chunk.into_no_null_iter().collect())
    }
}

impl PolarsCollectionType for f64 {
    fn to_series(name: &str, values: &[Self]) -> Series {
        Float64Chunked::from_slice(name.into(), values).into_series()
    }

    fn from_series(series: &Series) -> Result<Vec<Self>, CatalogError> {
        let chunk = series.f64().map_err(CatalogError::from)?;
        ensure_no_nulls(chunk.null_count(), series.name())?;
        Ok(chunk.into_no_null_iter().collect())
    }
}

impl PolarsCollectionType for f32 {
    fn to_series(name: &str, values: &[Self]) -> Series {
        Float32Chunked::from_slice(name.into(), values).into_series()
    }

    fn from_series(series: &Series) -> Result<Vec<Self>, CatalogError> {
        let chunk = series.f32().map_err(CatalogError::from)?;
        ensure_no_nulls(chunk.null_count(), series.name())?;
        Ok(chunk.into_no_null_iter().collect())
    }
}

impl PolarsCollectionType for bool {
    fn to_series(name: &str, values: &[Self]) -> Series {
        BooleanChunked::from_slice(name.into(), values).into_series()
    }

    fn from_series(series: &Series) -> Result<Vec<Self>, CatalogError> {
        let chunk = series.bool().map_err(CatalogError::from)?;
        ensure_no_nulls(chunk.null_count(), series.name())?;
        Ok(chunk.into_no_null_iter().collect())
    }
}

impl PolarsCollectionType for String {
    fn to_series(name: &str, values: &[Self]) -> Series {
        let refs: Vec<&str> = values.iter().map(|value| value.as_str()).collect();
        StringChunked::from_slice(name.into(), &refs).into_series()
    }

    fn from_series(series: &Series) -> Result<Vec<Self>, CatalogError> {
        let chunk = series.str().map_err(CatalogError::from)?;
        ensure_no_nulls(chunk.null_count(), series.name())?;
        Ok(chunk
            .into_no_null_iter()
            .map(|value: &str| value.to_string())
            .collect())
    }
}

/// Write a single-column collection to Parquet via Polars.
pub fn write_collection_parquet<T, C>(
    path: &Path,
    column_name: &str,
    collection: &C,
) -> Result<(), CatalogError>
where
    T: PolarsCollectionType + Clone,
    C: Collections<T>,
{
    let values = collection.as_slice();
    let series = T::to_series(column_name, values);
    let mut df = dataframe_from_series(series)?;
    let mut file = File::create(path).map_err(|e| CatalogError::Io(e.to_string()))?;
    ParquetWriter::new(&mut file)
        .finish(&mut df)
        .map_err(CatalogError::from)?;
    Ok(())
}

/// Read a single-column collection from Parquet via Polars.
pub fn read_collection_parquet<T>(path: &Path, column_name: &str) -> Result<Vec<T>, CatalogError>
where
    T: PolarsCollectionType,
{
    let mut file = File::open(path).map_err(|e| CatalogError::Io(e.to_string()))?;
    let df = ParquetReader::new(&mut file)
        .finish()
        .map_err(CatalogError::from)?;
    let series = df.column(column_name).map_err(CatalogError::from)?;
    let series = series.as_series().ok_or_else(|| {
        CatalogError::Polars(format!("Column '{column_name}' is not a Series column"))
    })?;
    T::from_series(series)
}

/// Write a single-column collection to Arrow IPC via Polars.
pub fn write_collection_ipc<T, C>(
    path: &Path,
    column_name: &str,
    collection: &C,
) -> Result<(), CatalogError>
where
    T: PolarsCollectionType + Clone,
    C: Collections<T>,
{
    let values = collection.as_slice();
    let series = T::to_series(column_name, values);
    let mut df = dataframe_from_series(series)?;
    let mut file = File::create(path).map_err(|e| CatalogError::Io(e.to_string()))?;
    IpcWriter::new(&mut file)
        .finish(&mut df)
        .map_err(CatalogError::from)?;
    Ok(())
}

/// Read a single-column collection from Arrow IPC via Polars.
pub fn read_collection_ipc<T>(path: &Path, column_name: &str) -> Result<Vec<T>, CatalogError>
where
    T: PolarsCollectionType,
{
    let mut file = File::open(path).map_err(|e| CatalogError::Io(e.to_string()))?;
    let df = IpcReader::new(&mut file)
        .finish()
        .map_err(CatalogError::from)?;
    let series = df.column(column_name).map_err(CatalogError::from)?;
    let series = series.as_series().ok_or_else(|| {
        CatalogError::Polars(format!("Column '{column_name}' is not a Series column"))
    })?;
    T::from_series(series)
}

/// Write a single-column collection to CSV via Polars.
pub fn write_collection_csv<T, C>(
    path: &Path,
    column_name: &str,
    collection: &C,
) -> Result<(), CatalogError>
where
    T: PolarsCollectionType + Clone,
    C: Collections<T>,
{
    let values = collection.as_slice();
    let series = T::to_series(column_name, values);
    let mut df = dataframe_from_series(series)?;
    let mut file = File::create(path).map_err(|e| CatalogError::Io(e.to_string()))?;
    CsvWriter::new(&mut file)
        .include_header(true)
        .finish(&mut df)
        .map_err(CatalogError::from)?;
    Ok(())
}

/// Read a single-column collection from CSV via Polars.
pub fn read_collection_csv<T>(path: &Path, column_name: &str) -> Result<Vec<T>, CatalogError>
where
    T: PolarsCollectionType,
{
    let mut file = File::open(path).map_err(|e| CatalogError::Io(e.to_string()))?;
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .into_reader_with_file_handle(&mut file)
        .finish()
        .map_err(CatalogError::from)?;
    let series = df.column(column_name).map_err(CatalogError::from)?;
    let series = series.as_series().ok_or_else(|| {
        CatalogError::Polars(format!("Column '{column_name}' is not a Series column"))
    })?;
    T::from_series(series)
}
