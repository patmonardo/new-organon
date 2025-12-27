use crate::types::catalog::GraphCatalog;
use serde_json::Value;
use std::sync::Arc;

pub fn handle_bfs(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::pathfinding::handle_bfs(request, catalog)
}

pub fn handle_dfs(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::pathfinding::handle_dfs(request, catalog)
}

pub fn handle_dijkstra(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::pathfinding::handle_dijkstra(request, catalog)
}

pub fn handle_bellman_ford(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::pathfinding::handle_bellman_ford(request, catalog)
}

pub fn handle_astar(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::pathfinding::handle_astar(request, catalog)
}

pub fn handle_delta_stepping(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::pathfinding::handle_delta_stepping(request, catalog)
}

pub fn handle_dag_longest_path(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::pathfinding::handle_dag_longest_path(request, catalog)
}

pub fn handle_kspanningtree(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::pathfinding::handle_kspanningtree(request, catalog)
}

pub fn handle_all_shortest_paths(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::pathfinding::handle_all_shortest_paths(request, catalog)
}

pub fn handle_spanning_tree(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::pathfinding::handle_spanning_tree(request, catalog)
}

pub fn handle_steiner_tree(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::pathfinding::handle_steiner_tree(request, catalog)
}

pub fn handle_topological_sort(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::pathfinding::handle_topological_sort(request, catalog)
}

pub fn handle_yens(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::pathfinding::handle_yens(request, catalog)
}

pub fn handle_random_walk(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::pathfinding::handle_random_walk(request, catalog)
}

pub fn handle_pagerank(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::centrality::pagerank::handle_pagerank(request, catalog)
}

pub fn handle_articulation_points(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::centrality::articulation_points::handle_articulation_points(request, catalog)
}

pub fn handle_betweenness(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::centrality::betweenness::handle_betweenness(request, catalog)
}

pub fn handle_bridges(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::centrality::bridges::handle_bridges(request, catalog)
}

pub fn handle_celf(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::centrality::celf::handle_celf(request, catalog)
}

pub fn handle_closeness(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::centrality::closeness::handle_closeness(request, catalog)
}

pub fn handle_degree_centrality(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::centrality::degree_centrality::handle_degree_centrality(request, catalog)
}

pub fn handle_harmonic(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::centrality::harmonic::handle_harmonic(request, catalog)
}

pub fn handle_hits(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    crate::applications::algorithms::centrality::hits::handle_hits(request, catalog)
}
