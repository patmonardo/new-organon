# Organon

Organon centers on the **GDS Kernel** (Rust) as the execution substrate, with TypeScript packages providing schemas, planning tools, and application-facing models that integrate with kernel outputs.

The stack inverts a typical “TS-first” framing:

- **GDS Kernel (Rust)** executes procedures, pipelines, and form processing.
- **TypeScript workspace** defines schemas, planning/orchestration, and application-facing models.
- **Eval → Print** is the boundary where kernel execution is rendered into IR/JSON/events/traces in TS space.

License: GPL-3.0-only (GNU GPLv3). See [LICENSE](LICENSE).

The dialectical language (BEC/MVC/TAW) is used here as an architectural naming scheme: it’s meant to clarify responsibilities and boundaries, not to be philosophy for its own sake.

## Glossary (preferred terms)

- **Application Engine**: runs structured requests (“programs”) against graph state and returns results.
- **Program**: a structured request such as a procedure call, an ML pipeline, or a form program.
- **Planning**: choosing what to run next; assembling config/resources; checking preconditions (no lasting effects).
- **Execution**: effectful running of a program (writes/streams/stats, artifacts, traces).
- **Projection**: a graph context boundary (graph views, filters, materialized properties) used for execution.
- **Projection Planning**: planning at the projection boundary (context-aware program assembly). Historically referred to as “ProjectEval”.

- **Sublingual Kernel (GDS)**: Rust `gds/` as the execution engine and constraint layer.
- **Discursive Understanding (TS)**: TypeScript user space (`gdsl/`, `logic/`, `model/`, `task/`) as schemas, planning, and application-facing models.
- **Eval → Print**: the boundary where kernel execution is rendered into discursive artifacts (IR/JSON/events/traces) in TS space.

Terminology policy: prefer “Planning/Execution” and avoid Lisp-style “Eval/Apply” in docs and design discussions.

## Repo layout (what lives where)

### GDS Kernel (Rust / Cargo workspace)

- `gds/`, `reality/` — performance-oriented kernel crates and experiments

Rust crates are not part of `pnpm -r build` right now (no stable JS binding). Build/test them with Cargo directly.

### TypeScript / PNPM workspace (actively built)

- `logic/` — **@organon/logic**: canonical graph encodings + validation/seed tooling
- `model/` — **@organon/model**: application-facing schemas + Prisma tooling
- `task/` — **@organon/task**: Task/Agent/Workflow schemas (runtime is intentionally minimal)
- `model/examples/dashboard/` — `dashboard-v4`: Next.js example app wired with its own Prisma schema

## Architecture (current intent)

**Execution lives in the GDS Kernel; planning/orchestration and app-facing models live in TypeScript.**

Planning vs Execution split (high-level):

- Rust `gds/` is execution-focused (procedures, pipelines, form processing kernels).
- `logic/` is primarily the Planning substrate (schemas, invariants, validation, agent-facing reasoning tools).
- `task/` is Planning orchestration shapes (Task/Agent/Workflow schemas).
- `model/` is application-facing state + integration surface (schemas, Prisma workflows, example app).

### BEC — Logic layer (`logic/`)

The “BEC” layer is where the repo keeps canonical encodings, IDs, and invariants that should be stable over time.

- Graph data is treated as a first-class artifact (IDs matter; referential integrity matters).
- Validation tooling exists to catch broken references and non-reversible transforms.
  - The canonical integrity checker lives at `logic/validate.ts`.

### MVC — App-facing layer (`model/`)

The “MVC” layer is where knowledge becomes something you can ship:

- Zod schemas and TypeScript types intended for application code
- Prisma workflows for database-backed features
- An example Next.js app under `model/examples/dashboard/` that demonstrates the intended layering and integration points

### TAW — Orchestration schema layer (`task/`)

The “TAW” layer defines the _shape_ of work (not a full runtime yet):

- Task — the unit of work
- Agent — an executor with capabilities/health/assignment shape
- Workflow — orchestration shape (steps, dependencies)

Note: the agent runtime is being rebuilt under `task/src/agent/`.

## Development

### Prereqs

- Node.js `>= 20.19`
- pnpm `>= 9`
- (optional) Rust toolchain if you’re working in `gds/`, `reality/`, etc.

### Install

```bash
pnpm install
```

### Build / test (workspace)

```bash
pnpm build
pnpm test
```

Notes:

- `pnpm build` runs `pnpm -r build` across the workspace packages.
- `pnpm test` currently excludes the dashboard example (`dashboard-v4`) at the root level.

### Common per-package commands

```bash
# logic
pnpm --filter @organon/logic build
pnpm --filter @organon/logic test

# task
pnpm --filter @organon/task build
pnpm --filter @organon/task test

# model
pnpm --filter @organon/model build
pnpm --filter @organon/model test
```

### Dashboard example (Next.js)

```bash
cd model/examples/dashboard
pnpm dev
pnpm build
pnpm test
```

### Prisma (model package)

```bash
pnpm --filter @organon/model db:generate
pnpm --filter @organon/model db:push
pnpm --filter @organon/model db:migrate
pnpm --filter @organon/model db:studio
```

### Rust crates (Cargo)

```bash
cargo build -p gds
cargo test -p gds

cargo build -p gdsl
cargo test -p gdsl
```

## Conventions (so the repo stays teachable)

- TypeScript packages are ESM (`"type": "module"`) and build with `tsc` + `tsc-alias`.
- Schema-first: Zod schemas live under `*/src/schema/*` with barrel exports from `*/src/schema/index.ts`.
- Prefer workspace imports (`@organon/logic`, `@organon/model`, etc.) over deep relative imports.
- When changing canonical graph/chunk data, keep IDs stable and preserve referential integrity.

## Docs

- Package docs live under `*/doc/` (especially `logic/doc/` and `gds/doc/`).
- API docs (where configured): `pnpm doc:api`

## License

See `LICENSE`.
