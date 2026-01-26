//! **Yen's Integration Tests**
//!
//! **Translation Source**: Integration tests for Yen's algorithm
//!
//! This module contains integration tests for Yen's algorithm with the executor runtime.

use super::YensComputationRuntime;
use super::spec::{YensConfig, YensResult};
use super::YensStorageRuntime;
use crate::algo::yens::YENSAlgorithmSpec;
use crate::core::utils::progress::{TaskProgressTracker, Tasks};
use crate::projection::eval::algorithm::AlgorithmSpec;
use crate::projection::eval::algorithm::{ExecutionContext, ExecutionMode, ProcedureExecutor};
use crate::types::prelude::DefaultGraphStore;
use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};
use serde_json::json;
use std::sync::Arc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yens_algorithm_spec_contract() {
        let spec = YENSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.name(), "yens");
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_yens_config_validation() {
        let mut config = YensConfig::default();
        assert!(config.validate().is_ok());

        config.concurrency = 0;
        assert!(config.validate().is_err());

        config.concurrency = 1;
        config.k = 0;
        assert!(config.validate().is_err());

        config.k = 3;
        config.target_node = config.source_node;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_yens_storage_runtime() {
        let storage = YensStorageRuntime::new(0, 3, 5, true, 4);
        assert_eq!(storage.source_node, 0);
        assert_eq!(storage.target_node, 3);
        assert_eq!(storage.k, 5);
        assert!(storage.track_relationships);
        assert_eq!(storage.concurrency, 4);
    }

    #[test]
    fn test_yens_computation_runtime() {
        let mut runtime = YensComputationRuntime::new(0, 3, 5, true, 1);
        runtime.initialize(1, 4, 3, false);

        assert_eq!(runtime.source_node, 1);
        assert_eq!(runtime.target_node, 4);
        assert_eq!(runtime.k, 3);
        assert!(!runtime.track_relationships);
    }

    #[test]
    fn test_yens_focused_macro_integration() {
        let spec = YENSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.name(), "yens");
        assert_eq!(spec.graph_name(), "test_graph");

        // Test that the algorithm can be created
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_yens_storage_computation_integration() {
        let config = RandomGraphConfig {
            seed: Some(7),
            node_count: 10,
            relationships: vec![RandomRelationshipConfig::new("REL", 0.7)],
            ..RandomGraphConfig::default()
        };

        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let graph = store.graph();

        let storage = YensStorageRuntime::new(0, 3, 3, false, 1);
        let mut computation = YensComputationRuntime::new(0, 3, 3, false, 1);

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("yens".to_string(), 3),
            1,
        );

        let result = storage
            .compute_yens(
                &mut computation,
                Some(graph.as_ref()),
                0,
                &mut progress_tracker,
            )
            .unwrap();

        assert!(result.path_count <= 3);
    }

    #[test]
    fn test_yens_result_serialization() {
        let result = YensResult {
            paths: vec![],
            path_count: 0,
            computation_time_ms: 5,
        };

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: YensResult = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.path_count, 0);
        assert_eq!(deserialized.computation_time_ms, 5);
    }

    #[test]
    fn test_yens_with_executor() {
        let context = ExecutionContext::new("test_user");
        let mut executor = ProcedureExecutor::new(context, ExecutionMode::Stream);

        let mut spec = YENSAlgorithmSpec::new("test_graph".to_string());

        let config = json!({
            "source_node": 0,
            "target_node": 3,
            "k": 5,
            "track_relationships": true,
            "concurrency": 4
        });

        let result = executor.compute(&mut spec, &config);

        // Should get GraphNotFound error since test_graph doesn't exist
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Graph not found"));
    }
}
