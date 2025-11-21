# ADR 0007: Relation -> ActiveRelation -> RelationEngine (technical contract)

Status: Proposed
Date: 2025-08-17

## Purpose
Define the precise contract for Relation → ActiveRelation → RelationEngine. This ADR specifies data shapes, required helpers, engine interface, invariants, canonical truth/ground semantics, ordering rules, tests, and code pointers.

## Context
- ActiveRelation is the canonical carrier for essential relations with activation metadata and confidence. It is already supported by helpers in `relation.ts` (truth score, ground score, canonical selection) and is consumed by a transactional RelationEngine.

## Decision (contract-level)
- Engines MUST implement the interface and semantics below and rely on the canonical helpers for truth and ground evaluation.

### Data shapes (TypeScript-like)

  interface ActiveRelation {
    id: string;                     // relation id (particular)
    kind?: 'relation' | 'essential';// expected by helpers
    particularityOf: string;        // absolute id (container relation)
    source?: { id: string; type?: string };
    target?: { id: string; type?: string };
    type?: string;                  // relation type label
    active?: boolean;               // explicit activation
    revoked?: boolean;              // explicit negation
    confidence?: number;            // 0..1 fallback
    weight?: number;                // aggregation weight
    provenance?: any;               // source metadata
  }

### RelationEngine interface

  interface RelationEngine<I = any, O = any> {
    process(
      relations: ActiveRelation[],
      snapshot?: any
    ): Promise<{ actions: O[]; snapshot: I }>;

    commit(actions: O[], snapshot: I): Promise<{ success: boolean; errors?: any[] }>;
  }

### Example actions
  type RelationActionType = 'upsert-relation' | 'remove-relation';

  interface RelationAction {
    type: RelationActionType;
    relationId: string;
    payload?: any;
    sourceRelationId?: string;  // backpointer to ActiveRelation.id
  }

## Canonical semantics (MUST)
- Truth normalization: use `truthScoreForRelation(r)` (alias of `truthScore`).
- Truth predicate: use `isActiveRelation(r, threshold)`; default threshold is the module default.
- Ground aggregation: when selecting canonical particulars for an absolute container, use `chooseCanonicalTruth(absId, relations)` and `groundScore` as appropriate.

## Invariants
- `particularityOf` MUST reference an existing absolute relation container in the provided set or in the engine’s accessible state; otherwise validation fails.
- `revoked===true` relations MUST be ignored except to emit removal actions.
- `active===true` takes precedence over `confidence`.
- `confidence` must be clamped to [0,1].
- Engines MUST NOT mutate input arrays in-place.

## Determinism & ordering
- For relations under the same `particularityOf`, prefer: explicit `active===true`, then highest `truthScoreForRelation`, then deterministic tie-break by provenance timestamp (if present) then relation id.

## Error semantics
- Validation failures cause `process` to reject.
- Transient failures during `process` cause rejection; do not commit.
- `commit` must be atomic per-call with structured errors; retries allowed at orchestration layer.

## Tests required
- Validation: missing `particularityOf` or invalid kind => error; clamped confidence.
- Truth predicate: boundary tests around threshold and revoked/active flags.
- Ground selection: multiple particulars → verify canonical selection matches helpers.
- Idempotent commit test.

## Code pointers (precise)
- Helpers and drivers:
  - `logic/src/absolute/essence/relation.ts` — `isActiveRelation`, `truthScoreForRelation`, `assertActiveRelationInvariants`, `findActiveRelationsFor`, `groundScore`, `chooseCanonicalTruth`.
  - `logic/src/absolute/essence/types.ts` — `ActiveRelation` type (or import from relation.ts alias if preferred).
- Engine placement and tests:
  - Implement `logic/src/essence/relation/engine.ts` implementing `RelationEngine`.
  - Tests: `logic/src/essence/relation/__tests__/engine.spec.ts`.

## Adoption checklist
1. Enforce runtime validation for ActiveRelation and referenced absolute containers.
2. Implement `engine.ts` with helper-driven truth/ground semantics as above.
3. Add tests for validation, truth predicate, ground selection, idempotent commit.
4. Document any deviations from canonical ordering.

## Consequences
- Using the canonical helper set ensures consistent epistemic semantics across engines and simplifies auditing and testing.

## References
- ADR 0001, ADR 0002, ADR 0003, ADR 0004, ADR 0005
- `logic/src/absolute/essence/relation.ts`
