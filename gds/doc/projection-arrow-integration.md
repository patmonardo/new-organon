# Projection + Collections + Application Forms (Arrow-first)

Goal: unify Projection Factory with the Collections backends (Arrow/vector/huge) so Application Forms can drive projections, eval, import/export, and IO, independent of algorithm dispatch. Arrow is the primary buffer format; other backends stay available for niche cases.

## Layers and roles

- Application Forms: public entrypoints (TSJSON dispatcher) for project/export/import/mutate/write ops.
- Projection Factory: builds graph views (CSR/topology + property columns) from GraphStore snapshots; handles type selection, batching, filters.
- Collections: concrete buffers. Default path is `collections/backends/arrow` (RecordBatch/Array); `vec` for small/in-memory; `huge` for sparse/oversized cases.
- GraphStore Snapshot: Arrow-backed storage (nodes/edges/property partitions + schema metadata) loaded via IO backends (file/s3/gcs/http/mem).

## Data flow (happy path)

1. Application Form receives config → resolves GraphStore (catalog) → asks Projection Factory for a projection handle (topology + selected properties).
2. Projection Factory pulls from GraphStore Snapshot (Arrow partitions) using Arrow importer/scanner and materializes:
   - Topology: CSR/COO in Arrow buffers (int64 ids).
   - Properties: Arrow arrays; auto-selects `vec` backend only for tiny graphs/tests.
3. Downstream operations:
   - stream\_\* apps: consume Arrow batches directly.
   - export/import: write/read Arrow/Parquet via IO backend.
   - projection eval: evaluate expressions over Arrow columns and emit new columns/batches.

## Projection Eval (minimal contract)

- Input: projection handle, expression set (column -> expr), optional filter predicate.
- Execution: use Arrow compute kernels where possible; fall back to `compute_kernels` traits for gaps; all columnar.
- Output: Arrow RecordBatch (or stream of batches) with added columns; optional materialization back into GraphStore Snapshot.

## IO backend (Rust fsspec-style)

- `FsBackend` trait: `open_read`, `open_write`, `exists`, `list`, `stat`, optional `range_read`.
- Implementations: `LocalFs`, `Http`, `S3`, `Gcs`, `Mem`; `FileCache` wrapper for caching + etag/mtime validation.
- Arrow helpers: `read_parquet_batches(uri, cols?, row_groups?) -> Stream<RecordBatch>`; `write_parquet(uri, Table, opts)`; IPC/Feather for fast local.

## GraphStore snapshot format (Arrow)

- nodes.parquet: columns `node_id:int64`, optional `label:string`, feature cols (f32/f64/i64/bool).
- edges.parquet: columns `src:int64`, `dst:int64`, `type:string`, edge feature cols.
- props/: optional partitioned property tables per label/type for sparse features.
- schema.json: node/edge type enums, property dtypes, categorical dictionaries, counts/checksums.

## Integration points to wire

- Projection Factory Arrow importer/scanner: prefer `collections/backends/arrow` buffers; ensure CSR builder accepts Arrow chunks from loader.
- GraphStore loaders/exporters: read/write snapshot format via IO backend; on load, hydrate `DefaultGraphStore` using Arrow-backed buffers.
- Dispatcher ops: add `importGraph`/`exportGraph`/`loadGraph`/`saveGraph` that call loaders/exporters; keep `dryRun` for estimates.
- Validation: `GraphStoreValidationService` ensures required columns and dtypes before projection/export; block missing `src/dst/type/node_id`.
- Progress/cancel: wrap long IO/projection in task/user-log registries and honor termination flags.

## Near-term tasks

1. Define `FsBackend` + `FileCache` + Arrow helpers (Parquet/IPC).
2. Implement Arrow snapshot loader/exporter and hook into GraphStore.
3. Update Projection Factory to default to Arrow backends and expose a Projection Eval stub.
4. Add dispatcher ops for import/export/load/save; add tests: round-trip snapshot and Python PyG loader contract.
