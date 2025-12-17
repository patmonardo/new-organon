//! Projection trace/events.
//!
//! This is a small “progress as temporal tool” surface.
//!
//! - Factories (concealing power) can emit import progress (records scanned, properties materialized).
//! - Evaluators (revealing power) can emit execution progress (phases, iterations, timing).

use super::power::ProjectionTag;

/// High-level event kind for projection tracing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectionEventKind {
    Begin,
    Progress,
    Metric,
    End,
}

/// A lightweight event emitted by factory/eval.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionEvent {
    pub tag: ProjectionTag,
    pub kind: ProjectionEventKind,

    /// Stable event name (e.g. "import.nodes", "eval.procedure").
    pub name: String,

    /// Human-readable message (optional).
    pub message: Option<String>,

    /// Current progress value (optional).
    pub current: Option<u64>,

    /// Total progress value (optional).
    pub total: Option<u64>,

    /// Unit label (optional), e.g. "records", "edges", "iterations".
    pub unit: Option<String>,
}

/// Sink for projection trace events.
///
/// Intentionally minimal; higher layers can adapt to logs, streams, or structured telemetry.
pub trait ProjectionEventSink: Send + Sync {
    fn emit(&self, event: ProjectionEvent);
}

/// Default no-op sink.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopProjectionEventSink;

impl ProjectionEventSink for NoopProjectionEventSink {
    fn emit(&self, _event: ProjectionEvent) {}
}
