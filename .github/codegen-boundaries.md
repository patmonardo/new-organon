# Codegen Boundaries: GDSL / SDSL (TS) vs GDS (Rust)

This repo intentionally treats **GDSL/SDSL as user-space languages** and **Rust GDS as a compute substrate**.

If you’re generating code (human or AI), follow these boundaries so we don’t re-create the same entanglement that exists in the Java GDS “Applications” surface.

## What is “technically ideal” here?

**Yes — for this architecture, keeping GDSL in TypeScript is technically sound.**

- **GDSL is primarily a *shell language***: user-facing, dynamic, interactive, schema-heavy, and orchestration-adjacent.
- **TypeScript is the right tool** for that surface: fast iteration, excellent structural typing for IR, Zod schemas, and a natural home for UI integration.
- **Rust is the right tool** for the kernel: compute-heavy algorithms, memory layouts, concurrency, and correctness-critical primitives.

The goal is not “everything in Rust because it’s ‘GDS’.” The goal is **a stable contract** between:

- **PureForm**: declarative IR + schemas + constraints (TS)
- **GivenForm**: execution + effects + compute (TS runtime + Rust kernel)

### A boundary that is easy to violate: pre-lingual kernel vs discursive user space

Treat Rust `gds/` as a **sublingual kernel**: lawful compute and constraint enforcement, not a discursive “thinking” surface.

Human-facing narration (schemas, explanations, traces) lives in TS user space.
See `gds/doc/PRELINGUAL-KERNEL-COGITO-MEDIATION.md`.

RealityFabric note: the intended long-run projection substrate is described in `reality/doc/REALITYFABRIC-PROJECTS-ORGANIC-UNITY.md`.

## Packages and responsibilities (canonical)

### TS workspace (user space)

- **`gdsl/` (`@organon/gdsl`)**
  - Owns the **IR / protocol**: types, schemas, serialization format, versioning.
  - No heavy compute. No direct dependency on Rust.

- **`model/` (`@organon/model`)**
  - Owns **SDSL runtime** (MVC “species DSL”): adapters, controllers/views, UI bindings.
  - Can interpret/emit GDSL IR.
  - May call into compute backends through **explicit ports/adapters**.

- **`task/` (`@organon/task`)**
  - Owns orchestration schemas (TAW) and agent/workflow protocols.
  - Provides Zod-first discriminated unions for events.

- **`logic/` (`@organon/logic`)**
  - Owns BEC schemas + canonical graph encoding / validation.

### Rust workspace (kernel space)

- **`gds/`**
  - Compute substrate: algorithms, storage primitives, concurrency, ML primitives.
  - Avoid “userland dynamic glue” (prompting, UI, orchestration).

## Agent coupling: RootAgent ("systemd") vs SingleAgent ("process")

This repo’s current direction assumes the Agent platform is real and central.
That means language boundaries should be drawn to **serve agent execution**, not to mimic Java GDS application surfaces.

### RootAgent: user-mode shell + supervisor

Treat the RootAgent like a **user-mode init/supervisor** (your “systemd” analogy):

- It is inherently *shell-like*: interactive, dynamic, environment-aware.
- It should be **tightly coupled to GDSL**.
- It owns the “control plane”: prompt routing, tool invocation, policy, state, and orchestration.

In practice: RootAgent should accept human inputs and emit **canonical, reproducible artifacts**:
- **GDSL IR** (for intent/model/execution plans)
- **TAW events** (for orchestration/execution streams)

### SingleAgent: SDSL-focused runtime worker

Treat a SingleAgent more like a **runtime worker / process**:

- More SDSL-focused (domain-specific runtime behavior, MVC bindings, UI adapters).
- Less “shell”, more “library/runtime”.
- It still speaks GDSL/TAW as the wire formats.

### "One language" without collapsing layers

If the long-term goal is “one language”, the safest path is:

- **GDSL is the canonical IR** (the shared language substrate).
- SDSLs are *species* that **compile/translate to GDSL IR**.
- RootAgent speaks GDSL natively, and every SDSL speaks GDSL by compilation.

