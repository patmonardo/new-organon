# GNN pipeline runbook (HashGNN)

How to take the Rust GNN pieces in `gds/src/ml` from code to a runnable pipeline. Focus is on the HashGNN embedding path surfaced through the facade layer.

## Pre-reqs
- Rust toolchain installed; compile with `cargo build -p gds`.
- Workspace deps already installed (see repository `AGENTS.md` for pnpm details if you are crossing into TS land).

## Quick start: run HashGNN embeddings
1) Build once: `cargo build -p gds`
2) Prepare a graph store (replace the random graph with your own loader if needed):
```rust
use std::sync::Arc;
use gds::algo::embeddings::hashgnn::GenerateFeaturesConfig;
use gds::procedures::Graph;
use gds::types::random::{RandomGraphConfig, RandomRelationshipConfig};
use gds::types::graph_store::DefaultGraphStore;

let config = RandomGraphConfig {
    seed: Some(7),
    node_count: 20,
    relationships: vec![RandomRelationshipConfig::new("REL", 0.3)],
    directed: true,
    ..RandomGraphConfig::default()
};
let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
let graph = Graph::new(store);
```
3) Configure and run HashGNN (binary output by default):
```rust
let (result, summary) = graph
    .hash_gnn()
    .iterations(2)               // message-passing steps
    .embedding_density(4)        // non-zero entries per hash bucket
    .neighbor_influence(1.0)     // how much neighbors contribute
    .generate_features(Some(GenerateFeaturesConfig {
        dimension: 64,           // base feature dim when you do not pass properties
        density_level: 3,        // sparsity of generated features
    }))
    // alternatively: .feature_properties(vec!["pagerank".into(), "community".into()])
    .output_dimension(None)      // set to Some(d) to densify to f32 vectors
    .random_seed(Some(42))
    .run_with_print(Some("demo-hashgnn".to_string()))
    .unwrap();

println!("embeddings: {:?}", result.embeddings);      // binary indices or dense vectors
println!("run summary: {}", serde_json::to_string(&summary).unwrap());
```
Key config knobs live in `gds/src/algo/embeddings/hashgnn/algo/spec.rs`:
- `iterations` (>0), `embedding_density` (>0), `neighbor_influence` (0..1e6)
- Feature source: `feature_properties` **or** `generate_features` (one is required)
- Optional: `output_dimension` to densify, `binarize_features`, `heterogeneous`, `concurrency`, `random_seed`

## Validating the pipeline
- HashGNN facade sanity: `cargo test -p gds hashgnn` (runs the facade print/embedding tests).
- Pipeline orchestration skeleton: `cargo test -p gds pipeline_executor` (covers the multi-step ML executor in `projection/eval/ml/pipeline_executor.rs`).

## Wiring into the ML pipeline executor
The ML executor in `gds/src/projection/eval/ml/pipeline_executor.rs` runs a sequence of node-property → feature → split → train steps via a `GraphProcedureRegistry`. To run HashGNN inside that flow:
- Implement a `GraphProcedure` that calls the facade shown above and adapts `HashGNNEmbeddings` into a `PropertyValues` implementation (e.g., one value per node, dense or binary).
- Register it in the registry with `registry.register(Arc::new(MyHashGnnProcedure::new(...)))`.
- Add a `StepDescriptor::NodeProperty` that names the procedure, then execute with `PipelineExecutor::run(...)`.

This keeps GNN computation in Rust (HashGNN runtime in `gds/src/procedures/embeddings/hashgnn`) while letting the pipeline orchestrator manage feature assembly and dataset splitting.
