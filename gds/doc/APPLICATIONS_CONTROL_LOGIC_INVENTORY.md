# Applications Control Logic Inventory (Rust gds)

Date: 2026-01-03

This note inventories the current **Applications** layer in the Rust `gds` crate: the orchestration/control logic that sits above `procedures/*` and is intended to become the primary “GDS Link” surface (JSON/TS-friendly).

## What “Applications control logic” is (in this repo)

- **Procedures** (`gds/src/procedures/*`) expose algorithm builders and modes (`stream/stats/mutate/write`).
- **Applications** (`gds/src/applications/*`) are the higher-level *application facade* that:
  - parses/validates request config (often JSON-driven),
  - resolves the target graph from the catalog,
  - runs the requested procedure mode consistently,
  - enforces memory estimation/guards and logging,
  - renders a stable JSON result payload.

## Current entrypoints

- Primary integration surface re-exported by Applications: `ApplicationsFacade`
- Algorithm request dispatch: `gds/src/applications/services/algorithms_dispatch.rs`
- Graph catalog facade trait (intended TS consumer surface): `GraphCatalogApplications`

### Algorithm dispatch handlers present

- Count: 49 `handle_*` functions in `gds/src/applications/services/algorithms_dispatch.rs`
- These are thin forwarders into `gds/src/applications/algorithms/<domain>/...` handlers.

### Graph store catalog facade surface

- `GraphCatalogApplications` trait methods: 21
- Methods include: graph_exists, list_graphs, list_graphs_json, graph_memory_usage, drop_graph, drop_graphs, drop_node_properties, drop_relationships, stream_node_properties, stream_relationship_properties, stream_relationships, write_node_properties …

## Major gap: orchestration template still panics

The core orchestration utility `AlgorithmProcessingTemplateConvenience` is described as “the heart of the Applications system”, but its mode processors currently `panic!` (mutate/write/stream/stats + hook variants).

- File: `gds/src/applications/algorithms/machinery/algorithm_processing_template_convenience.rs`
- Impact: even if individual procedures are correct, the Applications layer cannot yet provide the unified “standard pipeline” execution that a TS schema would wrap.

## Stub / TODO inventory

The following files contain `panic!/todo!/unimplemented!` or explicit “not implemented / not yet implemented” text (first ~3 hits shown).

### gds/src/applications/algorithms/centrality/hits_hook_generator.rs
- line 34: panic!("HITS ETL hook creation not yet implemented")

### gds/src/applications/algorithms/embeddings/fast_rp.rs
- line 99: "mutate mode not yet implemented for FastRP",
- line 104: "write mode not yet implemented for FastRP",

### gds/src/applications/algorithms/embeddings/gat.rs
- line 99: "stream mode not yet implemented for GAT",
- line 104: "mutate mode not yet implemented for GAT",
- line 109: "write mode not yet implemented for GAT",

### gds/src/applications/algorithms/embeddings/graphsage.rs
- line 103: "mutate mode not yet implemented for GraphSAGE",
- line 108: "write mode not yet implemented for GraphSAGE",

### gds/src/applications/algorithms/embeddings/hash_gnn.rs
- line 138: "stream mode not yet implemented for HashGNN",
- line 143: "mutate mode not yet implemented for HashGNN",
- line 148: "write mode not yet implemented for HashGNN",

### gds/src/applications/algorithms/embeddings/node2vec.rs
- line 130: "mutate mode not yet implemented for Node2Vec",
- line 135: "write mode not yet implemented for Node2Vec",

### gds/src/applications/algorithms/machine_learning/split_relationships.rs
- line 37: "splitRelationships is not yet implemented in Rust gds/",

### gds/src/applications/algorithms/machinery/algorithm_estimation_template.rs
- line 28: panic!("memory estimation not yet implemented")

### gds/src/applications/algorithms/machinery/algorithm_processing_template_convenience.rs
- line 68: panic!("mutate mode processing not yet implemented")
- line 97: panic!("write mode processing not yet implemented")
- line 122: panic!("stream mode processing not yet implemented")

### gds/src/applications/algorithms/machinery/memory_guard.rs
- line 40: write!(f, "Memory estimation not implemented")

### gds/src/applications/algorithms/similarity/filtered_knn.rs
- line 145: "mutate mode not yet implemented for Filtered KNN",
- line 162: "write mode not yet implemented for Filtered KNN",

### gds/src/applications/algorithms/similarity/knn.rs
- line 153: // Note: mutate not implemented in KnnBuilder yet
- line 157: "mutate mode not yet implemented for KNN",
- line 171: // Note: write not implemented in KnnBuilder yet

### gds/src/applications/graph_store_catalog/configs/graph_drop_node_properties_config.rs
- line 158: _ => panic!("Expected InvalidParameter error"),

### gds/src/applications/graph_store_catalog/services/graph_listing_service.rs
- line 64: panic!("GraphListingService::default() is not supported; construct with a GraphStoreCatalogService");

### gds/src/applications/graph_store_catalog/services/memory_usage_validator.rs
- line 105: panic!("{}", error_message); // In Java, this would throw an exception

## Practical definition of “fully implement Applications control logic”

A minimal-but-complete Applications pipeline typically includes:

1. **Request decoding + config validation**
   - parse request JSON into typed config structs (or a single internal request enum)
   - validate required fields, ranges, and mutually-exclusive options
2. **Catalog resolution**
   - locate graph by `user + database_id + graph_name` in the `GraphCatalog`
3. **Memory estimation + guardrails**
   - run estimation (if available) and apply a policy (fail/allow/warn)
4. **Procedure invocation**
   - run `procedures::Graph::...` builder with the requested mode
5. **Side effects** (mode-dependent)
   - mutate: write properties/labels into the projection or via catalog utilities
   - write: persist to a result store / DB / export surface
6. **Result rendering**
   - return a stable JSON payload (rows for stream, aggregate object for stats, etc.)

## Suggested next implementation focus (high-leverage)

- Implement the non-hook variants first in `AlgorithmProcessingTemplateConvenience` (stream/stats/mutate/write).
- Replace `panic!` in Applications paths with structured error returns (consistent error JSON).
- After the template is real, fill in any algorithm-specific “not yet implemented” mode handlers; many of those are currently blocked by procedure-level mutate/write stubs (now inventoried separately).
