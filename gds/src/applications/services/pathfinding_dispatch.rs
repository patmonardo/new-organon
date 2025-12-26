//! Pathfinding algorithm dispatch module.
//!
//! Consolidates pathfinding algorithm handling to reduce tsjson_napi.rs size.
//! Each algorithm has one entry point that dispatches on mode.

use crate::applications::algorithms::machinery::StreamProcessingTemplate;
use crate::applications::algorithms::metadata::Algorithm;
use crate::config::base_types::AlgoBaseConfig;
// GraphResources will be used when we add stats/mutate/write modes
#[allow(unused_imports)]
use crate::core::loading::GraphResources;
use crate::procedures::facades::pathfinding::Heuristic;
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

/// BFS dispatcher
pub fn handle_bfs(
    request: &Value,
    catalog: Arc<dyn GraphCatalog>,
) -> Value {
    let op = "bfs";
    let mode = parse_mode(request);

    let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: graphName");
    };

    let Some(source_node) = request.get("sourceNode").and_then(|v| v.as_u64()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: sourceNode");
    };

    let targets: Vec<u64> = request
        .get("targetNodes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|x| x.as_u64()).collect())
        .unwrap_or_default();

    let max_depth: Option<u32> = request
        .get("maxDepth")
        .and_then(|v| v.as_u64())
        .map(|d| d as u32);

    let track_paths: bool = request
        .get("trackPaths")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let concurrency: Option<usize> = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

    let delta: Option<usize> = request
        .get("delta")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

    let cfg = AlgoBaseConfig {
        concurrency: concurrency.unwrap_or_else(num_cpus::get),
        ..Default::default()
    };

    match mode {
        "stream" => {
            let template = StreamProcessingTemplate::new();
            let result = template.process(
                catalog.as_ref(),
                graph_name,
                &cfg,
                Algorithm::BFS,
                |resources| {
                    let mut builder = resources.facade().bfs().source(source_node);
                    if !targets.is_empty() {
                        builder = builder.targets(targets.clone());
                    }
                    if let Some(d) = max_depth {
                        builder = builder.max_depth(d);
                    }
                    if track_paths {
                        builder = builder.track_paths(true);
                    }
                    if let Some(c) = concurrency {
                        builder = builder.concurrency(c);
                    }
                    if let Some(d) = delta {
                        builder = builder.delta(d);
                    }
                    builder.stream()
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
                        })
                        .collect::<Vec<_>>()
                    })
                },
            );

            match result {
                Ok(Ok(rows)) => ok(op, json!({ "graphName": graph_name, "sourceNode": source_node, "rows": rows })),
                Ok(Err(e)) => err(op, "ERROR", &format!("BFS failed: {e}")),
                Err(e) => err(op, "ERROR", &format!("Processing failed: {e}")),
            }
        }
        "stats" => {
            // TODO: Implement stats mode
            err(op, "NOT_IMPLEMENTED", "BFS stats mode not yet implemented")
        }
        "mutate" => {
            // TODO: Implement mutate mode
            err(op, "NOT_IMPLEMENTED", "BFS mutate mode not yet implemented")
        }
        "write" => {
            // TODO: Implement write mode
            err(op, "NOT_IMPLEMENTED", "BFS write mode not yet implemented")
        }
        _ => err(op, "INVALID_MODE", &format!("Unknown mode: {mode}")),
    }
}

