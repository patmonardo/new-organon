# Pathfinding Application Layer — Machinery Usage Review

Summary
- **Scope**: review of application facades and pathfinding builders with emphasis on use of the Machinery module (progress tracking + memory guards + templates).
- **Quick take**: Pathfinding *applications* (under `gds/src/applications/algorithms/pathfinding`) largely use the Machinery templates; many *procedural builders* (under `gds/src/procedures/pathfinding`) still call storage/computation directly (via `compute()` / `run()` / `stream()`), producing an inconsistent surface.

Findings
- **Two invocation styles**: application handlers use `AlgorithmProcessingTemplate` / `AlgorithmProcessingTemplateConvenience` (see [gds/src/applications/algorithms/machinery/algorithm_processing_template_convenience.rs](gds/src/applications/algorithms/machinery/algorithm_processing_template_convenience.rs)), while several builders/procedures call storage directly with `compute(...)` or expose `run()`/`stream()`/`stats()` (e.g. [gds/src/procedures/pathfinding/astar.rs](gds/src/procedures/pathfinding/astar.rs), [gds/src/procedures/embeddings/graphsage.rs](gds/src/procedures/embeddings/graphsage.rs)).
- **Memory guard placement**: `ComputationService` shows a pattern where a memory guard is asserted before calling a `Computation.compute(graph, store)` hook ([gds/src/applications/algorithms/machinery/computation_service.rs](gds/src/applications/algorithms/machinery/computation_service.rs)). This mixes the guard pattern with the older compute-signature style.
- **Pathfinding apps using Machinery**: many pathfinding application modes call `convenience.process_*(...)` (examples: A* modes, Dijkstra, Delta-stepping, BFS) — these handlers wire in `GraphResources`, trackers and termination flags consistently (see `gds/src/applications/algorithms/pathfinding/*/modes/*.rs`).
- **Procedural builders bypass Machinery**: builders frequently create progress trackers and memory estimates inline and call storage runtimes directly (A* builder creates its own `TaskProgressTracker` and calls `storage.compute_astar_path(...)`). That leads to duplication and subtle differences in tracker lifecycle and how memory is enforced.

Assessment (Health)
- **Coverage**: Application-side handlers are healthy — they predominantly integrate with the Machinery templates (good: progress lifecycle, render-model separation, side-effects hooks).
- **Fracture point**: Procedures/builders are the main inconsistency. Having two orchestration idioms increases maintenance burden and risks subtle bugs (double trackers, inconsistent per-target tracker behavior, differing memory-guard semantics).
- **Naming inconsistency**: `compute()` is used by many application facades and storage runtimes; `run()`/`stream()`/`stats()` are used by builders and higher-level helpers. This is not a correctness issue but makes it harder to reason about which surface applies machinery semantics.

Recommendations (practical, ordered)
- **Standardize facade entrypoints**: adopt a clear rule — application facades (the orchestrating layer) should expose `compute()` / `compute_*` when they are thin adapters, and *algorithm entrypoints* (modeful builders) should expose `process_*` or `execute_*` that delegate into the Machinery templates. Prefer keeping `compute()` for implementations that match the `Computation` trait (Graph + GraphStore) and prefer `process_*`/`execute` for Machinery-wrapped flows.
- **Migrate builders to Machinery incrementally**: for each builder (start with A*, Dijkstra): replace inline tracker/memory-guard + direct storage call with a small adapter that returns a closure matching the template compute signature (GraphResources, ProgressTracker, TerminationFlag). This yields:
  - single place for tracker lifecycle and memory guard enforcement (use templates). 
  - consistent per-target task handling (avoid ad-hoc fresh tracker creation unless necessary; document exceptions).
- **Preserve valid exceptions**: where a builder must create a fresh tracker per-target (A* does this to avoid restarting finished tasks), keep that behaviour but expose a comment and a small compatibility adapter so application-level templates can still call into it predictably.
- **Unify memory estimate surface**: require builders to implement `estimate_memory()` returning `MemoryRange` (many already do). Machinery templates should prefer calling procedure-provided estimates rather than duplicated heuristics.
- **Small naming cleanup**: add a short mapping doc in `gds/doc/GDS-API.md` describing canonical meanings: `compute()` (low-level compute hook), `process_*()` (Machinery-wrapped invocation), `run()/stream()/stats()` (builder convenience). Optionally rename run->execute in new code, but prefer a documented convention over mass renames.

Next steps (suggested quick wins)
- Convert one representative builder (A* or Dijkstra) into the Machinery closure pattern and adapt the application handler to call it through `AlgorithmProcessingTemplateConvenience`. Use that PR as the migration template.
- Add a short checklist in the repo docs: "Facade migration to Machinery" with minimal steps and code snippets.

References
- Machinery convenience template: [gds/src/applications/algorithms/machinery/algorithm_processing_template_convenience.rs](gds/src/applications/algorithms/machinery/algorithm_processing_template_convenience.rs)
- Computation service (memory guard example): [gds/src/applications/algorithms/machinery/computation_service.rs](gds/src/applications/algorithms/machinery/computation_service.rs)
- Example builder bypassing Machinery: [gds/src/procedures/pathfinding/astar.rs](gds/src/procedures/pathfinding/astar.rs)
- Example application mode using Machinery: `gds/src/applications/algorithms/pathfinding/*/modes/*.rs` (many files)

If you want, I'll prototype migrating `AStarBuilder::compute` to return a Machinery-compatible closure and update one application handler to call `process_stream` as a concrete example. Proceed?
