//! Collections streaming example (LazyFrame-focused).
//!
//! Run with:
//!   cargo run -p gds --example collections_streaming_lazy

use gds::collections::dataframe::{
    expr_col, expr_gt, expr_lit_f64, PolarsStreamingFrame, TableBuilder,
};
use gds::collections::extensions::streaming::{StreamingConfig, StreamingSupport};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let table = TableBuilder::new()
        .with_i64_column("id", &[1, 2, 3, 4, 5])
        .with_f64_column("score", &[10.0, 25.0, 40.0, 15.0, 30.0])
        .build()?;

    let mut streaming = PolarsStreamingFrame::from(table.into_inner());
    streaming.enable_streaming(StreamingConfig {
        enable_new_streaming: false,
    })?;

    // Build a LazyFrame pipeline.
    let lazy = streaming
        .stream_lazy()
        .select([expr_col("id"), expr_col("score")])
        .filter(expr_gt(expr_col("score"), expr_lit_f64(20.0)));

    let df = streaming.collect_streaming_lazy(lazy)?;
    println!("Streaming LazyFrame result:\n{}", df);

    Ok(())
}
