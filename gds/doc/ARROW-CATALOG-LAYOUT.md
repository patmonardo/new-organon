# Arrow GraphStore Catalog Layout (proposal)

Goal: a simple, Arrow-first on-disk layout for a single graph that can be
loaded by the projection factory/loader without enterprise sprawl. Keep IO
idiomatic for Arrow/Polars and friendly to parallel reading.

## Directory shape (one directory per graph)

```
<graph-name>/
  schema.json          # optional: logical schema (labels/types/properties)
  config.toml          # optional: ingest/config hints (backends, concurrency)
  nodes.parquet        # node table (id, label(s), properties)
  edges.parquet        # edge table (source, target, type, properties)
  nodes/               # optional: partitioned nodes/ (parquet/arrow) by hash/range
  edges/               # optional: partitioned edges/ by hash/range
  metadata/            # optional: stats, histograms, sampling manifests
```

## Tables (minimum columns)

- `nodes` table: `id: Int64` (required), `label: Utf8` or `labels: Utf8` (optional), property columns (Int64, Float64, Boolean, Utf8).
- `edges` table: `source: Int64`, `target: Int64` (required), `type: Utf8` (optional), property columns (Int64, Float64, Boolean, Utf8).

Notes:
- IDs are stored as external IDs; the factory may remap to dense IDs internally.
- Partitioning (if used) should preserve column names and types across files.

## Metadata files

- `schema.json`: logical schema (labels, relationship types, property keys/types). Used for validation; can be omitted if schema is inferred.
- `config.toml`: loader/factory hints (e.g., `collections_backend = "arrow"`, batch sizes, partition info).
- `metadata/`: optional stats (row counts, min/max per column) to assist planning.

## Loader path

1) Read `config.toml` (if present) for backend/concurrency hints.
2) Read `schema.json` (if present) to validate columns/types.
3) Read `nodes.parquet` and `edges.parquet` (or partitioned dirs) into Arrow batches using `core::io::arrow::reader`.
4) Wrap as `NodeTableReference` / `EdgeTableReference` and feed `ArrowNativeFactory::from_tables` with the selected `collections_backend`.

## Parallelism

- Prefer partitioned `nodes/` and `edges/` directories for very large graphs; loader can scan partitions in parallel and merge.
- For single-file cases, rely on arrow2/Parquet readers’ row-group batching and the factory’s batch scanners in later phases.

## Extension points

- `metadata/` can house embeddings/features, histograms, or snapshots.
- Future: `properties/` subdir for columnar sidecar properties (per label/type) if needed.
- Future: index files (e.g., bloom/secondary) can live under `metadata/indexes/`.

