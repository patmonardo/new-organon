//! Collections Expr example (Expr as first-class components).
//!
//! Run with:
//!   cargo run -p gds --example collections_expr_basic

use gds::collections::dataframe::{expr_col, expr_gt, expr_lit_f64, expr_when, TableBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build a DataFrame via the Collections facade.
    let table = TableBuilder::new()
        .with_i64_column("id", &[1, 2, 3, 4, 5])
        .with_f64_column("score", &[10.0, 25.0, 40.0, 15.0, 30.0])
        .build()?;

    println!("Input table:\n{}", table.fmt_table());

    // Exprs are first-class: build them independently, then reuse.
    let score_col = expr_col("score");
    let high_score = expr_gt(score_col.clone(), expr_lit_f64(20.0));

    // Filter rows using a predicate expression.
    let filtered = table.filter_expr(high_score.clone())?;
    println!("Filtered (score > 20):\n{}", filtered.fmt_table());

    // Create a derived column using an expression.
    let tagged =
        table.with_columns_exprs(&[
            expr_when(high_score, expr_lit_f64(1.0), expr_lit_f64(0.0)).alias("is_high")
        ])?;
    println!("With derived column (is_high):\n{}", tagged.fmt_table());

    // Group by and aggregate using expressions.
    let grouped = tagged.group_by_columns(
        &["is_high"],
        &[expr_col("score").mean().alias("mean_score")],
    )?;
    println!("Grouped by is_high:\n{}", grouped.fmt_table());

    Ok(())
}
