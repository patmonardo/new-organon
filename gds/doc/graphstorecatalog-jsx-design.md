GraphStoreCatalog — JSX-like Declarative Design (Draft)

Purpose

- Capture a concise, implementation-focused design for a JSX/DSL surface that maps to the existing
  `graphstorecatalog` procedure-first Java API. This is a design document (not yet a prototype).

Design Goals

- High-level, composable declarative surface for catalog operations: `list`, `generate`, `sample`,
  `project`, `export`, and `write`.
- Preserve Procedure-First constraints: declarative specs compile to canonical JSON which is
  deserialized and executed by existing Java procedures; do not directly call internal `algo` modules.
- Backward-compatibility: provide an adapter path from existing TS-JSON formats to the canonical schema.
- Deterministic semantics: sampling seeds, projection determinism, and export atomicity should be explicit.

Constraints & Non-Goals

- Non-goal: Replace Java procedure implementations. The DSL is a translation/adapter layer only.
- Non-goal: Full Cypher compiler — cypher snippets remain opaque strings passed through to Java.

Canonical Schema (high-level)

- A small, serializable JSON schema is the canonical contract between DSL and Java.
- Minimal fields:
  - `op`: one of `list`, `generate`, `sample`, `project`, `export`, `write`, `drop`.
  - `target`: `{ database: string, graphName?: string }` (target location / result name).
  - `source`: optional source spec `{ type: 'catalog'|'database'|'snapshot', graphName?: string, uri?: string }`.
  - `config`: operation-specific config object (sampling, projection, export options).
  - `meta`: optional metadata: `{ seed?: number, runId?: string, description?: string }`.

Example canonical JSON (sample op)
{
"op": "sample",
"target": { "database": "default", "graphName": "friends_sampled" },
"source": { "type": "catalog", "graphName": "friends" },
"config": { "method": "induced", "sampleRatio": 0.1, "seed": 42 }
}

JSX / Component Model (high level)

- Components represent operations and are thin wrappers that produce canonical JSON.
- Suggested components:
  - `<GraphStore name="..." database="default">...children...</GraphStore>` — root composition.
  - `<Source type="catalog" graph="friends"/>`
  - `<Generate name="gen1" nodeCount={100} nodeLabels={["Person"]} />`
  - `<Sample from="friends" name="friends_sampled" method="induced" ratio={0.1} seed={42} />`
  - `<Project from="X" name="X_prj" nodeFilter="..." relationshipFilter="..." />`
  - `<Export to="csv" path="/tmp/out.csv" />`

TypeScript Prototype Surface

- API contract: `compileGraphStore(componentTree) -> CanonicalJSON[]`.
- Validation: lightweight schema validation (zod or similar) produces helpful developer errors.
- Output: array of canonical JSON operations (one component can emit multiple ops).

Adapter Design (JS -> JSON -> Java)

- Steps:
  1. JSX/TSX components compile to canonical JSON via `compileGraphStore`.
  2. JS runtime sends canonical JSON to Java adapter endpoint (HTTP/IPC/TSJSON/N-API) or writes to file.
  3. Java-side deserializer maps canonical JSON to existing `GraphCatalogApplications` facade calls.
- Error-handling: canonical JSON must include `meta.runId` and `meta.seed`; Java returns structured errors.

Migration Strategy

- Phase 0: Design doc (this file) and canonical schema.
- Phase 1: Implement TS-only compiler and validator; produce JSON artifacts for several example workflows.
- Phase 2: Add Java-side deserializer that accepts canonical JSON and delegates to current procedures.
- Phase 3: Provide a small `@organon/graphstore-jsx` package with `compile` and `cli` utilities.
- Phase 4: Migrate consumers (dashboards, scripts) by adding an adapter and compatibility tests.

Compatibility & Testing

- Round-trip tests: canonical JSON -> Java -> result -> canonical JSON (metadata preserved).
- Semantic tests: sampling with seed must reproduce node sets across runs.
- Backwards-compatibility: transform old TS-JSON to canonical JSON and run existing integration tests.

Risks & Mitigations

- Risk: semantic mismatch between DSL and Java procedures. Mitigate by starting with conservative ops
  (generate, sample, list) and adding guarded integration tests.
- Risk: performance overhead of adapter. Mitigate by streaming JSON via in-process channel or lightweight RPC.

Next Concrete Steps

- Draft the canonical JSON schema file (JSON Schema or Zod) and a small TS prototype `compileGraphStore`.
- Provide example JSX snippets and compile them to canonical JSON for review.

Questions / Decisions for you

- Prefer `zod` or JSON Schema for TS validation? (I can implement either prototype.)
- Which three operations should we prototype first? I suggest: `generate`, `sample`, `export`.
