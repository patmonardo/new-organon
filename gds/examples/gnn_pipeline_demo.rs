//! GNN Pipeline Demo using HashGNN embeddings
//!
//! This example demonstrates running a complete GNN pipeline using the Rust GDS library.
//! It creates a random graph, generates HashGNN embeddings, and shows the results.
//!
#![allow(clippy::all)]
//! Run with: cargo run -p gds --example gnn_pipeline_demo --features ml

#[cfg(not(feature = "ml"))]
fn main() {
    eprintln!(
        "This example requires the `ml` feature.\n\
Run: cargo run -p gds --example gnn_pipeline_demo --features ml"
    );
}

#[cfg(feature = "ml")]
mod enabled {
    use gds::algo::embeddings::hashgnn::GenerateFeaturesConfig;
    use gds::procedures::embeddings::HashGNNEmbeddings;
    use gds::types::graph_store::{DefaultGraphStore, GraphStore};
    use gds::types::random::{RandomGraphConfig, RandomRelationshipConfig};
    use gds::GraphFacade;
    use std::sync::Arc;

    pub fn main() {
        println!("=== GNN Pipeline Demo: HashGNN Embeddings ===\n");

        // Step 1: Create a random graph store
        println!("Step 1: Creating random graph store...");
        let config = RandomGraphConfig {
            seed: Some(42),
            node_count: 100,
            relationships: vec![RandomRelationshipConfig::new("FOLLOWS", 0.1)],
            directed: true,
            ..RandomGraphConfig::default()
        };

        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let graph = GraphFacade::new(store.clone());

        println!("Created graph with {} nodes", store.node_count());
        println!(
            "Created graph with {} relationships",
            store.relationship_count()
        );
        println!();

        // Step 2: Configure HashGNN
        println!("Step 2: Configuring HashGNN embeddings...");
        let hashgnn = graph
            .hash_gnn()
            .iterations(3) // 3 message-passing steps
            .embedding_density(8) // 8 non-zero entries per hash bucket
            .neighbor_influence(1.0) // neighbor contribution weight
            .generate_features(Some(GenerateFeaturesConfig {
                dimension: 128,   // base feature dimension
                density_level: 2, // feature sparsity
            }))
            .output_dimension(Some(64)) // densify to 64-dim f32 vectors
            .random_seed(Some(123));

        println!("HashGNN config:");
        println!("  - Iterations: 3");
        println!("  - Embedding density: 8");
        println!("  - Neighbor influence: 1.0");
        println!("  - Feature dimension: 128");
        println!("  - Output dimension: 64");
        println!();

        // Step 3: Run HashGNN
        println!("Step 3: Running HashGNN embeddings...");
        let (result, summary) = hashgnn
            .run_with_print(Some("gnn-demo".to_string()))
            .unwrap();

        println!("Embeddings computed successfully!");
        println!("Summary: {}", serde_json::to_string(&summary).unwrap());
        println!();

        // Step 4: Analyze results
        println!("Step 4: Analyzing embeddings...");

        match &result.embeddings {
            HashGNNEmbeddings::Dense { embeddings } => {
                println!("Dense embeddings generated");
                println!("Total nodes: {}", embeddings.len());
                // Show sample embeddings (first 5 nodes)
                println!("Sample embeddings (first 5 nodes):");
                for node_id in 0..5.min(embeddings.len()) {
                    let embedding = &embeddings[node_id];
                    println!(
                        "  Node {}: [{:.3}, {:.3}, {:.3}, ...] (dim={})",
                        node_id,
                        embedding[0],
                        embedding[1],
                        embedding[2],
                        embedding.len()
                    );
                }
            }
            HashGNNEmbeddings::BinaryIndices {
                embeddings,
                embedding_dimension,
            } => {
                println!("Binary embeddings generated");
                println!("Embedding dimension: {}", embedding_dimension);
                println!("Total nodes: {}", embeddings.len());
                // Show sample embeddings (first 5 nodes)
                println!("Sample embeddings (first 5 nodes):");
                for node_id in 0..5.min(embeddings.len()) {
                    let indices = &embeddings[node_id];
                    println!("  Node {}: {} non-zero entries", node_id, indices.len());
                }
            }
        }
        println!();

        // Step 5: Demonstrate pipeline potential
        println!("Step 5: Pipeline potential");
        println!("This HashGNN embedding can now be used for:");
        println!("  - Node classification tasks");
        println!("  - Link prediction");
        println!("  - Graph clustering");
        println!("  - Integration with ML pipelines in projection/eval/ml/");
        println!();

        println!("=== Demo Complete ===");
        println!("For production use:");
        println!("1. Replace random graph with your data loader");
        println!("2. Tune HashGNN parameters for your task");
        println!("3. Integrate with ML pipeline executor for training");
        println!("4. Use feature_properties instead of generate_features for real node features");
    }
}

#[cfg(feature = "ml")]
fn main() {
    enabled::main();
}
