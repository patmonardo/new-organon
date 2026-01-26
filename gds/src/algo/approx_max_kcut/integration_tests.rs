use super::spec::ApproxMaxKCutConfig;
use super::ApproxMaxKCutComputationRuntime;

#[test]
fn assigns_all_nodes() {
    let config = ApproxMaxKCutConfig {
        k: 3,
        iterations: 5,
        random_seed: 42,
        minimize: false,
        has_relationship_weight_property: false,
        min_community_sizes: vec![0, 0, 0],
        concurrency: 4,
    };
    let runtime = ApproxMaxKCutComputationRuntime::new(config);

    let adjacency = vec![
        vec![(1, 1.0), (2, 1.0)],
        vec![(0, 1.0), (2, 1.0)],
        vec![(0, 1.0), (1, 1.0)],
    ];

    let result = runtime.compute(3, |node| adjacency[node].clone());
    assert_eq!(result.communities.len(), 3);
    for &c in &result.communities {
        assert!(c < 3);
    }
}
