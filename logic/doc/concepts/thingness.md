# Thingness — Concrete Existence, Appearance, and the Law of Appearance

This document paraphrases Hegel's "Concrete existence" into an operational, modern form for the Relational Form Processor and captures the minimal "Theory of Thing" we will use for the Entity/Property/Relation engines.

## Modern paraphrase (short)
- Concrete existence = essence made immediate. A thing is not a bare atom: it is essence that has passed through mediation and appears in a determinate form.
- Mediation is essential: a thing appears only in relation to others, through determinations (properties) and relations.
- Ground (Morph) is the principle that energizes relations; it remains a fixed principle for a run. Its truth is realized as derived relations and derived properties.

## Core practical takeaways
- Thing ≈ Entity: identity + determinacy (shape/type).
- Property ≈ contextualized determination: a predicate/measure attached to an Entity; always carries provenance (contextId/contextVersion).
- Relation ≈ grounded ties among Entities: ordinary or Essential (constraint-bearing); may derive or enforce properties.
- Morph ≈ ground principle: rules/specs that seed/derive Essential Relations (principles, not mutated during a run).

## The Law of Appearance (operational)
A thing appears only insofar as its determinations are mediated and instantiated under a Context and through Relations. Operational steps:
1. Snapshot principles at run start: Shape, Context, Morph (immutable for the run).
2. Seed Entities from Shapes (Shape → Entity).
3. Contextualize: materialize the Property catalog and attach provenance { contextId, contextVersion }.
4. Ground: apply Morph rules to derive/validate Relations and derive/adjust Properties.
5. Propagate to fixpoint: iterate derivations/backpropagation until no changes or a budget/iteration limit.
6. Contradiction handling: when a Property violates Context determinations, mark it invalid (it "falls to ground") and emit an event or queue a Controller action.
7. Bumping Context.version invalidates properties tied to prior versions.

## Minimal schema recommendations (fields to add / verify)
- Property
  - entityId: string
  - key: string
  - value: unknown
  - contextId: string
  - contextVersion: number | string | null
  - status: "valid" | "invalid" | "derived"
  - provenance?: { source: string; viaRelation?: string }
- Relation
  - sourceId: string
  - targetId: string
  - type: string
  - kind?: "ordinary" | "essential"
  - ruleId?: string
  - directed?: boolean
- Entity
  - id: string
  - shapeRef?: string
  - facets?: Record<string, unknown>
- Morph
  - id: string
  - ruleSpec?: unknown
  - description?: string
  - stable?: true

## Ground stage algorithm (sketch)
- Input: Entities, Properties (with provenance), Morphs (rules), Context (version).
- Working sets: relations[], properties[].
- Iterate up to maxIters:
  - For each Morph rule:
    - Match its preconditions against working Entities/Properties.
    - Compute derived relations and derived properties.
    - Apply derived updates to working sets with provenance and contextVersion.
  - Stop when no new changes or when iteration/budget exhausted.
- Emit derived relations/properties, provenance, and events for invalidations/contradictions.

## Tests to add
- Property provenance & invalidation: create property under context v1 → bump context → property becomes invalid.
- EssentialRelation derivation: given property A on e1 and a Morph rule, derive R(e1,e2) and then derive property B on e2.
- Contradiction fall-to-ground: create property outside allowed domain → ground stage invalidates it and emits an event.
- Fixpoint boundedness: construct a derivation cycle and assert termination within configured iterations.

## Implementation notes
- Keep Morphs as principles (no mutation during a run).
- Always attach provenance {contextId, contextVersion} to properties and derived relations.
- Repository contract: `get(id) => T | null`, `create(doc)`, `update(id, mutateFn, { expectedRevision? })`, `delete(id)`.
- Ground stage must be pure-ish over a working copy; persist only after a run decides to commit.

## Next steps
- Add `src/absolute/ground.ts` implementing a bounded fixpoint runner (scaffolded).
- Add unit tests for the tests listed above.
- Add small shape/context/morph examples demonstrating a three-step derivation (A → R → B).
