# Procedure Facade — Configuration Audit (focused)

Date: 2026-01-25
Scope: Procedure facades (`gds/src/procedures/**`) with attention to where config types originate and duplication between `gds/src/config` and `gds/src/algo/*/spec.rs`.

## Executive summary ✅

- Procedures are the canonical public entrypoints used by Applications; auditing facades first gives the highest ROI.
- I found several _duplicate_ config types living both in `gds/src/config` and `gds/src/algo/*/spec.rs` (examples: **LouvainConfig**, **BetweennessCentralityConfig**, **NodeSimilarityConfig**). This duplication is the main source of confusion.
- Pattern observed:
  - Some facades consume typed `gds/src/config` types (e.g., `PageRankFacade` uses `crate::config::PageRankConfig`).
  - Others consume `algo::*::spec` config structs (e.g., `GraphSage`, `GAT`, `CELF`, `LabelProp`) or keep local builder fields and validate with `ConfigValidator`.
  - Several facades mix approaches (local builder fields _and_ call into `algo::*` runtimes which expect spec configs).

## Methodology 🔎

- Scanned `gds/src/procedures/**/*` for facade types (`pub struct .*Facade`) and looked for occurrences of:
  - `use crate::config::...` (i.e., canonical `gds/src/config` types)
  - `use crate::algo::...::spec::...Config` (algorithm-local spec types)
  - local `config:` fields on the facade structs
- Cross-referenced `gds/src/config/algo_config.rs` with `gds/src/algo/**/spec.rs` to find name collisions.

## High-level findings (concise) 📝

- Procedures that use `gds/src/config` types (example):
  - `PageRankFacade` — `crate::config::PageRankConfig` (procedures/centrality/pagerank.rs)
  - Multiple procedures import `GraphStoreConfig` from `gds/src/config/graph_store_config.rs` (bridges, articulation_points, kspanningtree, random_walk, etc.).

- Procedures that use `algo::*::spec` config types (examples):
  - `CELFFacade` — `crate::algo::celf::spec::CELFConfig`
  - `GraphSage` — `GraphSageConfig` (algo embedding spec)
  - `GAT` — `GATConfig`
  - `LabelPropagation` — `LabelPropConfig`
  - `ApproxMaxKCut` — `ApproxMaxKCutConfig`
  - `NodeSimilarity` (procedures/\*) — `NodeSimilarityConfig` (algo/similarity/spec)

- Procedures that use local builder-style fields (no single typed config):
  - `BetweennessCentralityFacade`, `HitsCentralityFacade`, `DegreeCentralityFacade`, `ClosenessCentralityFacade`, `RandomWalkBuilder`, `AStar`, etc.
  - These typically validate via `procedures::builder_base::ConfigValidator` and pass scalar values into runtime constructors.

- Duplicate config names across `config/` and `algo/*/spec.rs` (candidates):
  - `LouvainConfig` — defined in both `gds/src/config/algo_config.rs` and `gds/src/algo/louvain/spec.rs` (procedures import `crate::algo::louvain::LouvainConfig`).
  - `BetweennessCentralityConfig` — exists under `gds/src/config` and `gds/src/algo/betweenness/spec.rs`.
  - `NodeSimilarityConfig` — canonicalized to the algorithm spec (`gds/src/algo/similarity/node_similarity/spec.rs`). `gds/src/config` re-exports the spec type for backward compatibility.
- `LouvainConfig` — migrated canonically to `gds/src/algo/louvain/spec.rs` and re-exported from `gds/src/config` for compatibility.
- `BetweennessCentralityConfig` — canonicalized to `gds/src/algo/betweenness/spec.rs` and re-exported from `gds/src/config` for compatibility.
  - (There may be more; see "Duplicates" section below.)

## Per-subsystem quick map (recommend you start here) 🗺️

- Centrality (`procedures/centrality`): mostly builder-style facades. Exception: `PageRankFacade` uses a `config::PageRankConfig` instance when building the runtime.
- Community (`procedures/community`): some facades hold a `config` field (e.g., `LouvainFacade::config: LouvainConfig`) but the type currently comes from `algo::louvain::LouvainConfig` (duplicate with `gds/src/config`).
- Similarity (`procedures/similarity`): procedures use `NodeSimilarityConfig` from `algo::similarity::node_similarity::spec` (again duplicate with `gds/src/config` NodeSimilarityConfig).
- Embeddings (`procedures/embeddings`): many facades build a small runtime-local config (e.g., `GraphSageConfig`, `GATConfig`) coming from `algo::embeddings::*::spec`.
- Pathfinding: builder-style (parameters on builder structs), with occasional references to `GraphStoreConfig` for store-level settings.
- Pipelines (`procedures/pipelines`): pipeline-specific small config structs live under `procedures/pipelines/*` (these should be reviewed for centralization if shared).