/// DFS dispatcher
pub fn handle_dfs(
    request: &Value,
    catalog: Arc<dyn GraphCatalog>,
) -> Value {
    let op = "dfs";
    let mode = parse_mode(request);

    let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: graphName");
    };

    let Some(source_node) = request.get("sourceNode").and_then(|v| v.as_u64()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: sourceNode");
    };

    let targets: Vec<u64> = request
        .get("targetNodes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|x| x.as_u64()).collect())
        .unwrap_or_default();

    let max_depth: Option<u32> = request
        .get("maxDepth")
        .and_then(|v| v.as_u64())
        .map(|d| d as u32);

    let track_paths: bool = request
        .get("trackPaths")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let concurrency: Option<usize> = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

    let cfg = AlgoBaseConfig {
        concurrency: concurrency.unwrap_or_else(num_cpus::get),
        ..Default::default()
    };

    match mode {
        "stream" => {
            let template = StreamProcessingTemplate::new();
            let result = template.process(
                catalog.as_ref(),
                graph_name,
                &cfg,
                Algorithm::DFS,
                |resources| {
                    let mut builder = resources.facade().dfs().source(source_node);
                    if !targets.is_empty() {
                        builder = builder.targets(targets.clone());
                    }
                    if let Some(d) = max_depth {
                        builder = builder.max_depth(d);
                    }
                    if track_paths {
                        builder = builder.track_paths(true);
                    }
                    if let Some(c) = concurrency {
                        builder = builder.concurrency(c);
                    }
                    builder.stream()
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
                        })
                        .collect::<Vec<_>>()
                    })
                },
            );

            match result {
                Ok(Ok(rows)) => ok(op, json!({ "graphName": graph_name, "sourceNode": source_node, "rows": rows })),
                Ok(Err(e)) => err(op, "ERROR", &format!("DFS failed: {e}")),
                Err(e) => err(op, "ERROR", &format!("Processing failed: {e}")),
            }
        }
        _ => err(op, "NOT_IMPLEMENTED", &format!("{op} {mode} mode not yet implemented")),
    }
}

/// Dijkstra dispatcher
pub fn handle_dijkstra(
    request: &Value,
    catalog: Arc<dyn GraphCatalog>,
) -> Value {
    let op = "dijkstra";
    let mode = parse_mode(request);

    let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: graphName");
    };

    let Some(source_node) = request.get("sourceNode").and_then(|v| v.as_u64()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: sourceNode");
    };

    let target: Option<u64> = request.get("targetNode").and_then(|v| v.as_u64());
    let targets: Vec<u64> = request
        .get("targetNodes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|x| x.as_u64()).collect())
        .unwrap_or_default();

    let weight_property: String = request
        .get("weightProperty")
        .and_then(|v| v.as_str())
        .unwrap_or("weight")
        .to_string();

    let direction: String = request
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("outgoing")
        .to_string();

    let track_relationships: bool = request
        .get("trackRelationships")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let concurrency: Option<usize> = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

    let cfg = AlgoBaseConfig {
        concurrency: concurrency.unwrap_or_else(num_cpus::get),
        ..Default::default()
    };

    match mode {
        "stream" => {
            let template = StreamProcessingTemplate::new();
            let result = template.process(
                catalog.as_ref(),
                graph_name,
                &cfg,
                Algorithm::Dijkstra,
                |resources| {
                    let graph = resources.facade();
                    let mut builder = graph.dijkstra().source(source_node);

                    if let Some(t) = target {
                        builder = builder.target(t);
                    } else if !targets.is_empty() {
                        builder = builder.targets(targets.clone());
                    }

                    builder = builder
                        .weight_property(&weight_property)
                        .direction(&direction)
                        .track_relationships(track_relationships);

                    if let Some(c) = concurrency {
                        builder = builder.concurrency(c);
                    }

                    builder.stream()
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
                        })
                        .collect::<Vec<_>>()
                    })
                },
            );

            match result {
                Ok(Ok(rows)) => ok(op, json!({ "graphName": graph_name, "sourceNode": source_node, "rows": rows })),
                Ok(Err(e)) => err(op, "ERROR", &format!("Dijkstra failed: {e}")),
                Err(e) => err(op, "ERROR", &format!("Processing failed: {e}")),
            }
        }
        _ => err(op, "NOT_IMPLEMENTED", &format!("{op} {mode} mode not yet implemented")),
    }
}

