//! Integration tests for modularity

use super::computation::ModularityComputationRuntime;

#[test]
fn test_perfect_communities() {
    // Two separate cliques: no edges between them
    // 0-1-2 (community 0) and 3-4-5 (community 1)
    let edges = vec![
        vec![(1, 1.0), (2, 1.0)],
        vec![(0, 1.0), (2, 1.0)],
        vec![(0, 1.0), (1, 1.0)],
        vec![(4, 1.0), (5, 1.0)],
        vec![(3, 1.0), (5, 1.0)],
        vec![(3, 1.0), (4, 1.0)],
    ];

    let communities = vec![0, 0, 0, 1, 1, 1];

    let runtime = ModularityComputationRuntime::new();
    let result = runtime.compute(
        6,
        |node| Some(communities[node]),
        |node| edges[node].clone(),
    );

    // Perfect separation should have high modularity
    assert_eq!(result.community_count, 2);
    assert!(result.total_modularity > 0.3); // Should be positive and significant
}

#[test]
fn test_poor_communities() {
    // Linear chain split badly: 0-1-2-3 with communities [0,1] vs [2,3]
    let edges = vec![
        vec![(1, 1.0)],
        vec![(0, 1.0), (2, 1.0)],
        vec![(1, 1.0), (3, 1.0)],
        vec![(2, 1.0)],
    ];

    let communities = vec![0, 0, 1, 1];

    let runtime = ModularityComputationRuntime::new();
    let result = runtime.compute(
        4,
        |node| Some(communities[node]),
        |node| edges[node].clone(),
    );

    assert_eq!(result.community_count, 2);

    // Poor split: many edges cross communities
    // Should have lower modularity than perfect case
    assert!(result.total_modularity < 0.3);
}

#[test]
fn test_single_community() {
    // All nodes in one community
    let edges = vec![
        vec![(1, 1.0), (2, 1.0)],
        vec![(0, 1.0), (2, 1.0)],
        vec![(0, 1.0), (1, 1.0)],
    ];

    let communities = vec![0, 0, 0];

    let runtime = ModularityComputationRuntime::new();
    let result = runtime.compute(
        3,
        |node| Some(communities[node]),
        |node| edges[node].clone(),
    );

    assert_eq!(result.community_count, 1);

    // Single community: modularity depends on comparing to null model
    // Should be close to 0 (no structure to detect)
    assert!(result.total_modularity.abs() < 0.1);
}

#[test]
fn test_weighted_edges() {
    // Two nodes with weighted edge
    let edges = vec![vec![(1, 10.0)], vec![(0, 10.0)]];

    // Different communities
    let communities = vec![0, 1];

    let runtime = ModularityComputationRuntime::new();
    let result = runtime.compute(
        2,
        |node| Some(communities[node]),
        |node| edges[node].clone(),
    );

    assert_eq!(result.community_count, 2);

    // All edges cross communities: should have negative modularity
    assert!(result.total_modularity < 0.0);
}

#[test]
fn test_isolated_nodes() {
    // Node 1 has no community
    let edges = vec![
        vec![(2, 1.0)],
        vec![], // Isolated
        vec![(0, 1.0)],
    ];

    let communities = vec![Some(0), None, Some(0)];

    let runtime = ModularityComputationRuntime::new();
    let result = runtime.compute(
        3,
        |node| communities[node],
        |node| edges[node].clone(),
    );

    // Only one community (node 1 skipped)
    assert_eq!(result.community_count, 1);
}

#[test]
fn test_empty_graph() {
    let edges: Vec<Vec<(usize, f64)>> = vec![vec![], vec![], vec![]];
    let communities = vec![0, 0, 1];

    let runtime = ModularityComputationRuntime::new();
    let result = runtime.compute(
        3,
        |node| Some(communities[node]),
        |node| edges[node].clone(),
    );

    // No edges: modularity should be 0
    assert_eq!(result.total_modularity, 0.0);
}
