use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Memory estimate result.
///
/// Java parity: mirrors `MemoryEstimateResult` at the facade boundary.
/// Rust pass-1: a compact, JSON-friendly estimate that we can compute without DB loaders.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryEstimateResult {
    pub bytes_min: u64,
    pub bytes_max: u64,
    pub bytes_estimate: u64,
    pub human_readable: String,
    pub details: HashMap<String, Value>,
}

impl MemoryEstimateResult {
    pub fn new(bytes_estimate: u64, details: HashMap<String, Value>) -> Self {
        Self {
            bytes_min: bytes_estimate,
            bytes_max: bytes_estimate,
            bytes_estimate,
            human_readable: format_bytes(bytes_estimate),
            details,
        }
    }
}

fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let b = bytes as f64;
    if b >= GB {
        format!("{:.2} GiB", b / GB)
    } else if b >= MB {
        format!("{:.2} MiB", b / MB)
    } else if b >= KB {
        format!("{:.2} KiB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}
