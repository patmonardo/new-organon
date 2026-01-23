#[cfg(test)]
mod tests {
    use crate::config::GraphStoreConfig;
    use crate::types::graph::id_map::SimpleIdMap;
    use crate::types::graph::RelationshipTopology;
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, GraphName,
    };
    use crate::types::graph_store::{DefaultGraphStore, GraphStore};
    use crate::types::random::{RandomGraphConfig, Randomizable};
    use crate::types::schema::{
        Direction, GraphSchema, MutableGraphSchema, NodeLabel, RelationshipType,
    };
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use crate::types::graph::degrees::Degrees;
    use crate::types::properties::relationship::{
        RelationshipIterator, RelationshipPredicate,
    };

    #[test]
    fn to_undirected_makes_edges_symmetric() {
        let mut rng = StdRng::seed_from_u64(123);
        let cfg = RandomGraphConfig {
            graph_name: "g".to_string(),
            ..Default::default()
        };
        let store = DefaultGraphStore::random_with_rng(&cfg, &mut rng).unwrap();
        let undirected = store
            .to_undirected("g_undir".into())
            .expect("to_undirected");

        let g = undirected.graph();
        let n = GraphStore::node_count(&undirected);

        // For every observed (u->v), expect (v->u).
        for u in 0..n {
            let u = u as i64;
            for cursor in g.stream_relationships(u, 0.0) {
                assert!(g.exists(cursor.target_id(), cursor.source_id()));
            }
        }

        assert!(undirected.schema().is_undirected());
    }

    #[test]
    fn with_inverse_indices_enables_degree_inverse() {
        let mut rng = StdRng::seed_from_u64(7);
        let cfg = RandomGraphConfig {
            graph_name: "g".to_string(),
            ..Default::default()
        };
        let store = DefaultGraphStore::random_with_rng(&cfg, &mut rng).unwrap();

        let g0 = store.graph();
        // Before: might be None if there are no inverse indices.
        let _ = g0.degree_inverse(0);

        let indexed = store
            .with_inverse_indices("g_indexed".into())
            .expect("with_inverse_indices");
        let g1 = indexed.graph();

        // After: should be Some(...) for at least one node when graph has relationships.
        if GraphStore::relationship_count(&indexed) > 0 {
            assert!(g1.degree_inverse(0).is_some());
        }
    }

    #[test]
    fn collapse_paths_degree2_collapses_simple_chain() {
        // Build a tiny store with a single relationship type and a chain 0->1->2->3.
        let cfg = GraphStoreConfig::default();
        let graph_name = GraphName::new("chain");
        let db_info = DatabaseInfo::new(
            DatabaseId::new("test-db"),
            DatabaseLocation::remote("localhost", 7687, None, None),
        );
        let capabilities = Capabilities::default();

        let mut schema = MutableGraphSchema::empty();
        schema.node_schema_mut().add_label(NodeLabel::all_nodes());
        let rel_type = RelationshipType::of("R");
        schema
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Directed);
        let schema: GraphSchema = schema.build();

        let id_map = SimpleIdMap::from_original_ids([0, 1, 2, 3]);

        let outgoing = vec![vec![1], vec![2], vec![3], vec![]];
        let topo = RelationshipTopology::new(outgoing, None);
        let mut topologies = std::collections::HashMap::new();
        topologies.insert(rel_type.clone(), topo);

        let store = DefaultGraphStore::new(
            cfg,
            graph_name,
            db_info,
            schema,
            capabilities,
            id_map,
            topologies,
        );

        let collapsed = store
            .collapse_paths_degree2("collapsed".into(), None)
            .expect("collapse");

        let g = collapsed.graph();
        // Expect 0->3 exists (collapsed edge).
        assert!(g.exists(0, 3));
        // And original chain edges removed.
        assert!(!g.exists(0, 1));
        assert!(!g.exists(1, 2));
        assert!(!g.exists(2, 3));
    }
}
