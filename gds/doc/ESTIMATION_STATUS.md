# Estimation support status (memory / “estimate” mode)

Date: 2026-01-02

This note answers: **“Where do we stand with estimation support right now?”**

## What “estimation” currently means in this repo

In practice, “estimate” means **return a min/max memory range without executing the algorithm**, typically expressed as a `MemoryRange` (min/max bytes).

There are *two different* estimation “tracks” in the codebase:

1. **Algorithm procedure estimation**: `procedure/facade/builder.estimate_memory() -> MemoryRange` (or `Result<MemoryRange>`)
2. **Projection/import estimation**: `GraphStoreFactory::estimate_memory() -> (usize, usize)` (currently behaves more like a stub/placeholder)

There is also a third system that *looks* like it should unify things:

3. **Hierarchical memory trees**: `mem::MemoryEstimation::estimate() -> MemoryTree`

…but it’s currently used only in some subsystems (not universally wired through the procedure executor / guard).

## Key reality check: executor modes do NOT include “Estimate”

The procedure executor has execution modes for **Stream/Stats/Train/Write/Mutate**, but **no `Estimate` mode**.

- See [gds/src/projection/eval/procedure/execution_mode.rs](../src/projection/eval/procedure/execution_mode.rs)

So today, “estimate” is not a first-class `ExecutionMode`; instead it is handled as **an alternate branch in application/procedure dispatch** (usually a `match mode { "estimate" => ... }` or `"estimate_memory" => ...`).

## Where estimation is exposed to callers

### Application dispatch (JSON handler level)

A bunch of algorithm handlers in `gds/src/applications/algorithms/**` expose estimation via a `mode` string.

Important detail: **the mode string is currently inconsistent**:

- Pathfinding handlers use `"estimate"` (e.g. BFS/DFS/Dijkstra/A*/Bellman-Ford/etc.)
- Centrality/community handlers tend to use `"estimate_memory"`

Examples:

- BFS handler: [gds/src/applications/algorithms/pathfinding/bfs.rs](../src/applications/algorithms/pathfinding/bfs.rs)
- PageRank handler: [gds/src/applications/algorithms/centrality/pagerank.rs](../src/applications/algorithms/centrality/pagerank.rs)
- K1Coloring handler: [gds/src/applications/algorithms/community/k1coloring.rs](../src/applications/algorithms/community/k1coloring.rs)

### Procedure/facade/builder layer

Many procedures implement an explicit `estimate_memory()` method.

Examples that are implemented with a real-ish heuristic:

- BFS: [gds/src/procedures/pathfinding/bfs.rs](../src/procedures/pathfinding/bfs.rs)
- DFS: [gds/src/procedures/pathfinding/dfs.rs](../src/procedures/pathfinding/dfs.rs)

Examples that are explicitly placeholders:

- K1Coloring: [gds/src/procedures/community/k1coloring.rs](../src/procedures/community/k1coloring.rs)
  - Returns `Ok(MemoryRange::of_range(0, 1024 * 1024)) // placeholder`

So “estimate support” exists broadly as an API surface, but **accuracy varies wildly by algorithm**.

## The memory guard is not wired to real estimates yet

There is a `MemoryGuard` subsystem intended to block execution if memory requirements exceed availability.

However, the current implementation is explicitly deferred:

- It returns a placeholder requirement (`Ok(Self::new(1000))`) instead of using `MemoryEstimation::estimate()`.
- Graph dimensions are also stubbed (`GraphDimensionsImpl::new()`), with a comment that actual computation is deferred.

See [gds/src/applications/algorithms/machinery/memory_guard.rs](../src/applications/algorithms/machinery/memory_guard.rs).

Net: **estimates aren’t currently used as an enforcement mechanism**, and “guarding” should not be considered reliable.

## Projection/import estimation exists but is currently placeholder-ish

Graph projection factories provide an `estimate_memory()` method:

- Trait: [gds/src/projection/factory/mod.rs](../src/projection/factory/mod.rs)
- Arrow factory tests show the current behavior:
  - Default config estimate returns `(0, 0)` and is labeled placeholder.
  - With sample in-memory tables, it returns `(3, 2)`.

See [gds/src/projection/factory/arrow/factory.rs](../src/projection/factory/arrow/factory.rs).

Interpreting `(3, 2)` as “bytes” would not make sense, so today this is best understood as **not a real byte-based memory estimate**.

## The “MemoryTree” estimation system exists (selective usage)

There is a good foundation for Java-parity-style memory estimation:

- Trait: [gds/src/mem/memory_estimation.rs](../src/mem/memory_estimation.rs)
- Module overview: [gds/src/mem/mod.rs](../src/mem/mod.rs)

Some algorithms/subsystems already implement tree-shaped estimation (example: PageRank has a dedicated module under `algo/pagerank/memory_estimation.rs`).

But: this isn’t currently the universal estimate path for procedures, and it’s not hooked into `ExecutionMode` or the memory guard.

## Practical takeaways (where we stand)

- **There is visible/usable “estimate” output today** via application dispatch modes.
- **BFS/DFS estimates are implemented** (simple heuristic based on node count + rough relationship overhead).
- **Many other algorithms have estimate methods**, but some are placeholders (especially in community/graph ops).
- **No unified “Estimate” execution mode** exists in the core executor.
- **MemoryGuard does not enforce based on real estimates** yet.
- **Projection/import estimate APIs exist** but are not byte-accurate today.

## Low-effort probes to do next (no cleanup required)

If we want to keep today as “probing only”, good next checks are:

1. Pick 2–3 representative algos (e.g. BFS, PageRank, K1Coloring) and compare:
   - does estimate exist?
   - is it placeholder?
   - does it depend on graph size?
2. Add a tiny example (or extend an existing one) that prints:
   - BFS `estimate_memory()`
   - DFS `estimate_memory()`
   - PageRank `estimate_memory()`

(That would give a crisp snapshot of “platform state” without refactors.)
