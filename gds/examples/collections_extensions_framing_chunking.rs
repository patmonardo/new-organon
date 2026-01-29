//! Collections extensions example (framing + chunking).
//!
//! Run with:
//!   cargo run -p gds --example collections_extensions_framing_chunking

use gds::collections::backends::vec::VecInt;
use gds::collections::extensions::chunking::{ChunkedCollection, ChunkingConfig, ChunkingSupport};
use gds::collections::extensions::framing::{
    FrameCollection, FrameOrder, FrameShape, FramingConfig, FramingSupport,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- Framing example ---
    let framing_values: Vec<i32> = (1..=9).collect();
    let framing_base = VecInt::from(framing_values);
    let mut framed = FrameCollection::new(framing_base);

    let framing_config = FramingConfig {
        shape: FrameShape {
            rows: 3,
            cols: 3,
            order: FrameOrder::RowMajor,
        },
        strict_bounds: true,
    };

    framed.enable_framing(framing_config)?;

    let cell = framed.get_cell(1, 1)?;
    let row = framed.row_values(1)?;
    let col = framed.col_values(2)?;

    println!("Framing:");
    println!("  cell(1,1) = {cell}");
    println!("  row(1) = {row:?}");
    println!("  col(2) = {col:?}");

    // --- Chunking example ---
    let chunking_values: Vec<i32> = (1..=12).collect();
    let chunking_base = VecInt::from(chunking_values);
    let mut chunked = ChunkedCollection::new(chunking_base);

    let chunking_config = ChunkingConfig {
        chunk_size: 4,
        max_chunks: None,
        prefer_power_of_two: false,
        allow_tail: true,
    };

    chunked.enable_chunking(chunking_config)?;

    println!("Chunking:");
    chunked.for_each_chunk(|range, values| {
        println!("  chunk {:?} => {values:?}", range);
    })?;

    Ok(())
}
