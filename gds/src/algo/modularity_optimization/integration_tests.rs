use super::{ModularityOptimizationComputationRuntime, ModularityOptimizationInput};
use super::spec::ModularityOptimizationConfig;

#[test]
fn modopt_empty_ok() {
    let mut rt = ModularityOptimizationComputationRuntime::new();
    let cfg = ModularityOptimizationConfig::default();
    let input = ModularityOptimizationInput::new(0, Vec::new());
    let result = rt.compute(&input, &cfg);
    assert!(result.communities.is_empty());
    assert_eq!(result.modularity, 0.0);
}

#[test]
fn modopt_separates_isolated_node() {
    // 0--1 connected, 2 isolated.
    let mut rt = ModularityOptimizationComputationRuntime::new();
    let cfg = ModularityOptimizationConfig::default();
    let input = ModularityOptimizationInput::new(3, vec![vec![(1, 1.0)], vec![(0, 1.0)], vec![]]);
    let result = rt.compute(&input, &cfg);

    assert_eq!(result.communities.len(), 3);
    assert_eq!(result.communities[0], result.communities[1]);
    assert_ne!(result.communities[2], result.communities[0]);
}
