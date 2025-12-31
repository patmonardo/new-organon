//! Arrow IO helpers (IPC/Parquet) for in-memory ingestion.
//!
//! These helpers read columnar files into arrow2 record batches/chunks that
//! can be consumed by projection factories (e.g., ArrowNativeFactory). They
//! intentionally avoid any factory-specific logic.

use std::fs::File;
use std::io::BufReader;

use arrow2::chunk::Chunk;
use arrow2::datatypes::Schema;
use arrow2::error::Error as ArrowError;
use arrow2::io::ipc::read::{read_file_metadata, FileReader};
use arrow2::io::parquet::read as parquet_read;

/// Arrow data read from a file, consisting of schema and record batches.
#[derive(Debug)]
pub struct ArrowData {
    /// The schema describing the data structure
    pub schema: Schema,
    /// The record batches containing the actual data
    pub chunks: Vec<Chunk<Box<dyn arrow2::array::Array>>>,
}

/// Read Arrow IPC file (Feather/Arrow) into arrow data.
pub fn read_ipc_file(path: &str) -> Result<ArrowData, ArrowError> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let metadata = read_file_metadata(&mut reader)?;
    let schema = metadata.schema.clone();
    let chunks = FileReader::new(reader, metadata, None, None).collect::<Result<Vec<_>, _>>()?;
    Ok(ArrowData { schema, chunks })
}

/// Read Parquet file into arrow data.
pub fn read_parquet_file(path: &str) -> Result<ArrowData, ArrowError> {
    let mut reader = BufReader::new(File::open(path)?);
    let metadata = parquet_read::read_metadata(&mut reader)?;
    let schema = parquet_read::infer_schema(&metadata)?;
    let row_groups = metadata.row_groups;
    let chunks =
        parquet_read::FileReader::new(reader, row_groups, schema.clone(), None, None, None)
            .collect::<Result<Vec<_>, _>>()?;
    Ok(ArrowData { schema, chunks })
}
