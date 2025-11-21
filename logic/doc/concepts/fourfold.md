# Fourfold Architecture — Absolute / Relative / Kriya

Date: 2025-08-20

This short note records the project-level mapping of the philosophical fourfold (form, being, essence, concept) onto the repo layout and runtime patterns. It is an engineering translation of the design intent so future contributors and tests can read the architecture.

## Core idea (one line)
- Two domains: Absolute (noumenal, canonical theory) and Relative (phenomenal, runtime form).  
- Fourfold axes: form · being · essence · concept.  
- Kriya (action) is the orchestration glue: Drivers → Engines → Services → Events.

## Repo mapping
- absolute/ — canonical schemas, drivers, theory-bearing helpers
  - absolute/essence — essence-level drivers (batch/process → commit)
  - absolute/core — driver/engine base types, bus/contract primitives
  - absolute/form — schema-level form helpers and projections
- relative/ — runtime forms, engines, services, in-memory adapters
  - relative/form — engines + services for the six pillars (entity, context, property, morph, aspect, shape)
  - relative/essence, relative/being, relative/concept — runtime concerns and utilities
- schema/ — Zod schemas and create/update helpers (single source of truth)
- repository/ — in-memory and persisted repo adapters used by engines/drivers

## Responsibility summary
- Schema (absolute/schema): validate, normalize, and generate canonical shapes (shape.core.* fields).
- Driver (absolute/essence/*): batch-oriented translators — inputs → Active* → engine.process/commit. Use BaseDriver helpers for payload normalization.
- Engine (relative/form/*/engine.ts): behavior, persistence, and event emission. Emit canonical noun.verb kinds and payload.shape with shape.core.id for creates.
- Service (relative/form/*/service.ts): interactive API used by callers; call engine.handle(cmd) and consume emitted events.
- Kriya (action): the runtime flow — driver batches, engine processes/commits, bus distributes events, services orchestrate.

## Naming & contracts (canonical)
- Event/command kinds: noun.verb (lowercase noun, camelCase verb). Examples:
  - shape.create, entity.describe, aspect.setCore, morph.patchState
- Payload contract:
  - For create-like responses include full doc at `payload.shape` and id at `payload.shape.core.id`.
  - Legacy `payload.id` is deprecated. Drivers normalize legacy payloads to canonical shape.path.
- API modes:
  - Batch (drivers): process(actives) → commit(actions, snapshot) → commitResult (events).
  - Interactive (services): handle({ kind, payload }) → events[].

## Practical patterns & helpers
- Keep schema-first: use createX/updateX helpers to construct normalized shapes before handing to drivers/engines.
- BaseDriver.normalizeEvent / extractIdFromPayload: drivers should normalize engine events so services/tests can rely on `payload.shape.core.id`.
- Tests: expect noun.verb kinds and check `payload.shape.core.id` (or use driver normalization helpers).

## Example flow (create an entity)
1. Caller constructs input → call DefaultThingDriver.toActiveEntity or createEntity helper.
2. Driver.processEntities(...) creates ActiveEntities and calls engine.process(...)
3. Driver.commitEntities(...) calls engine.commit(...) and returns normalized commitResult.events where events contain `payload.shape.core.id`.
4. Service calls engine.handle({ kind: 'entity.create', payload }) for interactive creates and receives events with canonical payloads.

## Next actions
- Keep ADR 0001 (cmd/batch) authoritative for event-kind and payload format decisions.
- Sweep engines and tests to:
  - emit canonical noun.verb kinds,
  - include `payload.shape.core.id` on creates,
  - remove legacy `payload.id` checks (or use driver normalization).
- Add short README snippets in absolute/ and relative/ referencing this doc and ADR 0001.

Notes
- This document is intentionally concise. Put larger policy and examples in ADR 0001 and directory README files.

```// filepath: /home/pat/VSCode/organon/docs/architecture/fourfold.md
# Fourfold Architecture — Absolute / Relative / Kriya

Date: 2025-08-20

This short note records the project-level mapping of the philosophical fourfold (form, being, essence, concept) onto the repo layout and runtime patterns. It is an engineering translation of the design intent so future contributors and tests can read the architecture.

## Core idea (one line)
- Two domains: Absolute (noumenal, canonical theory) and Relative (phenomenal, runtime form).  
- Fourfold axes: form · being · essence · concept.  
- Kriya (action) is the orchestration glue: Drivers → Engines → Services → Events.

## Repo mapping
- absolute/ — canonical schemas, drivers, theory-bearing helpers
  - absolute/essence — essence-level drivers (batch/process → commit)
  - absolute/core — driver/engine base types, bus/contract primitives
  - absolute/form — schema-level form helpers and projections
- relative/ — runtime forms, engines, services, in-memory adapters
  - relative/form — engines + services for the six pillars (entity, context, property, morph, aspect, shape)
  - relative/essence, relative/being, relative/concept — runtime concerns and utilities
- schema/ — Zod schemas and create/update helpers (single source of truth)
- repository/ — in-memory and persisted repo adapters used by engines/drivers

## Responsibility summary
- Schema (absolute/schema): validate, normalize, and generate canonical shapes (shape.core.* fields).
- Driver (absolute/essence/*): batch-oriented translators — inputs → Active* → engine.process/commit. Use BaseDriver helpers for payload normalization.
- Engine (relative/form/*/engine.ts): behavior, persistence, and event emission. Emit canonical noun.verb kinds and payload.shape with shape.core.id for creates.
- Service (relative/form/*/service.ts): interactive API used by callers; call engine.handle(cmd) and consume emitted events.
- Kriya (action): the runtime flow — driver batches, engine processes/commits, bus distributes events, services orchestrate.

## Naming & contracts (canonical)
- Event/command kinds: noun.verb (lowercase noun, camelCase verb). Examples:
  - shape.create, entity.describe, aspect.setCore, morph.patchState
- Payload contract:
  - For create-like responses include full doc at `payload.shape` and id at `payload.shape.core.id`.
  - Legacy `payload.id` is deprecated. Drivers normalize legacy payloads to canonical shape.path.
- API modes:
  - Batch (drivers): process(actives) → commit(actions, snapshot) → commitResult (events).
  - Interactive (services): handle({ kind, payload }) → events[].

## Practical patterns & helpers
- Keep schema-first: use createX/updateX helpers to construct normalized shapes before handing to drivers/engines.
- BaseDriver.normalizeEvent / extractIdFromPayload: drivers should normalize engine events so services/tests can rely on `payload.shape.core.id`.
- Tests: expect noun.verb kinds and check `payload.shape.core.id` (or use driver normalization helpers).

## Example flow (create an entity)
1. Caller constructs input → call DefaultThingDriver.toActiveEntity or createEntity helper.
2. Driver.processEntities(...) creates ActiveEntities and calls engine.process(...)
3. Driver.commitEntities(...) calls engine.commit(...) and returns normalized commitResult.events where events contain `payload.shape.core.id`.
4. Service calls engine.handle({ kind: 'entity.create', payload }) for interactive creates and receives events with canonical payloads.

## Next actions
- Keep ADR 0001 (cmd/batch) authoritative for event-kind and payload format decisions.
- Sweep engines and tests to:
  - emit canonical noun.verb kinds,
  - include `payload.shape.core.id` on creates,
  - remove legacy `payload.id` checks (or use driver normalization).
- Add short README snippets in absolute/ and relative/ referencing this doc and ADR 0001.

Notes
- This document is intentionally concise. Put larger policy and examples in ADR 0001 and directory README