# ADR 0008: World Driver — Concept and Integration

Date: 2025-08-18
Status: Proposed

## Context

The Organon project recently separated the orchestrative, non-transactional core (Absolute) from transactional engines (Essence). ADRs 0001–0007 defined the triadic contracts and engine responsibilities. The codebase uses Zod schemas and Active\* bridge objects to carry runtime metadata across engines.

A recurring design need is a single, composable interface that represents the system's notion of a "World" — the set of contexts, entities, relations and drivers that engines can consult or drive updates from. Multiple teams have referred to this as a "World Model" or "World Driver." We need a canonical, minimal decision on what a World Driver is, how it integrates with `absolute/core`, and the guarantees it provides.

## Decision

We will adopt a lightweight "World Driver" concept implemented in `absolute/core` as:

- A typed interface (WorldDriver) that exposes read-only views of the Absolute state (contexts, entities, relations, shapes) and a controlled write API for engine-initiated proposals/commits.
- A default, in-process implementation (MemoryWorldDriver) that wraps the existing Absolute state and provides:
  - Snapshot reads with explicit stable revision tokens.
  - Proposal/commit lifecycle where engines submit a Proposal and receive a CommitResult describing success, conflict, or required resolution steps.
  - Optional pluggable backpropagation hooks that an engine (e.g., RelationEngine) can register with to receive incremental adjustments.
- WorldDriver responsibilities are limited: it is not a full truth engine or reflection layer. It enforces typing, revision checks, and provenance metadata; higher-level conflict resolution algorithms live in engines.

WorldDriver is documented as an ADR and exported from `absolute/core` so other packages can import the interface and the default MemoryWorldDriver.

## Rationale

- Centralizing read/write semantics reduces ad-hoc import patterns and duplicate revision/provenance logic across engines.
- Keeping the driver in `absolute/core` maintains the separation between non-transactional absolute state and transactional essence engines while providing a stable integration point.
- A pluggable proposal/commit lifecycle preserves engine flexibility (sync, async, batch) without hardcoding commit semantics into every engine.
- A default in-memory implementation enables fast local development and test harnesses; specialized drivers (DB-backed, remote, distributed) can implement the same interface for production.

## Consequences

Positive:

- Engines have a single, discoverable API for world reads and commits.
- Reduced duplication of revision and provenance checks.
- Easier to write integration and stress tests using MemoryWorldDriver.

Negative / Open:

- The ADR intentionally leaves conflict-resolution algorithms to engines; this means inconsistent policies may appear until a canonical set of helpers is introduced.
- Distribution and strong-consistency semantics are deferred to driver implementations beyond MemoryWorldDriver.

## Migration Plan

1. Add `WorldDriver` interface and `MemoryWorldDriver` implementation in `absolute/core`.
2. Export the interface from the `absolute` barrel so engines can import it from `@/logic/src/absolute` (or equivalent barrel path).
3. Gradually adapt one engine (RelationEngine) to use WorldDriver for reads and commits as a pilot.
4. Add a short dev guide and example under `logic/docs/examples/world-driver.md` showing the Proposal → Commit flow and backpropagation hook.

## Proposal / Commit API (sketch)

Types (informal):

- Proposal {
  id: string;
  actor: string; // engine id
  patch: Partial<AbsoluteChangePayload>;
  baseRevision?: number; // optional precondition
  meta?: Record<string, unknown>;
  }

- CommitResult { success: boolean; revision: number; conflicts?: Conflict[]; errors?: string[] }

- BackpropagationHook = (changes: ChangeSummary) => void | Promise<void>

The MemoryWorldDriver will implement simple optimistic concurrency: if baseRevision is provided and does not match the current revision, return a Conflict result.

## Tests

- Unit tests for MemoryWorldDriver: snapshot reads, optimistic commit success/failure, provenance propagation, hook invocation.
- Integration test: RelationEngine pilot uses driver to commit a relation, then backpropagates a weight update via registered hook.
- Stress test: apply many concurrent proposals against MemoryWorldDriver and assert deterministic conflict reporting.

## Example (high-level)

1. RelationEngine reads snapshot via worldDriver.readSnapshot().
2. RelationEngine computes a new relation and submits a Proposal with baseRevision set to the snapshot.
3. WorldDriver attempts commit; on success, it returns new revision and triggers registered backprop hooks.
4. RelationEngine may re-run or adapt on Conflict responses.

## Alternatives considered

- No driver (engines read Absolute directly): rejected for duplication and inconsistent revision handling.
- Make the driver authoritative and implement conflict resolution centrally: deferred to later, prefer keeping engines responsible for now.

## Open questions

- Should the driver provide first-class support for transactional batches across multiple engines?
- Should backpropagation include termination guarantees (damping, max iterations) or leave that to engines?
- Do we want a canonical conflict-resolution helper library in `absolute` or a separate `logic/src/engine-utils` package?

## Next steps

- If you approve this ADR, I will:
  - add the `WorldDriver` TypeScript interface and a `MemoryWorldDriver` prototype in `absolute/core` with unit tests,
  - update the RelationEngine to use the driver as a pilot,
  - add a short `logic/docs/examples/world-driver.md` demo.

---

Notes: this ADR keeps the World Driver intentionally small and pragmatic: a stable integration point that protects invariants and reduces duplication without prematurely imposing strong distributional guarantees.