## Duplicates & collisions (actionable list) ⚠️

(These are high-confidence conflicts found across `gds/src/config` and `gds/src/algo`.)

- `LouvainConfig` — `gds/src/config/algo_config.rs` vs `gds/src/algo/louvain/spec.rs`.
- `BetweennessCentralityConfig` — `gds/src/config/algo_config.rs` vs `gds/src/algo/betweenness/spec.rs`.
- `NodeSimilarityConfig` — `gds/src/config/algo_config.rs` vs `gds/src/algo/similarity/node_similarity/spec.rs`.
- Additional candidates: scan `gds/src/config` vs `gds/src/algo` for similarly-named `*Config` structs and decide canonical home.

## Recommendations (prioritized) ✅

1. Short-term (P0) — Procedure Facade rule enforcement:
   - Require **procedures** to accept/consume typed config objects from `gds/src/config` when a config is required and shared semantics apply.
   - If an algorithm has an implementation-local `spec::Config`, add simple `From`/`TryFrom` conversions between `gds::config::<Algo>Config` and `algo::...::spec::<Algo>Config` so facades can accept canonical config while runtime retains its internal representation.
   - For builder-style facades where a single `Config` makes sense (e.g., many centrality algos), create small typed config structs in `gds/src/config` and provide a `.build()` conversion in the facade.

2. Medium-term (P1) — Deduplicate & migrate:
   - Choose canonical owners for each `*Config` (prefer `gds/src/config` as single source-of-truth for public/JSON-driven configuration).
   - Migrate or add conversions in `gds/src/algo/*/spec.rs` to use or interoperate with the canonical `gds/src/config` types.

3. Tests & automation (P1–P2):
   - Add an automated audit test (quick grep unit test) failing CI if a `*Config` name exists in both `gds/src/config` and `gds/src/algo` without an explicit `#[allow]` or migration note.
   - Add a repository doc: `gds/doc/CONFIG-GUIDELINES.md` describing where to put configs and conversion conventions.

4. Back-compat & ergonomics:
   - Provide facade constructors that accept both concrete store types (`Arc<DefaultGraphStore>`) and trait-based `Arc<dyn GraphStoreFacade>` (as a migration path).
   - Keep builder-style methods for ergonomics while introducing typed config `From` conversions for reproducibility and config->JSON round-trips.

## Concrete next steps (short list you can action now) ▶️

1. Add this audit doc to the repo (done). ✅
2. Add a **procedure-only** automated test that:
   - Scans `gds/src/procedures` for files that build a config object (e.g., call `XConfig::builder()` or `XConfig {`) and records the origin of `XConfig`.
   - Fails if more than one definition of the same `XConfig` is found across `gds/src/config` and `gds/src/algo` without an implemented `From` conversion.
3. Pick 2–3 high-impact config types to centralize as a pilot (my recommendation: **LouvainConfig**, **BetweennessCentralityConfig**, **NodeSimilarityConfig**). Add `From`/`TryFrom` adapters in algo/spec to accept canonical `gds::config` types.
4. Add `gds/doc/CONFIG-GUIDELINES.md` documenting the policy (short, prescriptive).

## Appendix A — Notable code references

- `procedures/centrality/pagerank.rs` — uses `crate::config::PageRankConfig` and calls `PageRankConfig::builder()` to create runtime config.
- `procedures/community/louvain.rs` — holds `config: LouvainConfig` which currently is `crate::algo::louvain::LouvainConfig` (duplicate with `gds/src/config`).
- `procedures/embeddings/graphsage.rs` — constructs `GraphSageConfig` from builder fields (from `crate::algo::embeddings::graphsage::spec`).
- `gds/src/config/algo_config.rs` — central place where some algorithm configs are already defined (`PageRankConfig`, `LouvainConfig`, `NodeSimilarityConfig`, `BetweennessCentralityConfig`).
- `gds/src/algo/*/spec.rs` — multiple per-algo spec config structs that sometimes shadow or duplicate `gds/src/config` entries.

---

If you want, I can run the follow-up actions now:

- Option A (recommended): implement the **procedure-only automated audit test** (CI check) to catch duplicates and regresses. This is a small, focused change (adds a test under `gds/tests/`).
- Option B: create a pilot PR that **centralizes LouvainConfig** (pick one candidate) and adds a `From` conversion in `algospec` and updates `procedures/louvain.rs` to use the canonical config.
- Option C: generate the `gds/doc/CONFIG-GUIDELINES.md` document now.

Which would you like next?
