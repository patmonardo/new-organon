
# ADR 0002: Essence -> ActiveShape -> ShapeEngine (technical contract)

Status: Proposed
Date: 2025-08-17

## Purpose
Provide a concise, machine-actionable contract for the Essence → ActiveShape → ShapeEngine triad. This ADR specifies data shapes, the ShapeEngine interface, invariants, error semantics, required tests, and exact file/symbol pointers that implementers must follow.

## Context
- ActiveShape is the container-side driver artifact describing a form's canonical shape and minimal semantics; it is produced by the Absolute/essence drivers and consumed by transactional ShapeEngine(s) under `logic/src/essence/shape`.
- ShapeEngine deterministically synthesizes derived properties and actions from ActiveShape plus particulars (Entities/Properties/Relations). ShapeEngine is the authoritative committer for shape-derived transactions.

## Decision (contract-level)
- Define the `ActiveShape` data shape and the `ShapeEngine` interface. Implementations MUST adhere to these signatures and invariants.

### Data shapes (TypeScript-like)

  interface ActiveShape {
    id: string;                    // stable id
    kind?: string;                 // e.g. 'shape'
    name?: string;                 // human label
    schema?: any;                  // optional structural definition (ref or embedded schema)
    particularityOf?: string;      // optional absolute id this shape particularizes
    active?: boolean;
    revoked?: boolean;
    confidence?: number;           // 0..1
    provenance?: any;
    weight?: number;
  }

### ShapeEngine interface

  interface ShapeEngine<I = any, O = any> {
    process(
      shapes: ActiveShape[],
      particulars: any[],    // Entities/Properties/Relations
      context?: any
    ): Promise<{ actions: O[]; snapshot: I }>;

    commit(actions: O[], snapshot: I): Promise<{ success: boolean; errors?: any[] }>;
  }

### Example action
  interface ShapeAction {
    type: string;           // 'derive-property' | 'validate-entity' | ...
    targetId?: string;
    payload?: any;
    sourceShapeId?: string;
  }

## Invariants
- Ignore shapes with `revoked===true`.
- Treat `active===true` as highest-priority and definitive for conflicts.
- Clamp `confidence` to [0,1] when used as fallback.
- Do not mutate input arrays in-place; return new snapshot objects.

## Error semantics
- Input validation failures are fatal to `process` (Promise reject with validation error).
- `process` may reject for transient failures; no commit should occur if `process` rejects.
- `commit` must be atomic per-call or document partial behavior and provide structured errors on failure.

## Determinism and ordering
- When multiple shapes influence the same particulars: process order is active=true first, then descending weight, then confidence, then deterministic id-based tie-break.
- Engines must document any deviation from the ordering above.

## Tests required
- Schema validation tests for `ActiveShape` (missing id/name/schema => failure).
- Happy path: single ActiveShape (active=true) produces deterministic actions.
- Conflict resolution: multiple shapes with mixed active/weight produce ordered actions.
- Idempotent commit test.

## Code pointers (precise)
- Driver sources:
  - `logic/src/absolute/essence/essence.ts` — shape/essence exports.
  - `logic/src/absolute/essence/types.ts` — `ActiveShape` may be imported and tightened.
  - `logic/src/absolute/essence/relation.ts` — use ground/truth helpers when shapes rely on relation-derived evidence.
- Engine placement:
  - Implement `logic/src/essence/shape/engine.ts` exporting a class that implements `ShapeEngine`.
  - Tests: `logic/src/essence/shape/__tests__/engine.spec.ts`.

## Adoption checklist
1. Add a strict `ActiveShape` validator (Zod or equivalent).
2. Implement `logic/src/essence/shape/engine.ts` per `ShapeEngine` interface.
3. Add unit and integration tests listed above.
4. Document ordering/tie-break rules in engine README.

## Consequences
- Provides a narrow, implementable contract so ShapeEngine can be test-driven and integrated with other engines via Actions/Transactions.

## Next steps
- Implement the validator and the initial ShapeEngine with the minimal Kriya steps (seed/contextualize/reflect/ground/action).

## References
- ADR 0001, ADR 0004, `logic/src/absolute/essence/relation.ts`
