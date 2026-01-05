use crate::applications::algorithms::machinery::AlgorithmProcessingTimings;
use serde_json::{json, Value};

pub fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}

pub fn timings_json(timings: AlgorithmProcessingTimings) -> Value {
    json!({
        "preProcessingMillis": timings.pre_processing_millis,
        "computeMillis": timings.compute_millis,
        "sideEffectMillis": timings.side_effect_millis,
    })
}
