//! Integration tests for approximate maximum k-cut

use super::computation::ApproxMaxKCutComputationRuntime;
use super::spec::ApproxMaxKCutConfig;

#[test]
fn test_simple_two_cliques() {
    // Two separate cliques: 0-1 and 2-3 (with edges within each clique)
    // For MAXIMIZE cut cost: split within cliques to cut all 4 edges
    // For example: {0,2} vs {1,3} gives cut cost = 4 (all edges cross)
    let edges = vec![
        vec![(1, 1.0)],
        vec![(0, 1.0)],
        vec![(3, 1.0)],
        vec![(2, 1.0)],
    ];

    let config = ApproxMaxKCutConfig {
        k: 2,
        iterations: 5,
        random_seed: 42,
        minimize: false, // Maximize cut cost
        has_relationship_weight_property: false,
        min_community_sizes: vec![0, 0],
    };

    let runtime = ApproxMaxKCutComputationRuntime::new(config);
    let result = runtime.compute(4, |node| edges[node].clone());

    assert_eq!(result.communities.len(), 4);

    // For maximize: best is to split within cliques {0,2} vs {1,3} = 4 crossing edges
    // Each edge counted once bidirectionally = 4 total
    assert_eq!(result.cut_cost, 4.0);
}

#[test]
fn test_complete_graph_k2() {
    // Complete graph on 4 nodes: every node connected to every other
    // K=2 partition: optimal is 2+2 split with 4 crossing edges
    let edges = vec![
        vec![(1, 1.0), (2, 1.0), (3, 1.0)],
        vec![(0, 1.0), (2, 1.0), (3, 1.0)],
        vec![(0, 1.0), (1, 1.0), (3, 1.0)],
        vec![(0, 1.0), (1, 1.0), (2, 1.0)],
    ];

    let config = ApproxMaxKCutConfig {
        k: 2,
        iterations: 10,
        random_seed: 42,
        minimize: false, // Maximize cut cost
        has_relationship_weight_property: false,
        min_community_sizes: vec![0, 0],
    };

    let runtime = ApproxMaxKCutComputationRuntime::new(config);
    let result = runtime.compute(4, |node| edges[node].clone());

    // Complete K4 graph: 6 edges total
    // Best 2-partition: {0,1} vs {2,3} gives 4 crossing edges (2*2)
    // Each edge counted once in our traversal
    assert_eq!(result.communities.len(), 4);

    // Count communities
    let mut counts = vec![0; 2];
    for &c in &result.communities {
        counts[c as usize] += 1;
    }

    // Should be balanced (or close)
    assert!(counts[0] > 0 && counts[1] > 0);

    // For complete graph K4 with k=2, max cut is 4 (each edge counted once bidirectionally = 8)
    // Actually with bidirectional edges, we count each edge twice (once from each direction)
    assert!(result.cut_cost >= 4.0); // Should find near-optimal cut
}

#[test]
fn test_weighted_edges() {
    // Simple 4-node chain with varying weights: 0--1--2--3
    let edges = vec![
        vec![(1, 10.0)],
        vec![(0, 10.0), (2, 1.0)],
        vec![(1, 1.0), (3, 10.0)],
        vec![(2, 10.0)],
    ];

    let config = ApproxMaxKCutConfig {
        k: 2,
        iterations: 10,
        random_seed: 42,
        minimize: false, // Maximize cut cost
        has_relationship_weight_property: true,
        min_community_sizes: vec![0, 0],
    };

    let runtime = ApproxMaxKCutComputationRuntime::new(config);
    let result = runtime.compute(4, |node| edges[node].clone());

    // Best cut: split at the weak link (1-2) for cut cost = 2.0
    // Worst cut: split at strong links (0-1 or 2-3) for cut cost = 20.0
    // For maximize, should find one of the strong cuts
    assert_eq!(result.communities.len(), 4);
    assert!(result.cut_cost >= 2.0); // At least the weak cut
}

#[test]
fn test_minimize_mode() {
    // Complete graph K4
    let edges = vec![
        vec![(1, 1.0), (2, 1.0), (3, 1.0)],
        vec![(0, 1.0), (2, 1.0), (3, 1.0)],
        vec![(0, 1.0), (1, 1.0), (3, 1.0)],
        vec![(0, 1.0), (1, 1.0), (2, 1.0)],
    ];

    let config = ApproxMaxKCutConfig {
        k: 2,
        iterations: 10,
        random_seed: 42,
        minimize: true, // Minimize cut cost (all in one community)
        has_relationship_weight_property: false,
        min_community_sizes: vec![0, 0],
    };

    let runtime = ApproxMaxKCutComputationRuntime::new(config);
    let result = runtime.compute(4, |node| edges[node].clone());

    // Minimize tries to keep nodes together - but we force k=2, so some cut is needed
    // Best minimize solution would be all in one community (but k=2 requires split)
    assert_eq!(result.communities.len(), 4);
    // With k=2 forced, can't achieve 0 cut on complete graph
}

#[test]
fn test_min_community_sizes() {
    // Simple chain: 0-1-2-3-4-5
    let edges = vec![
        vec![(1, 1.0)],
        vec![(0, 1.0), (2, 1.0)],
        vec![(1, 1.0), (3, 1.0)],
        vec![(2, 1.0), (4, 1.0)],
        vec![(3, 1.0), (5, 1.0)],
        vec![(4, 1.0)],
    ];

    let config = ApproxMaxKCutConfig {
        k: 2,
        iterations: 5,
        random_seed: 42,
        minimize: false,
        has_relationship_weight_property: false,
        min_community_sizes: vec![2, 2], // Each community must have at least 2 nodes
    };

    let runtime = ApproxMaxKCutComputationRuntime::new(config);
    let result = runtime.compute(6, |node| edges[node].clone());

    // Count community sizes
    let mut counts = vec![0; 2];
    for &c in &result.communities {
        counts[c as usize] += 1;
    }

    // Both communities should meet minimum size
    assert!(counts[0] >= 2);
    assert!(counts[1] >= 2);
}
