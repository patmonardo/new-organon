# Form Shape Layering & Naming Plan

Goal: keep schema as a pure spec, runtime class for behavior, repo for persistence, engine for orchestration. Reduce name collisions and remove CRUD/document helpers from the schema.

## Layering Rules

- **Schema (Zod):** `FormShapeSchema` + `FormShapeRecord` type only. No CRUD, no state mutation, no doc helpers.
- **Runtime class:** `FormShape` in logic/src/relative/form/shape/shape-form.ts; owns behavior, state/facets/signature helpers, `fromRecord`/`toRecord`.
- **Repository:** `FormShapeRepository` in logic/src/repository/form.ts; persists/loads validated `FormShapeRecord` only.
- **Engine/Service:** logic/src/relative/form/shape/shape-engine.ts; orchestrates commands, converts class↔record, emits events.
- **Barrels:** Export schemas and runtime classes from distinct namespaces to avoid `FormShape` collisions.

## Naming Scheme

- Schema type: `FormShapeRecord = z.infer<typeof FormShapeSchema>`
- Runtime class: `FormShape` (class)
- Repo DTO: same as `FormShapeRecord`
- Keep Kernel schemas distinct (`KernelFormShape`) and avoid exporting them in the same barrel as UI FormShape.

## Tasks (ordered)

1. **Decouple schema from CRUD:** Remove `createForm`, `updateForm`, `getFormShape`, `setFormShape` from logic/src/schema/form.ts; relocate equivalents into runtime/service utilities.
2. **Runtime conversions:** In logic/src/relative/form/shape/shape-form.ts, add `static fromRecord(rec: FormShapeRecord)` and `toRecord(): FormShapeRecord`; ensure all mutators operate on class state only.
3. **Repository typing:** Update logic/src/repository/form.ts signatures to accept/return `FormShapeRecord`; validate on write and after reconstruction.
4. **Engine wiring:** In logic/src/relative/form/shape/shape-engine.ts, use `FormShapeRecord` for repo interactions; conversions go through `FormShape.fromRecord` / `.toRecord()`.
5. **Barrel hygiene:** In logic/src/index.ts (and schema barrels if needed), avoid exporting runtime `FormShape` alongside schema `FormShapeRecord`; prefer explicit namespaces.
6. **Model/example dedupe:** Point model/example consumers to the canonical `FormShapeSchema`/`FormShapeRecord` or rename their copies to avoid clashes (e.g., `ExampleFormShapeSchema`).
7. **Tests:** Add round-trip tests (schema ↔ class ↔ repo) and repo reconstruction validation (Neo4j) to prevent regressions.

## Risks / Notes

- Barrel changes may be breaking; adjust imports in dependents.
- Neo4j reconstruction must re-parse with Zod to catch drift.
- Model/example packages may have divergent schemas; decide to alias or rename.
