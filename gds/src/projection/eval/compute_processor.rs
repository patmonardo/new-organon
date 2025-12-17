//! Compute Processor (Enterprise / Runtime Resource Interface)
//!
//! If the IO Processor is the NIP-style boundary for *network reality*, the Compute
//! Processor is the boundary for *runtime resource reality*.
//!
//! In this codebase, many algorithms have two resource faces:
//! - **Storage runtime**: memory layout, graph store access, materialization costs
//! - **Computation runtime**: CPU/GPU work, parallelism, time
//!
//! Those are not “accidental properties” once you leave the lab. They become
//! contractual: quotas, placement, scheduling, isolation, billing, and audit.
//!
//! This module intentionally does not implement a scheduler. It just defines the
//! stable seam.

use std::collections::HashMap;

use serde_json::Value as JsonValue;

/// The kind of hardware/runtime the caller is requesting or the system selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeBackend {
    Cpu,
    Gpu,
}

/// Resource knobs and enterprise metadata for compute execution.
#[derive(Debug, Clone, Default)]
pub struct ComputeMeta {
    /// Correlation/idempotency key (often shared with IO-level request id).
    pub request_id: Option<String>,

    /// Tenant boundary for multi-tenant scheduling/quota.
    pub tenant_id: Option<String>,

    /// Subject identity (user/service principal) for auth + audit.
    pub subject: Option<String>,

    /// Trace correlation.
    pub trace_id: Option<String>,

    /// Preferred backend (policy may override).
    pub preferred_backend: Option<ComputeBackend>,

    /// Upper bound on wall time, if the caller provides one.
    pub max_wall_time_ms: Option<u64>,

    /// Upper bound on memory budget, if the caller provides one.
    pub max_memory_bytes: Option<u64>,

    /// Free-form labels for routing/scheduling (region, pool, priority class, etc.).
    pub labels: HashMap<String, String>,
}

/// Dispatch target for the compute boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComputeRoute {
    /// Execute a Procedure ISA algorithm.
    Procedure,

    /// Execute an ML pipeline/training job (feature-gated in this repo).
    Ml,
}

/// A generic compute request envelope.
#[derive(Debug, Clone)]
pub struct ComputeRequest {
    pub meta: ComputeMeta,
    pub route: ComputeRoute,

    /// Unstructured payload; the boundary is stable even if internal request
    /// types evolve.
    pub payload: JsonValue,
}

/// A generic compute response envelope.
#[derive(Debug, Clone)]
pub struct ComputeResponse {
    pub meta: ComputeMeta,
    pub payload: JsonValue,
}

#[derive(Debug, thiserror::Error)]
pub enum ComputeError {
    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("unsupported backend")]
    UnsupportedBackend,

    #[error("timeout")]
    Timeout,

    #[error("over quota")]
    OverQuota,

    #[error("internal error: {0}")]
    Internal(String),
}

/// Compute/runtime boundary contract.
///
/// This is where scheduling/placement/isolation can be enforced without pushing
/// those concerns into the algorithm implementations.
pub trait ComputeProcessor {
    fn handle(&mut self, request: ComputeRequest) -> Result<ComputeResponse, ComputeError>;
}
