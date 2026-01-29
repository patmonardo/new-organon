# ADR 0002: Disk Storage Model (Polars + Arrow)

- Status: Draft
- Date: 2026-01-28
- Owner: GDS Core
- Related: ADR 0001 (MCP)

## Context

We need a disk-first storage model for CoreGraphStore/CoreGraph using Polars and Arrow. The model must support multi-label nodes, large graphs, streaming IO (IPC), and schema evolution, while preserving the existing `gds/src/types` API.

## Decision

Adopt **Option A (normalized, scalable)** for multi-label nodes:

- A global `nodes` table for identifiers and base properties.
- A `node_label` join table for label membership.
- Per-label property tables for label-specific properties.
- Relationship tables partitioned by type (and optionally by source range).

## Goals

- Efficient label-based filtering and union views.
- Avoid wide sparse tables across all labels.
- Arrow-compatible schemas for interoperability.
- Partitioning and paging to support huge graphs.

## Storage Model

### Property Store Boundaries

GDS treats **Graph**, **Node**, and **Relationship** properties as three distinct stores with independent schemas. The disk model must preserve this separation so each store can evolve its schema without coupling to the others.

### Core Tables

1. **nodes**
   - Columns: `node_id` (internal), `original_id` (external), `labels` (optional bitset/list)
   - Purpose: canonical node identity + global properties

2. **node_label**
   - Columns: `node_id`, `label_id`
   - Purpose: explicit membership for multi-label nodes

3. **label\_{LabelName}** (one per label)
   - Columns: `node_id`, label-specific properties
   - Purpose: compact storage for properties specific to a label

4. **rel\_{RelType}** (one per relationship type)
   - Columns: `src`, `dst`, `type_id`, relationship properties
   - Purpose: typed relationship storage; optional partitioning by `src` ranges

5. **adj\_{RelType}** (optional adjacency index)
   - Columns: `src`, `offset`, `length`
   - Purpose: fast neighbor iteration without full scans

### Manifest / Catalog

- **graph_manifest.json** (or Arrow table) describing:
  - Graph id/name, version
  - Label registry (`label_id` ↔ label name)
  - Relationship registry (`type_id` ↔ type name)
  - Property schemas per table
  - Partition layout and file locations

### Schema Mapping (GDS → Arrow)

- GDS property types map directly to Arrow dtypes.
- Polars schema is treated as Arrow-compatible for IPC/Parquet.
- Schema evolution rules must be explicit in the manifest (additive changes first).

## Access Patterns

- **Label filter**: `node_label` join + `label_{LabelName}` table scan.
- **All nodes**: `nodes` table union with label tables via `node_id`.
- **Edge traversal**: use `rel_{RelType}` + `adj_{RelType}` (if available).

## Polars DSL Query Patterns (Planned)

These patterns rely on Polars `LazyFrame` joins and projection pushdown to keep IO minimal.

### Label filter (nodes by label)

1. Load `node_label` (filtered by `label_id`).
2. Join with `nodes` on `node_id`.
3. Optionally join with `label_{LabelName}` for label-specific properties.

### Multi-label intersection

1. Filter `node_label` by multiple `label_id` values.
2. Group by `node_id` and retain those with all requested labels.
3. Join with `nodes` and relevant label property tables.

### Edge traversal

1. Scan `rel_{RelType}` partitions for a node range.
2. If adjacency index exists, use `adj_{RelType}` to seek offsets.
3. Project only `(src, dst, needed_props)` for algorithm stages.

### Union view

1. Start from `nodes` for base properties.
2. Left-join label tables for optional properties as needed.
3. Use schema-aware projection to avoid wide materialization.

## Huge Graph Strategy

- Partition tables by `node_id` ranges.
- Store adjacency offsets per partition.
- Stream partitions via Arrow IPC.

## Open Questions

- Best catalog format: JSON manifest vs Arrow metadata table.
- Whether to persist `labels` on `nodes` as bitset or derive from `node_label`.
- Relationship property schema evolution policy.

## Alternatives Considered

- Wide single node table (rejected: sparse + inflexible for multi-label).
- One table per label-combination (rejected: combinatorial growth).

## Next Steps

- Prototype manifest format and naming conventions.
- Define MCP API calls for table discovery and partition listing.
- Align `CoreGraphStore` accessors with this model.