This gives you *unity of language* at the IR layer without forcing all ergonomics into one syntax or one runtime.

## A practical 4-layer model (optional but useful)

If a 2-layer (kernel/shell) split feels too Unix-ish for your needs, this 4-layer split tends to hold up well:

1. **Kernel (Rust)**: compute + storage + correctness primitives (`gds/`).
2. **Protocol (TS)**: canonical IR + schemas (`gdsl/`).
3. **Runtime (TS)**: MVC/SDSL execution, adapters, UI bindings (`model/`).
4. **Supervisor (TS)**: RootAgent orchestration/shell (“systemd”), emitting IR/events (`task/` + agent runtime).

Codegen should preserve these layers unless there is a concrete, measured reason to merge two.

- **`gdsl-native/` / N-API**
  - If/when used: *thin binding only*, ideally generated from stable contracts.

## Hard rules for codegen

### 1) Keep GDSL IR stable and boring

- Prefer **discriminated unions** with explicit `kind` tags.
- Version IR changes explicitly.
- Keep IR types **data-only** (no methods with side effects).

### 2) Don’t drag “Application APIs” into Rust GDS

- In Rust, implement **algorithms and primitives**.
- In TS, implement **orchestration and user-facing ergonomics**.

If you need a convenient “application API”, make it a TS façade that emits:
- GDSL IR (for reproducibility), and/or
- TAW events (for orchestration).

### 3) Ports and adapters are explicit

- TS → Rust calls must go through a **named port** (adapter module) with:
  - clear input/output schemas,
  - bounded error surface,
  - no leaking of internal Rust types into TS.

### 4) Avoid union parsing when returning specific event types

When a function returns a *specific* event type, parse/validate with the *specific* schema.
Example pattern:
- Prefer `TawActEventSchema.parse(x)` over `TawEventSchema.parse(x)`.

This prevents widening to unions and keeps inference stable.

### 5) Testing responsibilities

- **TS**: property-based checks for IR round-trips, schema validation, adapter contracts.
- **Rust**: algorithm correctness, performance characteristics, memory safety.

## Practical heuristics (quick decision table)

Put it in **TypeScript** if it’s:
- schemas, IR, event protocols
- UI/UX or human-facing interaction
- orchestration (agents, workflows)
- “shell language” dynamics

Put it in **Rust** if it’s:
- graph algorithms
- ML math kernels
- memory layouts / big arrays
- concurrency-heavy execution- **GraphStore extraction and materialization** (from Postgres to DuckDB/Polars)
- **GNN inference primitives** (constraint propagation, traversal, reachability)

## GraphStore and GNN inference layer

**New architectural component**: See `gds/doc/GNN-GRAPHSTORE-INFERENCE-LAYER.md` for full details.

**In brief**:
- **Postgres** stores SDSL instances (Prisma models = operational store).
- **GDS (Rust)** extracts GraphStores (in-memory, columnar, read-only analytic views).
- **GraphStores** live in DuckDB (SQL-queryable) or Polars/Arrow (zero-copy columnar).
- **GNN inference engine** (Rust traits) propagates constraints across domain boundaries.
- **GDSL (TS)** provides typed facade for Agent consumption, emitting GDSL operations.

**Boundary rules**:
- GDS builds and queries GraphStores (read-only, no Postgres mutation).
- Agents (TS) request inferences via GDSL facade.
- Results return as GDSL IR, persisted through Prisma if validated.
- LLMs propose, GNN verifies, Agent executes.

**Why this split**:
- Postgres = transactional (OLTP), row-oriented.
- GraphStores = analytic (OLAP), column-oriented, optimized for traversal.
- Rust handles extraction, zero-copy Arrow, and graph algorithms.
- TS handles orchestration, schema, and user-facing types.
## Non-goals

- We are not trying to make Rust the only source of truth for “language.”
- We are not trying to mirror Java GDS’s full surface area at the expense of maintainability.
