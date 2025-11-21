# ADR 0008 — EssentialRelation (noumenal) vs ActiveAspect (phenomenal)

Status: Accepted
Date: 2025-08-19
Targets: schema, form, drivers, engines, tests

## Context

The project models a philosophical stack where:
- "Essence" (noumenal) names the abstract, ideal relation (Hegelian EssentialRelation).
- "Appearance" (phenomenal) is how that essence shows up in the world and in processing (what the Form layer and Engines operate on).

We must preserve a clear conceptual and technical separation so that:
- EssentialRelation remains the canonical noumenal concept for higher-level reasoning and proofs.
- The runtime system operates on concrete, validated carriers (Active objects) that materialize essences in contexts (World, Morph, Aspect).

## Decision

Introduce and standardize two complementary terms and artifacts:

1. EssentialRelation (noumenal)
   - A conceptual / theory-bearing term used in documentation, ADRs, high-level reasoning modules and any second-order logic representations.
   - Not directly persisted as a runtime document; referenced from runtime artifacts as a canonical pointer (e.g., `particularityOf: "ess:..."`).

2. Aspect / ActiveAspect (phenomenal)
   - `Aspect` — the schema-level type that represents the domain relation as it appears (alias of current Relation schema during migration).
   - `ActiveAspect` — the carrier (runtime active object) that Engines and Drivers accept and emit (alias of current ActiveRelation).
   - Engines, Drivers, Repos, and Tests operate primarily on Aspects/ActiveAspects.

Mapping conventions:
- `particularityOf` (or `essenceId`) links each ActiveAspect to its EssentialRelation.
- Drivers implement a Dialectical (two‑sided) contract: Phenomenal mapping (toActive/fromActive) and optional noumenal interpretation/expressing.
- Morphs are the operators that manifest essences as Aspects: "Morph : Aspect" is the canonical pairing in the Form processor.

## Rationale

- Keeps the philosophical clarity: EssentialRelation stays a noumenal scientific concept (useful for theory, proofs, higher-order transforms).
- Keeps runtime simplicity: Engines and Drivers manipulate concrete validated data (ActiveAspect) and need deterministic behavior for commit/process flows.
- Enables higher-order logic: The system can export aspects to first/second-order predicate forms while keeping core persistence simple.
- Smooth migration: adopt Aspect names via aliases so codebase and tests can be renamed incrementally without breaking behavior.

## Consequences

- Code and tests will converge on `Aspect` / `ActiveAspect` names for runtime behavior.
- EssentialRelation remains available as a conceptual anchor (documentation, ADRs, and higher-level schemas/modules).
- Drivers will be extended/implemented against the DialecticalBase (DialecticalDriver) contract; they can optionally implement noumenal handlers.
- Schema barrels and test helpers will include alias exports so both names work during migration.

## Migration plan (safe, stepwise)

1. Alias-first (non-breaking)
   - Add alias exports in schema: `export const AspectSchema = RelationSchema; export const ActiveAspectSchema = ActiveRelationSchema; export type Aspect = Relation;`
   - Add driver adapter: `src/drivers/aspect/index.ts` that re-exports existing RelationDriver as `AspectDriver`.
   - Add test support aliases (testKit): `makeAspect`, `makeActiveAspect`.

2. Tests & helpers
   - Update support/testKit to provide Aspect-named factories and kits.
   - Update a small set of driver/engine tests to import Aspect symbols and confirm behavior.

3. Rename & remove
   - Once tests and CI are green, optionally rename files and symbols (git mv) to remove Relation* names in a single commit.

4. Noumenal integration
   - Keep EssentialRelation represented in documentation and in a dedicated schema or registry for essences (e.g., `schema/essence.ts` or `meta/essence-registry`).
   - Drivers/Engines may reference `essenceId` / `particularityOf` on ActiveAspect to connect phenomena ↔ noumena.

## Example artifacts & API notes

- Schema alias (example)
  - `src/schema/relation.ts`:
    - export RelationSchema, ActiveRelationSchema (existing)
    - Add: `export const AspectSchema = RelationSchema; export const ActiveAspectSchema = ActiveRelationSchema;`

- Driver adapter
  - `src/drivers/aspect/index.ts`:
    - Re-export existing RelationDriver as AspectDriver to allow progressive import changes.

- DialecticalDriver (core)
  - Drivers implement `toActive`, `toActiveBatch`, `fromActive` and may implement `interpretNoumenal` / `expressNoumenalFromActive`.

- Engine behavior
  - Engines continue to accept arrays of `ActiveAspect` (previously ActiveRelation) via `process(...)` and `commit(...)`.
  - Engines emit Events whose payloads include `essenceId` / `particularityOf` so the noumenal mapping is preserved.

## Testing & validation

- Add unit tests:
  - Driver round-trip: `core -> toActive -> fromActive === core`.
  - Engine idempotency: `process/commit` on ActiveAspect is idempotent.
  - Noumenal hint propagation: `ActiveAspect.meta.noumenal` preserved through driver -> engine -> repo cycles.

- Add logic export tests:
  - `Aspect.toPredicate()` (or equivalent exporter) yields canonical first-order predicate form for reasoning modules.
  - Tests for second-order representation where aspects quantify over aspects.

## Alternatives considered

- Rename Relation → Aspect in one atomic commit
  - Rejected: high churn and risk to CI; opted for alias-first incremental migration.

- Treat EssentialRelation as persisted document
  - Rejected: persistence model grows complexity; better to keep essence as registry / conceptual identifier and derive appearances (Aspects) at runtime.

## Action items

- Add schema aliases (`AspectSchema`, `ActiveAspectSchema`) and types.
- Add `src/drivers/aspect` adapter exporting `AspectDriver`.
- Create/update `logic/test/support/testKit.ts` factories `makeAspect`, `makeActiveAspect`.
- Add a small ADR-linked test demonstrating driver ↔ engine flow using Aspect vocabulary.
- Document EssentialRelation usage in `docs/` and in `schema/essence` (planned).

---

This ADR preserves the noumenal status of EssentialRelation while giving the runtime layer a clear, testable, and ergonomic name (Aspect / ActiveAspect). It enables first/second order logical exports and a migration path that keeps CI green and