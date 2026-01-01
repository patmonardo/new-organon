# Repository Guidelines

## Project Structure & Module Organization
- TypeScript workspace lives in `gdsl/` (IR + schemas), `logic/` (canonical graph + validation tooling), `task/` (agent/workflow schemas + runtime sketch), and `model/` (app-facing schemas + Prisma + `model/examples/dashboard` Next.js demo). Barrel schemas sit under `*/src/schema`.
- Rust workspace lives in `gds/` (compute/algorithms) and `reality/` (experiments); Cargo builds are separate from PNPM.
- Helper scripts and utilities sit in `tools/`. Architectural guardrails for TS↔Rust live in `.github/codegen-boundaries.md`.
 - In `gds/`, treat `applications/` → `procedures/` (facades) as the public surface; `algo/` is internal and should only be touched by facades and internal executors (e.g., projection/eval), not by applications or examples.

## Build, Test, and Development Commands
- Install: `pnpm install` (Node 20+, pnpm 9+).
- Workspace build/test: `pnpm build` / `pnpm test` (runs recursively). Dev: `pnpm dev` for parallel package watchers.
- Package-scoped: `pnpm --filter @organon/logic|gdsl|model|task build` (or `test`, `dev`, `start`).
- Dashboard example: `cd model/examples/dashboard && pnpm dev|build|test`.
- Rust: `cargo build -p gds` / `cargo test -p gds` (similarly for `reality` if needed).
- Hygiene: `pnpm lint` (eslint) and `pnpm format` (prettier) before opening a PR.

## Coding Style & Naming Conventions
- EditorConfig enforces LF endings, UTF-8, 2-space indents; Markdown keeps trailing spaces. Prettier uses single quotes and trailing commas.
- ESM everywhere; prefer workspace imports (`@organon/*`) over deep relatives. Keep schemas/data-only types in `*/src/schema` with explicit discriminated unions and stable IDs.
- Keep Rust code compute-focused; TypeScript handles orchestration, narration, and IR. Avoid cross-layer leakage—see `.github/codegen-boundaries.md` when adding adapters.

## Testing Guidelines
- Vitest backs TS packages; aim to cover schema validation, IR round-trips, and adapter contracts. Run targeted suites with `pnpm --filter <pkg> test`.
- Dashboard tests run within its folder. For Rust, add focused unit/prop tests near implementations.
- New features should carry meaningful coverage; document any gaps in PR notes.

## Commit & Pull Request Guidelines
- Commits: concise, present-tense summaries (e.g., “Tighten graph validation”), scoped to a clear change. Avoid WIP noise.
- PRs: include what changed, affected packages, commands run, and linked issues. Call out schema/IR changes and Rust↔TS adapter surfaces. Add screenshots for UI-visible updates.
