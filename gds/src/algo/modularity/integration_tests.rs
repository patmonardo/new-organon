use super::ModularityComputationRuntime;

#[test]
fn modularity_empty_is_zero() {
    let runtime = ModularityComputationRuntime::new();
    let result = runtime.compute(0, |_| None, |_| Vec::new());
    assert_eq!(result.total_modularity, 0.0);
    assert_eq!(result.community_count, 0);
}

#[test]
fn modularity_two_nodes_same_community_beats_split() {
    // Two nodes connected with weight 1.
    // - If both are in the same community, modularity is 0.0.
    // - If they are split, modularity is negative.
    // Adjacency is undirected (each endpoint sees the edge).
    let runtime = ModularityComputationRuntime::new();

    let get_neighbors = |node: usize| match node {
        0 => vec![(1, 1.0)],
        1 => vec![(0, 1.0)],
        _ => vec![],
    };

    let same = runtime.compute(2, |_| Some(7), get_neighbors);
    assert!((same.total_modularity - 0.0).abs() < 1e-12);
    assert_eq!(same.community_count, 1);

    let split = runtime.compute(2, |node| Some(node as u64), get_neighbors);
    assert!(split.total_modularity < same.total_modularity);
}
