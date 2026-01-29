# Polars Master Control Thread

## Intent

Launch a focused thread to reframe GDS I/O around Polars-first data control, reduce GraphStore complexity, and sequence integration work: Polars control → Collections integration → HugeGraphStore/CSR integration.

## Problem Statement

- Java GDS stack is too complex (excessive factories, naming drift).
- GraphStore is tightly coupled to HugeArray/HugeGraph in-memory model.
- We want a uniform, simpler approach that prioritizes I/O and Polars integration.

## Core Thesis

- Treat Polars as the primary data control surface for tabular/columnar state.
- Use Polars-backed CSR adjacency as the canonical graph representation for loading and I/O.
- Align Collections features with Polars’ table/expr model before rebuilding GraphStore layers.

## Positioning: Polars Extension (GraphFrame)

- We are not a Python client to Rust; we are a **Polars extension**.
- Think **GraphFrame on top of DataFrame**: graph semantics embedded in Polars tables.
- A GraphFrame-style layer should be the primary API, with GraphStore layered above it.

## Guiding Principles

- **Polars-first** for data movement, schema, and memory strategy.
- **IO before GraphStore**: stabilize ingestion/export, then build graph-store layers on top.
- **Minimal control plane**: prefer small, explicit controllers over nested factory stacks.
- **Uniform naming**: consistent module language across IO, Collections, and GraphStore.

## Proposed Sequence (Phased)

1. **Master Control Protocol (Polars)**
   - Establish Polars control surface (tables, columns, dtype policy, chunking).
   - Capture SIMD/GPU/memory behaviors as explicit policy knobs.
2. **Collections Integration**
   - Map existing Collections config to Polars capabilities (large tables, streaming, chunking).
   - Validate parity with current GDS features; define gaps and bridge plan.
3. **CSR Graph Layer**
   - Define CSR adjacency list schema in Polars (edges table + offset index).
   - Maintain minimal graph metadata (node/rel labels, properties) in Polars tables.
4. **GraphStore Rebuild (Optional/Deferred)**
   - Rebuild a thin GraphStore facade only after IO + Collections are stable.
   - Keep storage runtime minimal; avoid factory cascades.

## Key Questions

- What is the minimal Polars schema for graph nodes/relationships and properties?
- How do we model CSR offsets and adjacency lists in Polars consistently?
- Which Collections features map directly to Polars, and which need adapters?
- What are the concrete I/O paths (CSV, file, DB) that must be first-class?

## Immediate Actions (Next 1–2 weeks)

- Draft a Polars control spec: tables, dtypes, memory, execution policy.
- Prototype CSR adjacency representation in Polars (no GraphStore dependency).
- Enumerate Collections config → Polars capability mapping.
- Identify IO endpoints (CSV, Parquet, DB) and define minimal adapters.

## Ownership and Next Step

- This thread is the coordination anchor for Polars-first IO and graph representation work.
- All follow-up documents should link back here as the primary “launch” context.

## Related Drafts

- Polars ↔ Collections API: gds/doc/POLARS-COLLECTIONS-API.md
