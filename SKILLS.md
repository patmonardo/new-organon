# SKILLS — Repository Learning & Task Tracker

Purpose
-------
Single canonical place to record skills, focused learning tasks, and quick commands relevant to this repository (GraphStore, Projection, Rust IO, PyG interop).

Skills Matrix
-------------
| Topic | Level | Notes | Example |
|---|---:|---|---|
| GraphStore / Projection | Beginner → Intermediate | Control/data-plane, materialized vs ephemeral projections. | gds/doc/projection_runbook.md |
| Arrow IPC / Parquet | Beginner | Use `arrow2` for transfer, `parquet` for durable storage. | gds/doc/gnn_io_design.md |
| Object-store (S3/GCS) | Beginner | Presigned URLs, `object_store` crate. | - |
| Rust (gds) | Beginner | crate layout: `data`, `io`, `graphstore` modules. | gds/src/
| PyG Interop | Beginner | Export PyG `Data` -> Arrow/NPZ for ingestion. | - |
| Worktrees / Git | Intermediate | Use sibling or central `worktrees/` folder for feature branches. | - |

Tasks / Learning Plan
---------------------
- [ ] Read Arrow IPC primer and `arrow2` quickstart.
- [ ] Implement small Arrow RecordBatch writer in `gds/src/io` (PoC).
- [ ] Add object-store helper (S3) and canonical manifest writer.
- [ ] Implement a ProjectionReader that emits Arrow TransferChunks.
- [ ] Create Python export helper: PyG `InMemoryDataset` -> Arrow/NPZ manifest.

Quick Commands
--------------
```bash
# Run Rust tests for gds crate
cd gds
cargo test

# Create a sibling worktree (example)
mkdir -p ../worktrees/$(basename $(git rev-parse --show-toplevel))
git worktree add -b my-feature ../worktrees/$(basename $(git rev-parse --show-toplevel))/my-feature origin/main
```

Resources
---------
- Arrow: https://arrow.apache.org/docs/
- PyG IO patterns: torch_geometric/data and torch_geometric/io
- `arrow2` crate: https://docs.rs/arrow2
- `object_store` (arrow-rs): https://docs.rs/object-store

Validation / Tests
------------------
- Export a tiny PyG Data -> Arrow -> read back in Rust and assert node/edge counts.
- Materialize a projection to object-store and validate manifest + checksums.

Owner & Cadence
---------------
- Owner: Pat
- Checkpoint: 2026-01-12 — review PoC and next tasks.
