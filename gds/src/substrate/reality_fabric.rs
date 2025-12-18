//! Reality fabric (substrate): the Wheel (five-fold “fat pipe”).
//!
//! In this repo, “Fabric” is not a theme word — it names the universal interconnect
//! between the kernel (Projection Factory/Eval) and the substrate/reality.
//!
//! **Wheel framing (brahmachakra)**
//!
//! We model Reality as a *Wheel*:
//! - a **4-fold rim** (the breakdown of reality into quadrants)
//! - plus the **center conjunction** (the evaluative act where the folds are unified)
//!
//! In code this is expressed as a five-fold pipe:
//! - **Rim (four-fold)**: Storage, Compute, Control, Time
//! - **Center (conjunction)**: Witness (proof/trace/audit)
//!
//! The intent is that the *center* is where Form Eval becomes accountable as Real:
//! the act of execution is not just transformation, but *witnessed* transformation.
//!
//! The intent is that Reality-facing proc-macros codegen *bindings* that plug into
//! this fabric: Collections factories, config surfaces, Value/ValueType plumbing,
//! and property typing.
//!
//! The fabric is **five-fold**:
//! 1) Storage  — persistence/materialization surfaces
//! 2) Compute  — CPU/GPU allocation/execution surfaces
//! 3) Control  — identity/tenancy/policy labels
//! 4) Time     — budgets/leases/deadlines
//! 5) Witness  — trace/audit/proof sinks
//!
//! This module deliberately stays small and vocabulary-first. The concrete
//! “how” of scheduling, IO protocols, and enterprise policy lives at explicit
//! boundary seams (e.g. StorageProcessor / ComputeProcessor) rather than leaking
//! into evaluators.

use std::collections::HashMap;

use serde_json::Value as JsonValue;

/// Control plane for the RealityFabric.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FabricControl {
    pub tenant_id: Option<String>,
    pub subject: Option<String>,
    pub trace_id: Option<String>,

    /// Free-form routing/policy labels (pool, region, priority class, etc.).
    pub labels: HashMap<String, String>,
}

/// Time plane for the RealityFabric.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FabricTime {
    /// Optional wall-time budget for a unit of work.
    pub max_wall_time_ms: Option<u64>,
}

/// Witness plane for the RealityFabric.
///
/// Wheel framing: **the center** (conjunction) of the four-fold rim.
///
/// This is where proofs/traces/audit events can be recorded without forcing
/// evaluators to depend on any particular persistence or observability stack.
pub trait WitnessFabric: Send + Sync {
    fn record(&self, event: JsonValue);
}

/// Default witness implementation: do nothing.
#[derive(Debug, Clone, Default)]
pub struct NoopWitnessFabric;

impl WitnessFabric for NoopWitnessFabric {
    fn record(&self, _event: JsonValue) {}
}

/// The RealityFabric: a five-fold, “fat pipe” connection into reality.
///
/// Wheel framing:
/// - rim quadrants: `storage`, `compute`, `control`, `time`
/// - center conjunction: `witness`
///
/// `S` and `C` are intentionally generic. In early phases they may be simple
/// in-memory surfaces; later they can be concrete substrate services.
#[derive(Debug, Clone)]
pub struct RealityFabric<S, C, W = NoopWitnessFabric> {
    pub storage: S,
    pub compute: C,
    pub control: FabricControl,
    pub time: FabricTime,
    pub witness: W,
}

/// Alias used when we want to emphasize the “pipe” metaphor.
pub type RealityPipe<S, C, W = NoopWitnessFabric> = RealityFabric<S, C, W>;