/// Bellman-Ford dispatcher
pub fn handle_bellman_ford(
    request: &Value,
    catalog: Arc<dyn GraphCatalog>,
) -> Value {
    let op = "bellman_ford";
    let mode = parse_mode(request);

    let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: graphName");
    };

    let Some(source_node) = request.get("sourceNode").and_then(|v| v.as_u64()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: sourceNode");
    };

    let weight_property: String = request
        .get("weightProperty")
        .and_then(|v| v.as_str())
        .unwrap_or("weight")
        .to_string();

    let direction: String = request
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("outgoing")
        .to_string();

    let relationship_types: Vec<String> = request
        .get("relationshipTypes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|x| x.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let track_negative_cycles: bool = request
        .get("trackNegativeCycles")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let track_paths: bool = request
        .get("trackPaths")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let concurrency: Option<usize> = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

    let cfg = AlgoBaseConfig {
        concurrency: concurrency.unwrap_or_else(num_cpus::get),
        ..Default::default()
    };

    match mode {
        "stream" => {
            let template = StreamProcessingTemplate::new();
            let result = template.process(
                catalog.as_ref(),
                graph_name,
                &cfg,
                Algorithm::BellmanFord,
                |resources| {
                    let graph = resources.facade();
                    let mut builder = graph.bellman_ford().source(source_node);

                    builder = builder
                        .weight_property(&weight_property)
                        .direction(&direction)
                        .track_negative_cycles(track_negative_cycles)
                        .track_paths(track_paths);

                    if !relationship_types.is_empty() {
                        builder = builder.relationship_types(relationship_types.clone());
                    }

                    if let Some(c) = concurrency {
                        builder = builder.concurrency(c);
                    }

                    builder.stream()
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
                        })
                        .collect::<Vec<_>>()
                    })
                },
            );

            match result {
                Ok(Ok(rows)) => ok(op, json!({ "graphName": graph_name, "sourceNode": source_node, "rows": rows })),
                Ok(Err(e)) => err(op, "ERROR", &format!("Bellman-Ford failed: {e}")),
                Err(e) => err(op, "ERROR", &format!("Processing failed: {e}")),
            }
        }
        _ => err(op, "NOT_IMPLEMENTED", &format!("{op} {mode} mode not yet implemented")),
    }
}

/// A* dispatcher
pub fn handle_astar(
    request: &Value,
    catalog: Arc<dyn GraphCatalog>,
) -> Value {
    let op = "astar";
    let mode = parse_mode(request);

    let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: graphName");
    };

    let Some(source_node) = request.get("sourceNode").and_then(|v| v.as_u64()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: sourceNode");
    };

    let target: Option<u64> = request.get("targetNode").and_then(|v| v.as_u64());
    let targets: Vec<u64> = request
        .get("targetNodes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|x| x.as_u64()).collect())
        .unwrap_or_default();

    let weight_property: String = request
        .get("weightProperty")
        .and_then(|v| v.as_str())
        .unwrap_or("weight")
        .to_string();

    let direction: String = request
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("outgoing")
        .to_string();

    let relationship_types: Vec<String> = request
        .get("relationshipTypes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|x| x.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let heuristic_str = request
        .get("heuristic")
        .and_then(|v| v.as_str())
        .unwrap_or("manhattan");
    let heuristic = match heuristic_str.to_lowercase().as_str() {
        "euclidean" => Heuristic::Euclidean,
        "haversine" => Heuristic::Haversine,
        _ => Heuristic::Manhattan,
    };

    let concurrency: Option<usize> = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

    let cfg = AlgoBaseConfig {
        concurrency: concurrency.unwrap_or_else(num_cpus::get),
        ..Default::default()
    };

    match mode {
        "stream" => {
            let template = StreamProcessingTemplate::new();
            let result = template.process(
                catalog.as_ref(),
                graph_name,
                &cfg,
                Algorithm::AStar,
                |resources| {
                    let graph = resources.facade();
                    let mut builder = graph.astar().source(source_node);

                    if let Some(t) = target {
                        builder = builder.target(t);
                    } else if !targets.is_empty() {
                        builder = builder.targets(targets.clone());
                    }

                    builder = builder
                        .weight_property(&weight_property)
                        .direction(&direction)
                        .heuristic(heuristic);

                    if !relationship_types.is_empty() {
                        builder = builder.relationship_types(relationship_types.clone());
                    }

                    if let Some(c) = concurrency {
                        builder = builder.concurrency(c);
                    }

                    builder.stream()
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
                        })
                        .collect::<Vec<_>>()
                    })
                },
            );

            match result {
                Ok(Ok(rows)) => ok(op, json!({ "graphName": graph_name, "sourceNode": source_node, "rows": rows })),
                Ok(Err(e)) => err(op, "ERROR", &format!("A* failed: {e}")),
                Err(e) => err(op, "ERROR", &format!("Processing failed: {e}")),
            }
        }
        _ => err(op, "NOT_IMPLEMENTED", &format!("{op} {mode} mode not yet implemented")),
    }
}

