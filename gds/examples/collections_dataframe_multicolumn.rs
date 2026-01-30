//! Collections framing example (multi-column layout without Polars).
//!
//! Run with:
//!   cargo run -p gds --example collections_dataframe_multicolumn

use gds::collections::backends::vec::VecInt;
use gds::collections::extensions::framing::{
    FrameCollection, FrameOrder, FrameShape, FramingConfig, FramingSupport,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simulated multi-column layout: [id, score_x10, label_id]
    // rows = 4, cols = 3 (row-major)
    let values: Vec<i32> = vec![1, 5, 0, 2, 15, 1, 3, 25, 0, 4, 35, 2];

    let base = VecInt::from(values);
    let mut framed = FrameCollection::new(base);
    let framing_config = FramingConfig {
        shape: FrameShape {
            rows: 4,
            cols: 3,
            order: FrameOrder::RowMajor,
        },
        strict_bounds: true,
    };
    framed.enable_framing(framing_config)?;

    let cell = framed.get_cell(2, 1)?;
    let row = framed.row_values(2)?;
    let col = framed.col_values(0)?;

    println!("Framing (multi-column layout):");
    println!("  cell(2,1) = {cell:?}");
    println!("  row(2) = {row:?}");
    println!("  col(0) = {col:?}");

    Ok(())
}
