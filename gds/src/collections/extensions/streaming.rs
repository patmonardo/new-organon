//! Collections Streaming Extensions
//!
//! Defines streaming-friendly helpers that bridge LazyFrame pipelines and
//! future dataset-oriented streaming (chunk/partition/caching).

use polars::prelude::{DataFrame, LazyFrame, PolarsError};

/// Streaming configuration.
#[derive(Debug, Clone)]
pub struct StreamingConfig {
    /// Toggle the Polars "new streaming" engine when available.
    pub enable_new_streaming: bool,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            enable_new_streaming: false,
        }
    }
}

/// Streaming error types.
#[derive(Debug, thiserror::Error)]
pub enum StreamingError {
    #[error("Streaming failed: {0}")]
    StreamingFailed(String),
    #[error(transparent)]
    Polars(#[from] PolarsError),
}

/// Streaming support trait for DataFrame-like types.
pub trait StreamingSupport {
    /// Enable streaming with the provided configuration.
    fn enable_streaming(&mut self, config: StreamingConfig) -> Result<(), StreamingError>;

    /// Disable streaming.
    fn disable_streaming(&mut self);

    /// Check if streaming is enabled.
    fn is_streaming_enabled(&self) -> bool;

    /// Access the streaming configuration (if enabled).
    fn streaming_config(&self) -> Option<&StreamingConfig>;

    /// Build a LazyFrame for streaming-style execution.
    fn stream_lazy(&self) -> LazyFrame;

    /// Collect a LazyFrame using streaming settings when available.
    fn collect_streaming(&self) -> Result<DataFrame, StreamingError>;

    /// Collect a provided LazyFrame using streaming settings when available.
    fn collect_streaming_lazy(&self, lazy: LazyFrame) -> Result<DataFrame, StreamingError>;
}
