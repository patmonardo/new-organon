//! Column builders for DataFrame-backed tables.

use polars::prelude::Column;

use crate::collections::dataframe::series::{
    series_bool, series_f32, series_f64, series_i32, series_i64, series_string,
};

pub fn column_i64(name: &str, values: &[i64]) -> Column {
    Column::from(series_i64(name, values))
}

pub fn column_i32(name: &str, values: &[i32]) -> Column {
    Column::from(series_i32(name, values))
}

pub fn column_f64(name: &str, values: &[f64]) -> Column {
    Column::from(series_f64(name, values))
}

pub fn column_f32(name: &str, values: &[f32]) -> Column {
    Column::from(series_f32(name, values))
}

pub fn column_bool(name: &str, values: &[bool]) -> Column {
    Column::from(series_bool(name, values))
}

pub fn column_string(name: &str, values: &[String]) -> Column {
    Column::from(series_string(name, values))
}
