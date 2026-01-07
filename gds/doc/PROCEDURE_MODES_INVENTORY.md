# Procedure Modes Inventory (Pathfinding / Centrality / Community / Similarity)

Date: 2026-01-03

This document inventories the execution **modes** exposed by the Rust `gds` *procedure/facade* layer. The intent is to support the next workstream: improving/implementing **Stats** and **Mutate/Write** behaviors with a consistent “standard pipeline architecture”.

## What “modes” mean in this repo

- `stream()`: returns per-row results (iterators of rows)
- `stats()`: returns aggregated statistics (small summary payload)
- `run()`: returns the raw algorithm result (when present)
- `mutate(...)`: writes results back into the in-memory projection (node/rel properties)
- `write(...)`: exports results to some external sink or “write surface” (often a thin wrapper around `mutate`)
- `estimate_memory()`: best-effort planner-style memory estimate

## Scope + methodology

- Scope: `gds/src/procedures/{pathfinding,centrality,community,similarity}/*.rs` (excluding `mod.rs`).
- The scan is syntactic: it detects `pub fn <mode>` declarations and flags `mutate`/`write` as **stub** if their function body contains “not implemented / not yet implemented”.
- Note: `write()` can be marked `wraps-stub-mutate` when it simply calls a stubbed `mutate()` (so it is effectively non-functional until mutate is implemented).

## High-level findings

- All procedures in these four domains now have **ProgressTracking** support (completed in the earlier pass).
- `stream()` and `stats()` exist broadly, but many procedures are still in an early translated state where **mutate/write are placeholders**.
- A few procedures omit some modes entirely (common for newer/simple facades like KNN or KSpanningTree).

## Pathfinding

**Pathfinding**
- `stream`: 15 ok (all ok)
- `stats`: 15 ok (all ok)
- `run`: 0 ok (15 missing)
- `mutate`: 0 ok (14 stub, 1 missing)
- `write`: 0 ok (14 stub, 1 missing)
- `estimate_memory`: 14 ok (1 missing)

| Procedure | stream | stats | run | mutate | write | estimate_memory |
|---|---:|---:|---:|---:|---:|---:|
| gds/src/procedures/pathfinding/all_shortest_paths.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/pathfinding/astar.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/pathfinding/bellman_ford.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/pathfinding/bfs.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/pathfinding/dag_longest_path.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/pathfinding/delta_stepping.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/pathfinding/dfs.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/pathfinding/dijkstra.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/pathfinding/kspanningtree.rs | ok | ok | — | — | — | — |
| gds/src/procedures/pathfinding/prize_collecting_steiner_tree.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/pathfinding/random_walk.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/pathfinding/spanning_tree.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/pathfinding/steiner_tree.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/pathfinding/topological_sort.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/pathfinding/yens.rs | ok | ok | — | stub | stub | ok |

## Centrality

**Centrality**
- `stream`: 9 ok (all ok)
- `stats`: 9 ok (all ok)
- `run`: 3 ok (6 missing)
- `mutate`: 0 ok (9 stub)
- `write`: 0 ok (8 stub, 1 wraps-stub-mutate)
- `estimate_memory`: 9 ok (all ok)

| Procedure | stream | stats | run | mutate | write | estimate_memory |
|---|---:|---:|---:|---:|---:|---:|
| gds/src/procedures/centrality/articulation_points.rs | ok | ok | ok | stub | stub | ok |
| gds/src/procedures/centrality/betweenness.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/centrality/bridges.rs | ok | ok | ok | stub | stub | ok |
| gds/src/procedures/centrality/celf.rs | ok | ok | — | stub | wraps-stub-mutate | ok |
| gds/src/procedures/centrality/closeness.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/centrality/degree_centrality.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/centrality/harmonic.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/centrality/hits.rs | ok | ok | ok | stub | stub | ok |
| gds/src/procedures/centrality/pagerank.rs | ok | ok | — | stub | stub | ok |

## Community

**Community**
- `stream`: 13 ok (all ok)
- `stats`: 13 ok (all ok)
- `run`: 8 ok (5 missing)
- `mutate`: 0 ok (12 stub, 1 missing)
- `write`: 0 ok (10 stub, 2 wraps-stub-mutate, 1 missing)
- `estimate_memory`: 12 ok (1 missing)

