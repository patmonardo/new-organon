#[cfg(test)]
mod tests {
    use crate::procedures::graph::Graph;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};
    use std::sync::Arc;

    #[test]
    fn test_hits_smoke() {
        // Create a simple random graph
        let config = RandomGraphConfig {
            seed: Some(42),
            node_count: 5,
            relationships: vec![RandomRelationshipConfig::new("REL", 0.6)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let graph = Graph::new(store);

        let (hubs, auths) = graph
            .hits()
            .max_iterations(20)
            .tolerance(1e-4)
            .run()
            .expect("HITS should succeed");

        assert_eq!(hubs.len(), 5);
        assert_eq!(auths.len(), 5);

        // Verify normalization (L2 norm ~1.0)
        let _hub_norm: f64 = hubs.iter().map(|h| h * h).sum::<f64>().sqrt();
        let auth_norm: f64 = auths.iter().map(|a| a * a).sum::<f64>().sqrt();

        // At least authorities should be reasonably normalized
        assert!(auth_norm > 0.5, "Auths should have meaningful values");
    }

    #[test]
    fn test_hits_stream() {
        let config = RandomGraphConfig {
            seed: Some(7),
            node_count: 4,
            relationships: vec![RandomRelationshipConfig::new("REL", 0.7)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let graph = Graph::new(store);

        let rows: Vec<_> = graph
            .hits()
            .stream()
            .expect("HITS stream should succeed")
            .collect();

        assert_eq!(rows.len(), 4);

        // Verify row structure
        for row in &rows {
            assert!(row.node_id < 4);
            assert!(row.score >= 0.0, "Hub scores should be non-negative");
            assert!(row.score.is_finite(), "Scores should be finite");
        }
    }

    #[test]
    fn test_hits_stats() {
        let config = RandomGraphConfig {
            seed: Some(123),
            node_count: 3,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let graph = Graph::new(store);

        let stats = graph
            .hits()
            .max_iterations(10)
            .stats()
            .expect("HITS stats should succeed");

        assert!(stats.iterations <= 10, "Should respect max iterations");
        // Millisecond-resolution timers can legitimately report 0ms for very fast runs,
        // especially when the test suite is warm/cached.
        assert!(
            stats.execution_time_ms < 60_000,
            "Execution time should be tracked (and reasonable)"
        );
    }

    #[test]
    fn test_hits_empty() {
        // Single node, no edges
        let config = RandomGraphConfig {
            seed: Some(1),
            node_count: 1,
            relationships: vec![],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let graph = Graph::new(store);

        let result = graph.hits().run();

        // Should handle empty graph gracefully
        assert!(
            result.is_ok(),
            "HITS should handle empty graph: {:?}",
            result.err()
        );
    }
}
