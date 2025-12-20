# Organon

Organon is a TypeScript-first monorepo exploring a practical stack for:

- Encoding and validating a canonical knowledge graph (BEC)
- Building application-facing models and data workflows (MVC)
- Defining task/agent/workflow schemas for orchestration (TAW)

License: GPL-3.0-only (GNU GPLv3). See [LICENSE](LICENSE).

The dialectical language (BEC/MVC/TAW) is used here as an architectural naming scheme: it’s meant to clarify responsibilities and boundaries, not to be philosophy for its own sake.

## Glossary (preferred terms)

- **Application Engine**: runs structured requests (“programs”) against graph state and returns results.
- **Program**: a structured request such as a procedure call, an ML pipeline, or a form program.
- **Planning**: choosing what to run next; assembling config/resources; checking preconditions (no lasting effects).
- **Execution**: effectful running of a program (writes/streams/stats, artifacts, traces).
- **Projection**: a graph context boundary (graph views, filters, materialized properties) used for execution.
- **Projection Planning**: planning at the projection boundary (context-aware program assembly). Historically referred to as “ProjectEval”.

- **Sublingual Kernel (GDS)**: Rust `gds/` as a non-discursive model of “absolute knowing” (lawful activity/constraint/compute), not a linguistic thinker.
- **Discursive Understanding (TS)**: TypeScript user space (`gdsl/`, `logic/`, `model/`, `task/`) as the narration/judgment layer for humans.
- **Eval → Print**: the boundary where sublingual execution is rendered into discursive artifacts (IR/JSON/events/traces) in TS space.

Terminology policy: prefer “Planning/Execution” and avoid Lisp-style “Eval/Apply” in docs and design discussions.

## Repo layout (what lives where)

### TypeScript / PNPM workspace (actively built)

- `gdsl/` — **@organon/gdsl**: shared IR protocol and schema types (Zod)
- `logic/` — **@organon/logic**: canonical graph encodings + validation/seed tooling
- `task/` — **@organon/task**: Task/Agent/Workflow schemas (runtime is intentionally minimal)
- `model/` — **@organon/model**: application-facing schemas + Prisma tooling
- `model/examples/dashboard/` — `dashboard-v4`: Next.js example app wired with its own Prisma schema

### Rust / Cargo workspace (separate from PNPM build)

- `gds/`, `gdsl/` (Rust), `reality/` — performance-oriented kernel crates and experiments

Rust crates are not part of `pnpm -r build` right now (no stable JS binding). Build/test them with Cargo directly.

## Architecture (current intent)

Planning vs Execution split (high-level):

- `logic/` is primarily the Planning substrate (schemas, invariants, validation, agent-facing reasoning tools).
- `task/` is Planning orchestration shapes (Task/Agent/Workflow schemas).
- `model/` is application-facing state + integration surface (schemas, Prisma workflows, example app).
- Rust `gds/` is execution-focused (procedures, pipelines, form processing kernels).

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

The “TAW” layer defines the *shape* of work (not a full runtime yet):

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

# gdsl (TS)
pnpm --filter @organon/gdsl build
pnpm --filter @organon/gdsl test
```

### Dashboard example (Next.js)

```bash
pnpm --filter dashboard-v4 dev
pnpm --filter dashboard-v4 build
pnpm --filter dashboard-v4 test
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
