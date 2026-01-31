//! Dataset IO errors.

/// Dataset IO error types.
#[derive(Debug, thiserror::Error)]
pub enum DatasetIoError {
    #[error("Dataset IO error: {0}")]
    Io(String),
    #[error("Dataset HTTP error: {0}")]
    Http(String),
    #[error("Dataset archive error: {0}")]
    Archive(String),
    #[error("Dataset unsupported operation: {0}")]
    Unsupported(String),
}
