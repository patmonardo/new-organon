//! Collections chunking + rechunk example (Polars-first).
//!
//! Run with:
//!   cargo run -p gds --example collections_chunking_rechunk

use gds::collections::dataframe::{PolarsDataFrameCollection, TableBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let left = TableBuilder::new()
        .with_i64_column("id", &[1, 2, 3])
        .with_f64_column("score", &[10.0, 25.0, 40.0])
        .build()?;

    let right = TableBuilder::new()
        .with_i64_column("id", &[4, 5, 6])
        .with_f64_column("score", &[15.0, 30.0, 22.0])
        .build()?;

    // vstack adds chunks. This is typical when building up a DataFrame in pieces.
    let mut combined_df = left.clone().into_inner();
    combined_df.vstack_mut(&right.into_inner())?;
    let combined = PolarsDataFrameCollection::new(combined_df);

    println!("Combined (multiple chunks):\n{}", combined.fmt_table());
    print_chunk_counts("Before rechunk", &combined);

    // Rechunk to coalesce column chunks for faster scans.
    let mut rechunked_df = combined.clone().into_inner();
    rechunked_df.align_chunks_par();
    let rechunked = PolarsDataFrameCollection::new(rechunked_df);

    print_chunk_counts("After rechunk", &rechunked);

    Ok(())
}

fn print_chunk_counts(label: &str, table: &PolarsDataFrameCollection) {
    let counts: Vec<String> = table
        .dataframe()
        .get_columns()
        .iter()
        .map(|col| format!("{}: {}", col.name(), col.n_chunks()))
        .collect();
    println!("{} chunk counts -> {}", label, counts.join(", "));
}
