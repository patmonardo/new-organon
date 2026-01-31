//! Collections select/filter example (eager DataFrame + Expr usage).
//!
//! Run with:
//!   cargo run -p gds --example collections_select_filter

use gds::collections::dataframe::{
    expr_col, expr_gt, expr_lit_f64, PolarsSortMultipleOptions, TableBuilder,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let table = TableBuilder::new()
        .with_i64_column("id", &[1, 2, 3, 4, 5])
        .with_f64_column("score", &[10.0, 25.0, 40.0, 15.0, 30.0])
        .with_f64_column("weight", &[1.1, 0.8, 1.5, 1.0, 1.2])
        .with_string_column(
            "group",
            &[
                "A".to_string(),
                "B".to_string(),
                "A".to_string(),
                "B".to_string(),
                "A".to_string(),
            ],
        )
        .build()?;

    println!("Input table:\n{}", table.fmt_table());

    // Eager select by column names.
    let selected = table.select_columns(&["id", "score"])?;
    println!("Selected columns:\n{}", selected.fmt_table());

    // Filter with an expression (score > 20).
    let filtered = table.filter_expr(expr_gt(expr_col("score"), expr_lit_f64(20.0)))?;
    println!("Filtered rows (score > 20):\n{}", filtered.fmt_table());

    // Select using expressions (rename score -> score_x2).
    let projected = table.select_exprs(&[
        expr_col("id"),
        (expr_col("score") * expr_lit_f64(2.0)).alias("score_x2"),
    ])?;
    println!("Selected exprs (score_x2):\n{}", projected.fmt_table());

    // Order by score descending.
    let ordered = table.order_by_columns(
        &["score"],
        PolarsSortMultipleOptions::new().with_order_descending(true),
    )?;
    println!("Ordered by score desc:\n{}", ordered.fmt_table());

    // Group by a column and aggregate.
    let grouped = table.group_by_columns(
        &["group"],
        &[
            expr_col("score").mean().alias("avg_score"),
            expr_col("id").count().alias("rows"),
        ],
    )?;
    println!("Grouped by group:\n{}", grouped.fmt_table());

    Ok(())
}