/// Delta Stepping dispatcher
pub fn handle_delta_stepping(
    request: &Value,
    catalog: Arc<dyn GraphCatalog>,
) -> Value {
    let op = "delta_stepping";
    let mode = parse_mode(request);

    let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: graphName");
    };

    let Some(source_node) = request.get("sourceNode").and_then(|v| v.as_u64()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: sourceNode");
    };

    let delta: f64 = request
        .get("delta")
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0);

    let weight_property: String = request
        .get("weightProperty")
        .and_then(|v| v.as_str())
        .unwrap_or("weight")
        .to_string();

    let direction: String = request
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("outgoing")
        .to_string();

    let relationship_types: Vec<String> = request
        .get("relationshipTypes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|x| x.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let concurrency: Option<usize> = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

    let cfg = AlgoBaseConfig {
        concurrency: concurrency.unwrap_or_else(num_cpus::get),
        ..Default::default()
    };

    match mode {
        "stream" => {
            let template = StreamProcessingTemplate::new();
            let result = template.process(
                catalog.as_ref(),
                graph_name,
                &cfg,
                Algorithm::DeltaStepping,
                |resources| {
                    let graph = resources.facade();
                    let mut builder = graph.delta_stepping().source(source_node).delta(delta);

                    builder = builder.weight_property(&weight_property).direction(&direction);

                    if !relationship_types.is_empty() {
                        builder = builder.relationship_types(relationship_types.clone());
                    }

                    if let Some(c) = concurrency {
                        builder = builder.concurrency(c);
                    }

                    builder.stream()
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
                        })
                        .collect::<Vec<_>>()
                    })
                },
            );

            match result {
                Ok(Ok(rows)) => ok(op, json!({ "graphName": graph_name, "sourceNode": source_node, "rows": rows })),
                Ok(Err(e)) => err(op, "ERROR", &format!("Delta Stepping failed: {e}")),
                Err(e) => err(op, "ERROR", &format!("Processing failed: {e}")),
            }
        }
        _ => err(op, "NOT_IMPLEMENTED", &format!("{op} {mode} mode not yet implemented")),
    }
}

/// Yen's K Shortest Paths dispatcher
pub fn handle_yens(
    request: &Value,
    catalog: Arc<dyn GraphCatalog>,
) -> Value {
    let op = "yens";
    let mode = parse_mode(request);

    let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: graphName");
    };

    let Some(source_node) = request.get("sourceNode").and_then(|v| v.as_u64()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: sourceNode");
    };

    let Some(target_node) = request.get("targetNode").and_then(|v| v.as_u64()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: targetNode");
    };

    let k: usize = request
        .get("k")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize)
        .unwrap_or(3);

    let weight_property: String = request
        .get("weightProperty")
        .and_then(|v| v.as_str())
        .unwrap_or("weight")
        .to_string();

    let direction: String = request
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("outgoing")
        .to_string();

    let relationship_types: Vec<String> = request
        .get("relationshipTypes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|x| x.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let concurrency: Option<usize> = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

    let cfg = AlgoBaseConfig {
        concurrency: concurrency.unwrap_or(1),
        ..Default::default()
    };

    match mode {
        "stream" => {
            let template = StreamProcessingTemplate::new();
            let result = template.process(
                catalog.as_ref(),
                graph_name,
                &cfg,
                Algorithm::Yens,
                |resources| {
                    let graph = resources.facade();
                    let mut builder = graph
                        .yens()
                        .source(source_node)
                        .target(target_node)
                        .k(k);

                    builder = builder.weight_property(&weight_property).direction(&direction);

                    if !relationship_types.is_empty() {
                        builder = builder.relationship_types(relationship_types.clone());
                    }

                    if let Some(c) = concurrency {
                        builder = builder.concurrency(c);
                    }

                    builder.stream()
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
                        })
                        .collect::<Vec<_>>()
                    })
                },
            );

            match result {
                Ok(Ok(rows)) => ok(op, json!({ "graphName": graph_name, "sourceNode": source_node, "targetNode": target_node, "k": k, "rows": rows })),
                Ok(Err(e)) => err(op, "ERROR", &format!("Yen's failed: {e}")),
                Err(e) => err(op, "ERROR", &format!("Processing failed: {e}")),
            }
        }
        _ => err(op, "NOT_IMPLEMENTED", &format!("{op} {mode} mode not yet implemented")),
    }
}

