//! **DFS Integration Tests**
//!
//! **Translation Source**: `org.neo4j.gds.traversal.DFS`
//!
//! This module contains integration tests for DFS algorithm with the executor runtime.

use super::DfsComputationRuntime;
use super::spec::{DFSAlgorithmSpec, DfsConfig, DfsResult};
use super::DfsStorageRuntime;
use crate::core::utils::progress::{TaskProgressTracker, Tasks};
use crate::projection::eval::algorithm::AlgorithmSpec;
use crate::projection::eval::algorithm::{ExecutionContext, ExecutionMode, ProcedureExecutor};
use serde_json::json;

#[test]
fn test_dfs_algorithm_spec_contract() {
    let spec = DFSAlgorithmSpec::new("test_graph".to_string());
    assert_eq!(spec.name(), "dfs");
    assert_eq!(spec.graph_name(), "test_graph");

    // Test that the algorithm can be created
    assert_eq!(spec.graph_name(), "test_graph");
}

#[test]
fn test_dfs_config_validation() {
    let config = DfsConfig {
        source_node: 0,
        target_nodes: vec![1, 2],
        max_depth: Some(5),
        track_paths: true,
        concurrency: 4,
    };

    assert!(config.validate().is_ok());

    let invalid_config = DfsConfig {
        concurrency: 0,
        ..config
    };
    assert!(invalid_config.validate().is_err());
}

#[test]
fn test_dfs_storage_runtime() {
    let storage = DfsStorageRuntime::new(0, vec![3], Some(5), true, 4);
    assert_eq!(storage.source_node, 0);
    assert_eq!(storage.target_nodes, vec![3]);
    assert_eq!(storage.max_depth, Some(5));
    assert!(storage.track_paths);
    assert_eq!(storage.concurrency, 4);
}

#[test]
fn test_dfs_computation_runtime() {
    let mut computation = DfsComputationRuntime::new(0, true, 4, 10);
    computation.initialize(0, Some(5), 10);

    assert_eq!(computation.source_node, 0);
    assert!(!computation.check_max_depth(5.0));
    assert_eq!(computation.visited_count(), 1);
    assert!(computation.is_visited(0));
}

#[test]
fn test_dfs_focused_macro_integration() {
    let spec = DFSAlgorithmSpec::new("test_graph".to_string());
    assert_eq!(spec.name(), "dfs");
    assert_eq!(spec.graph_name(), "test_graph");

    // Test that the algorithm can be created
    assert_eq!(spec.graph_name(), "test_graph");
}

#[test]
fn test_dfs_storage_computation_integration() {
    let storage = DfsStorageRuntime::new(0, vec![3], None, true, 1);
    let mut computation = DfsComputationRuntime::new(0, true, 1, 10);

    let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("DFS".to_string()));
    let dfs_result = storage
        .compute_dfs(&mut computation, None, &mut progress_tracker)
        .unwrap();

    assert!(!dfs_result.visited_nodes.is_empty());
    assert!(dfs_result.visited_nodes.contains(&0));
}

#[test]
fn test_dfs_result_serialization() {
    let result = DfsResult {
        visited_nodes: vec![0, 1, 2, 3],
        computation_time_ms: 5,
    };

    // Test serialization
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("visited_nodes"));
    assert!(json.contains("computation_time_ms"));

    // Test deserialization
    let deserialized: DfsResult = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.visited_nodes.len(), 4);
    assert_eq!(deserialized.computation_time_ms, 5);
}

#[test]
fn test_dfs_with_executor() {
    let context = ExecutionContext::new("test_user");
    let mut executor = ProcedureExecutor::new(context, ExecutionMode::Stream);
    let mut algorithm = DFSAlgorithmSpec::new("test_graph".to_string());

    let config_input = json!({
        "source_node": 0,
        "target_nodes": [3],
        "max_depth": 5,
        "track_paths": true,
        "concurrency": 4
    });

    let result = executor.compute(&mut algorithm, &config_input);

    // Should fail with GraphNotFound since we don't have a real graph store
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Graph not found"));
}
