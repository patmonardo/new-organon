# ADR 0001: Master Control Protocol (MCP) for Disk-First Core Graph IO

- Status: Draft
- Date: 2026-01-28
- Owner: GDS Core

## Context

We are introducing a new disk-first graph storage system backed by Polars. The existing `DefaultGraphStore`/`DefaultGraph` are in-memory implementations used for fast development and will remain. The public API surface is defined by `gds/src/types/*` and must remain stable. We need a new implementation that supports large graphs, IPC streaming, and disk-based storage, aligned with prior Java/TS core designs.

## Decision

Create a **Master Control Protocol (MCP)** layer that manages and monitors Polars-backed datasets and IO. Implement `CoreGraphStore` and `CoreGraph` as disk-first implementations of the existing `gds/src/types` traits, delegating storage, projection, and IO operations to MCP.

## Goals

- Preserve the existing `gds/src/types` API.
- Provide disk-first storage using Polars (IPC/Parquet).
- Support large (“huge”) graphs via partitioning, paging, and streaming.
- Separate control plane (MCP) from data plane (CoreGraphStore/CoreGraph).

## Non-Goals

- Replace `DefaultGraphStore`/`DefaultGraph`.
- Introduce NAPI bindings or JS/Rust loader stubs.
- Change external APIs in `gds/src/types`.

## Architecture Overview

### Control Plane: MCP

MCP is a control layer for Polars-backed graph data.
Responsibilities:

- Dataset lifecycle: open/close, pin/unpin, reference counts.
- Storage management: partitioning, compaction, snapshotting.
- IO streaming: Arrow IPC, Parquet reads/writes.
- Monitoring: memory usage, IO throughput, task progress.
- Projections: materialized or ephemeral projections.

### Data Plane: CoreGraphStore/CoreGraph

- Implements traits in `gds/src/types` (same surface as `DefaultGraphStore`).
- Delegates storage/indexing/projection to MCP.
- Exposes graph views and query semantics expected by algorithms.

## Key Interfaces (Planned)

### MCP Control API (Rust traits)

- `McpManager`: open/close datasets, query status, list resources.
- `McpDatasetHandle`: fetch schemas, partitions, node/edge tables.
- `McpProjectionService`: create/read/write projections.
- `McpMonitor`: metrics and logs.

### Core Graph Interfaces

- `CoreGraphStore`: implements `GraphStore` and disk-first storage.
- `CoreGraph`: implements `Graph` and provides topology/property access.

## IO & Storage Model

- Node table: node id, labels, properties.
- Edge table: src, dst, type, properties.
- Optional adjacency index: disk-backed partitioned offsets.
- Formats: Arrow IPC (streaming) + Parquet (durable).

## Disk Format Requirements (GDS-Specific)

### Property Graph → Disk Mapping

GDS treats label sets as first-class. Disk layout must support:

- **Per-label node tables**: one table per active label, containing the nodes that carry that label.
- **Union view**: a logical “all-nodes” view across label tables for algorithms that operate on the full graph.
- **Graph schema**: a global view that records all labels, relationship types, and property schemas even when data is partitioned by label.

### GraphStore → Disk Structures

The disk-first store must provide durable equivalents of in-memory components:

- **Id map**: mapping between external/original IDs and internal contiguous IDs.
- **Node properties**: columnar storage per label with a shared schema registry.
- **Relationship tables**: at least `(src, dst, type)` plus property columns, optionally partitioned by type.
- **Adjacency index (optional)**: precomputed offsets for fast neighbor iteration; partitioned by relationship type and node range.

### Schema Mapping (GDS → Polars/Arrow)

- GDS property types must map to Arrow-compatible dtypes.
- Polars schema should be treated as Arrow-compatible for IPC/Parquet interoperability.
- The catalog must persist: graph name, labels, relationship types, property schemas, and partition layout.

### Metadata & Catalog

- **Manifest**: a durable metadata file (or table) describing dataset version, partitions, and schema.
- **Projections**: stored as separate manifests with lineage back to the source graph.
- **Evolution rules**: explicit behavior for schema changes and property addition/removal.

### Open Questions (to validate with Polars)

- Best practice for multi-table dataset catalogs and schema evolution.
- How Polars handles schema registry across multiple tables/partitions.
- Optimal layout for adjacency offsets using Arrow-compatible buffers.

## Huge Graph Support

- Partitioned topology with paging.
- Incremental property loading.
- Filtered graph views over partitions.

## Integration Points

- Catalog: track MCP datasets and storage URIs.
- Loader: build CoreGraphStore from disk inputs.
- Projection: route to MCP’s projection service.

## Phased Implementation

1. **Scaffold MCP traits + CoreGraphStore/CoreGraph stubs**.
2. **Implement disk-backed schemas & node/edge tables**.
3. **Add streaming projection and IPC**.
4. **Add huge-graph paging and topology indexes**.

## Risks

- Performance tuning for large graphs.
- Consistency guarantees across partitions.
- Mapping Polars columnar model to graph adjacency expectations.

## Alternatives Considered

- Maintain only in-memory stores (rejected: not disk-first).
- Direct filesystem IO without MCP (rejected: lacks control/monitoring).

## References

- Java GDS core and API (graph-data-science).
- TS translation (organon.sav/gds/src/core).
- Polars repository (gds/polars).
