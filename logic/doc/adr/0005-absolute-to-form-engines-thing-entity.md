# ADR 0005: Thing -> ActiveEntity -> EntityEngine (technical contract)

Status: Proposed
Date: 2025-08-17

## Purpose
Define a precise, implementable contract for the Contained triad: Thing → ActiveEntity → EntityEngine. Specify data shapes, engine interface, invariants, error semantics, determinism/ordering, required tests, and exact code pointers.

## Context
- ActiveEntity is the canonical carrier for entity state used by engines; it is consumed transactionally by EntityEngine to create/update/remove entities and to attach/detach properties and relations.
- EntityEngine is responsible for deterministic synthesis of entity actions from ActiveEntity plus relevant particulars (properties, relations) and for idempotent commits.

## Decision (contract-level)
- Engines MUST implement the shapes and interfaces below and honor the invariants and error semantics.

### Data shapes (TypeScript-like)

  interface ActiveEntity {
    id: string;                    // required stable id
    entityType: string;            // required type/classifier
    particularityOf?: string;      // optional absolute container id (Thing)
    labels?: string[];             // optional tags/labels
    active?: boolean;              // explicit activation
    revoked?: boolean;             // explicit negation
    confidence?: number;           // 0..1 fallback
    weight?: number;               // optional priority bias
    provenance?: any;              // source metadata
  }

  interface EntityParticulars {
    properties?: ActiveProperty[];
    relations?: ActiveRelation[];
  }

### EntityEngine interface

  interface EntityEngine<I = any, O = any> {
    process(
      entities: ActiveEntity[],
      particulars: EntityParticulars,
      context?: any
    ): Promise<{ actions: O[]; snapshot: I }>;

    commit(actions: O[], snapshot: I): Promise<{ success: boolean; errors?: any[] }>;
  }

### Example actions
  type EntityActionType =
    | 'upsert-entity'
    | 'remove-entity'
    | 'attach-property'
    | 'detach-property'
    | 'link-relation'
    | 'unlink-relation';

  interface EntityAction {
    type: EntityActionType;
    targetId?: string;           // entity id for upsert/remove
    subjectId?: string;          // entity id for attach/detach/link/unlink
    relatedId?: string;          // property id or relation id
    payload?: any;               // serialized changes
    sourceEntityId?: string;     // backpointer to ActiveEntity.id
  }

## Invariants
- `id` and `entityType` are required; missing either is a validation error.
- Ignore entities with `revoked===true` except to emit removal/unlink actions.
- `active===true` takes precedence over `confidence` for conflicts.
- Clamp `confidence` to [0,1] when present.
- Input arrays MUST NOT be mutated in-place.
- If `particularityOf` is present, and an absolute Thing container is referenced, it MUST exist in the provided absolute set (if supplied) or be resolvable by the engine; otherwise validation should fail.

## Determinism & ordering
- When multiple ActiveEntity objects target the same `id`, process in this order: active=true, then descending weight, then descending confidence, then deterministic id-based tiebreak (string compare of provenance timestamp if provided, else entity id).

## Error semantics
- Validation errors (missing required fields, id collisions with contradictory states) cause `process` to reject.
- Transient runtime failures during `process` should reject; no commit should occur.
- `commit` must be atomic per-call or explicitly document partial behavior; failures must include structured errors.

## Tests required
- Schema validation: missing `id` or `entityType` => error; `confidence` clamped.
- Happy path: single ActiveEntity (active=true) → upsert-entity action emitted deterministically.
- Attach/detach: with property/relation particulars, engine emits attach/detach actions deterministically.
- Conflict resolution: two ActiveEntity entries for same `id` with different priorities → ordered, predictable result.
- Idempotent commit: committing same actions twice should be no-op on second call (or documented idempotence behavior).

## Code pointers (precise)
- Drivers / helpers:
  - `logic/src/absolute/essence/thing.ts` — Thing/Entity helper exports.
  - `logic/src/absolute/essence/world.ts` — property/world helpers (for property attachments).
  - `logic/src/absolute/essence/relation.ts` — relation helpers for linking/unlinking decisions.
  - `logic/src/absolute/essence/types.ts` — `ActiveEntity`, `ActiveProperty`, `ActiveRelation` types (tighten as needed).

- Engine placement and tests:
  - Implement `logic/src/essence/entity/engine.ts` exposing a class implementing `EntityEngine`.
  - Tests in `logic/src/essence/entity/__tests__/engine.spec.ts` covering the cases listed above.

## Adoption checklist
1. Add strict runtime validation for `ActiveEntity` (Zod or equivalent).
2. Implement `engine.ts` per `EntityEngine` interface.
3. Add unit tests for validation, happy path, conflict resolution, and idempotent commit.
4. Document ordering/tie-break rules in engine README.

## Consequences
- A clear contract enables test-first implementation and predictable integration with Property/Relation engines via Actions/Transactions.

## References
- ADR 0001, ADR 0002, ADR 0003, ADR 0004
- `logic/src/absolute/essence/thing.ts`, `logic/src/absolute/essence/world.ts`, `logic/src/absolute/essence/relation.ts`
