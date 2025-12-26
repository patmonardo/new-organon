//! Pathfinding algorithm dispatch module.
//!
//! Consolidates pathfinding algorithm handling to reduce tsjson_napi.rs size.
//! Each algorithm has one entry point that dispatches on mode.

use crate::applications::algorithms::machinery::StreamProcessingTemplate;
use crate::applications::algorithms::metadata::Algorithm;
use crate::config::base_types::AlgoBaseConfig;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Common response builders
fn ok(op: &str, data: Value) -> Value {
    json!({ "ok": true, "op": op, "data": data })
}

fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}

/// Parse mode from request, defaulting to "stream"
fn parse_mode(request: &Value) -> &str {
    request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stream")
}

/// BFS dispatcher (minimal, facade-based)
pub fn handle_bfs(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "bfs";
    let mode = parse_mode(request);
    let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: graphName");
    };
    let cfg = AlgoBaseConfig { ..Default::default() };
    match mode {
        "stream" => {
            let template = StreamProcessingTemplate::new();
            let result = template.process(
                catalog.as_ref(),
                graph_name,
                &cfg,
                Algorithm::BFS,
                |resources| {
                    resources.facade().bfs().from_json(request)
                        .map_err(|e| crate::projection::eval::procedure::AlgorithmError::Execution(e))
                        .and_then(|builder| builder.stream())
                },
                |stream_result| {
                    stream_result.map(|iter| {
                        iter.map(|r| {
                            json!({
                                "sourceNode": r.source,
                                "targetNode": r.target,
                                "path": r.path,
                                "cost": r.cost,
                            })
                        }).collect::<Vec<_>>()
                    })
                },
            );
            match result {
                Ok(Ok(rows)) => ok(op, json!({ "graphName": graph_name, "rows": rows })),
                Ok(Err(e)) => err(op, "ERROR", &format!("BFS failed: {e}")),
                Err(e) => err(op, "ERROR", &format!("Processing failed: {e}")),
            }
        }
        _ => err(op, "NOT_IMPLEMENTED", &format!("{op} {mode} mode not yet implemented")),
    }
}

/// DFS dispatcher
pub fn handle_dfs(request: &Value, _catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "dfs";
    err(op, "NOT_IMPLEMENTED", "DFS not yet implemented")
}

/// Dijkstra dispatcher (minimal, facade-based)
pub fn handle_dijkstra(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "dijkstra";
    let mode = parse_mode(request);
    let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: graphName");
    };
    let cfg = AlgoBaseConfig { ..Default::default() };
    match mode {
        "stream" => {
            let template = StreamProcessingTemplate::new();
            let result = template.process(
                catalog.as_ref(),
                graph_name,
                &cfg,
                Algorithm::Dijkstra,
                |resources| {
                    resources.facade().dijkstra().from_json(request)
                        .map_err(|e| crate::projection::eval::procedure::AlgorithmError::Execution(e))
                        .and_then(|builder| builder.stream())
                },
                |stream_result| {
                    stream_result.map(|iter| {
                        iter.map(|r| {
                            json!({
                                "sourceNode": r.source,
                                "targetNode": r.target,
                                "path": r.path,
                                "cost": r.cost,
                            })
                        }).collect::<Vec<_>>()
                    })
                },
            );
            match result {
                Ok(Ok(rows)) => ok(op, json!({ "graphName": graph_name, "rows": rows })),
                Ok(Err(e)) => err(op, "ERROR", &format!("Dijkstra failed: {e}")),
                Err(e) => err(op, "ERROR", &format!("Processing failed: {e}")),
            }
        }
        _ => err(op, "NOT_IMPLEMENTED", &format!("{op} {mode} mode not yet implemented")),
    }
}

/// Bellman-Ford dispatcher
pub fn handle_bellman_ford(request: &Value, _catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "bellman_ford";
    err(op, "NOT_IMPLEMENTED", "Bellman-Ford not yet implemented")
}

/// A* dispatcher
pub fn handle_astar(request: &Value, _catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "astar";
    err(op, "NOT_IMPLEMENTED", "A* not yet implemented")
}

/// Delta Stepping dispatcher
pub fn handle_delta_stepping(request: &Value, _catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "delta_stepping";
    err(op, "NOT_IMPLEMENTED", "Delta Stepping not yet implemented")
}

/// Yen's K Shortest Paths dispatcher
pub fn handle_yens(request: &Value, _catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "yens";
    err(op, "NOT_IMPLEMENTED", "Yen's not yet implemented")
}

/// All Shortest Paths dispatcher
pub fn handle_all_shortest_paths(request: &Value, _catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "all_shortest_paths";
    err(op, "NOT_IMPLEMENTED", "All Shortest Paths not yet implemented")
}

/// Spanning Tree dispatcher
pub fn handle_spanning_tree(request: &Value, _catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "spanning_tree";
    err(op, "NOT_IMPLEMENTED", "Spanning Tree not yet implemented")
}

/// Topological Sort dispatcher
pub fn handle_topological_sort(request: &Value, _catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "topological_sort";
    err(op, "NOT_IMPLEMENTED", "Topological Sort not yet implemented")
}

/// Random Walk dispatcher
pub fn handle_random_walk(request: &Value, _catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "random_walk";
    err(op, "NOT_IMPLEMENTED", "Random Walk not yet implemented")
}
