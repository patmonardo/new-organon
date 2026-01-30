//! Polars DataFrame integration for Collections.

pub mod chunked;
pub mod collection;
pub mod column;
pub mod frame;
pub mod row;
pub mod series;
pub mod table;

pub use chunked::PolarsChunkedSeries;
pub use collection::{
    DataFrameCollection, PolarsColumn, PolarsDataFrameCollection, PolarsDataType, PolarsSeries,
};
pub use column::{column_bool, column_f32, column_f64, column_i32, column_i64, column_string};
pub use frame::PolarsDataFrame;
pub use row::{row_to_owned, PolarsRow, RowValue};
pub use series::{series_bool, series_f32, series_f64, series_i32, series_i64, series_string};
pub use table::{
    read_table_csv, read_table_ipc, read_table_parquet, scale_f64_column, write_table_csv,
    write_table_ipc, write_table_parquet, TableBuilder,
};
