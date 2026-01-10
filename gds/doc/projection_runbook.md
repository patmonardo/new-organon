# Projection Runbook — GraphStore Projection Workflow

Status: initial draft

Purpose
-------
Describe the Projection lifecycle (ephemeral vs materialized), control/data-plane responsibilities, formats, and operational steps for implementing Projection reader/writer in `gds`.

Key concepts
------------
- Projection: a derived dataset computed from a source `Graph` stored in a `GraphStore`.
- Ephemeral Projection: computed on-demand and streamed back to the caller; not stored permanently.
- Materialized Projection: computed, written to object-store (Arrow/Parquet/NPZ), cataloged and versioned for reuse.
- Snapshot: a consistent graph version id used to guarantee projection determinism.
- Job: asynchronous materialization job with checkpointing and status tracking.

Formats
-------
- Canonical transfer: Arrow IPC RecordBatches (preferred for zero-copy and columnar semantics).
- Durable storage: Parquet for columnar durability; NPZ for light-weight arrays.
- Metadata: small JSON/Protobuf manifest describing schema, feature dtypes, offsets, checksums.

Control-plane vs Data-plane
---------------------------
- Control-plane (gRPC/HTTP): create/describe/list/delete projections; start materialization; query job status.
- Data-plane (streaming): Transfer binary chunks (Arrow frames or framed Parquet/NPZ blobs). Prefer streaming gRPC (or presigned object-store URIs for direct transfer).

Projection lifecycle (recommended)
----------------------------------
1. CreateProjection(control): client submits {graph_id, expr, materialize?, output_format, storage_uri?}.
2. Assign snapshot: server resolves `snapshot_id` for the graph at projection start.
3. If materialize:
   - Enqueue Job (JobHandle) and return ProjectionHandle immediately.
   - Job worker: reads graph snapshot, evaluates expression, emits Arrow RecordBatches.
   - Checkpoint periodically (upload intermediate files or persist last processed node/edge range).
   - On completion: write manifest + parts to `storage_uri` (object-store) and update Catalog with `ProjectionMeta` including schema and version.
4. If ephemeral:
   - Execute projection on-demand and stream TransferChunks back to client; do not write to catalog unless client asks to persist.

Streaming & chunking
--------------------
- Chunk size: choose based on memory and network (e.g., ~64–256MB uncompressed Arrow batches).
- Each TransferChunk carries: chunk_bytes, format, seq_no, schema_id, optional checksum and stats.
- Support resume: include last-acked seq_no and server-side checkpointing when materializing.

Object-store integration
------------------------
- Use object-store (S3/GCS/local) for durable projection parts. Prefer presigned URLs for large uploads/downloads.
- Store a small manifest alongside data parts: manifest.json containing schema, part list, checksums, created_at, snapshot_id, version.

Job handling & operations
------------------------
- Jobs should be idempotent and resumable.
- Provide JobHandle APIs: GetStatus(JobId), Cancel(JobId), ListJobs(filter).
- Support retries and backoff for transient failures; report partial progress to clients.

Security & access
-----------------
- Authenticate control-plane calls (mTLS/Bearer tokens) and authorize per-store/graph/projection.
- Use presigned URLs for data-plane transfers to avoid proxying large payloads through the control-plane.

Observability
-------------
- Emit metrics: bytes emitted, records processed, elapsed time, job success/failure counts.
- Log manifest events: job start/complete/failure; manifest writes.

Testing checklist
-----------------
- Small graph round-trip: create small graph, create projection, stream result, verify schema and counts.
- Large graph checkpointing: force interrupt mid-job, resume, and verify final output.
- Format compatibility: read materialized projection via Arrow reader and via Parquet reader, validate equivalence.

Next implementation steps (developer tasks)
-----------------------------------------
1. Scaffold `gds/src/graphstore/projection.rs` with `ProjectionSpec`, `ProjectionMeta`, `TransferChunk`, `ProjectionReader` and `ProjectionWriter` traits. (done: scaffolded)
2. Implement Arrow writer/reader helpers (use `arrow2`/`parquet` crates) and object-store helper (use `object_store` crate).
3. Implement Job runner with checkpointing and manifest writer.
4. Add gRPC proto for Projection control + streaming TransferChunk and generate clients.
5. Add unit and integration tests.

References
----------
- Arrow IPC: zero-copy framed RecordBatches for efficient transfer.
- Parquet: columnar durable storage for large materialized projections.

---

Created as the initial operational runbook for Projection implementation in `gds`.