| Procedure | stream | stats | run | mutate | write | estimate_memory |
|---|---:|---:|---:|---:|---:|---:|
| gds/src/procedures/community/approx_max_kcut.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/community/conductance.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/community/k1coloring.rs | ok | ok | ok | stub | stub | ok |
| gds/src/procedures/community/kcore.rs | ok | ok | ok | stub | stub | ok |
| gds/src/procedures/community/kmeans.rs | ok | ok | ok | — | — | — |
| gds/src/procedures/community/label_propagation.rs | ok | ok | ok | stub | stub | ok |
| gds/src/procedures/community/leiden.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/community/local_clustering_coefficient.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/community/louvain.rs | ok | ok | ok | stub | wraps-stub-mutate | ok |
| gds/src/procedures/community/modularity.rs | ok | ok | — | stub | stub | ok |
| gds/src/procedures/community/scc.rs | ok | ok | ok | stub | stub | ok |
| gds/src/procedures/community/triangle_count.rs | ok | ok | ok | stub | stub | ok |
| gds/src/procedures/community/wcc.rs | ok | ok | ok | stub | wraps-stub-mutate | ok |

## Similarity

**Similarity**
- `stream`: 4 ok (all ok)
- `stats`: 4 ok (all ok)
- `run`: 0 ok (4 missing)
- `mutate`: 0 ok (2 stub, 2 missing)
- `write`: 0 ok (2 stub, 2 missing)
- `estimate_memory`: 0 ok (4 missing)

| Procedure | stream | stats | run | mutate | write | estimate_memory |
|---|---:|---:|---:|---:|---:|---:|
| gds/src/procedures/similarity/filtered_knn.rs | ok | ok | — | — | — | — |
| gds/src/procedures/similarity/filtered_node_similarity.rs | ok | ok | — | stub | stub | — |
| gds/src/procedures/similarity/knn.rs | ok | ok | — | — | — | — |
| gds/src/procedures/similarity/node_similarity.rs | ok | ok | — | stub | stub | — |

## Mutate/Write TODO inventory (action-oriented)

This section is intentionally redundant with the tables above, but grouped by the most actionable next steps for “mode completion”.

### Pathfinding

**Missing `mutate(...)` (no method present)**
- gds/src/procedures/pathfinding/kspanningtree.rs

**Missing `write(...)` (no method present)**
- gds/src/procedures/pathfinding/kspanningtree.rs

**Stubbed `mutate(...)` (explicit “not implemented”)**
- gds/src/procedures/pathfinding/all_shortest_paths.rs
- gds/src/procedures/pathfinding/astar.rs
- gds/src/procedures/pathfinding/bellman_ford.rs
- gds/src/procedures/pathfinding/bfs.rs
- gds/src/procedures/pathfinding/dag_longest_path.rs
- gds/src/procedures/pathfinding/delta_stepping.rs
- gds/src/procedures/pathfinding/dfs.rs
- gds/src/procedures/pathfinding/dijkstra.rs
- gds/src/procedures/pathfinding/prize_collecting_steiner_tree.rs
- gds/src/procedures/pathfinding/random_walk.rs
- gds/src/procedures/pathfinding/spanning_tree.rs
- gds/src/procedures/pathfinding/steiner_tree.rs
- gds/src/procedures/pathfinding/topological_sort.rs
- gds/src/procedures/pathfinding/yens.rs

**Stubbed `write(...)` (explicit “not implemented”)**
- gds/src/procedures/pathfinding/all_shortest_paths.rs
- gds/src/procedures/pathfinding/astar.rs
- gds/src/procedures/pathfinding/bellman_ford.rs
- gds/src/procedures/pathfinding/bfs.rs
- gds/src/procedures/pathfinding/dag_longest_path.rs
- gds/src/procedures/pathfinding/delta_stepping.rs
- gds/src/procedures/pathfinding/dfs.rs
- gds/src/procedures/pathfinding/dijkstra.rs
- gds/src/procedures/pathfinding/prize_collecting_steiner_tree.rs
- gds/src/procedures/pathfinding/random_walk.rs
- gds/src/procedures/pathfinding/spanning_tree.rs
- gds/src/procedures/pathfinding/steiner_tree.rs
- gds/src/procedures/pathfinding/topological_sort.rs
- gds/src/procedures/pathfinding/yens.rs

