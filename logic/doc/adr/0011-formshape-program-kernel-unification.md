# ADR 0011 — FormShape Program-Kernel Unification (JSON First)

Status: Proposed
Date: 2026-01-12

## Context

- Logic absolute/core + form already model graph semantics and user-facing schemas; GDS kernel exposes a minimal FormShape and wants Program features (GraphStore + ML procedures) with a richer schema.
- Current gap: Rust FormShape is sparse (Shape/Context/Morph only); ApplicationForm, GraphOperation, ML/hybrid routing, and kernel execute API are only described in docs. TS logic schemas act as “language” definitions but are not aligned with the kernel structs.
- Architectural guardrails: applications/examples must call procedures, not `::algo::`; TS is orchestration/IR, Rust is compute; PNPM build must stay Rust-free; transport must be JSON first (process boundary), NAPI later.
- Need a single, versioned IR that covers: graph shape, control/ops, GraphStore access, ML procedure specs, execution modes (stream/stats/mutate/write/estimate), and observability/validation hooks.

## Decision

- Adopt FormShape as the unified wire protocol and wrap Program features via an ApplicationForm envelope that carries GraphOperation variants (procedure, ML, hybrid, GDSL). The kernel accepts and returns FormShape (or ShapeStream) over JSON.
- Version the IR: `formshape.version: string` (e.g., `fs-0.1.0`) and `program.version: string`; reject or down-level when incompatible.
- Define the schema in TS (Zod) under `logic/src/schema/formshape` and mirror in Rust under `gds/src/form`. TS is the canonical authoring/validation surface; Rust is the canonical executor.
- Keep controller pattern: applications -> procedures -> storage runtime -> computation runtime. Only procedures call `::algo::`; applications/examples never do.
- Modes remain explicit (`stream|stats|mutate|write|estimate`) and are routed through `ApplicationsFacade`/`GraphCatalogApplications` in the kernel.
- Transport is JSON over a process boundary for now; a NAPI/FFI path may be added later without changing the IR.

## FormShape / Program envelope (minimum contract)

- `FormShape`: `id`, `kind`, `shape`, `context`, `morph`, optional `content`, `provenance`, `constraints`, `metrics`, `controls`, `version`.
- `Program` (ApplicationForm): `id`, `version`, `graph`: GraphStore handle or inlined snapshot ref, `ops: GraphOperation[]`, `controls` (mode, retries, timeout, compensate), `inputs`, `bindings`, `otlp` hooks, `return: FormShape | ShapeStream` selector.
- `GraphOperation` variants: `procedure { name, args, graphRef? }`, `ml { model, task, features, target, graphRef? }`, `hybrid { plan[], graphRef? }`, `gdsl { text|ast, graphRef? }` (gdsl compiles to procedure/ml plans before execute).
- `ShapeStream`: streaming FormShape updates plus progress/metrics envelope.

## Execution contract (kernel)

- API: `execute(formshape_or_program: FormShape | ApplicationForm) -> FormShape | ShapeStream`.
- Steps: validate (version + schema) -> normalize -> build FormSpec -> route ops via storage runtime -> call computation runtime -> emit FormShape/ShapeStream; enforce memory/progress guards; surface OTEL spans.
- GraphStore is the “Given”; ops declare which graph handle they touch. ML/hybrid ops must run under the same controller (no direct `algo` calls from applications).

## TS authoring/validation

- Provide Zod schemas + helpers to build/validate FormShape and ApplicationForm; ensure reversible JSON. Add round-trip tests against Rust fixtures.
- Expose a TS client (logic absolute facade) that posts JSON to kernel and consumes FormShape/ShapeStream responses.

## Observability and safety

- OTEL: span per execute call; attributes for version, mode, graph handle, op kinds; events for progress; errors surfaced with structured codes.
- Validation: strict ids, version checks, mode whitelist, graph handle presence per op, retry/timeout bounds, optional compensations.

## Out of scope (for this ADR)

- Transport implementation beyond JSON (NAPI/FFI later).
- Specific ML model zoo or algo catalog; only the envelope is defined here.
- Full GraphStore schema redesign; reuse existing handles/refs.

## Risks and mitigations

- Version skew between TS and Rust: mitigate with explicit `version` and compatibility checks.
- Over-minimal Rust structs: extend in place with backward-compatible fields; gate required fields via version.
- Controller gaps (progress/memory guards): must be completed in `gds/src/applications` before exposing streaming modes.

## Adoption checklist

1. Add TS Zod schema for FormShape + ApplicationForm under `logic/src/schema/formshape` and export via barrel; add validators and builders.
2. Mirror structs + serde in `gds/src/form` with version checks; add JSON fixtures for round-trip tests.
3. Implement `execute(FormShape|ApplicationForm)` path in kernel with validation + controller routing; wire OTEL.
4. Update `ApplicationsFacade`/`GraphCatalogApplications` to accept the new envelope, honor modes, and forbid direct `::algo::` from apps/examples.
5. Add integration tests: TS builds FormShape/ApplicationForm -> JSON -> Rust execute -> FormShape/ShapeStream; include stream/mutate/write/estimate cases.
6. Document usage in `gds/doc` and `logic/README.md`; note JSON-first transport and procedure-first enforcement.

## References

- Existing ADRs in `logic/doc/adr/`
- Kernel form docs in `gds/doc` (Form\*, ApplicationForm, GraphOperation)
- Controller pattern notes in `.github/copilot-instructions.md` and `AGENTS.md`
