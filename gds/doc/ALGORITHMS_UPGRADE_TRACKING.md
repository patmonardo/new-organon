# Algorithms Upgrade Tracking

This note is a **working checklist** to take the current translated procedures from “compiles” → “correct + debuggable + agent-ready”.

## Goals (tomorrow)

- Upgrade and debug existing translated algorithms/procedures.
- Establish a repeatable workflow: **config → parse/validate → execute → consume result**.
- Expand coverage from ~40% translated to as close to 100% as practical, without destabilizing the repo.

## What exists right now

### Procedure modules present

Directory: `gds/src/procedures/`

Observed modules:
- Centrality/community-ish: `pagerank/`, `hits/`, `louvain/`, `betweenness/`, `closeness/`, `harmonic/`, `degree_centrality/`
- Path/traversal: `bfs/`, `dfs/`, `dijkstra/`, `bellman_ford/`, `delta_stepping/`, `astar/`, `yens/`, `all_shortest_paths/`, `traversal/`
- Components/structure: `scc/`, `wcc/`, `kcore/`, `k1coloring/`
- Triangles/cluster: `triangle_count/`, `local_clustering_coefficient/`
- Misc placeholders / grouping: `algorithms/`, `facades/`, `core/`

### Algorithms using `define_algorithm_spec!`

These have an explicit “spec” entrypoint wired via the focused macros:
- `all_shortest_paths/spec.rs`
- `astar/spec.rs`
- `bellman_ford/spec.rs`
- `bfs/spec.rs`
- `degree_centrality/spec.rs`
- `delta_stepping/spec.rs`
- `dfs/spec.rs`
- `dijkstra/spec.rs`
- `hits/spec.rs`
- `local_clustering_coefficient/spec.rs`
- `scc/spec.rs`
- `spanning_tree/spec.rs`
- `sum/spec_focused.rs`
- `wcc/spec.rs`
- `yens/spec.rs`

### Existing tests

There are many unit + integration tests scattered across procedures (notably: `*_integration_tests.rs`, plus module-local tests in `computation.rs`, `storage.rs`, and `spec.rs`).

This is good news: we can upgrade algorithm correctness while keeping the harness stable.

## Known upgrade hotspots

These are patterns seen in the codebase that tend to indicate “translated but not yet correct”:

- Hard-coded placeholders like `let node_count = 100; // Note: Replace with actual graph store`.
- TODOs for missing graph-store accessors (edge weights, node counts, property access).
- Specs validating config with `projection::codegen::config::validation::ConfigError` while other config flows use `config::validation::ConfigError`.

## Recommended upgrade workflow (repeat per algorithm)

1. **Spec sanity**: ensure `parse_config` + `validate()` match expected parameters and defaults.
2. **GraphStore contract**: replace placeholder graph values with real `GraphStore` queries.
3. **Correctness tests**:
   - Add 1–3 minimal graph fixtures per algorithm.
   - Assert exact outputs for small graphs.
4. **Performance knobs**: wire concurrency / chunking fields (don’t micro-optimize yet).
5. **Agent UX**: ensure errors are structured and actionable (field + message).

## Suggested first targets (high leverage)

- `dijkstra`, `yens`, `delta_stepping`: unlock pathfinding surface.
- `pagerank`, `louvain`, `hits`: unlock core centrality/community surface.
- `wcc`, `scc`: unlock component analytics.

## Notes

- Keep changes surgical: prioritize correctness and stable contracts over refactors.
- Collections/Arrow work should be pulled in only when an algorithm is blocked by missing property backends or materialization.
