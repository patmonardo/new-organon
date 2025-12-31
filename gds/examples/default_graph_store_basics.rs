//! DefaultGraphStore basics (small, Vec-first configuration).
//!
//! Shows how to:
//! - Spin up a tiny deterministic `DefaultGraphStore`
//! - Inspect labels/types and property keys
//! - Add node + graph properties with the built-in Vec-backed helpers
//! - Read values back through the `GraphStore` trait
//!
//! Run with:
//!   cargo run -p gds --example default_graph_store_basics

mod enabled {
    use gds::types::prelude::{DefaultGraphStore, GraphStore, RandomGraphConfig};
    use gds::types::random::RandomRelationshipConfig;

    pub fn main() {
        if let Err(err) = run() {
            eprintln!("default_graph_store_basics failed: {err}");
            std::process::exit(1);
        }
    }

    fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("=== DefaultGraphStore basics ===\n");

        let config = RandomGraphConfig {
            graph_name: "demo-graph".into(),
            database_name: "in-memory".into(),
            node_count: 6,
            node_labels: vec!["Person".into(), "Movie".into()],
            relationships: vec![RandomRelationshipConfig::new("KNOWS", 0.35)],
            directed: false,
            inverse_indexed: false,
            seed: Some(7),
        };

        let mut store = DefaultGraphStore::random(&config)?;

        println!(
            "Graph: nodes={} relationships={} labels={:?} relationship_types={:?}",
            store.node_count(),
            store.relationship_count(),
            store.node_labels(),
            store.relationship_types()
        );

        // Add typed properties using the store's Vec-first config.
        let ages: Vec<i64> = (0..store.node_count() as i64).map(|i| 20 + i).collect();
        store.add_node_property_i64("age".into(), ages)?;
        store.add_graph_property_f64("temperature".into(), vec![21.5])?;

        println!(
            "Node properties: {:?} | Graph properties: {:?}",
            store.node_property_keys(),
            store.graph_property_keys()
        );

        let age_values = store.node_property_values("age")?;
        println!(
            "Sample ages (first 3): {:?}",
            (0..3u64)
                .filter_map(|id| age_values.long_value(id).ok())
                .collect::<Vec<_>>()
        );

        // Show a tiny induced projection onto the first 3 original node ids.
        let selection: Vec<i64> = (0..3).collect();
        let result = store.commit_induced_subgraph_by_original_node_ids(
            "projected-subgraph".into(),
            &selection,
        )?;
        let projected = result.store;
        let mapping = result.old_to_new_mapping;
        let kept = result.relationships_kept_by_type;
        println!(
            "Projected subgraph: nodes={} relationships={} kept_by_type={:?}",
            projected.node_count(),
            projected.relationship_count(),
            kept
        );
        println!(
            "Mapping old→new ids (first 3): {:?}",
            mapping.iter().collect::<Vec<_>>()
        );

        Ok(())
    }
}

fn main() {
    enabled::main();
}
