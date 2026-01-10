use std::collections::HashMap;

/// Projection specification: input from caller describing the desired projection.
pub struct ProjectionSpec {
    pub graph_id: String,
    pub name: Option<String>,
    pub expr: String, // expression language (filter/projection), e.g. "nodes where x>0"
    pub materialize: bool,
    pub output_format: String, // e.g. "arrow", "parquet", "npz"
    pub storage_uri: Option<String>,
}

/// Metadata describing a materialized or ephemeral projection.
pub struct ProjectionMeta {
    pub id: String,
    pub graph_id: String,
    pub name: Option<String>,
    pub expr: String,
    pub created_at: Option<String>,
    pub status: String,
    pub storage_uri: Option<String>,
    pub schema: HashMap<String, String>, // field -> dtype
    pub version: Option<String>,
}

/// TransferChunk is an opaque chunk of bytes representing a portion of the
/// projection output. Prefer Arrow IPC frames, but allow Parquet/NPZ blobs.
pub struct TransferChunk {
    pub seq_no: u64,
    pub format: String,
    pub bytes: Vec<u8>,
    pub stats: Option<HashMap<String, String>>,
}

/// ProjectionReader: given a `ProjectionSpec` or a materialized projection id,
/// produce a sequence of `TransferChunk`s to stream to a client or write to
/// object-store. Implementations can be synchronous or asynchronous; this
/// trait keeps a minimal synchronous contract to avoid adding async deps in
/// the scaffold.
pub trait ProjectionReader {
    /// Produce all chunks for the projection (blocking/sync).
    fn read_projection(&self, spec: &ProjectionSpec) -> Result<Vec<TransferChunk>, String>;
}

/// ProjectionWriter: accept TransferChunks and persist to a storage backend
/// or forward them to a client. In practice this will wrap object-store
/// uploads or streaming gRPC sinks.
pub trait ProjectionWriter {
    /// Write a sequence of chunks; typically write manifest at the end.
    fn write_chunks(&self, projection_id: &str, chunks: Vec<TransferChunk>) -> Result<(), String>;
}

// NOTE: real implementations should integrate with `arrow2`, `parquet`, and
// `object_store` crates. This file intentionally avoids external crate
// references and provides a clear place to implement the concrete readers/
// writers later.

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyReader;

    impl ProjectionReader for DummyReader {
        fn read_projection(&self, _spec: &ProjectionSpec) -> Result<Vec<TransferChunk>, String> {
            Ok(vec![TransferChunk { seq_no: 0, format: "npz".to_string(), bytes: vec![1,2,3], stats: None }])
        }
    }

    #[test]
    fn dummy_read() {
        let r = DummyReader;
        let spec = ProjectionSpec { graph_id: "g1".to_string(), name: None, expr: "".to_string(), materialize: false, output_format: "npz".to_string(), storage_uri: None };
        let chunks = r.read_projection(&spec).unwrap();
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].format, "npz");
    }
}
