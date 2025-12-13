# Relative Runtime Platform (Disabled)

This folder preserves an early “relative” runtime platform that explored a Nest-flavored implementation of Task/Agent/Workflow execution.

Status
- Present for historical/contextual value.
- Currently **disabled** (excluded from the active `@organon/task` SDK build; see `task/tsconfig.json` excludes).
- Do not treat this as the canonical runtime surface.

Why it exists
- These files capture early implementation attempts and naming experiments (“speculative bubble”).
- Keeping them here lets us revisit ideas without coupling the Task schema surface to a specific framework.

Canonical direction (seed-level)
- `task/src/schema/**` is the canonical “TAW concept surface” (vocabulary + Zod schemas).
- Runtime substrate (bus/orchestration) is explored in `@organon/model` as the SDSL runtime substrate.
- The Form Processor remains singular (canonical), while many client/SDK runtimes can exist.

Future linking (Absolute)
- The long-term intent is that TS surfaces (schemas + runtime substrate) can link into the Rust kernel (`gds/`, `gdsl/`, `reality/`) as the performance-critical “Absolute” substrate.
- Until stable bindings exist, treat Rust and TS as separate build/test worlds.
