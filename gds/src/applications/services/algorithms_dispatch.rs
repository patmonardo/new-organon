use crate::applications::algorithms::prelude::*;
use crate::types::catalog::GraphCatalog;
use serde_json::Value;
use std::sync::Arc;

pub fn handle_bfs(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    pathfinding::handle_bfs(request, catalog)
}

pub fn handle_dfs(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    pathfinding::handle_dfs(request, catalog)
}

pub fn handle_dijkstra(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    pathfinding::handle_dijkstra(request, catalog)
}

pub fn handle_bellman_ford(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    pathfinding::handle_bellman_ford(request, catalog)
}

pub fn handle_astar(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    pathfinding::handle_astar(request, catalog)
}

pub fn handle_delta_stepping(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    pathfinding::handle_delta_stepping(request, catalog)
}

pub fn handle_dag_longest_path(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    pathfinding::handle_dag_longest_path(request, catalog)
}

pub fn handle_kspanningtree(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    pathfinding::handle_kspanningtree(request, catalog)
}

pub fn handle_all_shortest_paths(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    pathfinding::handle_all_shortest_paths(request, catalog)
}

pub fn handle_spanning_tree(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    pathfinding::handle_spanning_tree(request, catalog)
}

pub fn handle_steiner_tree(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    pathfinding::handle_steiner_tree(request, catalog)
}

pub fn handle_topological_sort(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    pathfinding::handle_topological_sort(request, catalog)
}

pub fn handle_yens(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    pathfinding::handle_yens(request, catalog)
}

pub fn handle_random_walk(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    pathfinding::handle_random_walk(request, catalog)
}

pub fn handle_pagerank(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    centrality::pagerank::handle_pagerank(request, catalog)
}

pub fn handle_articulation_points(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    centrality::articulation_points::handle_articulation_points(request, catalog)
}

pub fn handle_betweenness(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    centrality::betweenness::handle_betweenness(request, catalog)
}

pub fn handle_bridges(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    centrality::bridges::handle_bridges(request, catalog)
}

pub fn handle_celf(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    centrality::celf::handle_celf(request, catalog)
}

pub fn handle_closeness(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    centrality::closeness::handle_closeness(request, catalog)
}

pub fn handle_degree_centrality(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    centrality::degree_centrality::handle_degree_centrality(request, catalog)
}

pub fn handle_harmonic(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    centrality::harmonic::handle_harmonic(request, catalog)
}

pub fn handle_hits(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    centrality::hits::handle_hits(request, catalog)
}

pub fn handle_approx_max_kcut(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    community::approx_max_kcut::handle_approx_max_kcut(request, catalog)
}

pub fn handle_conductance(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    community::conductance::handle_conductance(request, catalog)
}

pub fn handle_k1coloring(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    community::k1coloring::handle_k1coloring(request, catalog)
}

pub fn handle_kcore(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    community::kcore::handle_kcore(request, catalog)
}

pub fn handle_kmeans(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    community::kmeans::handle_kmeans(request, catalog)
}

pub fn handle_label_propagation(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    community::label_propagation::handle_label_propagation(request, catalog)
}

pub fn handle_leiden(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    community::leiden::handle_leiden(request, catalog)
}

pub fn handle_louvain(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    community::louvain::handle_louvain(request, catalog)
}

pub fn handle_modularity(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    community::modularity::handle_modularity(request, catalog)
}

pub fn handle_scc(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    community::scc::handle_scc(request, catalog)
}

pub fn handle_triangle(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    community::triangle::handle_triangle(request, catalog)
}

pub fn handle_wcc(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    community::wcc::handle_wcc(request, catalog)
}

pub fn handle_knn(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    similarity::knn::handle_knn(request, catalog)
}

pub fn handle_node_similarity(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    similarity::node_similarity::handle_node_similarity(request, catalog)
}

pub fn handle_filtered_knn(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    similarity::filtered_knn::handle_filtered_knn(request, catalog)
}

pub fn handle_filtered_node_similarity(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    similarity::filtered_node_similarity::handle_filtered_node_similarity(request, catalog)
}

pub fn handle_fast_rp(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    embeddings::fast_rp::handle_fast_rp(request, catalog)
}

pub fn handle_gat(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    embeddings::gat::handle_gat(request, catalog)
}

pub fn handle_hash_gnn(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    embeddings::hash_gnn::handle_hash_gnn(request, catalog)
}

pub fn handle_graphsage(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    embeddings::graphsage::handle_graphsage(request, catalog)
}

pub fn handle_node2vec(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    embeddings::node2vec::handle_node2vec(request, catalog)
}

pub fn handle_to_undirected(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    miscellaneous::handle_to_undirected(request, catalog)
}

pub fn handle_scale_properties(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    miscellaneous::handle_scale_properties(request, catalog)
}

pub fn handle_index_inverse(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    miscellaneous::handle_index_inverse(request, catalog)
}

pub fn handle_collapse_path(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    miscellaneous::handle_collapse_path(request, catalog)
}
