# Algorithms Applications (Machinery) - Control Logic Review

## Scope

This note is a **control-logic / guardrails review** for:

- `gds/src/applications/algorithms/**`

It is focused on two invariants:

1. **Applications should call only Procedures** (never `crate::algo::*` directly)
2. **Applications should use the Machinery templates** (where applicable) for consistent progress/error handling

This is intentionally **not** the “big refactor” to a true Applications Component system.

---

## Current Status (High-level)

### ✅ Good news: Procedures-first is largely already true

A repo-wide scan of `gds/src/applications/algorithms/**/*.rs` shows **heavy use of** `crate::procedures::*` imports across centrality/community/pathfinding/similarity/embeddings/machine_learning.

### ✅ Machinery is already widely used

Many modes (stream/stats/mutate/write/estimate) instantiate:

- `DefaultAlgorithmProcessingTemplate`
- `AlgorithmProcessingTemplateConvenience`

and route execution through the template runner, which in turn calls `AlgorithmMachinery::run_algorithms_and_manage_progress_tracker`.

---

## Responsibility Split (Who Owns What?)

This is the main seam to keep clean: there are **two “facade layers”** involved in practice.

- **Applications + Machinery** (request orchestration, standard pipeline)
- **Procedures** (algorithm entrypoints, controller-pattern runtimes)

### Applications + Machinery owns

- **Progress tracker creation + lifecycle**
  - Creates the request-scoped tracker (`ProgressTrackerCreator`).
  - Owns the lifecycle envelope: `begin_subtask` / `end_subtask` / `end_subtask_with_failure` and optional `release()`.
  - Mechanism:
    - `DefaultAlgorithmProcessingTemplate::process(...)` creates the tracker.
    - `AlgorithmMachinery::run_algorithms_and_manage_progress_tracker(...)` wraps compute in begin/end/failure/release.

- **Task framing + timing**
  - Chooses the top-level `Task` tree for the run and records timings (pre-processing / compute / side-effect).

- **Memory guard policy (can this run at all?)**
  - The “Machinery contract” includes a first-class `process_with_memory_guard(...)` which runs a `MemoryGuard` before compute.
  - `DefaultMemoryGuard` decides min vs max estimate and enforces reservation using `MemoryTracker`.

- **Rendering and side-effects**
  - Converts algorithm results into caller payloads (stats/stream/mutate/write).
  - Applies mutate/write side effects (graph store writes, DB export hooks, etc.).

- **Termination wiring (currently mixed semantics)**
  - The template passes a `TerminationFlag` into compute.
  - `TerminationAwareProgressTracker` exists to gate progress calls, but it enforces termination via panic (see gaps below).

### Procedures owns

- **Algorithm semantics + controller-pattern execution**
  - Procedures validate inputs, select projections, create storage+computation runtimes, call `storage.compute_*`.

- **Domain progress reporting**
  - Procedures/algorithms should report domain progress through the passed-in tracker (volume/steps/progress).
  - Procedures should never create or release trackers.

- **Memory estimation numbers**
  - Procedures are best positioned to define the estimate (they know the algorithm and config).
  - Many procedures already expose `estimate_memory()` (and some pipelines expose estimate modes).

---

## Where We’re Not Yet Clear (Concrete Gaps)

### 1) Who sets tracker "estimated resource footprint"?

- There are *two* memory-related mechanisms:
  - **Guard**: `MemoryGuard` reserves memory (or skips on NotImplemented).
  - **Progress UI hint**: `ProgressTracker::set_estimated_resource_footprint(MemoryRange)`.

But the template path doesn’t obviously standardize whether/when the estimate is written into the tracker.

Recommendation (small and low-risk):
- If an estimate exists, **Machinery should always call** `tracker.set_estimated_resource_footprint(...)` before compute, because it owns tracker creation and tasks.
- The **estimate itself should come from Procedures** (or a procedure-owned estimator helper).

### 2) Memory estimation ownership is split across two entrypoints

- `AlgorithmProcessingTemplate::process_with_memory_guard(...)` runs the guard.
- `ComputationService::compute_algorithm(...)` also runs the guard.

That’s fine internally, but for clarity we should pick a single “standard path” for algorithm applications.

Recommendation:
- Prefer the **template path** as the canonical “application mode runner” because it also owns rendering and side-effects.
- Keep `ComputationService` as an internal building block (or phase it out later).

### 3) Termination currently crosses the boundary as panic

- `TerminationAwareProgressTracker` uses `termination_flag.assert_running()` before delegating.
- The comment explicitly states it panics with `TerminatedException` and suggests `catch_unwind` for `Result` surfaces.
- `AlgorithmMachinery` does not currently `catch_unwind`; it assumes the compute closure returns `Result`.

