//! Pathfinding algorithm dispatch module.
//!
//! Consolidates pathfinding algorithm handling to reduce tsjson_napi.rs size.
//! Each algorithm has one entry point that dispatches on mode.

use crate::applications::algorithms::pathfinding::all_shortest_paths;
use crate::applications::algorithms::pathfinding::astar;
use crate::applications::algorithms::pathfinding::bellman_ford;
use crate::applications::algorithms::pathfinding::bfs;
use crate::applications::algorithms::pathfinding::delta_stepping;
use crate::applications::algorithms::pathfinding::dfs;
use crate::applications::algorithms::pathfinding::dijkstra;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Common response builders
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}

/// BFS dispatcher
pub fn handle_bfs(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    bfs::handle_bfs(request, catalog)
}

/// DFS dispatcher
pub fn handle_dfs(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    dfs::handle_dfs(request, catalog)
}

/// Dijkstra dispatcher
pub fn handle_dijkstra(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    dijkstra::handle_dijkstra(request, catalog)
}

/// Bellman-Ford dispatcher
pub fn handle_bellman_ford(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    bellman_ford::handle_bellman_ford(request, catalog)
}

/// A* dispatcher
pub fn handle_astar(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    astar::handle_astar(request, catalog)
}

/// Delta Stepping dispatcher
pub fn handle_delta_stepping(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    delta_stepping::handle_delta_stepping(request, catalog)
}

/// Yen's K Shortest Paths dispatcher
pub fn handle_yens(_request: &Value, _catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "yens";
    err(op, "NOT_IMPLEMENTED", "Yen's not yet implemented")
}

/// All Shortest Paths dispatcher
pub fn handle_all_shortest_paths(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    all_shortest_paths::handle_all_shortest_paths(request, catalog)
}

/// Spanning Tree dispatcher
pub fn handle_spanning_tree(_request: &Value, _catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "spanning_tree";
    err(op, "NOT_IMPLEMENTED", "Spanning Tree not yet implemented")
}

/// Topological Sort dispatcher
pub fn handle_topological_sort(_request: &Value, _catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "topological_sort";
    err(op, "NOT_IMPLEMENTED", "Topological Sort not yet implemented")
}

/// Random Walk dispatcher
pub fn handle_random_walk(_request: &Value, _catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "random_walk";
    err(op, "NOT_IMPLEMENTED", "Random Walk not yet implemented")
}
