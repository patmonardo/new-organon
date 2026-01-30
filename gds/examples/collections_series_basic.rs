//! Collections Series example (eager Series operations).
//!
//! Run with:
//!   cargo run -p gds --example collections_series_basic

use gds::collections::dataframe::{series_bool, series_f64, series_i64};
use polars::prelude::{ChunkAgg, SortOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build Series using the Collections helpers.
    let values = series_i64("values", &[-3, -1, 2, 4]);
    let weights = series_f64("weights", &[0.5, 1.0, 1.5, 2.0]);

    println!("values:\n{values}");
    println!("weights:\n{weights}");

    // Basic Series operations.
    let abs_values_vec: Vec<i64> = values.i64()?.into_no_null_iter().map(|v| v.abs()).collect();
    let abs_values = series_i64("abs_values", &abs_values_vec);
    println!("abs(values):\n{abs_values}");

    let sum_values = values.i64()?.sum().unwrap_or(0);
    println!("sum(values) = {sum_values}");

    let mean_weights = weights.f64()?.mean().unwrap_or(0.0);
    println!("mean(weights) = {mean_weights}");

    // Filter using a boolean mask.
    let mask = series_bool("mask", &[true, false, true, true]);
    let filtered = values.filter(mask.bool()?)?;
    println!("filtered(values):\n{filtered}");

    // Sort and take head.
    let sorted = values.sort(SortOptions::default())?;
    let head = sorted.head(Some(2));
    println!("sorted(values):\n{sorted}");
    println!("head(sorted, 2):\n{head}");

    Ok(())
}