Recommendation:
- Decide the contract:
  - **Option A**: Termination is allowed to panic and the *outer transport* converts it (needs a consistent catch boundary).
  - **Option B**: Termination becomes a typed error and no panics cross Applications.

---

## Control Protocol (Target Narrative)

This is the “clean story” for how Applications/Machinery and Procedures should interact.

1. **Application handler parses request and chooses mode** (stream/stats/mutate/write/estimate).
2. **Application handler routes strictly to Procedures** (Applications never call into `algo`).
3. **Machinery builds the run envelope**: task tree, request-scoped dependencies, timings, termination wiring.
  - Reference: [gds/src/applications/algorithms/machinery/algorithm_processing_template.rs](../src/applications/algorithms/machinery/algorithm_processing_template.rs)
  - Tracker creation: [gds/src/applications/algorithms/machinery/progress_tracker_creator.rs](../src/applications/algorithms/machinery/progress_tracker_creator.rs)
  - Lifecycle wrapper: [gds/src/applications/algorithms/machinery/algorithm_machinery.rs](../src/applications/algorithms/machinery/algorithm_machinery.rs)
4. **Machinery obtains an algorithm-specific memory estimate from Procedures** (single source of truth).
  - Example estimate surfaces today:
    - BFS estimate mode: [gds/src/applications/algorithms/pathfinding/bfs/modes/estimate.rs](../src/applications/algorithms/pathfinding/bfs/modes/estimate.rs)
    - NodeSimilarity estimate mode: [gds/src/applications/algorithms/similarity/node_similarity/modes/estimate.rs](../src/applications/algorithms/similarity/node_similarity/modes/estimate.rs)
5. **Machinery writes the estimate into the progress tracker** (UI/telemetry hint) and enforces the guard.
  - Guard mechanism: [gds/src/applications/algorithms/machinery/memory_guard.rs](../src/applications/algorithms/machinery/memory_guard.rs)
  - Template hook is present but not yet widely used: `AlgorithmProcessingTemplate::process_with_memory_guard(...)`.
6. **Procedure validates inputs and constructs runtimes** (controller pattern: procedure → storage runtime → computation runtime).
7. **Storage runtime drives the loop and reports domain progress** through the tracker that was injected from Machinery.
8. **Machinery renders results + applies side effects** and closes the tracker envelope (success/failure/release).

This matches the intent you described: both layers participate, but **hierarchically**:

- Machinery owns the request-scoped envelope (identity, lifecycle, policies).
- Procedures own algorithm semantics (and the estimate numbers), and they report domain progress.

---

## Observed Reality (Today)

Today we’re partway there: Machinery is used for a consistent *response envelope* in many modes, but progress/memory enforcement is often not yet unified.

### Example: BFS (manual applications path)

- Application stream and stats bypass Machinery and call the procedure builder directly:
  - Stream: [gds/src/applications/algorithms/pathfinding/bfs/modes/stream.rs](../src/applications/algorithms/pathfinding/bfs/modes/stream.rs)
  - Stats: [gds/src/applications/algorithms/pathfinding/bfs/modes/stats.rs](../src/applications/algorithms/pathfinding/bfs/modes/stats.rs)
- Procedure creates its own request-local tracker (`TaskProgressTracker::with_concurrency(...)`) instead of receiving one from Machinery:
  - Procedure: [gds/src/procedures/pathfinding/bfs.rs](../src/procedures/pathfinding/bfs.rs)
- The storage runtime does meaningful incremental progress:
  - Storage: [gds/src/algo/bfs/storage.rs](../src/algo/bfs/storage.rs)
  - Volume is set to relationship count (when known) and progress increments by neighbors scanned.

### Example: Node Similarity (Machinery envelope, but tracker not threaded)

- Application uses Machinery templates, but the compute closure ignores both the tracker and termination flag:
  - [gds/src/applications/algorithms/similarity/node_similarity/modes/stream.rs](../src/applications/algorithms/similarity/node_similarity/modes/stream.rs)
- Procedure also creates its own local tracker and logs progress as a single “one shot” update:
  - [gds/src/procedures/similarity/node_similarity.rs](../src/procedures/similarity/node_similarity.rs)

### Memory guard status in algorithm applications

- Machinery provides a full guard implementation, but algorithm application handlers are not yet consistently invoking it.
  - No call sites found for `process_with_memory_guard(...)` under `gds/src/applications/algorithms/**`.

---

## Non-Refactor Polish Checklist (Narrative Cleanup)

These are small, incremental moves that preserve current structure but make the story coherent.

1. **Pick and document the canonical “execution path”**
  - Either: all algorithm application modes use Machinery, or explicit exceptions are documented (like BFS today).
2. **Make “estimate → tracker footprint → guard → compute” the standard sequence**
  - Even if some estimates are `NotImplemented`, the sequence should be visible and consistent.
