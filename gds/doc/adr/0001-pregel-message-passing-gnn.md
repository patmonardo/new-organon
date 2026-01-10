<!--
ADR: Pregel-style Message-Passing GNN for GDS
Date: 2026-01-09
Author: Organon / AI pairing
-->

# ADR 0001: Pregel-style Message-Passing GNN (HashGNN integration)

Status

- Proposed -> Accepted

Context

- PyG (PyTorch Geometric) establishes message-passing as the canonical pattern for GNNs: per-edge messages, per-vertex aggregation, and per-vertex updates across synchronous layers.
- Our `gds/` GraphStore targets extremely large graphs where memory and network bandwidth are primary constraints. We also have a compact embedding idea (`HashGNN`) that favors sketching/hashing to reduce footprint.
- We already aim to keep Python/PyTorch separate from the core runtime; heavy compute can be offloaded or exported (TorchScript/ONNX) where necessary.

Decision

- Adopt a Pregel-style, superstep-based message-passing execution model as the canonical GNN runtime in `gds/`.
- Design the runtime as a controller + worker architecture:
  - Controller (Rust procedure): orchestrates supersteps, partition scheduling, combiners, checkpointing, and failure recovery.
  - Worker (compute runtime): pure computation applying `f_msg` (message), `f_agg` (aggregation/combine), and `f_update` (vertex update) over partition-local slices.
- Use Arrow IPC for in-memory, zero-copy transfers and Parquet/object-store for durable snapshots. TransferChunk semantics (from the GraphStore proto) are the wire/data plane.
- Integrate `HashGNN` as an optional embedding representation & message combiner: workers may compress outgoing messages or local aggregates with hashing sketches before transfer, trading precision for scale.

Rationale

- Pregel's synchronous superstep model maps naturally to message-passing GNN layers and gives deterministic semantics for debugging and checkpointing.
- A controller+worker split keeps orchestration, metadata, and durability in Rust (safe, high-throughput), while allowing compute kernels to be implemented in Rust or isolated Python/GPU workers.
- Arrow IPC preserves performance for in-memory transfers; object-store checkpoints provide resilience and reproducibility.
- HashGNN sketches reduce network and storage costs on huge graphs; integrated as a pluggable combiner/compression stage.

Consequences

- Positive:
  - Deterministic, reproducible runs with snapshotting and manifest metadata (versions, seeds, environment).
  - Scales to very large graphs via partitioning, combiners, and message compression.
  - Clear adapter points for Python/Torch (black-box workers) and GPU acceleration.
- Negative:
  - Synchronous supersteps can hurt latency for iterative, small-batch workloads (asynchronous mode will be an advanced feature).
  - Hashing/sketching trades some accuracy for scale — must be tunable and testable per workload.

Alternatives considered

- Fully asynchronous, actor-style message passing: higher concurrency, eventual consistency, but harder to reason about and checkpoint deterministically.
- Pure in-memory GPU-only runtime: great for small-to-medium graphs, but not feasible for web-scale graphs without sharding and orchestration.
- Requiring all transforms to be exported as ONNX/TorchScript: more portable but brittle for research workflows with custom ops.

Implementation notes

- New types & locations:
  - `gds/src/graphstore/pregel.rs` — controller loop and job management.
  - `gds/src/graphstore/pregel_worker.rs` — worker runtime skeleton and plugin interface.
  - `gds/src/hashgnn/mod.rs` — HashGNN sketch utilities (serialize/deserialize, merge/combine).
  - `gds/doc/pregel_runbook.md` — operational runbook for running Pregel jobs, checkpoints, and failure recovery.
- Core API: `PregelJobSpec` (input graph URI, partitioning strategy, supersteps, combiner, sampling strategy, worker plugin spec).
- Worker plugin interface: expose a minimal trait so compute kernels can be implemented in Rust, or invoked as a black-box Python worker via object-store URIs or gRPC streaming. Prefer exported TorchScript/ONNX models for production inference paths.
- Checkpoint format: Arrow IPC files for per-partition vertex tables + manifest JSON with `superstep`, `job_id`, `rand_seed`, and `model_params_uri`.

Testing & Validation

- Unit tests: run small graphs (few dozen nodes) with a reference PyG implementation to validate outputs for basic GNN layers (GCN, GAT-like attention approximations).
- Integration tests: end-to-end job on small partitioned dataset with snapshot/restore and HashGNN enabled/disabled.
- Performance tests: measure message volume with and without HashGNN; tune combiner thresholds and sketch sizes.

Migration

- Existing projection/transform flows can call the new `PregelGNN` procedure in `gds` by providing a `PregelJobSpec`; no breaking API changes to core GraphStore are necessary.

Open questions

- Default sampling strategy for training on massive graphs (layer-wise neighbor sampling vs. random walk minibatches).
- Precise HashGNN sketch algorithm and its error-bound tradeoffs for downstream tasks.
- Async mode semantics and whether a hybrid (bulk-synchronous with local async updates) is useful later.

References

- Leslie G. Valiant, Google Pregel paper; PyTorch Geometric message-passing API; internal `GraphStore` transfer/Projection design; HashGNN research notes.

Decision recorded: 2026-01-09 — Pregel-style, controller+worker, Arrow IPC, HashGNN optional compression.