/// All Shortest Paths dispatcher
pub fn handle_all_shortest_paths(
    request: &Value,
    catalog: Arc<dyn GraphCatalog>,
) -> Value {
    let op = "all_shortest_paths";
    let mode = parse_mode(request);

    let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: graphName");
    };

    let weighted: bool = request
        .get("weighted")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let weight_property: String = request
        .get("weightProperty")
        .and_then(|v| v.as_str())
        .unwrap_or("weight")
        .to_string();

    let direction: String = request
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("outgoing")
        .to_string();

    let relationship_types: Vec<String> = request
        .get("relationshipTypes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|x| x.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let max_results: Option<usize> = request
        .get("maxResults")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

    let concurrency: Option<usize> = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

    let cfg = AlgoBaseConfig {
        concurrency: concurrency.unwrap_or_else(num_cpus::get),
        ..Default::default()
    };

    match mode {
        "stream" => {
            let template = StreamProcessingTemplate::new();
            let result = template.process(
                catalog.as_ref(),
                graph_name,
                &cfg,
                Algorithm::AllShortestPaths,
                |resources| {
                    let graph = resources.facade();
                    let mut builder = graph.all_shortest_paths();

                    if weighted {
                        builder = builder.weighted();
                    }

                    builder = builder
                        .weight_property(&weight_property)
                        .direction(&direction);

                    if !relationship_types.is_empty() {
                        let refs: Vec<&str> = relationship_types.iter().map(|s| s.as_str()).collect();
                        builder = builder.relationship_types(refs);
                    }

                    if let Some(max) = max_results {
                        builder = builder.max_results(Some(max));
                    }

                    if let Some(c) = concurrency {
                        builder = builder.concurrency(c);
                    }

                    builder.stream()
                },
                |stream_result| {
                    stream_result.map(|iter| {
                        iter.map(|r| {
                            json!({
                                "source": r.source,
                                "target": r.target,
                                "distance": r.distance,
                            })
                        })
                        .collect::<Vec<_>>()
                    })
                },
            );

            match result {
                Ok(Ok(rows)) => ok(op, json!({ "graphName": graph_name, "rows": rows })),
                Ok(Err(e)) => err(op, "ERROR", &format!("All Shortest Paths failed: {e}")),
                Err(e) => err(op, "ERROR", &format!("Processing failed: {e}")),
            }
        }
        _ => err(op, "NOT_IMPLEMENTED", &format!("{op} {mode} mode not yet implemented")),
    }
}

/// Spanning Tree dispatcher
pub fn handle_spanning_tree(
    request: &Value,
    catalog: Arc<dyn GraphCatalog>,
) -> Value {
    let op = "spanning_tree";
    let mode = parse_mode(request);

    let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: graphName");
    };

    let start_node: u64 = request
        .get("startNode")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let compute_minimum: bool = request
        .get("computeMinimum")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let weight_property: String = request
        .get("weightProperty")
        .and_then(|v| v.as_str())
        .unwrap_or("weight")
        .to_string();

    let direction: String = request
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("undirected")
        .to_string();

    let relationship_types: Vec<String> = request
        .get("relationshipTypes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|x| x.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let concurrency: Option<usize> = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

    let cfg = AlgoBaseConfig {
        concurrency: concurrency.unwrap_or(1),
        ..Default::default()
    };

    match mode {
        "stream" => {
            let template = StreamProcessingTemplate::new();
            let result = template.process(
                catalog.as_ref(),
                graph_name,
                &cfg,
                Algorithm::SpanningTree,
                |resources| {
                    let graph = resources.facade();
                    let mut builder = graph.spanning_tree().start_node(start_node);

                    if !compute_minimum {
                        builder = builder.maximum();
                    }

                    builder = builder
                        .weight_property(&weight_property)
                        .direction(&direction);

                    if !relationship_types.is_empty() {
                        let refs: Vec<&str> = relationship_types.iter().map(|s| s.as_str()).collect();
                        builder = builder.relationship_types(refs);
                    }

                    if let Some(c) = concurrency {
                        builder = builder.concurrency(c);
                    }

                    builder.stream()
                },
                |stream_result| {
                    stream_result.map(|iter| {
                        iter.map(|r| {
                            json!({
                                "node": r.node,
                                "parent": r.parent,
                                "costToParent": r.cost_to_parent,
                            })
                        })
                        .collect::<Vec<_>>()
                    })
                },
            );

            match result {
                Ok(Ok(rows)) => ok(op, json!({ "graphName": graph_name, "startNode": start_node, "rows": rows })),
                Ok(Err(e)) => err(op, "ERROR", &format!("Spanning Tree failed: {e}")),
                Err(e) => err(op, "ERROR", &format!("Processing failed: {e}")),
            }
        }
        _ => err(op, "NOT_IMPLEMENTED", &format!("{op} {mode} mode not yet implemented")),
    }
}

