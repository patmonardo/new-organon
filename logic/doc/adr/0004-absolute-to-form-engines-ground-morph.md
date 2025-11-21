# ADR 0004: Ground -> ActiveMorph -> MorphEngine (precise contract)

Status: Proposed
Date: 2025-08-17

## Purpose
Provide a focused, technical specification for the Ground → ActiveMorph → MorphEngine triad. This ADR defines the minimal, precise contracts (data shapes and function signatures), runtime invariants, error modes, and verification tests required before any implementation. It does not re-state project process or high-level motivations.

## Context
- Ground is the terminal/aggregative moment: it produces grounded carrier judgments that tie particulars to absolutes and supplies weighting/trace for downstream action.
- An `ActiveMorph` is the canonical carrier that encodes morphic transforms, grounding metadata, and actionable intent that MorphEngine consumes.
- `MorphEngine` is the transactional engine that deterministically applies morphic transforms to particulars (entities, properties, relations) and emits Actions/Transactions for commit.

## Decision (contract-level)
- Define the `ActiveMorph` shape and the `MorphEngine` interface below. Engines MUST implement the interface exactly (inputs/outputs) and uphold the invariants.

### Data shapes (TypeScript-like signatures)
- ActiveMorph (canonical fields required by engines):

  interface ActiveMorph {
    id: string;                       // stable identifier
    kind?: string;                    // optional kind metadata (e.g. 'morph')
    particularityOf?: string;         // absolute id this morph applies to (optional)
    transform: string;                // canonical transform identifier (fn name or DSL)
    params?: Record<string, any>;     // transform parameters
    active?: boolean;                 // explicit activation
    revoked?: boolean;                // explicit negation
    confidence?: number;              // 0..1 fallback
    weight?: number;                  // weight used in ground scoring
    provenance?: any;                 // source metadata
  }

- MorphEngine contract (interface):

  interface MorphEngine<I = any, O = any> {
    // Deterministic processor: pure transformation step (no side-effects).
    process(
      morphs: ActiveMorph[],
      particulars: any[],          // Entities/Properties/Relations consumed
      context?: any                // Optional ActiveContext or snapshot
    ): Promise<{ actions: O[]; snapshot: I }>;

    // Idempotent commit application: apply actions to a transactional store.
    commit(actions: O[], snapshot: I): Promise<{ success: boolean; errors?: any[] }>;
  }

### Example action shape
  interface MorphAction {
    type: string;          // e.g. 'update-property' | 'create-relation'
    targetId?: string;     // entity/property/relation id
    payload?: any;         // action payload
    sourceMorphId?: string;// backpointer to ActiveMorph.id
  }

## Invariants and runtime checks
- Morphs with `revoked===true` MUST be ignored by `process`.
- If `active===true` on a morph, `process` MUST treat it as definitive (highest priority) regardless of `confidence`.
- `confidence` MUST be honored when `active` is absent; `confidence` values outside [0,1] MUST be clamped to the range.
- `weight` is optional; when present it MAY be used by engines to bias aggregation in ground computations.
- Engines MUST NOT mutate the provided `morphs` or `particulars` arrays in-place; they must return new snapshot/objects.

## Error modes and failure semantics
- Validation errors (missing required fields, invalid transform id) are fatal at `process` input and SHOULD result in a rejected Promise with a validation error.
- Transient runtime failures during `process` (e.g., dependency lookup) should return a rejected Promise; the engine may include partial actions if appropriate, but must not commit them.
- `commit` returns success/failure; failed commits MUST include structured errors. Commits must be atomic per-call or explicitly document partial-commit behavior (atomic preferred).

## Edge cases and design choices
- Multiple morphs targeting the same `particularityOf` or same particulars must be ordered by: explicit `active===true` first, then descending `weight` (or `confidence`), then deterministic tiebreak (morph id). Engines MUST document the ordering used.
- Time-based provenance: engines MAY optionally use timestamps in `provenance` for ordering; if used it MUST be part of the deterministic tie-break.
- Non-deterministic transforms are forbidden; `process` must be deterministic given same inputs.

## Tests (minimal suite required before implementation)
- Unit: validate `ActiveMorph` schema
  - missing `id` or `transform` => validation error
  - `confidence` out of range => clamped

- Unit: `process` correctness (happy path)
  - Input: one ActiveMorph (active=true) + particulars fixture
  - Expect: deterministic actions list referencing `sourceMorphId`

- Unit: ordering/priority
  - Input: multiple morphs with mixed `active`, `weight`, `confidence`
  - Expect: morphs processed in the specified priority order; actions reflect priority

- Integration: idempotent commit
  - Run `process` then `commit` twice with same actions and snapshot; second commit should be a no-op or explicitly idempotent.

## Code pointers (precise)
- Drivers / helpers (Ground side):
  - `logic/src/absolute/essence/ground.ts` — ground driver helpers and expected driver exports.
  - `logic/src/absolute/essence/relation.ts` — `groundScore`, `chooseCanonicalTruth` and ActiveRelation helpers used by grounding logic.
  - `logic/src/absolute/essence/types.ts` — `ActiveMorph` and other Active* types (import and tighten as needed).

- Engine placement and suggested files:
  - Create engine: `logic/src/essence/morph/engine.ts` implementing `MorphEngine` interface above.
  - Engine tests: `logic/src/essence/morph/__tests__/engine.spec.ts` (unit + integration tests listed above).

## Adoption checklist (concrete)
1. Implement strict runtime validation for `ActiveMorph` (Zod or equivalent). Failure to validate must block `process`.
2. Implement `logic/src/essence/morph/engine.ts` with `process` and `commit` per interface.
3. Use `relation.ts` `groundScore` where engine aggregates relation-derived weights.
4. Add unit tests and integration idempotence tests; run via package test runner.
5. After implementation, produce a short ADR follow-up describing any non-default ordering or partial-commit behavior.

## Consequences
- A precise contract reduces ambiguity for implementers and allows test-driven implementation of the MorphEngine before deploying side-effecting commits.
- Engines built to this contract are composable with other Form Engines (Shape/Context/Relation) via Actions/Transactions.

## Next steps
- Author the `ActiveMorph` Zod schema and implement the minimal `engine.ts` and tests listed in the adoption checklist.

## References
- ADR 0001 — Absolute Form (drivers and Active* definitions)
- ADR 0002 — ShapeEngine exemplar
- `logic/src/absolute/essence/relation.ts` — ground/truth helpers
