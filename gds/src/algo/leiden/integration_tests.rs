use super::{AdjacencyGraph, LeidenComputationRuntime};
use super::spec::LeidenConfig;
use crate::concurrency::TerminationFlag;

fn build_adj(node_count: usize, edges: &[(usize, usize, f64)]) -> AdjacencyGraph {
    let mut adj = vec![Vec::new(); node_count];
    for &(u, v, w) in edges {
        adj[u].push((v, w));
        adj[v].push((u, w));
    }
    AdjacencyGraph::new(node_count, adj)
}

#[test]
fn leiden_empty_graph() {
    let graph = build_adj(0, &[]);
    let config = LeidenConfig::default();
    let mut runtime = LeidenComputationRuntime::new();
    let result = runtime
        .compute(&graph, &config, &TerminationFlag::default())
        .unwrap();
    assert_eq!(result.communities.len(), 0);
    assert_eq!(result.levels, 0);
}

#[test]
fn leiden_isolated_nodes_each_community() {
    let graph = build_adj(5, &[]);
    let config = LeidenConfig::default();
    let mut runtime = LeidenComputationRuntime::new();
    let result = runtime
        .compute(&graph, &config, &TerminationFlag::default())
        .unwrap();

    let mut ids = result.communities.clone();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 5);
}

#[test]
fn leiden_two_components_stay_separate() {
    // Component A: 0-1-2 ; Component B: 3-4
    let graph = build_adj(5, &[(0, 1, 1.0), (1, 2, 1.0), (3, 4, 1.0)]);

    let config = LeidenConfig {
        max_iterations: 10,
        tolerance: 0.0,
        ..LeidenConfig::default()
    };

    let mut runtime = LeidenComputationRuntime::new();
    let result = runtime
        .compute(&graph, &config, &TerminationFlag::default())
        .unwrap();

    let c0 = result.communities[0];
    let c1 = result.communities[1];
    let c2 = result.communities[2];
    let c3 = result.communities[3];
    let c4 = result.communities[4];

    assert_eq!(c0, c1);
    assert_eq!(c1, c2);
    assert_eq!(c3, c4);
    assert_ne!(c0, c3);
}

#[test]
fn leiden_refinement_splits_disconnected_seeded_community() {
    // Force all nodes into the same starting community, but with two disconnected components.
    let graph = build_adj(4, &[(0, 1, 1.0), (2, 3, 1.0)]);

    let config = LeidenConfig {
        seed_communities: Some(vec![0, 0, 0, 0]),
        max_iterations: 1,
        tolerance: 0.0,
        ..LeidenConfig::default()
    };

    let mut runtime = LeidenComputationRuntime::new();
    let result = runtime
        .compute(&graph, &config, &TerminationFlag::default())
        .unwrap();

    assert_ne!(result.communities[0], result.communities[2]);
    assert_eq!(result.communities[0], result.communities[1]);
    assert_eq!(result.communities[2], result.communities[3]);
}
