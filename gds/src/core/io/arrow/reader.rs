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

/// Read Arrow IPC file (Feather/Arrow) into a vector of record batches and schema.
pub fn read_ipc_file(
    path: &str,
) -> Result<(Schema, Vec<Chunk<Box<dyn arrow2::array::Array>>>), ArrowError> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let metadata = read_file_metadata(&mut reader)?;
    let schema = metadata.schema.clone();
    let batches =
        FileReader::new(reader, metadata, None, None).collect::<Result<Vec<_>, _>>()?;
    Ok((schema, batches))
}

/// Read Parquet file into a vector of record batches and schema.
pub fn read_parquet_file(
    path: &str,
) -> Result<(Schema, Vec<Chunk<Box<dyn arrow2::array::Array>>>), ArrowError> {
    let mut reader = BufReader::new(File::open(path)?);
    let metadata = parquet_read::read_metadata(&mut reader)?;
    let arrow_schema = parquet_read::infer_schema(&metadata)?;
    let row_groups = metadata.row_groups;
    let chunks = parquet_read::FileReader::new(
        reader,
        row_groups,
        arrow_schema.clone(),
        None,
        None,
        None,
    )
    .collect::<Result<Vec<_>, _>>()?;
    Ok((arrow_schema, chunks))
}