3. **Avoid split-brain progress tracking**
  - If an application mode uses Machinery, it should not ignore the provided tracker (thread it down or don’t use Machinery for that mode).
4. **Keep procedure-level progress domain-meaningful**
  - Procedures/storage decide what volume means (nodes vs relationships vs iterations) and how to increment.
5. **Surface the current placeholder status of progress registries**
  - Many Machinery call sites use `TaskRegistryFactories::empty()`; that’s fine, but we should treat it as “no durable progress sink yet”.
6. **Keep memory estimates owned by Procedures, but policies owned by Machinery**
  - Procedures provide the numbers; Machinery decides min/max policy, reservation, and how to surface it.

---

## Remaining Guardrail Violations / Exceptions

The scan also found a small number of direct `crate::algo::*` imports.

These are not necessarily “bad behavior” (some are just shared result/metric types), but they are **structurally suspicious** for the Applications → Procedures-only rule.

### Direct `crate::algo::*` imports found

- [gds/src/applications/algorithms/pathfinding/dijkstra/steps/mutate_step.rs](../src/applications/algorithms/pathfinding/dijkstra/steps/mutate_step.rs)
  - `use crate::algo::common::result_builders::PathResult;`

- [gds/src/applications/algorithms/pathfinding/dijkstra/steps/write_step.rs](../src/applications/algorithms/pathfinding/dijkstra/steps/write_step.rs)
  - `use crate::algo::common::result_builders::PathResult;`

- [gds/src/applications/algorithms/miscellaneous/scale_properties.rs](../src/applications/algorithms/miscellaneous/scale_properties.rs)
  - `use crate::algo::scale_properties::ScalePropertiesScaler;`

- [gds/src/applications/algorithms/similarity/knn/request.rs](../src/applications/algorithms/similarity/knn/request.rs)
  - `use crate::algo::similarity::knn::metrics::SimilarityMetric;`

- [gds/src/applications/algorithms/similarity/filtered_knn/request.rs](../src/applications/algorithms/similarity/filtered_knn/request.rs)
  - `use crate::algo::similarity::knn::metrics::SimilarityMetric;`

- [gds/src/applications/algorithms/similarity/node_similarity/request.rs](../src/applications/algorithms/similarity/node_similarity/request.rs)
  - `use crate::algo::similarity::NodeSimilarityMetric;`

- [gds/src/applications/algorithms/similarity/filtered_node_similarity/request.rs](../src/applications/algorithms/similarity/filtered_node_similarity/request.rs)
  - `use crate::algo::similarity::NodeSimilarityMetric;`

- [gds/src/applications/algorithms/similarity/node_similarity/modes/stream.rs](../src/applications/algorithms/similarity/node_similarity/modes/stream.rs)
  - `use crate::algo::similarity::NodeSimilarityResult;`

- [gds/src/applications/algorithms/similarity/filtered_node_similarity/modes/stream.rs](../src/applications/algorithms/similarity/filtered_node_similarity/modes/stream.rs)
  - `use crate::algo::similarity::NodeSimilarityResult;`

- [gds/src/applications/algorithms/similarity/knn/modes/stream.rs](../src/applications/algorithms/similarity/knn/modes/stream.rs)
  - `use crate::algo::similarity::knn::KnnResultRow;`

- [gds/src/applications/algorithms/similarity/filtered_knn/modes/stream.rs](../src/applications/algorithms/similarity/filtered_knn/modes/stream.rs)
  - `use crate::algo::similarity::filtered_knn::FilteredKnnResultRow;`

---

## Recommended “Small Cleanup” Strategy (No Big Refactor)

If you decide to do this after lunch, the lowest-risk approach is:

1. **Re-export shared result/metric types via Procedures**
   - If Applications needs a metric enum or a result row type for rendering, it should import it from `crate::procedures::*` (or from an explicitly “facade types” module), not from `crate::algo::*`.

2. **Move result-shape types out of `algo`** (if needed)
   - Some of these look like pure DTOs (rows, metrics). Those can live in a shared `types`/`facade` module so both procedures and applications can depend on them without crossing the `algo` boundary.

3. **Keep “steps/*” files purely about orchestration**
   - The pathfinding “steps” should consume `procedures::traits::PathResult` (or a facade equivalent), not `algo::common::result_builders::PathResult`.

---

## Notes

- This scan did **not** find `panic!/todo!/unimplemented!` within `gds/src/applications/algorithms/**` via the basic search used here.
- There are known Application stubs elsewhere (notably Machinery convenience methods), but that’s tracked in [gds/doc/APPLICATIONS_CONTROL_LOGIC_INVENTORY.md](APPLICATIONS_CONTROL_LOGIC_INVENTORY.md).

