//! **BFS Integration Tests**
//!
//! **Translation Source**: `org.neo4j.gds.traversal.BFS`
//!
//! This module contains integration tests for BFS algorithm with the executor runtime.

use super::spec::{BFSAlgorithmSpec, BfsConfig, BfsResult};
use super::BfsComputationRuntime;
use super::BfsStorageRuntime;
use crate::core::utils::progress::{TaskProgressTracker, Tasks};
use crate::projection::eval::algorithm::AlgorithmSpec;
use crate::projection::eval::algorithm::{ExecutionContext, ExecutionMode, ProcedureExecutor};
use serde_json::json;

#[test]
fn test_bfs_algorithm_spec_contract() {
    let spec = BFSAlgorithmSpec::new("test_graph".to_string());
    assert_eq!(spec.name(), "bfs");
    assert_eq!(spec.graph_name(), "test_graph");

    // Test that the algorithm can be created
    assert_eq!(spec.graph_name(), "test_graph");
}

#[test]
fn test_bfs_config_validation() {
    let config = BfsConfig {
        source_node: 0,
        target_nodes: vec![1, 2],
        max_depth: Some(5),
        track_paths: true,
        concurrency: 4,
        delta: 1,
    };

    assert!(config.validate().is_ok());

    let invalid_config = BfsConfig {
        concurrency: 0,
        ..config
    };
    assert!(invalid_config.validate().is_err());
}

#[test]
fn test_bfs_storage_runtime() {
    let storage = BfsStorageRuntime::new(0, vec![3], Some(5), true);
    assert_eq!(storage.source_node, 0);
    assert_eq!(storage.target_nodes, vec![3]);
    assert_eq!(storage.max_depth, Some(5));
    assert!(storage.track_paths);
}

#[test]
fn test_bfs_computation_runtime() {
    let mut computation = BfsComputationRuntime::new(0, true, 4, 10);
    computation.initialize(0, Some(5), 10);

    assert_eq!(computation.source_node, 0);
    assert!(computation.check_max_depth(4.0));
    assert!(!computation.check_max_depth(5.0));
    assert_eq!(computation.visited_count(), 0);
}

#[test]
fn test_bfs_focused_macro_integration() {
    let spec = BFSAlgorithmSpec::new("test_graph".to_string());
    assert_eq!(spec.name(), "bfs");
    assert_eq!(spec.graph_name(), "test_graph");

    // Test that the algorithm can be created
    assert_eq!(spec.graph_name(), "test_graph");
}

#[test]
fn test_bfs_storage_computation_integration() {
    let storage = BfsStorageRuntime::new(0, vec![3], None, true);
    let mut computation = BfsComputationRuntime::new(0, true, 1, 10);

    let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("BFS".to_string()));
    let bfs_result = storage
        .compute_bfs(&mut computation, None, &mut progress_tracker)
        .unwrap();

    assert!(!bfs_result.visited_nodes.is_empty());
}

#[test]
fn test_bfs_result_serialization() {
    let result = BfsResult {
        visited_nodes: vec![0, 1, 2, 3],
        computation_time_ms: 5,
    };

    // Test serialization
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("visited_nodes"));
    assert!(json.contains("computation_time_ms"));

    // Test deserialization
    let deserialized: BfsResult = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.visited_nodes.len(), 4);
    assert_eq!(deserialized.computation_time_ms, 5);
}

#[test]
fn test_bfs_with_executor() {
    let context = ExecutionContext::new("test_user");
    let mut executor = ProcedureExecutor::new(context, ExecutionMode::Stream);
    let mut algorithm = BFSAlgorithmSpec::new("test_graph".to_string());

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
