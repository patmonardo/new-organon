//! High-level Dataset wrapper over Polars-backed tables.
//!
//! This is intentionally lightweight and ergonomic for “dataset-first” workflows
//! without forcing callers to pull in Polars types directly.

use std::path::Path;

use polars::error::PolarsError;
use polars::prelude::{Expr, SortMultipleOptions};

use crate::collections::dataframe::collection::{DataFrameCollection, PolarsDataFrameCollection};
use crate::collections::dataframe::table::{
    read_table_csv, read_table_ipc, read_table_parquet, write_table_csv, write_table_ipc,
    write_table_parquet, TableBuilder,
};

/// Minimal dataset wrapper (Polars-backed).
#[derive(Debug, Clone)]
pub struct Dataset {
    name: Option<String>,
    table: PolarsDataFrameCollection,
}

impl Dataset {
    pub fn new(table: PolarsDataFrameCollection) -> Self {
        Self { name: None, table }
    }

    pub fn named(name: impl Into<String>, table: PolarsDataFrameCollection) -> Self {
        Self {
            name: Some(name.into()),
            table,
        }
    }

    pub fn from_builder(builder: TableBuilder) -> Result<Self, PolarsError> {
        let table = builder.build()?;
        Ok(Self::new(table))
    }

    pub fn from_csv(path: impl AsRef<Path>) -> Result<Self, PolarsError> {
        let table = read_table_csv(path.as_ref())?;
        Ok(Self::new(table))
    }

    pub fn from_parquet(path: impl AsRef<Path>) -> Result<Self, PolarsError> {
        let table = read_table_parquet(path.as_ref())?;
        Ok(Self::new(table))
    }

    pub fn from_ipc(path: impl AsRef<Path>) -> Result<Self, PolarsError> {
        let table = read_table_ipc(path.as_ref())?;
        Ok(Self::new(table))
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn table(&self) -> &PolarsDataFrameCollection {
        &self.table
    }

    pub fn table_mut(&mut self) -> &mut PolarsDataFrameCollection {
        &mut self.table
    }

    pub fn into_table(self) -> PolarsDataFrameCollection {
        self.table
    }

    pub fn row_count(&self) -> usize {
        self.table.row_count()
    }

    pub fn column_count(&self) -> usize {
        self.table.column_count()
    }

    pub fn column_names(&self) -> Vec<String> {
        self.table.column_names()
    }

    pub fn dtypes(&self) -> Vec<polars::prelude::DataType> {
        self.table.dtypes()
    }

    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    pub fn head(&self, n: usize) -> Self {
        let table = self.table.head(n);
        Self {
            name: self.name.clone(),
            table,
        }
    }

    pub fn tail(&self, n: usize) -> Self {
        let table = self.table.tail(n);
        Self {
            name: self.name.clone(),
            table,
        }
    }

    pub fn slice(&self, offset: i64, length: usize) -> Self {
        let table = self.table.slice(offset, length);
        Self {
            name: self.name.clone(),
            table,
        }
    }

    pub fn select_columns(&self, columns: &[&str]) -> Result<Self, PolarsError> {
        let table = self.table.select_columns(columns)?;
        Ok(Self {
            name: self.name.clone(),
            table,
        })
    }

    /// Python-Polars alias for select columns.
    pub fn select(&self, columns: &[&str]) -> Result<Self, PolarsError> {
        self.select_columns(columns)
    }

    pub fn select_exprs(&self, exprs: &[Expr]) -> Result<Self, PolarsError> {
        let table = self.table.select_exprs(exprs)?;
        Ok(Self {
            name: self.name.clone(),
            table,
        })
    }

    pub fn filter_expr(&self, predicate: Expr) -> Result<Self, PolarsError> {
        let table = self.table.filter_expr(predicate)?;
        Ok(Self {
            name: self.name.clone(),
            table,
        })
    }

    /// Python-Polars alias for filter expression.
    pub fn filter(&self, predicate: Expr) -> Result<Self, PolarsError> {
        self.filter_expr(predicate)
    }

    pub fn with_columns_exprs(&self, exprs: &[Expr]) -> Result<Self, PolarsError> {
        let table = self.table.with_columns_exprs(exprs)?;
        Ok(Self {
            name: self.name.clone(),
            table,
        })
    }

    /// Python-Polars alias for with_columns.
    pub fn with_columns(&self, exprs: &[Expr]) -> Result<Self, PolarsError> {
        self.with_columns_exprs(exprs)
    }

    pub fn order_by_columns(
        &self,
        columns: &[&str],
        options: SortMultipleOptions,
    ) -> Result<Self, PolarsError> {
        let table = self.table.order_by_columns(columns, options)?;
        Ok(Self {
            name: self.name.clone(),
            table,
        })
    }

    /// Python-Polars alias for sort by columns.
    pub fn sort(
        &self,
        columns: &[&str],
        options: SortMultipleOptions,
    ) -> Result<Self, PolarsError> {
        self.order_by_columns(columns, options)
    }

    pub fn order_by_exprs(
        &self,
        exprs: &[Expr],
        options: SortMultipleOptions,
    ) -> Result<Self, PolarsError> {
        let table = self.table.order_by_exprs(exprs, options)?;
        Ok(Self {
            name: self.name.clone(),
            table,
        })
    }

    /// Python-Polars alias for sort by expressions.
    pub fn sort_by_exprs(
        &self,
        exprs: &[Expr],
        options: SortMultipleOptions,
    ) -> Result<Self, PolarsError> {
        self.order_by_exprs(exprs, options)
    }

    pub fn group_by_exprs(&self, keys: &[Expr], aggs: &[Expr]) -> Result<Self, PolarsError> {
        let table = self.table.group_by_exprs(keys, aggs)?;
        Ok(Self {
            name: self.name.clone(),
            table,
        })
    }

    pub fn group_by_columns(&self, keys: &[&str], aggs: &[Expr]) -> Result<Self, PolarsError> {
        let table = self.table.group_by_columns(keys, aggs)?;
        Ok(Self {
            name: self.name.clone(),
            table,
        })
    }

    /// Python-Polars alias for group_by columns.
    pub fn group_by(&self, keys: &[&str], aggs: &[Expr]) -> Result<Self, PolarsError> {
        self.group_by_columns(keys, aggs)
    }

    pub fn to_csv(&self, path: impl AsRef<Path>) -> Result<(), PolarsError> {
        write_table_csv(path.as_ref(), &self.table)
    }

    pub fn to_parquet(&self, path: impl AsRef<Path>) -> Result<(), PolarsError> {
        write_table_parquet(path.as_ref(), &self.table)
    }

    pub fn to_ipc(&self, path: impl AsRef<Path>) -> Result<(), PolarsError> {
        write_table_ipc(path.as_ref(), &self.table)
    }
}
