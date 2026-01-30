//! Expression helpers and selectors for Polars-backed DataFrames.

use polars::prelude::{col, lit, when, DataType, Expr};

pub type PolarsExpr = Expr;

/// Select a column by name.
pub fn expr_col(name: &str) -> Expr {
    col(name)
}

/// Select multiple columns by name.
pub fn expr_cols(names: &[&str]) -> Vec<Expr> {
    names.iter().map(|name| col(*name)).collect()
}

/// Literal expressions.
pub fn expr_lit_i64(value: i64) -> Expr {
    lit(value)
}

pub fn expr_lit_i32(value: i32) -> Expr {
    lit(value)
}

pub fn expr_lit_i16(value: i16) -> Expr {
    lit(value)
}

pub fn expr_lit_i8(value: i8) -> Expr {
    lit(value)
}

pub fn expr_lit_u64(value: u64) -> Expr {
    lit(value)
}

pub fn expr_lit_u32(value: u32) -> Expr {
    lit(value)
}

pub fn expr_lit_u16(value: u16) -> Expr {
    lit(value)
}

pub fn expr_lit_u8(value: u8) -> Expr {
    lit(value)
}

pub fn expr_lit_f64(value: f64) -> Expr {
    lit(value)
}

pub fn expr_lit_f32(value: f32) -> Expr {
    lit(value)
}

pub fn expr_lit_bool(value: bool) -> Expr {
    lit(value)
}

pub fn expr_lit_str(value: &str) -> Expr {
    lit(value)
}

/// Comparison and logical helpers.
pub fn expr_eq(lhs: Expr, rhs: Expr) -> Expr {
    lhs.eq(rhs)
}

pub fn expr_neq(lhs: Expr, rhs: Expr) -> Expr {
    lhs.neq(rhs)
}

pub fn expr_gt(lhs: Expr, rhs: Expr) -> Expr {
    lhs.gt(rhs)
}

pub fn expr_gte(lhs: Expr, rhs: Expr) -> Expr {
    lhs.gt_eq(rhs)
}

pub fn expr_lt(lhs: Expr, rhs: Expr) -> Expr {
    lhs.lt(rhs)
}

pub fn expr_lte(lhs: Expr, rhs: Expr) -> Expr {
    lhs.lt_eq(rhs)
}

pub fn expr_and(lhs: Expr, rhs: Expr) -> Expr {
    lhs.and(rhs)
}

pub fn expr_or(lhs: Expr, rhs: Expr) -> Expr {
    lhs.or(rhs)
}

pub fn expr_not(expr: Expr) -> Expr {
    expr.not()
}

pub fn expr_alias(expr: Expr, name: &str) -> Expr {
    expr.alias(name)
}

/// Null-handling helpers.
pub fn expr_is_null(expr: Expr) -> Expr {
    expr.is_null()
}

pub fn expr_is_not_null(expr: Expr) -> Expr {
    expr.is_not_null()
}

pub fn expr_fill_null(expr: Expr, value: Expr) -> Expr {
    expr.fill_null(value)
}

/// Type coercion.
pub fn expr_cast(expr: Expr, dtype: DataType) -> Expr {
    expr.cast(dtype)
}

/// Aggregations.
pub fn expr_sum(expr: Expr) -> Expr {
    expr.sum()
}

pub fn expr_mean(expr: Expr) -> Expr {
    expr.mean()
}

pub fn expr_min(expr: Expr) -> Expr {
    expr.min()
}

pub fn expr_max(expr: Expr) -> Expr {
    expr.max()
}

pub fn expr_count(expr: Expr) -> Expr {
    expr.count()
}

/// Conditional expression builder.
pub fn expr_when(predicate: Expr, then_expr: Expr, otherwise_expr: Expr) -> Expr {
    when(predicate).then(then_expr).otherwise(otherwise_expr)
}
