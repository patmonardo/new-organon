# ADR 0006: World -> ActiveProperty -> PropertyEngine (technical contract)

Status: Proposed
Date: 2025-08-17

## Purpose
Define a precise contract for the World → ActiveProperty → PropertyEngine triad. Specify data shapes, engine interface, invariants, ordering, errors, tests, and code pointers necessary for an implementable engine.

## Context
- ActiveProperty carries property key/value assertions on entities (and optionally on relations if the domain allows) with activation metadata.
- PropertyEngine deterministically upserts/removes properties and can derive secondary properties from drivers and particulars.

## Decision (contract-level)
- Implementers MUST use the following shapes and interface and honor the invariants.

### Data shapes (TypeScript-like)

  interface ActiveProperty {
    id: string;                  // unique property id
    subjectId: string;           // entity id this property belongs to
    key: string;                 // property key/name
    value: unknown;              // property value (domain-specific)
    dtype?: string;              // optional data type descriptor
    particularityOf?: string;    // optional absolute id
    active?: boolean;
    revoked?: boolean;
    confidence?: number;         // 0..1
    weight?: number;
    provenance?: any;
  }

### PropertyEngine interface

  interface PropertyEngine<I = any, O = any> {
    process(
      properties: ActiveProperty[],
      context?: any
    ): Promise<{ actions: O[]; snapshot: I }>;

    commit(actions: O[], snapshot: I): Promise<{ success: boolean; errors?: any[] }>;
  }

### Example actions
  type PropertyActionType = 'upsert-property' | 'remove-property';

  interface PropertyAction {
    type: PropertyActionType;
    subjectId: string;           // entity id
    key: string;
    value?: unknown;             // required on upsert
    sourcePropertyId?: string;   // backpointer to ActiveProperty.id
  }

## Invariants
- `id`, `subjectId`, and `key` are required.
- Ignore properties with `revoked===true` except to emit removal actions.
- `active===true` overrides `confidence` for conflict resolution.
- Clamp `confidence` to [0,1].
- Engines MUST NOT mutate input arrays in-place.
- If `subjectId` entity is not present in engine-visible state, property actions SHOULD be deferred or fail validation based on engine policy (documented in engine README).

## Determinism & ordering
- For same `subjectId` and `key`, prioritize active=true, then weight, then confidence, then deterministic tie-break (timestamp/provenance then id).

## Error semantics
- Validation errors (missing fields, invalid value types per domain) cause `process` to reject.
- Transient failures during `process` cause rejection; no commit should occur.
- `commit` should be atomic with structured errors; retries are allowed at the orchestration layer.

## Tests required
- Validation: missing required fields → error; confidence clamped.
- Happy path: single upsert; removal with `revoked===true`.
- Conflict resolution: multiple competing properties for same key → deterministic action set.
- Idempotent commit test.

## Code pointers (precise)
- Drivers / helpers:
  - `logic/src/absolute/essence/world.ts` — property/world helpers.
  - `logic/src/absolute/essence/types.ts` — `ActiveProperty` type (tighten as needed).
  - `logic/src/absolute/essence/relation.ts` — truth helpers when property derivations rely on relations.
- Engine placement and tests:
  - Implement `logic/src/essence/property/engine.ts` implementing `PropertyEngine`.
  - Tests in `logic/src/essence/property/__tests__/engine.spec.ts`.

## Adoption checklist
1. Implement strict `ActiveProperty` validator (Zod or equivalent).
2. Implement `engine.ts` per `PropertyEngine` interface.
3. Add tests: validation, happy path, conflict resolution, idempotent commit.
4. Document deferred/validation policy for missing `subjectId` entities.

## Consequences
- Establishes a tight property contract enabling reliable integration with Entity and Relation engines via actions.

## References
- ADR 0001, ADR 0002, ADR 0003, ADR 0004, ADR 0005
- `logic/src/absolute/essence/world.ts`, `logic/src/absolute/essence/relation.ts`
