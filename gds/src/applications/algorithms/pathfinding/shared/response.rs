use crate::applications::algorithms::machinery::AlgorithmProcessingTimings;
use serde_json::{json, Value};

pub fn err(op: &str, code: &str, message: &str) -> Value {
    json!({
        "ok": false,
        "op": op,
        "error": { "code": code, "message": message }
    })
}

pub fn timings_json(t: AlgorithmProcessingTimings) -> Value {
    json!({
        "preProcessingMillis": t.pre_processing_millis,
        "computeMillis": t.compute_millis,
        "sideEffectMillis": t.side_effect_millis,
    })
}
