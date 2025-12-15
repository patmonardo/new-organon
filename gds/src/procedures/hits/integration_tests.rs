//! HITS Integration Tests

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use crate::procedures::facades::Graph;
    use crate::projection::RelationshipType;
    use crate::types::graph::{RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::schema::{Direction, MutableGraphSchema};

    fn store_from_outgoing(outgoing: Vec<Vec<i64>>) -> DefaultGraphStore {
        let node_count = outgoing.len();

        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        for (source, targets) in outgoing.iter().enumerate() {
            for &target in targets {
                if target >= 0 {
                    let t = target as usize;
                    if t < node_count {
                        incoming[t].push(source as i64);
                    }
                }
            }
        }

        let rel_type = RelationshipType::of("REL");

        let mut schema_builder = MutableGraphSchema::empty();
        schema_builder
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Directed);
        let schema = schema_builder.build();

        let mut relationship_topologies = HashMap::new();
        relationship_topologies.insert(
            rel_type,
            RelationshipTopology::new(outgoing, Some(incoming)),
        );

        let original_ids: Vec<i64> = (0..node_count as i64).collect();
        let id_map = SimpleIdMap::from_original_ids(original_ids);

        DefaultGraphStore::new(
            crate::config::GraphStoreConfig::default(),
            GraphName::new("g"),
            DatabaseInfo::new(
                DatabaseId::new("db"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            schema,
            Capabilities::default(),
            id_map,
            relationship_topologies,
        )
    }

    #[test]
    fn hits_simple_chain() {
        // Chain: 0 -> 1 -> 2
        // Node 1 has high authority (pointed to by 0), high hub (points to 2)
        let store = store_from_outgoing(vec![vec![1], vec![2], vec![]]);
        let graph = Graph::new(Arc::new(store));

        let result = graph.hits().max_iterations(10).run().unwrap();
        
        assert_eq!(result.hub_scores.len(), 3);
        assert_eq!(result.authority_scores.len(), 3);
        
        // Node 0: high hub (points to 1), low authority (no incoming)
        // Node 1: medium hub and authority (receives from 0, points to 2)
        // Node 2: low hub (no outgoing), high authority (pointed to by 1)
        assert!(result.hub_scores[0] > result.hub_scores[2]);
        assert!(result.authority_scores[2] > result.authority_scores[0]);
    }

    #[test]
    fn hits_triangle() {
        // Triangle: 0 -> 1 -> 2 -> 0 (cycle)
        let store = store_from_outgoing(vec![vec![1], vec![2], vec![0]]);
        let graph = Graph::new(Arc::new(store));

        let result = graph.hits().max_iterations(20).run().unwrap();
        
        assert_eq!(result.hub_scores.len(), 3);
        
        // All nodes should have similar scores (symmetric structure)
        let hub_avg = result.hub_scores.iter().sum::<f64>() / 3.0;
        let auth_avg = result.authority_scores.iter().sum::<f64>() / 3.0;
        
        for &hub in &result.hub_scores {
            assert!((hub - hub_avg).abs() < 0.01);
        }
        for &auth in &result.authority_scores {
            assert!((auth - auth_avg).abs() < 0.01);
        }
    }

    #[test]
    fn hits_hub_authority_separation() {
        // Star pattern: 0 -> 1, 0 -> 2, 0 -> 3
        // Node 0 is a pure hub, nodes 1,2,3 are pure authorities
        let store = store_from_outgoing(vec![vec![1, 2, 3], vec![], vec![], vec![]]);
        let graph = Graph::new(Arc::new(store));

        let result = graph.hits().max_iterations(10).run().unwrap();
        
        // Node 0 should have highest hub score
        assert!(result.hub_scores[0] > result.hub_scores[1]);
        assert!(result.hub_scores[0] > result.hub_scores[2]);
        assert!(result.hub_scores[0] > result.hub_scores[3]);
        
        // Nodes 1,2,3 should have higher authority than node 0
        assert!(result.authority_scores[1] > result.authority_scores[0]);
        assert!(result.authority_scores[2] > result.authority_scores[0]);
        assert!(result.authority_scores[3] > result.authority_scores[0]);
    }

    #[test]
    fn hits_empty_graph() {
        let store = store_from_outgoing(vec![]);
        let graph = Graph::new(Arc::new(store));

        let result = graph.hits().run().unwrap();
        
        assert!(result.hub_scores.is_empty());
        assert!(result.authority_scores.is_empty());
        assert!(result.converged);
    }

    #[test]
    fn hits_isolated_nodes() {
        // Three isolated nodes (no edges)
        let store = store_from_outgoing(vec![vec![], vec![], vec![]]);
        let graph = Graph::new(Arc::new(store));

        let result = graph.hits().run().unwrap();
        
        assert_eq!(result.hub_scores.len(), 3);
        
        // All scores should be zero or normalized equally
        let hub_sum: f64 = result.hub_scores.iter().sum();
        let auth_sum: f64 = result.authority_scores.iter().sum();
        
        // With no edges, scores should all be zero after normalization
        assert!(hub_sum < 1e-10 || (hub_sum - 1.0).abs() < 1e-10);
        assert!(auth_sum < 1e-10 || (auth_sum - 1.0).abs() < 1e-10);
    }

    #[test]
    fn hits_stream_returns_rows() {
        // Simple chain
        let store = store_from_outgoing(vec![vec![1], vec![2], vec![]]);
        let graph = Graph::new(Arc::new(store));

        let rows = graph.hits().max_iterations(10).stream().unwrap();
        
        assert_eq!(rows.len(), 3);
        for row in rows {
            assert!(row.node_id >= 0 && row.node_id < 3);
            assert!(row.hub_score >= 0.0);
            assert!(row.authority_score >= 0.0);
        }
    }

    #[test]
    fn hits_stats_reports_convergence() {
        let store = store_from_outgoing(vec![vec![1], vec![2], vec![]]);
        let graph = Graph::new(Arc::new(store));

        let stats = graph.hits().max_iterations(50).tolerance(1e-6).stats().unwrap();
        
        assert!(stats.iterations > 0);
        assert!(stats.execution_time_ms >= 0);
        // May or may not converge depending on tolerance
    }
}
