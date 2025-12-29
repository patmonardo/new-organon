//! Canonical print/trace envelope emitted at the Eval → Print boundary.
//!
//! This is intentionally kernel-safe and discursively thin:
//! - Rust emits structured artifacts (prints) with provenance.
//! - TS layers can narrate/interpret/render these prints without pushing prose into the kernel.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

/// High-level print kind.
///
/// Keep this small and stable: the envelope is canonical; payloads can evolve.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrintKind {
    /// Task/Agent/Workflow orchestration.
    Taw,

    /// Fact-trace / witness-style traces.
    #[serde(rename = "facttrace")]
    FactTrace,

    /// Machine learning pipeline outputs.
    Ml,

    /// Graph algorithm outputs.
    Graph,

    /// Proof-like artifacts (opaque or structured).
    Proof,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrintProvenance {
    /// E.g. "gds::pagerank" or "gds::projection::eval".
    pub source: String,

    /// Optional correlation id for a run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,

    /// Optional kernel version identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
}

/// Canonical envelope for kernel-emitted artifacts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrintEnvelope {
    pub id: String,
    pub kind: PrintKind,
    pub timestamp: DateTime<Utc>,
    pub provenance: PrintProvenance,

    /// Kind-specific payload.
    pub payload: JsonValue,

    /// Optional auxiliary metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<JsonValue>,

    /// Optional proof/witness material.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<JsonValue>,
}

impl PrintEnvelope {
    /// Create a new print with a generated `id` and `timestamp`.
    pub fn new(kind: PrintKind, provenance: PrintProvenance, payload: JsonValue) -> Self {
        Self {
            id: format!("print-{}", Uuid::new_v4()),
            kind,
            timestamp: Utc::now(),
            provenance,
            payload,
            metadata: None,
            proof: None,
        }
    }

    pub fn with_metadata(mut self, metadata: JsonValue) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn with_proof(mut self, proof: JsonValue) -> Self {
        self.proof = Some(proof);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_with_expected_shape() {
        let print = PrintEnvelope::new(
            PrintKind::Graph,
            PrintProvenance {
                source: "gds::pagerank".to_string(),
                run_id: Some("run-42".to_string()),
                kernel_version: None,
            },
            serde_json::json!({"algo": "pagerank", "nodes": 3}),
        )
        .with_metadata(serde_json::json!({"note": "fixture"}))
        .with_proof(serde_json::json!({"trace_id": "t-1"}));

        let value = serde_json::to_value(&print).expect("serialize");

        assert!(value
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap()
            .starts_with("print-"));
        assert_eq!(value["kind"], "graph");
        assert_eq!(value["provenance"]["source"], "gds::pagerank");
        assert_eq!(value["provenance"]["run_id"], "run-42");
        assert_eq!(value["payload"]["algo"], "pagerank");
        assert_eq!(value["metadata"]["note"], "fixture");
        assert_eq!(value["proof"]["trace_id"], "t-1");
    }

    #[test]
    fn facttrace_kind_serializes_as_facttrace() {
        let print = PrintEnvelope::new(
            PrintKind::FactTrace,
            PrintProvenance {
                source: "gds::witness".to_string(),
                run_id: None,
                kernel_version: None,
            },
            serde_json::json!({"ok": true}),
        );

        let value = serde_json::to_value(&print).expect("serialize");
        assert_eq!(value["kind"], "facttrace");
    }
}
