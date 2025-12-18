//! Integration tests for conductance

use super::computation::ConductanceComputationRuntime;
use super::spec::ConductanceConfig;

#[test]
fn test_perfect_communities() {
    // Two separate cliques: 0-1-2 and 3-4-5 (no edges between)
    let edges = vec![
        vec![(1, 1.0), (2, 1.0)],
        vec![(0, 1.0), (2, 1.0)],
        vec![(0, 1.0), (1, 1.0)],
        vec![(4, 1.0), (5, 1.0)],
        vec![(3, 1.0), (5, 1.0)],
        vec![(3, 1.0), (4, 1.0)],
    ];

    let communities = vec![0, 0, 0, 1, 1, 1];

    let config = ConductanceConfig::default();
    let runtime = ConductanceComputationRuntime::new(config);

    let result = runtime.compute(
        6,
        |node| Some(communities[node]),
        |node| edges[node].clone(),
    );

    // Perfect communities: no external edges
    assert_eq!(result.community_conductances.get(&0), Some(&0.0));
    assert_eq!(result.community_conductances.get(&1), Some(&0.0));
    assert_eq!(result.average_conductance, 0.0);
}

#[test]
fn test_poor_communities() {
    // Linear chain: 0-1-2-3 with bad split [0,1] vs [2,3]
    let edges = vec![
        vec![(1, 1.0)],
        vec![(0, 1.0), (2, 1.0)],
        vec![(1, 1.0), (3, 1.0)],
        vec![(2, 1.0)],
    ];

    let communities = vec![0, 0, 1, 1];

    let config = ConductanceConfig::default();
    let runtime = ConductanceComputationRuntime::new(config);

    let result = runtime.compute(
        4,
        |node| Some(communities[node]),
        |node| edges[node].clone(),
    );

    // Community 0: internal edges = 2 (0-1 bidirectional), external = 1 (1->2)
    // Conductance = 1 / (1 + 2) = 0.333...
    let cond0 = result.community_conductances.get(&0).unwrap();
    assert!((cond0 - 0.333333).abs() < 0.001);

    // Community 1: internal edges = 2 (2-3 bidirectional), external = 1 (2->1)
    // Conductance = 1 / (1 + 2) = 0.333...
    let cond1 = result.community_conductances.get(&1).unwrap();
    assert!((cond1 - 0.333333).abs() < 0.001);

    assert!((result.average_conductance - 0.333333).abs() < 0.001);
}

#[test]
fn test_weighted_conductance() {
    // Two nodes with weighted edges
    // 0 -> 1 (weight 10.0)
    // 1 -> 0 (weight 10.0)
    let edges = vec![vec![(1, 10.0)], vec![(0, 10.0)]];

    let communities = vec![0, 1];

    let config = ConductanceConfig {
        has_relationship_weight_property: true,
    };
    let runtime = ConductanceComputationRuntime::new(config);

    let result = runtime.compute(
        2,
        |node| Some(communities[node]),
        |node| edges[node].clone(),
    );

    // All edges are external (different communities)
    // Community 0: external = 10.0, internal = 0.0, conductance = 1.0
    assert_eq!(result.community_conductances.get(&0), Some(&1.0));
    // Community 1: external = 10.0, internal = 0.0, conductance = 1.0
    assert_eq!(result.community_conductances.get(&1), Some(&1.0));
    assert_eq!(result.average_conductance, 1.0);
}

#[test]
fn test_unweighted_ignores_weights() {
    // Same graph as above but unweighted
    let edges = vec![vec![(1, 10.0)], vec![(0, 10.0)]];

    let communities = vec![0, 1];

    let config = ConductanceConfig {
        has_relationship_weight_property: false,
    };
    let runtime = ConductanceComputationRuntime::new(config);

    let result = runtime.compute(
        2,
        |node| Some(communities[node]),
        |node| edges[node].clone(),
    );

    // Weights treated as 1.0
    assert_eq!(result.community_conductances.get(&0), Some(&1.0));
    assert_eq!(result.community_conductances.get(&1), Some(&1.0));
    assert_eq!(result.average_conductance, 1.0);
}

#[test]
fn test_nodes_without_community() {
    // Some nodes have no community assignment
    let edges = vec![vec![(1, 1.0)], vec![(0, 1.0), (2, 1.0)], vec![(1, 1.0)]];

    // Node 1 has no community
    let communities = vec![Some(0), None, Some(1)];

    let config = ConductanceConfig::default();
    let runtime = ConductanceComputationRuntime::new(config);

    let result = runtime.compute(3, |node| communities[node], |node| edges[node].clone());

    // Node 1 is skipped, so communities 0 and 1 have no valid edges
    // (edges to/from node 1 are ignored)
    assert_eq!(result.community_conductances.len(), 0);
    assert_eq!(result.average_conductance, 0.0);
}
