# ADR 0001: Absolute Form (core)

Status: Accepted
Date: 2025-08-17

## Context
- We need a clear “Absolute Form” that anchors the system. In our codebase this lives in two places:
  - `logic/src/absolute/core` — core root artifacts: Contracts, Action, Engine (non-transactional Root).
  - `logic/src/absolute/essence` — driver artifacts (Container/Contained drivers). It supplies drivers for two complementary triads used across the system:
    - Container (form/shape) triad: Shape, Context, Morph (how forms are structured semantically)
    - Contained (carrier) triad: Entity, Property, Relation (how content is carried and grounded)
- The Absolute Root must be non-transactional. It validates inputs and dispatches canonical objects; all hard transactional work is performed by engines under `logic/src/essence` (the Essence engines).
- Hegelian alignment: Essence → Reflection → Ground corresponds to Thing, World, Relation and is expressed through Container/Contained drivers that map into the transactional engines under `logic/src/essence`.

## Decision
- Define `absolute/core` and `absolute/essence` as the Absolute Form surface:
  - `absolute/core`: Contracts (canonical schemas such as `ProcessorInputs`), Action orchestration contracts, Action signatures, and the non-transactional root `FormProcessor` API surface.
  - `absolute/essence`: Drivers (Container/Contained drivers) that name and supply semantics for both the Container and Contained triads; these drivers feed the transactional engines located under `logic/src/essence`.
  - The Absolute surface exposes six Active objects that act as the bridge vocabulary and drivers: ActiveShape, ActiveContext, ActiveMorph, ActiveEntity, ActiveProperty, ActiveRelation. Container-side Active objects are non-transactional unities; Contained-side Active objects are the transactional carriers consumed by engines.
  - Action: externalized effects and decisions are emitted by the root but executed transactionally by engines below.
- The Root remains non-transactional. It validates and dispatches; transactional engines (the only true transaction committers) live in `logic/src/essence` and perform property derivations, relation truthing and world assembly.
- The Absolute surface dispatches a precise set of objects: Container drivers (Shape, Context, Morph) and Contained carriers (Entity, Property, Relation), plus content, concepts, judgments, syllogisms.

## Consequences
- Clear separation of concerns: the Absolute (root) orchestrates; Essence engines perform transactional work.
- Stable top-level API via Contracts enables tooling, tests, and cross-package composition.

## Decision (merged from ADR 0002)
- Both Driver names (Shape, Context, Morph) and Carrier names (Entity, Property, Relation) are descriptors within the Absolute/Essence vocabulary: they name aspects of structure and meaning rather than implying a strict left/right ontological split.
- `logic/src/absolute/essence` holds Drivers for both sides of the bridge:
  - Container side: Shape, Context, Morph — corresponding to Essence, Reflection, Ground.
  - Contained side: Entity, Property, Relation — corresponding to Thing, World, Relation.
- `logic/src/essence` contains the transactional Engines. These engines execute processes and perform commits; they realize the semantics declared by Drivers while the vocabulary remains descriptive of meaning (not execution mode).
- Active objects are the explicit bridges between the Container side (non-transactional drivers) and the Contained side (transactional carriers):

  - Container (non-transactional, unity/drivers): ActiveShape, ActiveContext, ActiveMorph — noumenal unities naming the trinitarian moments (begin/middle/end) of form and semantics.
  - Contained (transactional carriers/committers): ActiveEntity, ActiveProperty, ActiveRelation — objects engines under `logic/src/essence` operate on and commit.

  Mapping (driver → active → engine):

  - Absolute(Essence)  -> ActiveShape   -> ShapeEngine
  - Absolute(Reflection) -> ActiveContext -> ContextEngine
  - Absolute(Ground)    -> ActiveMorph   -> MorphEngine
  - Absolute(Thing)     -> ActiveEntity  -> EntityEngine
  - Absolute(World)     -> ActiveProperty-> PropertyEngine
  - Absolute(Relation)  -> ActiveRelation-> RelationEngine

  (we prefer `ActiveRelation` as the canonical carrier name)

## References
- `logic/src/absolute/core/contracts.ts` (ProcessorInputs et al.)
- `logic/src/absolute/core/kriya.ts`
- `logic/src/absolute/core/engine.ts`
- ADR 0002 (Bridge), ADR 0003 (World/Property Law)

### Code pointers (quick jump)
- Absolute core:
  - `logic/src/absolute/core/engine.ts` — FormProcessor / root API surface.
  - `logic/src/absolute/core/contracts.ts` — canonical boundary types (ProcessorInputs, ProcessorSnapshot).
  - `logic/src/absolute/core/kriya.ts` — Kriya pipeline types and options.
- Absolute essence (drivers & helpers):
  - `logic/src/absolute/essence/index.ts` — exports for driver modules.
  - `logic/src/absolute/essence/essence.ts` — Shape/Essence definitions.
  - `logic/src/absolute/essence/reflect.ts` — Reflection drivers.
  - `logic/src/absolute/essence/ground.ts` — Ground/morph drivers.
  - `logic/src/absolute/essence/world.ts` — World/Property helpers.
  - `logic/src/absolute/essence/thing.ts` — Thing/Entity helpers.
  - `logic/src/absolute/essence/relation.ts` — relation helpers: `truthScore`, `isActiveTruth`/`isActiveRelation`, `groundScore`, `chooseCanonicalTruth`.

#### ADR-preferred symbols (code)
- `ActiveRelation` (type alias) — `logic/src/absolute/essence/relation.ts` exports `ActiveRelation` as an ADR-aligned alias.
- `isActiveRelation` — alias for `isActiveTruth` in `relation.ts` to decide truthiness per ADR thresholds.
- `truthScoreForRelation` — alias for `truthScore` returning normalized [0,1] score.
- `assertActiveRelationInvariants` — alias for `assertActiveTruthInvariants` used in dev/test assertions.
- `findActiveRelationsFor` — alias for `findActiveParticularsFor` to enumerate active particulars for an Absolute id.
- Form engines (transactional):
  - `logic/src/essence/` — intended location for ShapeEngine/ContextEngine/MorphEngine and Entity/Property/Relation engines.
  - Example existing modules to inspect and extend: `logic/src/essence/world/law.ts`, `logic/src/essence/reflection/*`, `logic/src/essence/ground/*`, `logic/src/essence/relation/*`.
