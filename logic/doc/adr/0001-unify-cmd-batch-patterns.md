# ADR 0001 — Unify engine/driver/service interaction patterns (Cmd vs Batch)

Date: 2025-08-20  
Status: accepted

## Context

Recent refactor moved Relation → Aspect and cleaned up schema-first shapes across the repo. Several layers expose overlapping APIs:

- Absolute/essence drivers (batch-oriented, schema-first)
- Relative/form engines (behavior, persistence, events)
- Services (interactive APIs used by higher-level code)
- Tests and older code used mixed event kinds and payload shapes (e.g., `created` vs `create`, `payload.id` vs `payload.shape.core.id`, `ActiveRelation`)

Inconsistent verbs (past tense / dotted subverbs) and mixed command/event protocols increased cognitive load and caused fragile tests and cross-layer leaks.

## Decision

Adopt a consistent, minimal contract across the codebase:

1. Command/event naming
   - Use noun.verb command/event kinds (lowercase noun, camelCase verb).
   - Examples: `shape.create`, `shape.setCore`, `aspect.describe`, `property.setValue`.

2. Two API modes
   - Batch mode (drivers → engines):
     - Drivers are batch-first, schema-first.
     - Drivers convert inputs → canonical Active/Form objects and call engine.process(...)/engine.commit(...).
     - No engine command types leak from drivers.
   - Interactive mode (services → engines):
     - Services are interactive and send Cmd objects to engine.handle(cmd).
     - Engines may expose handle(cmd) for orchestration and immediate events.

3. Payload shape
   - Canonical create/emit payload includes the full persisted doc under `payload.shape`.
   - Engines must include `payload.shape.core.id` for created/created-like responses.
   - Legacy `payload.id` is deprecated; no support for `.id` on payload in new code.

4. Schema-first validation
   - All normalization goes through Zod schema helpers (createX/updateX).
   - Absolute layer provides parseActive* helpers for conversion to Active objects used by drivers.

5. Engine event kinds
   - Engines emit events that mirror command kinds (services expect command-kind events).
   - Engines use noun.verb for emitted kinds (no past tense, no dotted subverbs like `core.set`).

6. Types locality
   - Engine command/event types are module-private to the engine or service that owns them.
   - Drivers expose only engine-agnostic functions (processActive*/commitActive*).

## Rationale

- Clear division of concerns: drivers handle batch ingestion/translation, engines handle behavior/persistence, services orchestrate commands.
- Consistent naming simplifies searching, testing, and onboarding.
- Schema-first ensures invariants remain the single source of truth.
- Localizing Cmd types prevents API leakage and reduces coupling between absolute and relative layers.

## Consequences

- Tests must be updated to expect `noun.verb` event kinds and `payload.shape.core.id`.
- Some legacy helpers (fallbacks for `payload.id`, ActiveRelation types) removed or deprecated.
- Short-term churn in engine emits and services, but long-term clarity and fewer context switches.

## Migration plan

1. Engines
   - Ensure emits mirror Cmd kinds (e.g., `shape.created` → `shape.create`).
   - Include `payload.shape` with `core.id` on create responses.

2. Drivers
   - Convert to batch-style: inputs → parseActive* → engine.process(...)/commit(...).
   - Remove or make private any exported Cmd types.

3. Services
   - Use engine.handle(cmd) for interactive operations; expect command-kind events.

4. Tests
   - Replace legacy verb expectations and payload.id checks.
   - Add small unit tests for parseActive* helpers and engine.handle() behavior.

5. Docs
   - Add this ADR to `/docs/adr/`.
   - Add short README snippets in `absolute/essence` and `relative/form` pointing to the ADR.

## Example conventions (summary)

- Command/event kinds:
  - create/delete/describe/setCore/setState/patchState/setFacets/mergeFacets/setSignature/mergeSignature
  - nouns: shape, entity, context, property, morph, aspect

- Payload shape for create:
  - { shape: { core: { id, type, name, ... }, state: {...}, signature: {...}, facets: {...} } }

## Next steps

- Sweep repo for remaining legacy emits / payload usages and patch (rg patterns suggested in codebase).
- Update a small set of representative tests and run full test suite.
- Document engine handle() contract signatures where appropriate.
