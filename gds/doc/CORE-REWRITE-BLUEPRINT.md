# Core rewrite blueprint (from TS/Java, without the industrial complex)

This note exists to extract the *inevitable* Core substrate from the TS Core translation (and, by extension, Java GDS), while explicitly rejecting the “factory industrial complex” as an implementation style.

## Non-negotiable boundary
- **Projection = Factory → Eval**.
- **Core exists to serve Factory** (and to provide the runtime graph substrate Eval reads).
- **Form ends at a GraphStore**: it returns a new graph object (“apodictic ResultStore”) plus proof. No IO assumptions.

## What TS Core is actually decomposing (the unavoidable nouns)
From the TS Core translation, the main nouns are:

1. **GraphStore** (a product)
   - Owns: schema, node id mapping, relationship topologies, property stores, metadata, capabilities.
   - Provides: `union`/views into typed graphs.

2. **Graph** (a runtime view)
   - Cursor/iterator oriented traversal over adjacency; sometimes optional inverse adjacency.
   - Property access is mostly orthogonal and can be layered.

3. **Catalog / Loading** (a lifecycle service)
   - “Build a GraphStore from config + source(s) + validation hooks”.
   - TS introduces `GraphLoader`, `GraphStoreCatalogService`, builder objects.

4. **Import / Export** (IO, not ontology)
   - TS has explicit exporter patterns (including ID-mapping strategies: mapped vs original).
   - This is a *capability layer* over the substrate.

The ceremony largely comes from expressing each noun as a service graph plus builder patterns plus configuration plumbing.

## The core simplification: collapse services into *product structs + traits*
### Keep
- A small number of *product structs* representing the in-memory substrate.
- A small set of *traits* representing capabilities and the import/serve/export lifecycle.

### Delete (as a Rust design principle)
- Builder objects whose only job is “ensure all fields set”.
- Multi-layer services that exist only to move data between “catalog → builder → store”.
- Type parameter explosions where a single enum/trait object is sufficient.

## Naming in this repo (important)
This repo already has a substantial Rust module at `gds/src/core/` (a Java-faithful translation layer) and it already contains `core/utils`.

For the *new* Core substrate described in this note, prefer a separate top-level module name such as:
- `gds/src/substrate/` (recommended)
- or `gds/src/kernel/`

Hard rule: do **not** introduce a new `*/utils` mega-module inside the new substrate. Keep helpers private to the module that owns them, or promote them to a named type/trait.

## Proposed Rust Core surface (minimal and future-proof)
### 1) Substrate
- `CoreGraphStore` (product struct)
  - Fields (conceptual):
    - `schema`
    - `id_map`
    - `relationship_topologies` (by rel-type)
    - `node_properties` / `rel_properties`
    - `metadata`
    - `capabilities`

- `CoreGraph` (view)
  - Backed by a specific topology store.
  - Cursor-based adjacency iteration.

This is intentionally “boring”: a store and a view.

### 2) Lifecycle (Import / Serve / Export)
Treat IO as a triadic lifecycle over the substrate:

- `Import` (build a store)
  - Inputs: a source descriptor + projection config + concurrency.
  - Output: `CoreGraphStore`.

- `Serve` (hand out stable graph handles)
  - Inputs: store + roots + policy.
  - Output: graph handles / views (read-only by default).

- `Export` (materialize to formats)
  - Inputs: store + export config.
  - Output: bytes/files.

Import/Serve/Export should be *traits with concrete implementations*, not a spiderweb of services.

### 3) Roots + GC (future “Huge” stance)
The thing TS/Java implies but doesn’t fully encode at the type level is the lifecycle reality:

- A store is “huge” when you need:
  - **roots** (what must remain live)
  - **garbage collection / compaction semantics**
  - **handle-based serving** (stable references despite internal moves)

This is the real “Enterprise” substrate, not export code.

## How this maps to the TS translation (for reference)
(Reference only; do not reproduce structure.)

- “Catalog/Loading”
  - `GraphLoader` is essentially: (config, factory) → store → graph view.
  - `GraphStoreCatalogService` adds registry/validation/policy.

- “GraphStore product”
  - `CSRGraphStore` is the central product that aggregates: schema + idmap + relationships + properties + metadata.

- “Graph view”
  - `HugeGraph` demonstrates cursor-first adjacency iteration and optional inverses.

- “Export”
  - `GraphStoreExporter` shows the *real* externalized contract points:
    - ID mapping strategy
    - batching + concurrency
    - node/relationship/property flattening

## Integration with current Rust `gds`
### Short term (now)
- Keep using `DefaultGraphStore` as the Eval substrate.
- Keep Form returning `Arc<DefaultGraphStore>` + proof.

### Medium term (swap substrate)
- Introduce `CoreGraphStore` behind the existing graph/store traits used by Eval.
- Make `DefaultGraphStore` either:
  - an adapter over `CoreGraphStore`, or
  - a “small store” implementation that shares the same trait surface.

### Long term (Huge)
- Add a Huge backend behind `CoreGraphStore` (mmap/arrow/vec/etc.) without changing Eval APIs.

## Guardrails (anti-industrial-complex rules)
- Every new Core type must answer: “is this a noun (data) or a verb (capability)?”
  - Noun → struct/enum.
  - Verb → trait + impl.
- No builders whose only purpose is required-field validation.
  - Use constructors that take the full product, or use `Result`-returning `try_new`.
- No “catalog service” until there is a concrete need for multi-store caching/sharing.
  - Start with a plain function/module that returns a store.

## Next concrete step (if you want)
Create a tiny Rust `substrate/` module with:
- `substrate/store.rs`: `CoreGraphStore` skeleton (schema/idmap/topologies)
- `substrate/graph.rs`: `CoreGraph` view trait + one CSR-backed impl
- `substrate/lifecycle.rs`: `Import/Serve/Export` traits (no implementations yet)

Then wire one evaluator path (Procedure or Form commit) to accept either `DefaultGraphStore` or `CoreGraphStore` through a shared trait.
