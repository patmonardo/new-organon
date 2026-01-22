# Organon monorepo (TypeScript-first)

## Big picture

- **Dialectical cube layers**:
  - **BEC**: `logic/` (@organon/logic) — canonical knowledge-graph encoding + schemas.
  - **MVC**: `model/` (@organon/model) — Next.js + Prisma + UI + “SDSL/Malloy” modeling docs.
  - **TAW**: `task/` (@organon/task) — Task/Agent/Workflow orchestration (schema-first, framework-agnostic).
- Rust crates exist under `gds/`, `gdsl/`, `reality/` (Cargo workspace), but **they are not part of the PNPM workspace build** right now (no JS/Rust binding yet). Don’t add scripts that implicitly build NAPI.

## Codegen boundaries (read before generating)

- See `.github/codegen-boundaries.md` for the intended split between **GDSL/SDSL (TS user space)** and **GDS (Rust kernel)**.

## ARCHITECTURAL IMPERATIVE - READ THIS FIRST

**APPLICATIONS/EXAMPLES:** **NEVER EVER EVER** call into any `::algo::` modules.
**APPLICATIONS/EXAMPLES:** **ONLY ONLY ONLY** call into `::procedures::` modules.

Procedures are allowed to call into `::algo::{storage,computation}` as part of the required controller pattern.

## Procedure-First Controller Pattern (Required)

- **Applications talk only to procedures.** Do not call `::algo::` modules from applications.
- A **procedure** is the top-level compute entrypoint. It:
  - Validates and normalizes inputs.
  - Creates both the **storage runtime** (controller) and **computation runtime** (ephemeral state).
  - Calls `storage.compute_{algo}(...)` as the **single top-level driver**.
- The **storage runtime** orchestrates the algorithm loop, graph access, concurrency control, progress tracking, and delegates state operations to the computation runtime.
- The **computation runtime** is pure state management (no graph access).

## Repo workflows (do this)

- Install: `pnpm install`
- Build all TS packages: `pnpm -r build`
- Test all TS packages: `pnpm -r test`
- Run a single package:
  - `pnpm --filter @organon/logic test`
  - `pnpm --filter @organon/model build`
  - `pnpm --filter @organon/task test`

## Package conventions (important)

- All TS packages are **ESM** (`"type": "module"`) and built with `tsc` + `tsc-alias`.
- **Schema-first** style:
  - Zod schemas live under `*/src/schema/*` and are exported via a barrel `*/src/schema/index.ts` (see `task/src/schema/*`).
  - Prefer **workspace imports** across packages (e.g. `@organon/logic`, `@organon/model`) instead of deep relative paths.

## Logic package specifics

- `logic/validate.ts` is the canonical integrity/reversibility checker (see `logic/README.md`). When changing chunk/operation data, keep IDs stable and maintain referential integrity.

## Model package specifics

- Prisma workflows live in `model/` scripts (`db:generate`, `db:push`, `db:migrate`, `db:studio`).
- The dashboard exemplar `model/examples/dashboard/` demonstrates the intended MVC layering:
  - routing: `app/(controller)`
  - orchestration: `app/controller`
  - presentation logic: `app/view`
  - UI components: `app/graphics`
  - data access: `app/model` + `app/data/*`

## Rust crates (current stance)

- Treat Rust as **separate**: build/test with Cargo directly when needed (`cargo build -p gds`, etc.).
- Avoid editing auto-generated loader stubs (e.g. `gds/index.js`), and don’t introduce new JS APIs that assume `.node` bindings exist.

## Recent refactor goals and implementation notes

These are practical, repo-specific conventions and concerns introduced during a sweep to reduce verbose fully-qualified Rust paths and improve module hygiene. They are conservative and intended to keep public APIs stable while cleaning call-sites.

- **Replace inline `crate::...` expressions with local `use` imports**: prefer importing types at the top of a module rather than using fully-qualified paths in expressions and signatures (except in `use` statements themselves). This reduces visual noise and improves onboarding.
- **Minimal preludes only**: where a small submodule (e.g., `applications::algorithms`) benefits, add a tiny `prelude` that re-exports a small, stable set of helpers. Avoid exporting core or unstable types via preludes — import those explicitly (notably `GraphResources`).
- **Explicit import for core types**: keep central/unstable types (e.g., `GraphResources`, `LogLevel` enums if ambiguous) imported explicitly via `use crate::...` rather than re-exporting through a prelude. This prevents accidental API surface expansion and clarifies where types come from.
- **Keep implementation internals private**: do not re-export internal components (rayon pools, low-level storage) publicly; provide a small, controlled facade instead.
- **Move test-only imports into `#[cfg(test)]` modules**: to avoid unused-import warnings and accidental test-only dependencies leaking into library builds.
- **Resolve name collisions explicitly**: alias imports when two modules define similarly-named types (e.g., `LogLevel` from different modules) to avoid ambiguity.
- **Small, incremental batches with validation**: apply edits in small groups, run `cargo check -p gds` between batches, and run `cargo test -p gds` before broad commits. Commit with focused messages.

Known issues and deferred work

- `LogError` and `AlgorithmError` placement: noted as misplaced — will address separately to avoid coupling during the current sweep.
- `gds/src/algo` and `procedures` are large; defer full refactor of those directories to focused, follow-up batches.
- Keep the prelude surface minimal and review any new re-exports in PRs.

Suggested workflow for future cleanups

- Identify 5–10 files per batch, update imports to local `use` where appropriate, run `cargo check -p gds`.
- Fix warnings (unused imports, name clashes) as they arise; move test imports into `#[cfg(test)]` blocks.
- Run `cargo test -p gds` after a full module sweep (e.g., `ml`, `pregel`, `projection/eval`).
- Open a focused PR per module with the changes and CI run attached; keep changes small for easy review.
