# Polars ↔ Collections API (Draft)

## Purpose

Define a single API surface that merges Collections with Polars, positioning Collections as the **direct Polars client** and treating `GraphStore`/`Graph` as the **GraphFrame**.

## Scope

- Convert Collections config into Polars table/column policy.
- Provide a stable mapping for performance, memory, and extension features.
- Keep GraphStore integration **co-equal** with GraphFrame semantics (they are the same layer).

## Design Tenets

- **Polars-first**: Collections are expressed as Polars columns and tables.
- **GraphStore = GraphFrame**: Graph semantics live in the GraphStore/Graph layer on Polars tables.
- **Config-to-policy**: Collections config becomes explicit Polars execution/memory policy.
- **Small control plane**: avoid factory cascades and ambiguous naming.

## Master Control Protocol (MCP)

The MCP must coordinate:

- **Collections layer**: direct Polars DataFrame/LazyFrame control (schema, dtype, memory, IO).
- **GraphStore/Graph layer**: graph semantics on top of Polars (CSR index, nodes/edges tables).

The MCP should be the single entrypoint that normalizes config, selects policies, and
dispatches to Collections and GraphStore builders.

### MCP at the DataOps Layer (Skrub-aligned)

MCP should align with **Skrub-style DataOps**:

- declarative dataflow (variables, DataOp graphs)
- execution planning and evaluation
- dataset preparation + feature engineering
- cross-validation and hyperparameter search integration

This positions MCP as the orchestration layer for Polars-backed Collections and
GraphStore dataflows, not just storage IO.

## Core Abstractions

### 1) PolarsCollectionsPolicy

A normalized policy derived from CollectionsConfig that drives Polars behavior.

**Proposed fields**

- data_model: columnar | lazy | streaming
- dtype_policy: strict | coercing
- chunking: fixed | adaptive | streaming
- memory: in_memory | disk_spill | hybrid
- compute: simd | gpu | auto
- io: csv | parquet | arrow | database
- extensions: metrics | paging | partitioning | compression | encryption

### 2) GraphStore/Graph (GraphFrame)

GraphStore/Graph is the graph-aware wrapper over Polars tables.

**Minimum tables**

- nodes: id, labels, properties
- edges: src, dst, type, properties
- index: CSR offsets + adjacency list columns

### 3) Collections ↔ Polars Mapping

| Collections area         | Polars feature          | Notes                                     |
| ------------------------ | ----------------------- | ----------------------------------------- |
| backend.primary          | execution/memory policy | replace Vec/Huge/Arrow with Polars policy |
| element_type.value_type  | dtype                   | map to Polars dtypes                      |
| element_type.nullability | nullable columns        | map to Polars nullable                    |
| performance.\*           | lazy/streaming/simd     | use Polars query/execution settings       |
| extensions.metrics       | metrics hooks           | map to Polars profiling                   |
| extensions.paging        | chunking/spill          | map to lazy/streaming                     |
| extensions.partitioning  | partitioned datasets    | map to Polars partitions                  |
| dataset/data_source      | scan/read configs       | map to Polars readers                     |

## API Surface (Conceptual)

- CollectionsConfig → PolarsCollectionsPolicy
- PolarsCollectionsPolicy → Polars LazyFrame/DataFrame build
- GraphStore/Graph → CSR adjacency + node/edge tables

## Migration Plan (Incremental)

1. Define PolarsCollectionsPolicy schema and adapter.
2. Implement CollectionsConfig → Policy mapping.
3. Prototype GraphStore tables + CSR index in Polars.
4. Route import/IO through Polars policy.
5. Stabilize GraphStore/Graph as the GraphFrame surface.

## Open Questions

- What is the minimal CSR schema that keeps joins fast?
- Which Collections extensions map 1:1 to Polars?
- How do we express GPU policy in a Polars-native way?
