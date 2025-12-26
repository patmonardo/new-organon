use crate::types::catalog::GraphCatalog;
use serde_json::Value;
use std::sync::Arc;

pub fn handle_bfs(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::services::pathfinding_dispatch::handle_bfs(request, catalog)
}

pub fn handle_dfs(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::services::pathfinding_dispatch::handle_dfs(request, catalog)
}

pub fn handle_dijkstra(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::services::pathfinding_dispatch::handle_dijkstra(request, catalog)
}

pub fn handle_bellman_ford(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::services::pathfinding_dispatch::handle_bellman_ford(request, catalog)
}

pub fn handle_astar(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::services::pathfinding_dispatch::handle_astar(request, catalog)
}

pub fn handle_delta_stepping(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::services::pathfinding_dispatch::handle_delta_stepping(request, catalog)
}

pub fn handle_kspanningtree(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::services::pathfinding_dispatch::handle_kspanningtree(request, catalog)
}

pub fn handle_yens(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::services::pathfinding_dispatch::handle_yens(request, catalog)
}

pub fn handle_all_shortest_paths(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::services::pathfinding_dispatch::handle_all_shortest_paths(request, catalog)
}

pub fn handle_spanning_tree(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::services::pathfinding_dispatch::handle_spanning_tree(request, catalog)
}

pub fn handle_topological_sort(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::services::pathfinding_dispatch::handle_topological_sort(request, catalog)
}

pub fn handle_random_walk(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::services::pathfinding_dispatch::handle_random_walk(request, catalog)
}
