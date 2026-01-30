//! Row helpers for Polars-backed DataFrames.

use polars::frame::row::Row;
use polars::prelude::AnyValue;

pub type PolarsRow<'a> = Row<'a>;
pub type RowValue = AnyValue<'static>;

pub fn row_to_owned(row: &Row) -> Vec<AnyValue<'static>> {
    row.0
        .iter()
        .map(|value: &AnyValue| value.clone().into_static())
        .collect()
}
