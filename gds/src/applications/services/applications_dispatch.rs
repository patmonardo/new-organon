use crate::applications::services::algorithms_dispatch;
use crate::applications::services::graph_store_catalog_dispatch;
use crate::core::User;
use crate::types::catalog::GraphCatalog;
use crate::types::graph_store::DatabaseId;
use serde_json::{json, Value};
use std::sync::Arc;

fn err(op: &str, code: &str, message: &str) -> Value {
    json!({
        "ok": false,
        "op": op,
        "error": { "code": code, "message": message }
    })
}

/// Dispatches algorithm requests by op name.
///
/// This keeps the TS-JSON boundary (`tsjson_napi.rs`) thin and makes the routing
/// shape stable for schema generation.
pub fn handle_algorithms(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = request.get("op").and_then(|v| v.as_str()).unwrap_or("");

    match op {
        // Pathfinding
        "bfs" => algorithms_dispatch::handle_bfs(request, catalog),
        "dfs" => algorithms_dispatch::handle_dfs(request, catalog),
        "dijkstra" => algorithms_dispatch::handle_dijkstra(request, catalog),
        "bellman_ford" => algorithms_dispatch::handle_bellman_ford(request, catalog),
        "astar" => algorithms_dispatch::handle_astar(request, catalog),
        "delta_stepping" => algorithms_dispatch::handle_delta_stepping(request, catalog),
        "dag_longest_path" => algorithms_dispatch::handle_dag_longest_path(request, catalog),
        "kspanningtree" => algorithms_dispatch::handle_kspanningtree(request, catalog),
        "yens" => algorithms_dispatch::handle_yens(request, catalog),
        "all_shortest_paths" => algorithms_dispatch::handle_all_shortest_paths(request, catalog),
        "spanning_tree" => algorithms_dispatch::handle_spanning_tree(request, catalog),
        "steiner_tree" => algorithms_dispatch::handle_steiner_tree(request, catalog),
        "topological_sort" => algorithms_dispatch::handle_topological_sort(request, catalog),
        "random_walk" => algorithms_dispatch::handle_random_walk(request, catalog),

        // Centrality
        "pagerank" => algorithms_dispatch::handle_pagerank(request, catalog),
        "articulation_points" => algorithms_dispatch::handle_articulation_points(request, catalog),
        "betweenness" => algorithms_dispatch::handle_betweenness(request, catalog),
        "bridges" => algorithms_dispatch::handle_bridges(request, catalog),
        "celf" => algorithms_dispatch::handle_celf(request, catalog),
        "closeness" => algorithms_dispatch::handle_closeness(request, catalog),
        "degree_centrality" => algorithms_dispatch::handle_degree_centrality(request, catalog),
        "harmonic" => algorithms_dispatch::handle_harmonic(request, catalog),
        "hits" => algorithms_dispatch::handle_hits(request, catalog),

        // Community
        "approx_max_kcut" => algorithms_dispatch::handle_approx_max_kcut(request, catalog),
        "conductance" => algorithms_dispatch::handle_conductance(request, catalog),
        "k1coloring" => algorithms_dispatch::handle_k1coloring(request, catalog),
        "kcore" => algorithms_dispatch::handle_kcore(request, catalog),
        "kmeans" => algorithms_dispatch::handle_kmeans(request, catalog),
        "label_propagation" => algorithms_dispatch::handle_label_propagation(request, catalog),
        "leiden" => algorithms_dispatch::handle_leiden(request, catalog),
        "louvain" => algorithms_dispatch::handle_louvain(request, catalog),
        "modularity" => algorithms_dispatch::handle_modularity(request, catalog),
        "scc" => algorithms_dispatch::handle_scc(request, catalog),
        "triangle" => algorithms_dispatch::handle_triangle(request, catalog),
        "wcc" => algorithms_dispatch::handle_wcc(request, catalog),

        // Similarity
        "knn" => algorithms_dispatch::handle_knn(request, catalog),
        "node_similarity" => algorithms_dispatch::handle_node_similarity(request, catalog),
        "filtered_knn" => algorithms_dispatch::handle_filtered_knn(request, catalog),
        "filtered_node_similarity" => {
            algorithms_dispatch::handle_filtered_node_similarity(request, catalog)
        }

        // Embeddings
        "fast_rp" => algorithms_dispatch::handle_fast_rp(request, catalog),
        "gat" => algorithms_dispatch::handle_gat(request, catalog),
        "hash_gnn" => algorithms_dispatch::handle_hash_gnn(request, catalog),
        "graphsage" => algorithms_dispatch::handle_graphsage(request, catalog),
        "node2vec" => algorithms_dispatch::handle_node2vec(request, catalog),

        // Misc
        "to_undirected" => algorithms_dispatch::handle_to_undirected(request, catalog),
        "scale_properties" => algorithms_dispatch::handle_scale_properties(request, catalog),
        "index_inverse" => algorithms_dispatch::handle_index_inverse(request, catalog),
        "collapse_path" => algorithms_dispatch::handle_collapse_path(request, catalog),

        _ => err(op, "UNSUPPORTED_OP", "Unsupported algorithms operation."),
    }
}

/// Dispatches graph-store-catalog requests.
///
/// Routing is delegated to the graph-store-catalog dispatcher.
pub fn handle_graph_store_catalog(
    request: &Value,
    user: &dyn User,
    db: &DatabaseId,
    catalog: Arc<dyn GraphCatalog>,
) -> Value {
    graph_store_catalog_dispatch::handle_graph_store_catalog(request, user, db, catalog)
}
