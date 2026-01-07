//! ScaleProperties dispatch handler.

use crate::algo::scale_properties::ScalePropertiesScaler;
use crate::procedures::miscellaneous::ScalePropertiesFacade;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn handle_scale_properties(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "scale_properties";

    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let graph_store = match catalog.get(graph_name) {
        Some(store) => store,
        None => {
            return err(
                op,
                "GRAPH_NOT_FOUND",
                &format!("Graph '{}' not found", graph_name),
            )
        }
    };

    let mode = request.get("mode").and_then(|v| v.as_str()).unwrap_or("stream");

    let mut node_properties: Vec<String> = request
        .get("nodeProperties")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();

    if node_properties.is_empty() {
        if let Some(prop) = request
            .get("sourceProperty")
            .or_else(|| request.get("property"))
            .and_then(|v| v.as_str())
        {
            node_properties.push(prop.to_string());
        }
    }

    let scaler = match request
        .get("scaler")
        .or_else(|| request.get("variant"))
        .and_then(|v| v.as_str())
    {
        Some("stdScore") => ScalePropertiesScaler::StdScore,
        Some("mean") => ScalePropertiesScaler::Mean,
        Some("max") => ScalePropertiesScaler::Max,
        Some("center") => ScalePropertiesScaler::Center,
        Some("log") => ScalePropertiesScaler::Log,
        Some("none") => ScalePropertiesScaler::None,
        _ => ScalePropertiesScaler::MinMax,
    };

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as usize;

    let facade = ScalePropertiesFacade::new(graph_store)
        .node_properties(node_properties.clone())
        .scaler(scaler.clone())
        .concurrency(concurrency);

    if mode == "estimate" {
        if node_properties.is_empty() {
            return err(op, "INVALID_REQUEST", "Missing 'nodeProperties' (or 'property') parameter");
        }
        let memory = facade.estimate_memory();
        return json!({
            "ok": true,
            "op": op,
            "data": {
                "min_bytes": memory.min(),
                "max_bytes": memory.max()
            }
        });
    }

    match mode {
        "stream" => match facade.stream() {
            Ok(rows_iter) => {
                let rows: Vec<Value> = rows_iter
                    .map(|row| json!({ "nodeId": row.node_id, "values": row.values }))
                    .collect();
                json!({ "ok": true, "op": op, "data": rows })
            }
            Err(e) => err(op, "EXECUTION_ERROR", &format!("scaleProperties failed: {e}")),
        },
        "stats" => match facade.stats() {
            Ok(stats) => json!({ "ok": true, "op": op, "data": stats }),
            Err(e) => err(op, "EXECUTION_ERROR", &format!("scaleProperties failed: {e}")),
        },
        "mutate" | "write" => err(op, "EXECUTION_ERROR", "scaleProperties mutate/write not implemented"),
        other => err(op, "INVALID_REQUEST", &format!("Invalid mode '{other}'")),
    }
}

fn err(op: &str, code: &str, message: &str) -> Value {
    json!({
        "ok": false,
        "op": op,
        "error": { "code": code, "message": message }
    })
}
