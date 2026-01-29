# ADR 0003: CoreGraphStore on MCP (Control Plane Integration)

- Status: Draft
- Date: 2026-01-28
- Owner: GDS Core
- Related: ADR 0001 (MCP), ADR 0002 (Storage Model)

## Context

We are adding a disk-first `CoreGraphStore`/`CoreGraph` implementation that must satisfy the existing `gds/src/types` traits. The physical layout is Polars/Arrow-based and managed by MCP. We need a clear boundary where MCP hides disk layout and provides a stable interface for loaders and algorithms.

## Decision

`CoreGraphStore` is implemented as an MCP client. MCP owns the storage layout, lifecycle, and IO. The loader pipeline talks only to `CoreGraphStore`, which obtains data via MCP handles and returns graph views that comply with `gds/src/types`.

## Goals

- Keep the `gds/src/types` API stable.
- Decouple algorithms from physical storage details.
- Centralize lifecycle/monitoring in MCP.
- Allow multiple storage backends behind MCP without changing the loader.

## Non-Goals

- Replace the in-memory `DefaultGraphStore`/`DefaultGraph`.
- Expose MCP internals to algorithms.
- Bind to external NAPI or JS loaders.

## Architecture Overview

### Control Plane (MCP)

- Manages datasets, partitions, schemas, and IO streams.
- Provides stable handles for graph resources.
- Tracks metrics, refcounts, and lifecycle.

### Data Plane (CoreGraphStore/CoreGraph)

- Implements the `GraphStore`/`Graph` traits from `gds/src/types`.
- Converts MCP handles into graph views and property accessors.
- Delegates all persistence and streaming to MCP.

## Interface Boundary (Conceptual)

- `CoreGraphStore::from_mcp(handle) -> CoreGraphStore`
- `CoreGraphStore::graph() -> CoreGraph`
- `CoreGraph` supplies topology and property access via MCP-backed cursors/iterators.
- Loader code remains unchanged except for selecting the CoreGraphStore factory.

## Loader Flow

1. Loader requests a graph by name (catalog entry).
2. Catalog resolves MCP dataset handle and storage URIs.
3. MCP returns a dataset handle (schemas + partitions).
4. `CoreGraphStore` wraps MCP handle and exposes `GraphStore` APIs.
5. Algorithms consume `Graph` APIs without storage knowledge.

## Failure and Recovery

- MCP is authoritative for dataset health, corruption detection, and recovery steps.
- `CoreGraphStore` returns typed errors while preserving `gds/src/types` error contracts.

## Open Questions

- Exact trait surface needed for MCP handle types.
- How to model streaming cursors for huge graphs within existing traits.
- Which loader entrypoint selects CoreGraphStore vs DefaultGraphStore.

## Next Steps

- Define MCP trait interfaces in Rust.
- Add CoreGraphStore/CoreGraph stubs that wrap MCP handles.
- Integrate catalog entries with MCP dataset metadata.
