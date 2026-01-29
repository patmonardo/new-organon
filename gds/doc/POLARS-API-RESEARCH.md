# Polars API Research (Draft)

## Goal

Identify which Polars interface (context) best matches Rust Collections semantics and define the mapping to our Collections API.

## Known Polars Interfaces (Contexts)

- **DataFrame**: eager, in-memory, columnar.
- **LazyFrame**: query plans + deferred execution.
- **Streaming**: chunked execution for large datasets.
- **SQL/Expr Contexts**: expression DSL and SQL planner.

> Validate the exact names + scope during research.

## Research Questions

1. Which interface most closely resembles Rust Collections semantics (iterable, eager, in-memory)?
2. How does Polars represent column ownership, mutation, and appends?
3. What are the performance knobs that map to our Collections config?
4. What is the minimal Polars API surface to support GraphFrame tables?
5. How does Polars handle >4GB row tables and memory spill?
6. Which Polars IO connectors map cleanly to GraphStore inputs (CSV, Parquet, JSON, DB, cloud, BigQuery, HF)?

## Mapping Hypotheses (Initial)

- **Collections API ↔ DataFrame** for eager, mutable-ish workflows.
- **Collections API ↔ LazyFrame** for query/planning and large-scale transforms.
- **Extensions (paging/partitioning)** ↔ streaming and lazy execution.

## Action Plan

- Review Polars docs for DataFrame, LazyFrame, Streaming, and SQL/Expr contexts.
- Document the API operations that correspond to Rust Collections patterns:
  - create, append, set/get, slice, filter, map, reduce
  - type/dtype control
  - memory and spill control
- Record findings and decide primary “Collections-compatible” context.

## Polars IO Surface (Initial Inventory)

- CSV
- Parquet
- JSON
- Excel
- Multiple/partitioned sources
- Hive
- Databases
- Cloud storage
- Google BigQuery
- Hugging Face datasets

## Polars ChunkedArray API (Key Focus)

The ChunkedArray API is a likely **closest match** to Rust Collections semantics.
Key trait groups to evaluate:

- `ChunkAgg`, `ChunkApply`, `ChunkApplyKernel`
- `ChunkCast`, `ChunkCompareEq`, `ChunkCompareIneq`
- `ChunkFilter`, `ChunkTake`, `ChunkSort`, `ChunkUnique`
- `ChunkFillNullValue`, `ChunkShift`, `ChunkZip`

This should be the primary surface for Collections parity research.

## Series vs ChunkedArray (Rust Surface)

- Polars `Series` often requires downcasting to concrete `ChunkedArray<T>`.
- `ChunkedArray` implements Rust `Iterator` and `DoubleEndedIterator` traits.
- This makes it a natural bridge to our Collections API and iterator semantics.

## Collections Direction (Rust + Polars)

- Expand Collections to reflect **Rust stdlib idioms** (Iterator patterns, slicing, filtering).
- Treat Polars `ChunkedArray` as the primary interop surface for Collections.
- GraphCatalog/GraphStore remains the **GraphFrame surface** (no separate GraphFrame layer).
- Integrate with **Concurrency**, **Memory**, and **Progress** tools at the Collections layer.

## Polars Module Surface (Expansion Targets)

- `polars-ops` (ops/expr helpers)
- `polars-io` (CSV/IPC/Parquet/Cloud)
- `lazy` (planning/execution)
- `chunkedarray` + temporal utilities

These modules define the breadth we can expose via a unified Collections API.

## Polars vs Arrow (Schema Naming)

- Compare Polars schema naming and dtype taxonomy with Arrow.
- Identify naming/typing mismatches and define a stable mapping for GraphStore schemas.
- Capture any Polars-specific conventions that should become canonical in Collections.

## Delta Lake Notes

Delta Lake (via delta-rs + Polars) adds:

- ACID transactions and versioned tables
- Time travel (read older versions)
- File skipping and Z-ordering for faster scans
- Append/overwrite workflows on Parquet-backed tables

This is relevant to GraphStore-style durability, snapshots, and catalog evolution.

## Related Ecosystem: skrub (Python)

Skrub is a Python library for tabular ML preprocessing that is:

- scikit-learn compatible
- focused on database preprocessing
- supports heterogeneous Pandas and Polars dataframes

It is relevant as a **data-prep pipeline model** and may inform how MCP schedules
ETL + feature prep workflows around Polars-backed tables.

## Related Ecosystem: PyTorch + PyG

- **PyTorch**: training/runtime target for many ML workflows.
- **PyG (PyTorch Geometric)**: graph ML ecosystem with data loaders and graph ops.

These are key integration targets; MCP should support ETL + export flows that
prepare features and adjacency data for PyTorch/PyG consumption.

## Outputs

- Context comparison table
- Recommended primary interface
- Mapping notes for Collections config → Polars policy
