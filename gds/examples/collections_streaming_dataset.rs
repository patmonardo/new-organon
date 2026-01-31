//! Streaming Dataset example (Dataset + LazyFrame pipeline).
//!
//! Run with:
//!   cargo run -p gds --example collections_streaming_dataset

use gds::collections::dataframe::{expr_col, expr_lit_f64, expr_when, TableBuilder};
use gds::collections::datasets::{Dataset, StreamingDataset};
use gds::collections::extensions::streaming::StreamingConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build a dataset (Polars-backed).
    let dataset = Dataset::from_builder(
        TableBuilder::new()
            .with_i64_column("id", &[1, 2, 3, 4, 5, 6, 7, 8])
            .with_f64_column("score", &[10.0, 25.0, 40.0, 15.0, 30.0, 22.0, 12.0, 35.0])
            .with_f64_column("weight", &[1.1, 0.8, 1.5, 1.0, 1.2, 0.7, 1.3, 0.9]),
    )?;

    println!("Dataset rows: {}", dataset.row_count());

    // Build a streaming dataset with a LazyFrame transform.
    let streaming_summary = StreamingDataset::with_config(
        dataset.clone(),
        3,
        StreamingConfig {
            enable_new_streaming: false,
        },
    )
    .with_transform(|lazy| {
        lazy.with_columns([(expr_col("score") * expr_col("weight")).alias("weighted_score")])
            .with_columns([expr_when(
                expr_col("score").gt(expr_lit_f64(20.0)),
                expr_lit_f64(1.0),
                expr_lit_f64(0.0),
            )
            .alias("is_high")])
            .filter(expr_col("score").gt(expr_lit_f64(12.0)))
            .group_by([expr_col("is_high")])
            .agg([
                expr_col("weighted_score").mean().alias("avg_weighted"),
                expr_col("id").count().alias("rows"),
            ])
            .sort_by_exprs([expr_col("is_high")], Default::default())
    });

    let summary = streaming_summary
        .iter()
        .next()
        .ok_or("missing summary batch")??;
    println!("Streaming dataset summary:\n{}", summary);

    // Stream raw batches (no transform) for inspection.
    let streaming_batches = StreamingDataset::new(dataset, 3);
    for (index, batch) in streaming_batches.iter().enumerate() {
        let batch_df = batch?;
        let offset = index * streaming_batches.batch_size();
        println!("Batch @{}:\n{}", offset, batch_df);
    }

    Ok(())
}
