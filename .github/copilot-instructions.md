# Organon monorepo (TypeScript-first)

## Big picture
- **Dialectical cube layers**:
  - **BEC**: `logic/` (@organon/logic) — canonical knowledge-graph encoding + schemas.
  - **MVC**: `model/` (@organon/model) — Next.js + Prisma + UI + “SDSL/Malloy” modeling docs.
  - **TAW**: `task/` (@organon/task) — Task/Agent/Workflow orchestration (schema-first, framework-agnostic).
- Rust crates exist under `gds/`, `gdsl/`, `reality/` (Cargo workspace), but **they are not part of the PNPM workspace build** right now (no JS/Rust binding yet). Don’t add scripts that implicitly build NAPI.

## Codegen boundaries (read before generating)
- See `.github/codegen-boundaries.md` for the intended split between **GDSL/SDSL (TS user space)** and **GDS (Rust kernel)**.

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
