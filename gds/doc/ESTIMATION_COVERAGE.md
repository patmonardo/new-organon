# Estimation coverage & API consistency snapshot

Date: 2026-01-02

Goal: assess **coverage/consistency of the “estimate memory” API surface** (not numeric correctness).

## TL;DR

- Procedure layer coverage is **pretty good**: there are ~36 `estimate_memory()` methods under `gds/src/procedures/**`.
- Community algorithms have **many placeholders**: several return `Ok(MemoryRange::of_range(0, 1024 * 1024))`.
- Application dispatch has a **naming inconsistency**:
  - Pathfinding handlers use mode string `"estimate"`.
  - Centrality/community handlers use mode string `"estimate_memory"`.

This is the main “API surface not ready yet” smell: clients need to know which family uses which mode token.

## What exists today (procedure layer)

These procedures define `estimate_memory()` (either `MemoryRange` or `Result<MemoryRange>`):

- Pathfinding: BFS, DFS, A*, Dijkstra, Bellman-Ford, Delta-Stepping, Yen’s, Random Walk, etc.
- Centrality: PageRank, DegreeCentrality, Betweenness, Closeness, HITS, Harmonic, Bridges, Articulation Points, CELF.
- Community: WCC, Louvain, plus many that are currently placeholders.

A quick executable snapshot is in the example:

- [gds/examples/estimation_probe.rs](../examples/estimation_probe.rs)

Application-handler matrix (per `mode` token):

- [gds/doc/ESTIMATION_HANDLER_MATRIX.md](ESTIMATION_HANDLER_MATRIX.md)

## Known placeholder estimates (procedure layer)

These currently return the exact sentinel range `0 .. 1 MiB`:

- `K1ColoringFacade::estimate_memory()`
- `SccFacade::estimate_memory()`
- `KCoreFacade::estimate_memory()`
- `TriangleCountFacade::estimate_memory()`
- `LabelPropagationFacade::estimate_memory()`
- `ModularityFacade::estimate_memory()`
- `ApproxMaxKCutFacade::estimate_memory()`
- `LeidenFacade::estimate_memory()`
- `LocalClusteringCoefficientFacade::estimate_memory()`
- `ConductanceFacade::estimate_memory()`

Those are “covered” in the sense that the method exists, but they’re not informative yet.

## Application dispatch mode-string inconsistency

Under `gds/src/applications/algorithms/**`, the request handlers accept a `mode` string.

### Uses `"estimate"`

These pathfinding handlers expose estimation as `mode: "estimate"`:

- BFS
- DFS
- Dijkstra
- A*
- Bellman-Ford
- Delta-Stepping
- Yen’s

(see grep results for `"estimate" =>`)

### Uses `"estimate_memory"`

Most centrality/community handlers expose estimation as `mode: "estimate_memory"`:

- Centrality: PageRank, DegreeCentrality, Betweenness, HITS, Closeness, Harmonic, Bridges, Articulation Points, CELF
- Community: WCC, Louvain, SCC, K1Coloring, etc.

(see grep results for `"estimate_memory" =>`)

### Why this matters

If a client wants a uniform API, they currently need branching logic like:

- if pathfinding: send `mode=estimate`
- else: send `mode=estimate_memory`

That’s the main reason the API surface isn’t “ready/consistent” yet.

## Probe results: “does it behave like a real estimate?”

From running `cargo run -p gds --example estimation_probe` on a 5k-node random graph:

- BFS/DFS/PageRank/DegreeCentrality/WCC/Louvain: return non-trivial ranges
- SCC/K1Coloring: flagged as `[placeholder?]` by the probe (exact `0..1MiB`)

## What would make the surface feel consistent (without big refactors)

Low-impact steps (not done yet):

1. Standardize the request mode token to **one string** (pick `estimate_memory` or `estimate`).
2. Standardize JSON response shape:
   - some handlers return `{min_bytes,max_bytes}`
   - BFS currently returns nested `{memoryBytes:{min,max}}`
3. Add a tiny convention comment somewhere central (even a doc) so new handlers follow the same pattern.

This doc + the probe example should make it easy to do that later without a “global cleanup day”.