/// Topological Sort dispatcher
pub fn handle_topological_sort(
    request: &Value,
    catalog: Arc<dyn GraphCatalog>,
) -> Value {
    let op = "topological_sort";
    let mode = parse_mode(request);

    let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: graphName");
    };

    let compute_max_distance: bool = request
        .get("computeMaxDistance")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let concurrency: Option<usize> = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

    let cfg = AlgoBaseConfig {
        concurrency: concurrency.unwrap_or(1),
        ..Default::default()
    };

    match mode {
        "stream" => {
            let template = StreamProcessingTemplate::new();
            let result = template.process(
                catalog.as_ref(),
                graph_name,
                &cfg,
                Algorithm::TopologicalSort,
                |resources| {
                    let graph = resources.facade();
                    let builder = graph.topological_sort().compute_max_distance(compute_max_distance);
                    builder.stream()
                },
                |stream_result| {
                    stream_result.map(|iter| {
                        iter.map(|r| {
                            json!({
                                "nodeId": r.node_id,
                                "maxDistance": r.max_distance,
                            })
                        })
                        .collect::<Vec<_>>()
                    })
                },
            );

            match result {
                Ok(Ok(rows)) => ok(op, json!({ "graphName": graph_name, "rows": rows })),
                Ok(Err(e)) => err(op, "ERROR", &format!("Topological Sort failed: {e}")),
                Err(e) => err(op, "ERROR", &format!("Processing failed: {e}")),
            }
        }
        _ => err(op, "NOT_IMPLEMENTED", &format!("{op} {mode} mode not yet implemented")),
    }
}

/// Random Walk dispatcher
pub fn handle_random_walk(
    request: &Value,
    catalog: Arc<dyn GraphCatalog>,
) -> Value {
    let op = "random_walk";
    let mode = parse_mode(request);

    let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
        return err(op, "INVALID_REQUEST", "Missing required field: graphName");
    };

    let walks_per_node: usize = request
        .get("walksPerNode")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize)
        .unwrap_or(10);

    let walk_length: usize = request
        .get("walkLength")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize)
        .unwrap_or(80);

    let return_factor: f64 = request
        .get("returnFactor")
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0);

    let in_out_factor: f64 = request
        .get("inOutFactor")
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0);

    let source_nodes: Vec<u64> = request
        .get("sourceNodes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|x| x.as_u64()).collect())
        .unwrap_or_default();

    let random_seed: Option<u64> = request.get("randomSeed").and_then(|v| v.as_u64());

    let concurrency: Option<usize> = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

    let cfg = AlgoBaseConfig {
        concurrency: concurrency.unwrap_or(1),
        ..Default::default()
    };

    match mode {
        "stream" => {
            let template = StreamProcessingTemplate::new();
            let result = template.process(
                catalog.as_ref(),
                graph_name,
                &cfg,
                Algorithm::RandomWalk,
                |resources| {
                    let graph = resources.facade();
                    let mut builder = graph
                        .random_walk()
                        .walks_per_node(walks_per_node)
                        .walk_length(walk_length)
                        .return_factor(return_factor)
                        .in_out_factor(in_out_factor);

                    if !source_nodes.is_empty() {
                        builder = builder.source_nodes(source_nodes.clone());
                    }

                    if let Some(seed) = random_seed {
                        builder = builder.random_seed(seed);
                    }

                    builder.stream()
                },
                |stream_result| {
                    stream_result.map(|iter| {
                        iter.map(|r| {
                            json!({
                                "path": r.path,
                            })
                        })
                        .collect::<Vec<_>>()
                    })
                },
            );

            match result {
                Ok(Ok(rows)) => ok(op, json!({ "graphName": graph_name, "rows": rows })),
                Ok(Err(e)) => err(op, "ERROR", &format!("Random Walk failed: {e}")),
                Err(e) => err(op, "ERROR", &format!("Processing failed: {e}")),
            }
        }
        _ => err(op, "NOT_IMPLEMENTED", &format!("{op} {mode} mode not yet implemented")),
    }
}

