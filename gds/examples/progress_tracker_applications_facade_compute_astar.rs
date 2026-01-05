//! Applications → Facade → Compute path (A*) walkthrough.
//!
//! Purpose:
//! - Mock the “TS-JSON boundary” setup: a `GraphCatalog` + a JSON request.
//! - Call the Applications handler (`handle_astar`) which delegates to the procedure facade.
//! - Observe progress logging emitted by the progress tracker during compute.
//!
//! Run with:
//!   cargo run -p gds --example progress_tracker_applications_facade_compute_astar

mod enabled {
    use gds::applications::algorithms::pathfinding::handle_astar;
    use gds::types::catalog::{GraphCatalog, InMemoryGraphCatalog};
    use gds::types::prelude::{DefaultGraphStore, GraphStore, RandomGraphConfig};
    use gds::types::random::RandomRelationshipConfig;
    use serde_json::json;
    use std::sync::Arc;

    pub fn main() {
        if let Err(err) = run() {
            eprintln!("progress_tracker_applications_facade_compute_astar failed: {err}");
            std::process::exit(1);
        }
    }

    fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("=== Applications → Facade → Compute (A*) walkthrough ===\n");

        // 1) Mock the app's graph catalog.
        //
        // In prod this would be request-scoped or service-scoped state.
        let catalog: Arc<dyn GraphCatalog> = Arc::new(InMemoryGraphCatalog::new());

        // 2) Mock a graph store like a loader would create.
        //
        // Keep it deterministic so this example stays stable.
        let graph_name = "astar-demo";
        let config = RandomGraphConfig {
            graph_name: graph_name.into(),
            database_name: "in-memory".into(),
            node_count: 12,
            node_labels: vec!["Node".into()],
            relationships: vec![RandomRelationshipConfig::new("REL", 0.35)],
            directed: false,
            inverse_indexed: false,
            seed: Some(7),
        };

        let store = DefaultGraphStore::random(&config)?;
        println!(
            "Graph created: name='{}' nodes={} relationships={} rel_types={:?}",
            graph_name,
            store.node_count(),
            store.relationship_count(),
            store.relationship_types()
        );

        catalog.set(graph_name, Arc::new(store));

        // 3) Mock an incoming JSON request.
        //
        // This is the Applications layer’s input surface.
        let request = json!({
            "graphName": graph_name,
            "sourceNode": 0,
            "targetNode": 3,
            "weightProperty": "weight",
            "direction": "outgoing",
            "heuristic": "euclidean",
            "concurrency": 2,
            "mode": "stream"
        });

        // 4) Applications → Facade → Compute.
        //
        // `handle_astar` parses JSON, fetches the store from the catalog, and runs
        // the procedure facade (`AStarBuilder`), which performs compute and emits
        // progress logs.
        let response = handle_astar(&request, Arc::clone(&catalog));

        println!("\nResponse:\n{}", serde_json::to_string_pretty(&response)?);
        Ok(())
    }
}

fn main() {
    enabled::main();
}
