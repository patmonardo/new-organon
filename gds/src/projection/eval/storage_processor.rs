//! Storage Processor (Enterprise / Storage Interface)
//!
//! This is the *enterprise seam*.
//!
//! The core evaluators under `projection/eval/` (Procedure / ML / Form) are about
//! computation over graphs.
//!
//! From the kernel’s point of view, most “IO” is really *storage*:
//! reading/writing graphs, committing projections, persisting proofs, emitting artifacts.
//!
//! The Storage Processor is the dedicated boundary that accepts requests from “outside
//! the kernel”, applies enterprise concerns, and then dispatches into the kernel
//! (e.g. `EvalRunner`).
//!
//! Concretely, this is where you put:
//! - authentication + authorization (who may do what)
//! - tenancy + quota/rate limits (how much, for whom)
//! - idempotency + retries + timeouts (network reality)
//! - tracing + audit/lineage persistence (accountability)
//! - protocol translation (HTTP/gRPC/stream → internal request)
//!
//! The point is not to “add networking” here yet, but to define the *shape* of the
//! boundary so we can keep the kernel clean.

use std::collections::HashMap;
use std::time::SystemTime;

use serde_json::Value as JsonValue;

/// The enterprise/storage boundary contract.
///
/// Implementations can be:
/// - in-process (CLI/tests)
/// - HTTP server adapter
/// - message bus consumer
/// - “front-end NIP” that talks to a remote kernel
pub trait StorageProcessor {
    fn handle(&mut self, request: StorageRequest) -> Result<StorageResponse, StorageError>;
}

/// Metadata that rides alongside a request/response.
///
/// Keep this small and boring: it exists so we can attach enterprise guarantees
/// without threading ad-hoc parameters through every executor.
#[derive(Debug, Clone, Default)]
pub struct StorageMeta {
    /// Stable correlation/idempotency key as seen by the caller.
    pub request_id: Option<String>,

    /// Optional tenant boundary.
    pub tenant_id: Option<String>,

    /// Optional subject identity (user/service principal).
    pub subject: Option<String>,

    /// Optional trace correlation.
    pub trace_id: Option<String>,

    /// Optional free-form headers (protocol translation layer can preserve what it needs).
    pub headers: HashMap<String, String>,

    /// When the envelope was created at the boundary.
    pub received_at: Option<SystemTime>,
}

/// A generic request envelope.
#[derive(Debug, Clone)]
pub struct StorageRequest {
    pub meta: StorageMeta,

    /// Route for dispatch inside the kernel.
    pub route: StorageRoute,

    /// Payload is intentionally untyped at this boundary.
    ///
    /// Protocol layers (HTTP/gRPC/etc.) can decode into structured types before
    /// calling the kernel, but `JsonValue` keeps the seam stable.
    pub payload: JsonValue,
}

/// A generic response envelope.
#[derive(Debug, Clone)]
pub struct StorageResponse {
    pub meta: StorageMeta,
    pub payload: JsonValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageRoute {
    /// Route to the Form ISA (most common for “FormShape → ResultStore”).
    Form,

    /// Route to the Procedure ISA.
    Procedure,

    /// Route to the ML ISA (feature-gated in this repo).
    Ml,
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("timeout")]
    Timeout,

    #[error("over quota")]
    OverQuota,

    #[error("internal error: {0}")]
    Internal(String),
}
