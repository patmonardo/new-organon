# Progress Tracking in `gds` (Rust)

Date: 2026-01-02

This note captures the current “shape” of progress tracking in the Rust `gds` crate, using BFS/DFS as the working exemplar.

## TL;DR

- **Progress belongs in the top-level driver loop** (typically `storage.compute_*`), because that’s where the algorithm naturally observes the real units of work.
- **Facades/specs inject the tracker** (task name + concurrency + best-effort volume), then pass it into `compute_*`.
- **Work-unit choice matters more than perfection.** Ballpark, stable, monotonic progress is enough early on.

## What is implemented today

### Algorithm-facing API

The algorithm-facing handle is `ProgressTracker` in `gds/src/core/utils/progress/mod.rs`.

It provides:
- `begin_subtask(volume: usize)` / `begin_subtask_unknown()`
- `log_progress(amount: usize)`
- `end_subtask()` / `end_subtask_with_failure()`

Internally it is backed by `BatchingProgressLogger` which prints progress lines in **debug builds** (`cfg(debug_assertions)`), currently to **stderr**.

### Where it is wired

BFS/DFS are fully wired end-to-end:
- Facades (procedures): construct and pass a tracker into the storage runtime.
- Specs (focused-macro algorithm specs): do the same.
- Storage runtimes: emit begin/progress/end around the traversal loops.

### Work unit chosen for BFS/DFS

For both BFS and DFS we track progress in terms of:

- **relationships examined** ≈ `neighbors.len()` each time we expand a node.

Total volume is:

- `graph.relationship_count()` when a graph view is present
- otherwise `UNKNOWN_VOLUME`

This mirrors the “driver loop emits progress” approach used by Java GDS.

## How to integrate progress in a new algorithm (template)

1. **Pick the driver loop** (usually `storage.compute_*`).
2. **Pick the work unit** (what you can increment naturally per inner-loop step).
3. **Pick a best-effort volume**:
   - Prefer `relationship_count()` for edge-scan algorithms
   - Prefer `node_count()` for pure node-iteration algorithms
   - Prefer `iterations * relationship_count()` for iterative edge-sweep algorithms when iteration count is known
   - Otherwise use `UNKNOWN_VOLUME`
4. **Facade/spec injection:**
   - Create `ProgressTracker::with_concurrency(Tasks::leaf("NAME", volume_or_unknown), concurrency)`
   - Pass `&mut progress_tracker` into `compute_*`
5. **Driver emissions:**
   - Call `begin_subtask(_)/begin_subtask_unknown()` at the start
   - Call `log_progress(...)` in the inner loop
   - Call `end_subtask()` on success, `end_subtask_with_failure()` on error paths

### Concurrency note (for later)

Even though true parallel execution isn’t enabled everywhere yet, the design should stay the same:

- The facade/spec is where `concurrency` comes from (config)
- The driver loop will eventually either:
  - clone a tracker handle into worker threads, or
  - use an immutable handle with interior mutability for concurrent logging

The goal is: **no algorithm-specific logging framework** — just consistent tracker injection + driver emissions.

## Work-unit mapping table (non-ML core families)

This is a “ballpark correct” mapping designed to help schedule and standardize progress across algorithms.

### Pathfinding

| Algorithm | Recommended work unit (`log_progress`) | Suggested volume (`begin_subtask`) | Natural subtasks (future `Tasks::task`) |
|---|---|---|---|
| BFS | relationships examined (`neighbors.len()`) | `relationship_count()` else unknown | `traverse`, `build_paths` |
| DFS | relationships examined (`neighbors.len()`) | `relationship_count()` else unknown | `traverse`, `reconstruct_paths` |
| Dijkstra | relaxations attempted (per neighbor-edge processed) | `relationship_count()` baseline | `init`, `relax_edges`, `finalize` |
| A* | relaxations attempted (neighbor expansions) | usually unknown | `search`, `reconstruct_path` |
| Bellman–Ford | relaxations attempted (edge scan per iter) | `iters * relationship_count()` if known else unknown | `iteration_i` |
| Delta-stepping | relaxations attempted (scanned edges) | unknown or `relationship_count()` baseline | `bucket_processing` |
| Yen’s (k shortest) | candidate expansions / inner shortest-path runs | unknown (or rough `k * relationship_count()`) | `k_iterations` |
| All shortest paths | edge scans / relaxations | unknown or `relationship_count()` | `init`, `sweep`, `collect` |
| Topological sort | nodes processed | `node_count()` | `compute_indegrees`, `process_queue` |
| Spanning tree / k-spanning | edges considered | `relationship_count()` | `select_edges`, `finalize` |
| Random walk | steps executed | `walk_length * num_walks` (config-derived) | `walks`, `collect` |
| DAG longest path | edges relaxed | `relationship_count()` | `toposort`, `dp_relax` |

### Centrality

| Algorithm | Recommended work unit | Suggested volume | Natural subtasks |
|---|---|---|---|
| PageRank | edges processed per iteration | `iters * relationship_count()` if known else unknown | `iteration_i` |
| Betweenness | SSSP sweeps + edges scanned | often unknown; rough `node_count * relationship_count()` | `source_sweep_i` |
| Closeness | SSSP/BFS expansions | unknown; rough `node_count * relationship_count()` | `source_sweep_i` |
| Harmonic | same as closeness | unknown | `source_sweep_i` |
| Degree centrality | nodes processed (degree calc) | `node_count()` (or `relationship_count()` if edge scan) | `scan_degrees` |
| HITS | edges processed per iteration | `iters * relationship_count()` | `iteration_i` |
| Bridges | edges scanned (DFS lowlink) | `relationship_count()` | `dfs`, `lowlink_eval` |
| Articulation points | edges scanned (DFS lowlink) | `relationship_count()` | `dfs`, `lowlink_eval` |

### Community

| Algorithm | Recommended work unit | Suggested volume | Natural subtasks |
|---|---|---|---|
| WCC | edges scanned / union ops | `relationship_count()` | `scan_edges`, `compress_labels` |
| SCC | edges scanned (multi-phase) | `relationship_count()` | `phase1_dfs`, `phase2_assign` |
| Label propagation | edge scans per iteration | `iters * relationship_count()` if known else unknown | `iteration_i` |
| Louvain | move-evaluations / neighbor community checks | unknown; `relationship_count()` baseline | `phase_i`, `aggregate_graph` |
| Leiden | similar to Louvain | unknown | `phase_i` |
| Modularity | edges scanned to compute score | `relationship_count()` | `scan_edges` |
| Triangle count | wedge checks / adjacency intersections | unknown; `relationship_count()` baseline | `count_wedges`, `aggregate` |
| Local clustering coefficient | neighbor-pair checks per node | unknown (sum of d(v)^2) | `per_node_eval` |
| Conductance | edges scanned | `relationship_count()` | `scan_edges` |
| K-core | degree updates / edge removals | `relationship_count()` baseline | `peel_rounds` |

## Suggested follow-ups (platform “state exposure”)

You mentioned the execution modes feel more structurally important / unknown.

A productive next step is to create a small set of examples that demonstrate, side-by-side:

1. **Estimate memory mode**
   - Show how an algorithm returns `MemoryRange` without executing
   - Surface which parameters drive the estimate (e.g., concurrency, max_depth, iterations)

2. **Stats mode**
   - Execute and return aggregated stats only
   - Show how stats differs from stream (no per-row output)

3. **Stream mode**
   - Execute and return row-like outputs / paths

A single “Modes Walkthrough” example that runs (estimate → stats → stream) on the same random graph is often the fastest way to see whether the platform’s layering is coherent.
