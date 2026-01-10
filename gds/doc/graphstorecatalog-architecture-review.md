**GraphStoreCatalog — Architectural Review**

**Overview:**

- **Purpose:** Concise review of the `graphstorecatalog` top-level Java API: design goals, current surface, and migration considerations toward a declarative JSX-like representation.
- **Scope:** Focus on correctness, completeness, and compatibility with the Java GDS codebase and its Procedure-First controller pattern.

**Correctness:**

- **Input validation:** Java classes must validate inputs early (IDs, label sets, file paths). Strengthen or centralize validation in `PreconditionsService`-style components.
- **Referential integrity:** Ensure stable IDs and cross-references when mutating graph metadata (names, stored properties). Keep invariant enforcement in catalog-level operations.
- **Transactional semantics:** Catalog operations that span storage and metadata (generate, export, drop) need clear atomicity contracts or compensating flows.
- **Consistency checks & validators:** Expand unit and integration checks (e.g., `GraphStoreValidationService`, `MemoryUsageValidator`) and add automated CI tests for round-trip export/import.

**Completeness:**

- **API coverage:** Verify that every user-visible intent (list, project, sample, stream, write, export, drop) is exposed consistently and discoverably across `applications/*` modules.
- **Edge-case handling:** Add explicit behavior for empty graphs, name collisions, partial export failures, and incremental/streaming operations.
- **Observability:** Standardize progress reporting (e.g., `ProgressTrackerFactory`) and error codes so callers can react programmatically.
- **Test surface:** Add tests that exercise combinations (export after mutations, sampling then project) to ensure composability.

**Compatibility with Java GDS (Procedure-First):**

- **API boundaries:** Applications must call `procedures` facades rather than internal `algo` modules. Confirm each `application` entrypoint follows this pattern.
- **Stable surface:** Avoid changing method signatures or JSON formats without an adapter path. Preserve serialization formats used by external tooling.
- **Language-bridge considerations:** If we introduce a TS/JS declarative layer, provide a thin, well-documented adapter that translates the declarative spec into the existing Java procedure calls.

**Recommendations — short term:**

- **Document invariants:** Capture validation rules, ID stability expectations, and transactional guarantees in a dedicated doc (this file + API reference).
- **Increase test coverage:** Add CI tests for exporters/importers, sampling reproducibility, and streaming scenarios.
- **Centralize validation & progress:** Consolidate validation logic into a single service and standardize progress events so new frontend/DSL layers can consume them.

**Recommendations — medium term (JSX-like declarative config):**

- **Canonical schema:** Define a minimal canonical schema for a GraphStore declaration (name, source, transforms, sampling, export). Use this schema as the ground truth for codegen.
- **JSX/DSL surface:** Implement a small TypeScript/JSX wrapper (example: `<GraphStore name="..." source={...} />`) that compiles to the canonical schema and then to the Java procedure invocation.
- **Adapter generator:** Create a generator that emits Java procedure invocations (or a JSON payload the Java side can consume) from the canonical schema. Keep the generator opt-in and backward-compatible.
- **Round-trip & compatibility tests:** Ensure declarative specs can be serialized to/from existing TS-JSON where present, and that Java consumers accept the serialized form.

**Migration path (practical steps):**

- 1. Inventory all `applications/*` Java modules and produce a mapping to high-level responsibilities (list, export, project, write, stream, sample).
- 2. Define a canonical JSON schema for GraphStore declarations and a minimal TypeScript prototype that validates it.
- 3. Implement a Java-side lightweight deserializer that can accept canonical JSON and delegate to existing procedures.
- 4. Prototype a JSX->canonical compiler in TS that outputs the canonical JSON.
- 5. Add compatibility tests and a deprecation schedule for old TS-JSON formats.

**Risks & mitigations:**

- **Breaking API changes:** Mitigate with an adapter layer and feature flags; keep old endpoints until consumers migrate.
- **Behavioral drift:** Ensure tests cover semantics (sampling seeds, export formats) so behavior is preserved across translation layers.
- **Performance impact:** Benchmark adapter and serialization overhead; prefer in-process invocation or lightweight RPC to avoid penalties.

**Next steps (immediate):**

- **This review file was added:** use it as the canonical conversation starter.
- **Inventory Java classes:** produce a companion mapping document that lists files and their primary responsibilities.
- **Prototype canonical schema:** small TS validator and a sample JSX snippet that compiles to canonical JSON.

---

Generated: architectural review for GraphStoreCatalog. If you want, I can now:

- produce the inventory mapping document for `applications/*` Java modules;
- or create the initial canonical JSON schema + a tiny TS prototype for JSX compilation.
