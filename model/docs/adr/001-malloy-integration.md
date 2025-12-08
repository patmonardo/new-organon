# ADR 001: Malloy-Compatible Semantic Data Integration

## Status

Proposed

## Context

The project aims to achieve "Complete Ontological Reflective Reasoning" by integrating a semantic Modeling layer similar to [Malloy](https://github.com/malloydata/malloy).

We currently have:

1.  **Frontend**: React components driven by declarative schemas (Radix Control Patterns).
2.  **Backend/Data**: A functional `PolarsExecutionEngine` and `SemanticHydrator` that supports fundamental semantic concepts (Measures, Dimensions, Joins, Aggregations).
3.  **SDSL**: A TypeScript-based schema definition language (`sdsl.ts`) that mirrors many Malloy concepts.

The user desire is to "make this official" and support Malloy compatibility, ensuring that our data architecture is aligned with best practices in the Semantic Web/BI space, while leveraging our existing high-performance stack (Arrow/Polars/DuckDB).

## Decision

We will adopt a **"Malloy-Compatible"** architecture for the `@model` package.

### 1. Conceptual Alignment

We will explicitly map our internal `DataModel` concepts to Malloy's:

- **Source**: Maps to our `PolarsDataset` / Arrow Table.
- **Dimension**: Maps to `DimensionDefinition` in `sdsl.ts`.
- **Measure**: Maps to `MeasureDefinition` in `sdsl.ts`.
- **View**: Maps to `DataView` (a query definition: group by dimensions + aggregate measures).
- **Join**: Maps to `JoinDefinition`.

### 2. Integration Point: SDSL

Instead of replacing our stack with the Malloy VSCode extension or runtime directly (which might be heavy or misaligned with our client-side goals), we will:

- **Maintain `sdsl.ts` as the Interface Definition Language**.
- Ensure `sdsl.ts` structures can be _generated from_ or _compiled to_ Malloy source code in the future.
- Use `SemanticHydrator` as the bridge that executes these "Malloy-like" queries and populates the "FormDB" (our Reactive UI State).

### 3. Execution Engine

We will continue to use **Polars/DuckDB/Arrow** as the execution substrate. This provides:

- **Speed**: In-memory Arrow processing.
- **Flexibility**: Capability to run in-browser (WASM) or on-server (Node/Rust).
- **Observability**: We will expose Execution Plans (`$plan`) that show the logical query structure, similar to `EXPLAIN` in SQL/Malloy.

## Consequences

### Positive

- **Unified Language**: Developers can speak in "Measures" and "Dimensions" rather than raw SQL or array reductions.
- **Portability**: If we strictly adhere to the semantics, we could swap out the engine for the official Malloy runtime later if needed.
- **UI decoupling**: The UI (Radix Controller) only knows about the _Result_ (HydratorSnapshot), not the query complexity.

### Negative

- **Maintenance**: We are essentially building a lightweight query engine in `polars-engine.ts`. We must limit scope to avoid building a full SQL database.
- **Complexity**: `SemanticHydrator` adds a layer of indirection between the raw data and the form.

## Action Items

1.  Refactor `sdsl.ts` to ensure strict naming alignment with Malloy (e.g., ensure `primary_key`, `foreign_key` concepts are clear).
2.  Add a "Malloy Compatibility" test suite that validates our engine against standard semantic query patterns.
