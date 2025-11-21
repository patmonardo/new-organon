# FormProcessor — seed notes

This folder hosts the FormProcessor: a small, deterministic pipeline that turns Form-level inputs into a World snapshot plus lightweight indexes.

- Name: FormProcessor.
- Scope: Bridge schemas and wrappers under `form/` and `thing/` to produce an assembled `world` and minimal indexes for consumers.

## Contracts

# Absolute — Processor layer

This folder implements the Processor (Absolute) that orchestrates Logic stages
and exposes a stable engine contract. It assembles a `World` snapshot from
`ProcessorInputs`, runs optional reflection and derivation (ground), builds
projections/indexes, computes control plans, and can commit derived results.

Quick guide

- API validation/persistence:
  - Use Zod contracts in `core/contracts.ts` (e.g. `ProcessorInputs`,
    `ProcessorSnapshot`) to validate inputs at the boundary.

- Processor internals:
  - Use runtime-friendly shapes (see `essence/reflect.ts`: `ThingLike`,
    `PropertyLike`) and the canonical `KriyaOptions` in
    `core/kriya.ts`.

Pipeline (where things live)

- assembleWorld — `essence/world.ts`
- projectContentFromContexts — `form/project.ts`
- reflectStage (ThingLike/PropertyLike) — `essence/reflect.ts`
- groundStage (derivation/fixpoint) — `essence/ground.ts`
- orchestrator / runKriya — `core/orchestrator.ts`
- StageFns (model/control/plan) helpers — `core/kriya.ts`
- commitGroundResults (triad) — `essence/ground.ts`

Import guidance (recommended)

- At API boundaries (validators/persistence) import schema types:
  - `import { ProcessorInputs } from './core/contracts';`
  - `import type { Property } from '../../schema/property';`

- Inside processor stages import runtime aliases instead of schema types:
  - `import type { ThingLike, PropertyLike } from './essence/reflect';`
  - `import type { KriyaOptions } from './core/kriya';`

Tests and smoke harness

- Unit tests exist under `logic/test/absolute`.
- A small smoke test `test/absolute/kriya-smoke.spec.ts` demonstrates
  calling `runKriya` with a minimal input — useful when exploring the
  processor locally.

Next steps

1. Add stronger typing for `RuleSpec`/Morphs and remove `any` casts in
   `essence/ground.ts` (medium effort).
2. Add additional smoke fixtures that exercise concept-level drivers
   (`concept/*`) to see dialectical flows end-to-end.

Node/TS notes

- NodeNext is used; TypeScript imports omit extensions in source. Keep
  imports via barrels where convenient to avoid churn.
