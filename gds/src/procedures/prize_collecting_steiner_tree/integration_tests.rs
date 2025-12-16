use crate::procedures::prize_collecting_steiner_tree::computation::PCSTreeComputationRuntime;
use crate::procedures::prize_collecting_steiner_tree::spec::{PCSTreeConfig, PRUNED};

fn create_neighbors(edges: Vec<Vec<(usize, f64)>>) -> impl Fn(usize) -> Vec<(usize, f64)> {
    move |node: usize| {
        if node < edges.len() {
            edges[node].clone()
        } else {
            Vec::new()
        }
    }
}

#[test]
fn test_pcst_simple_high_prize_node() {
    // Linear graph: 0 - 1 - 2 - 3
    // Prizes: [1.0, 1.0, 10.0, 1.0]
    // Edge weights: all 1.0
    // Node 2 has very high prize (10.0) vs edge cost (1.0)
    let edges = vec![
        vec![(1, 1.0)],
        vec![(0, 1.0), (2, 1.0)],
        vec![(1, 1.0), (3, 1.0)],
        vec![(2, 1.0)],
    ];

    let get_neighbors = create_neighbors(edges);
    let config = PCSTreeConfig {
        prizes: vec![1.0, 1.0, 10.0, 1.0],
        relationship_weight_property: Some("weight".to_string()),
    };

    let runtime = PCSTreeComputationRuntime::new(config);
    let result = runtime.compute(4, get_neighbors);

    // Node 2 has prize 10.0 - should definitely be included
    assert_ne!(result.parent_array[2], PRUNED,
        "Node 2 with prize 10.0 should be included");

    // At least one node should be in the tree
    assert!(result.effective_node_count >= 1,
        "Should include at least 1 node, got {}",
        result.effective_node_count);

    // Total prize should be at least 10.0 (node 2's prize)
    assert!(result.total_prize >= 9.0,
        "Total prize should include node 2's contribution");
}

#[test]
fn test_pcst_pruning_low_value_branch() {
    // Star graph:     1
    //                 |
    //             0 - 2 - 3
    //                 |
    //                 4
    // Prizes: [0.5, 0.1, 5.0, 0.1, 0.1]
    // Edge weights: all 2.0
    // Should include only 2 and maybe 0, pruning low-prize leaves
    let edges = vec![
        vec![(2, 2.0)],
        vec![(2, 2.0)],
        vec![(0, 2.0), (1, 2.0), (3, 2.0), (4, 2.0)],
        vec![(2, 2.0)],
        vec![(2, 2.0)],
    ];

    let get_neighbors = create_neighbors(edges);
    let config = PCSTreeConfig {
        prizes: vec![0.5, 0.1, 5.0, 0.1, 0.1],
        relationship_weight_property: Some("weight".to_string()),
    };

    let runtime = PCSTreeComputationRuntime::new(config);
    let result = runtime.compute(5, get_neighbors);

    // Node 2 should be in tree (highest prize)
    assert_ne!(result.parent_array[2], PRUNED);

    // Low-prize nodes (1, 3, 4) likely pruned
    let pruned_count = [1, 3, 4]
        .iter()
        .filter(|&&n| result.parent_array[n] == PRUNED)
        .count();
    assert!(pruned_count >= 2, "Should prune at least 2 low-value leaves");

    // Net value should be positive
    assert!(result.net_value > 0.0);
}

#[test]
fn test_pcst_all_high_prizes() {
    // Triangle: 0 - 1
    //           |   |
    //           2 - +
    // All prizes: 10.0
    // All edge weights: 1.0
    // Should include all nodes (prizes >> costs)
    let edges = vec![
        vec![(1, 1.0), (2, 1.0)],
        vec![(0, 1.0), (2, 1.0)],
        vec![(0, 1.0), (1, 1.0)],
    ];

    let get_neighbors = create_neighbors(edges);
    let config = PCSTreeConfig {
        prizes: vec![10.0, 10.0, 10.0],
        relationship_weight_property: Some("weight".to_string()),
    };

    let runtime = PCSTreeComputationRuntime::new(config);
    let result = runtime.compute(3, get_neighbors);

    // All nodes should be included
    assert_eq!(result.effective_node_count, 3);

    // Check all nodes are in tree
    for node in 0..3 {
        assert_ne!(result.parent_array[node], PRUNED);
    }

    // Total prize should be 30.0
    assert!((result.total_prize - 30.0).abs() < 0.01);

    // Net value should be very positive
    assert!(result.net_value > 20.0);
}

#[test]
fn test_pcst_expensive_edges() {
    // Path: 0 - 1 - 2
    // Prizes: [5.0, 2.0, 5.0]
    // Edge weights: [10.0, 10.0] (very expensive)
    // Should keep only highest prize node
    let edges = vec![
        vec![(1, 10.0)],
        vec![(0, 10.0), (2, 10.0)],
        vec![(1, 10.0)],
    ];

    let get_neighbors = create_neighbors(edges);
    let config = PCSTreeConfig {
        prizes: vec![5.0, 2.0, 5.0],
        relationship_weight_property: Some("weight".to_string()),
    };

    let runtime = PCSTreeComputationRuntime::new(config);
    let result = runtime.compute(3, get_neighbors);

    // Should include only 1 or 2 nodes (connecting is too expensive)
    assert!(result.effective_node_count <= 2);

    // Net value should still be positive
    assert!(result.net_value >= 0.0);
}

#[test]
fn test_pcst_balanced_tradeoff() {
    // Y-shaped graph:
    //     1
    //     |
    //     0 - 2
    //     |
    //     3
    // Prizes: [1.0, 3.0, 3.0, 3.0]
    // Edge weights: all 2.0
    // Should include center and 2-3 highest prize neighbors
    let edges = vec![
        vec![(1, 2.0), (2, 2.0), (3, 2.0)],
        vec![(0, 2.0)],
        vec![(0, 2.0)],
        vec![(0, 2.0)],
    ];

    let get_neighbors = create_neighbors(edges);
    let config = PCSTreeConfig {
        prizes: vec![1.0, 3.0, 3.0, 3.0],
        relationship_weight_property: Some("weight".to_string()),
    };

    let runtime = PCSTreeComputationRuntime::new(config);
    let result = runtime.compute(4, get_neighbors);

    // Should include 3-4 nodes
    assert!(result.effective_node_count >= 3);
    assert!(result.effective_node_count <= 4);

    // Net value should be positive
    assert!(result.net_value > 0.0);

    // Total prize should be significant
    assert!(result.total_prize >= 6.0);
}

#[test]
fn test_pcst_zero_prizes() {
    // All prizes zero - should include minimal tree
    let edges = vec![
        vec![(1, 1.0)],
        vec![(0, 1.0)],
    ];

    let get_neighbors = create_neighbors(edges);
    let config = PCSTreeConfig {
        prizes: vec![0.0, 0.0],
        relationship_weight_property: Some("weight".to_string()),
    };

    let runtime = PCSTreeComputationRuntime::new(config);
    let result = runtime.compute(2, get_neighbors);

    // Should include at least 1 node
    assert!(result.effective_node_count >= 1);

    // Net value should be 0 or slightly negative
    assert!(result.net_value <= 0.01);
}
