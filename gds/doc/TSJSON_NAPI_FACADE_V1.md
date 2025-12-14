# TS-JSON NAPI Facade (V1)

Goal: make GDS callable from TypeScript via NAPI-RS using a **stable JSON protocol** (“TS-JSON”) while keeping the internal Rust `applications/` layer free to mirror Java GDS.

## Principles

- **Java parity inside, TS parity outside**
  - Internal Rust applications: mirror Java GDS APIs and semantics.
  - External NAPI surface: a small set of FFI-safe entrypoints.

- **No TB data over NAPI**
  - GraphStores can be terabytes.
  - NAPI calls must return **handles** + **paged reads** + **file/Arrow exports**.

- **Protocol-first**
  - One generic `invoke(json)` entrypoint is easy to version and keeps TS bindings stable.

## V1 transport: JSON envelope

Request is a JSON object. Minimum field:

```json
{ "op": "ping" }
```

Response is a JSON object:

- success:

```json
{ "ok": true, "op": "version", "data": { "crate": "gds", "version": "0.1.0" } }
```

- error:

```json
{ "ok": false, "op": "…", "error": { "code": "UNSUPPORTED_OP", "message": "…" } }
```

## V1 operations (proposed)

### Core

- `ping` → echoes a nonce
- `version` → returns crate version

### Graph catalog (handle-based)

- `graph.list` → returns graph metadata list (small)
- `graph.info` → returns schema + counts (small)
- `graph.drop` → returns deletion counts (small)
- `graph.load` → returns `{ graphId }` (handle)

### Jobs

Long-running work must be job-based:

- `job.start` → returns `{ jobId }`
- `job.status` → returns progress + state
- `job.cancel` → best-effort cancellation
- `job.result` → returns `{ resultId }` or exported artifact handles

### Results (TB-safe)

- `result.page`
  - Reads a bounded page (`offset`/`limit`) and returns rows or columnar chunks.
  - Intended for “small” interactive exploration.

- `result.export`
  - Writes results to disk as Arrow IPC / Parquet / NDJSON.
  - Preferred for large outputs.

## nodejs-polars alignment

nodejs-polars supports IPC/Parquet read/write flows. The easiest and most robust interop pattern is:

1. GDS writes Arrow IPC or Parquet to a file (`result.export`).
2. TS code uses nodejs-polars `readIPC` / `readParquet` to consume it.

This avoids copying large buffers through NAPI and lets Polars handle efficient columnar reads.

## Current implementation status

- Implemented in Rust: `gds::tsjson_napi::invoke(json)` and `gds::tsjson_napi::version()`.
- Next: expand `invoke` to route `graph.*`, `job.*`, and `result.*` operations.