### Centrality

**Stubbed `mutate(...)` (explicit “not implemented”)**
- gds/src/procedures/centrality/articulation_points.rs
- gds/src/procedures/centrality/betweenness.rs
- gds/src/procedures/centrality/bridges.rs
- gds/src/procedures/centrality/celf.rs
- gds/src/procedures/centrality/closeness.rs
- gds/src/procedures/centrality/degree_centrality.rs
- gds/src/procedures/centrality/harmonic.rs
- gds/src/procedures/centrality/hits.rs
- gds/src/procedures/centrality/pagerank.rs

**Stubbed `write(...)` (explicit “not implemented”)**
- gds/src/procedures/centrality/articulation_points.rs
- gds/src/procedures/centrality/betweenness.rs
- gds/src/procedures/centrality/bridges.rs
- gds/src/procedures/centrality/closeness.rs
- gds/src/procedures/centrality/degree_centrality.rs
- gds/src/procedures/centrality/harmonic.rs
- gds/src/procedures/centrality/hits.rs
- gds/src/procedures/centrality/pagerank.rs

**`write(...)` wraps a stubbed `mutate(...)`**
- gds/src/procedures/centrality/celf.rs

### Community

**Missing `mutate(...)` (no method present)**
- gds/src/procedures/community/kmeans.rs

**Missing `write(...)` (no method present)**
- gds/src/procedures/community/kmeans.rs

**Stubbed `mutate(...)` (explicit “not implemented”)**
- gds/src/procedures/community/approx_max_kcut.rs
- gds/src/procedures/community/conductance.rs
- gds/src/procedures/community/k1coloring.rs
- gds/src/procedures/community/kcore.rs
- gds/src/procedures/community/label_propagation.rs
- gds/src/procedures/community/leiden.rs
- gds/src/procedures/community/local_clustering_coefficient.rs
- gds/src/procedures/community/louvain.rs
- gds/src/procedures/community/modularity.rs
- gds/src/procedures/community/scc.rs
- gds/src/procedures/community/triangle_count.rs
- gds/src/procedures/community/wcc.rs

**Stubbed `write(...)` (explicit “not implemented”)**
- gds/src/procedures/community/approx_max_kcut.rs
- gds/src/procedures/community/conductance.rs
- gds/src/procedures/community/k1coloring.rs
- gds/src/procedures/community/kcore.rs
- gds/src/procedures/community/label_propagation.rs
- gds/src/procedures/community/leiden.rs
- gds/src/procedures/community/local_clustering_coefficient.rs
- gds/src/procedures/community/modularity.rs
- gds/src/procedures/community/scc.rs
- gds/src/procedures/community/triangle_count.rs

**`write(...)` wraps a stubbed `mutate(...)`**
- gds/src/procedures/community/louvain.rs
- gds/src/procedures/community/wcc.rs

### Similarity

**Missing `mutate(...)` (no method present)**
- gds/src/procedures/similarity/filtered_knn.rs
- gds/src/procedures/similarity/knn.rs

**Missing `write(...)` (no method present)**
- gds/src/procedures/similarity/filtered_knn.rs
- gds/src/procedures/similarity/knn.rs

**Stubbed `mutate(...)` (explicit “not implemented”)**
- gds/src/procedures/similarity/filtered_node_similarity.rs
- gds/src/procedures/similarity/node_similarity.rs

**Stubbed `write(...)` (explicit “not implemented”)**
- gds/src/procedures/similarity/filtered_node_similarity.rs
- gds/src/procedures/similarity/node_similarity.rs

## Notes for “owning” the control logic

- The Procedure layer is the correct owner of modes. A good “standard pipeline” is: `validate → build graph view → compute core → adapt to mode (stream/stats/mutate/write)`.
- Implementing `mutate/write` in earnest likely needs a consistent graph-property write API (or a small adapter layer) so procedures can set node/relationship properties in the projection safely and uniformly.
- `stats()` is already widely present; future improvements typically aim to compute stats without materializing full row outputs when possible (optional optimization).

