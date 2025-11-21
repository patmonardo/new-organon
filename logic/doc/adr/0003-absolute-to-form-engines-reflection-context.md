
# ADR 0003: Reflection -> ActiveContext -> ContextEngine (technical contract)

Status: Proposed
Date: 2025-08-17

## Purpose
Set a precise, technical contract for the Reflection → ActiveContext → ContextEngine triad. This ADR prescribes data shapes, the `ContextEngine` interface, strict invariants, error modes, determinism requirements, and the minimal test matrix required before implementation.

## Context
- ActiveContext encodes contextual rules, scoping, and activation metadata that engines use to view particulars under a named context.
- ContextEngine consumes ActiveContext and particulars (Entities/Properties/Relations), produces deterministic intermediate reflections and emits Actions/Transactions for commit.

## Decision (contract-level)
- Define `ActiveContext` shape and `ContextEngine` interface; require engines to conform to these signatures and constraints.

### Data shapes (TypeScript-like)

  interface ActiveContext {
    id: string;
    name?: string;
    particularityOf?: string;
    active?: boolean;
    revoked?: boolean;
    confidence?: number; // 0..1
    weight?: number;
    scope?: { world?: string[]; ids?: string[] } | string; // optional scoping
    rules?: any; // optional rule-set reference or DSL
    provenance?: any;
  }

### ContextEngine interface

  interface ContextEngine<I = any, O = any> {
    process(
      contexts: ActiveContext[],
      particulars: any[],
      snapshot?: any
    ): Promise<{ actions: O[]; snapshot: I }>;

    commit(actions: O[], snapshot: I): Promise<{ success: boolean; errors?: any[] }>;
  }

## Invariants and runtime rules
- Ignore contexts with `revoked===true`.
- Respect `scope` when selecting relevant particulars; if absent, the engine should operate globally over provided particulars.
- Active contexts (`active===true`) take precedence over non-active contexts when conflicts arise.
- Engines MUST be deterministic given the same inputs (including ordering rules and rule evaluation results).

## Error semantics
- Malformed `ActiveContext` objects cause `process` to reject with a validation error.
- Engine `process` may reject for transient failures; no commit should be attempted if `process` rejects.
- `commit` must communicate atomic success/failure with structured errors on failure.

## Tests required
- Schema validation: missing id or invalid `scope` => validation error.
- Scope enforcement: contexts with restricted `scope` must only affect expected particulars.
- Conflict resolution: multiple contexts with active/weight should produce predictable ordering and actions.
- Idempotent commit test.

## Code pointers (precise)
- Driver sources:
  - `logic/src/absolute/essence/reflect.ts` — ActiveContext exports and driver helpers.
  - `logic/src/absolute/essence/types.ts` — tighten `ActiveContext` type here.
  - `logic/src/absolute/essence/relation.ts` — use relation grounding helpers when contexts intersect with relations.
- Engine placement:
  - Implement `logic/src/essence/context/engine.ts` exporting a class that implements `ContextEngine`.
  - Tests: `logic/src/essence/context/__tests__/engine.spec.ts`.

## Adoption checklist
1. Add a strict `ActiveContext` validator (Zod or equivalent).
2. Implement `logic/src/essence/context/engine.ts` per `ContextEngine` interface.
3. Add unit & integration tests listed above.
4. Document scope semantics and any non-default ordering in engine README.

## Consequences
- A compact, testable contract for ContextEngine makes it straightforward to implement and integrate with Shape/Morph engines through Actions/Transactions.

## Next steps
- Implement the `ActiveContext` Zod schema and scaffold the initial `ContextEngine` with minimal Kriya steps.

## References
- ADR 0001, ADR 0002, ADR 0004, and `logic/src/absolute/essence/relation.ts`
