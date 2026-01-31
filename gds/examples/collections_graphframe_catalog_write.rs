//! GraphFrame → DatasetCatalog write example (Polars-backed tables).
//!
//! Run with:
//!   cargo run -p gds --example collections_graphframe_catalog_write

use std::path::PathBuf;

use gds::collections::catalog::CollectionsIoFormat;
use gds::collections::datasets::catalog::DatasetCatalog;
use gds::collections::plugins::write_graph_store_polars32_to_catalog;
use gds::types::prelude::{DefaultGraphStore, RandomGraphConfig};
use gds::types::random::RandomRelationshipConfig;

fn main() {
    if let Err(err) = run() {
        eprintln!("collections_graphframe_catalog_write failed: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = RandomGraphConfig {
        graph_name: "graphframe-demo".into(),
        database_name: "in-memory".into(),
        node_count: 8,
        node_labels: vec!["Person".into(), "Movie".into()],
        relationships: vec![RandomRelationshipConfig::new("KNOWS", 0.4)],
        directed: false,
        inverse_indexed: false,
        seed: Some(21),
    };

    let store = DefaultGraphStore::random(&config)?;

    let root = PathBuf::from("target/collections_graphframe_catalog");
    let graph_name = "graphframe-demo";
    let nodes_name = format!("{graph_name}.nodes");
    let edges_name = format!("{graph_name}.edges");
    let graph_table_name = format!("{graph_name}.graph");

    // Allow re-running the example by removing prior entries.
    let mut catalog = DatasetCatalog::load(&root)?;
    let _ = catalog.catalog_mut().remove(&nodes_name);
    let _ = catalog.catalog_mut().remove(&edges_name);
    let _ = catalog.catalog_mut().remove(&graph_table_name);
    catalog.save()?;

    let written = write_graph_store_polars32_to_catalog(
        &store,
        &root,
        graph_name,
        CollectionsIoFormat::Parquet,
    )?;

    println!(
        "Wrote GraphFrame tables: nodes={}, edges={}, graph={}",
        written.nodes.data_path, written.edges.data_path, written.graph.data_path
    );

    let catalog = DatasetCatalog::load(&root)?;
    let nodes = catalog.load_table(&nodes_name)?;
    let edges = catalog.load_table(&edges_name)?;
    let graph = catalog.load_table(&graph_table_name)?;

    println!("Nodes (head):\n{}", nodes.head(5).table().fmt_table());
    println!("Edges (head):\n{}", edges.head(5).table().fmt_table());
    println!("Graph (head):\n{}", graph.head(1).table().fmt_table());

    Ok(())
}
