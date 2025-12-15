# Projection/Factory tracking note (Arrow + Collections boundary)

This note is a lightweight map of what the current `projection/factory/` layer does today, where it couples to `collections/`, and what to keep an eye on as we upgrade Collections and Arrow.

## Scope

- This is **not** a full design for GraphStore ingestion.
- It documents the **current state** in `gds/src/projection/factory/**` and the **boundary** we want to preserve as we refactor.

## Quick mental model

- **Projection** owns: turning an external/native representation (Arrow tables, etc.) into:
  - graph topology (nodes/edges)
  - id mapping (original IDs → mapped/internal IDs)
  - property stores (node/relationship/graph properties)
  - schema/metadata

- **Collections** owns: storage backends + construction APIs:
  - Vec/Huge/Arrow implementations
  - `Collections<T>` and `CollectionsFactory<T>`
  - Arrow-backed collection wrappers and their factories

The long-term goal is that Projection/Factory chooses *what* to import and supplies data in a backend-agnostic way, while Collections chooses *how* values are stored.

## Where things stand today

### Factory surface

- `GraphStoreFactory` in [gds/src/projection/factory/mod.rs](../src/projection/factory/mod.rs)
  - returns `DefaultGraphStore`
- `GraphStoreFactoryTyped` in [gds/src/projection/factory/mod.rs](../src/projection/factory/mod.rs)
  - intended as the “future-proof” interface (`type Store`)

### Arrow factory modules (inside Projection)

The Arrow subsystem under [gds/src/projection/factory/arrow](../src/projection/factory/arrow/mod.rs) contains a fairly complete *pipeline* (references, scanners, tasks, consumers, importers), but the **top-level `ArrowNativeFactory` entrypoint is still Phase-1 skeleton**:

- `ArrowNativeFactory` is in [gds/src/projection/factory/arrow/factory.rs](../src/projection/factory/arrow/factory.rs)
  - currently validates config then returns “not implemented”

The “real work” lives in these modules:

- `reference.rs` — Arrow2 table/schema wrappers + conventions
- `scanner.rs` — batch cursors and parallel scanning
- `task.rs`, `consumer.rs`, `consumer_task.rs` — orchestration + buffering
- `importer.rs` — accumulation + conversion into `PropertyValues`-compatible structures

### Where Projection currently couples to Collections

In [gds/src/projection/factory/arrow/importer.rs](../src/projection/factory/arrow/importer.rs), property accumulation ends by constructing `Default*NodePropertyValues` using **Vec-backed Collections**:

- `crate::collections::backends::vec::VecLong`, `VecDouble`, `VecLongArray`, `VecDoubleArray`

This is currently the concrete “mating point”:

- Projection chooses a backend (Vec) inside the importer.
- That makes later migration to Arrow/Huge backends a refactor inside Projection unless we introduce a backend-agnostic adapter.

### Collections Arrow factory already exists

There is already an Arrow factory concept in Collections:

- [gds/src/collections/backends/arrow/factory.rs](../src/collections/backends/arrow/factory.rs)

This is consistent with the direction “Arrow factory belongs in Collections”. The open issue is that Projection’s Arrow importer isn’t using it yet.

## What to monitor during Collections upgrades

### Codegen/collections macros

Keep an eye on the macro-generated trait impls in:

- `gds/src/projection/codegen/collections/*`

Why:
- these macros emit `Collections`/`CollectionsFactory` impls and must remain consistent with the collections trait surface as we evolve Arrow.

### Projection/factory integration points

As we upgrade Arrow in Collections, track these spots in Projection/Factory:

1. **Property materialization**
   - today: importer builds dense `Vec<T>` → `Vec*` collections
   - goal: allow selecting backend (`Vec`/`Huge`/`Arrow`) without rewriting importer logic

2. **Topology materialization**
   - where edge lists / CSR-ish structures are built
   - watch for backend decisions that should move into Collections

3. **Entrypoint wiring**
   - `ArrowNativeFactory::build_graph_store()` is still a stub
   - pipeline modules exist and need to be connected in a minimal, testable way

4. **Error + config ownership**
   - `ArrowProjectionConfig` / `ArrowProjectionError` currently live in Projection
   - confirm which options are “ingestion semantics” vs “collections backend selection”

## Suggested “next-week” refactor shape (non-binding)

A low-risk path that keeps boundaries clean:

- Add a small abstraction in Projection/Factory: “materialize dense property column into a `Collections<T>` via a `CollectionsFactory<T>`”.
- Keep the importer responsible for:
  - mapping original IDs → mapped IDs
  - applying default values and null policy
- Move backend choice to:
  - config (which backend to target)
  - and/or a factory injected into the importer

This lets Arrow backend adoption happen by swapping the `CollectionsFactory` implementation, instead of rewriting Projection’s importer.

## Open questions (to answer before major work)

- Do we want Arrow ingestion to be:
  - **zero-copy** (wrap Arrow arrays directly into Arrow collections), or
  - **copy/materialize** (Vec/Huge), possibly with an Arrow fast-path later?

- Should `GraphStoreFactoryTyped` become the canonical trait, and `GraphStoreFactory` become a type-erased convenience wrapper?

## Looking ahead: CoreGraph + agent-driven factories

You mentioned the long-term shape where agents can call into Projection/Factory directly, and where additional concepts like `GraphFactory` (and a CoreGraph system) will exist.

Current repo status:

- `GraphStoreFactory` / `GraphStoreFactoryTyped` exist today as the public ingestion contract.
- `GraphFactory` / `CoreGraph` do **not** exist yet in this crate (no stubs found as of 2025-12-14).

When those land, this is the key boundary to protect:

- The agent-facing API should depend on a **stable, minimal set of traits** (likely a small facade over `GraphStoreFactoryTyped`) rather than wiring agents to concrete Arrow/Vec/Huge details.
- Collections backends should remain swappable without breaking the agent API.

Suggested shape (conceptual):

- `GraphFactory` (future): a higher-level orchestrator that composes:
  - source adapters (Arrow / Polars / Neo4j, etc.)
  - projection policies (type mapping, default/null policy)
  - backend selection (Vec/Huge/Arrow) via `CollectionsFactory`
  - progress/reporting hooks

Agent-facing considerations (to prevent “make-or-break” churn):

- Prefer **versioned configs** (or tagged schema) for factory inputs.
- Ensure deterministic error surfaces (typed errors; no stringly-typed branching).
- Keep “zero-copy” optimizations as internal strategies; the public contract should describe semantics, not implementation.

This section is intentionally a placeholder to keep the future API in view while we keep Arrow + Collections upgrades mechanical.

---

If we keep this boundary crisp, the Arrow migration becomes mostly mechanical (swap materializers), and Projection/Factory can grow into the larger “gigantic concept” without entangling backend implementation details.
